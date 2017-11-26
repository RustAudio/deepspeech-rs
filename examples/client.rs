extern crate deepspeech;
extern crate audrey;

use std::path::Path;
use std::env::args;
use std::fs::File;

use deepspeech::Model;
use audrey::read::Reader;

// These constants are taken from the C++ sources of the client.

const N_CEP :u16 = 26;
const N_CONTEXT :u16 = 9;
const BEAM_WIDTH :u16 = 500;

const LM_WEIGHT :f32 = 1.75;
const WORD_COUNT_WEIGHT :f32 = 1.0;
const VALID_WORD_COUNT_WEIGHT :f32 = 1.0;

/*
TODO list:
* resampling
* channel cropping
* use clap or something to parse the command line arguments
*/
fn main() {
	let model_dir_str = args().nth(1).unwrap();
	let audio_file_path = args().nth(2).unwrap();
	let dir_path = Path::new(&model_dir_str);
	let mut m = Model::load_from_files(
		&dir_path.join("output_graph.pb"),
		N_CEP,
		N_CONTEXT,
		&dir_path.join("alphabet.txt"),
		BEAM_WIDTH);
	m.enable_decoder_with_lm(
		&dir_path.join("alphabet.txt"),
		&dir_path.join("lm.binary"),
		&dir_path.join("trie"),
		LM_WEIGHT,
		WORD_COUNT_WEIGHT,
		VALID_WORD_COUNT_WEIGHT);

	let audio_file = File::open(audio_file_path).unwrap();
	let mut reader = Reader::new(audio_file).unwrap();
	let desc = reader.description();
	assert_eq!(1, desc.channel_count(),
		"The channel count is required to be one, at least for now");
	assert_eq!(16000, desc.sample_rate(),
		"The sample rate is required to be 16kHz, at least for now");
	let audio_buf = reader.samples().map(|s| s.unwrap()).collect::<Vec<_>>();
	let result = m.stt(
		&audio_buf,
		desc.sample_rate()).unwrap();
	println!("{}", result);
}
