extern crate deepspeech;
extern crate audrey;

use std::path::Path;
use std::env::args;
use std::fs::File;

use deepspeech::Model;
use deepspeech::Metadata;
use audrey::read::Reader;
use audrey::sample::interpolate::{Converter, Linear};
use audrey::sample::signal::{from_iter, Signal};

// These constants are taken from the C++ sources of the client.

const BEAM_WIDTH :u16 = 500;

const LM_WEIGHT :f32 = 0.75;
const VALID_WORD_COUNT_WEIGHT :f32 = 1.85;

// The model has been trained on this specific
// sample rate.
const SAMPLE_RATE :u32 = 16_000;

fn metadata_to_string(m: Metadata) -> String {
	let mut s = String::new();
	for item in m.items() {
		s += item.character().unwrap();
	}
	s
}

/*
TODO list:
* better resampling (right now it seems that recognition is impaired compared to manual resampling)...
  maybe use sinc?
* channel cropping
* use clap or something to parse the command line arguments
*/
fn main() {
	let model_dir_str = args().nth(1)
		.expect("Please specify model dir");
	let audio_file_path = args().nth(2)
		.expect("Please specify an audio file to run STT on");
	let dir_path = Path::new(&model_dir_str);
	let mut m = Model::load_from_files(
		&dir_path.join("output_graph.pb"),
		&dir_path.join("alphabet.txt"),
		BEAM_WIDTH).unwrap();
	m.enable_decoder_with_lm(
		&dir_path.join("lm.binary"),
		&dir_path.join("trie"),
		LM_WEIGHT,
		VALID_WORD_COUNT_WEIGHT);

	let audio_file = File::open(audio_file_path).unwrap();
	let mut reader = Reader::new(audio_file).unwrap();
	let desc = reader.description();
	assert_eq!(1, desc.channel_count(),
		"The channel count is required to be one, at least for now");

	// Obtain the buffer of samples
	let audio_buf :Vec<_> = if desc.sample_rate() == SAMPLE_RATE {
		reader.samples().map(|s| s.unwrap()).collect()
	} else {
		// We need to interpolate to the target sample rate
		let interpolator = Linear::new([0i16], [0]);
		let conv = Converter::from_hz_to_hz(
			from_iter(reader.samples::<i16>().map(|s| [s.unwrap()])),
			interpolator,
			desc.sample_rate() as f64,
			SAMPLE_RATE as f64);
		conv.until_exhausted().map(|v| v[0]).collect()
	};

	// Run the speech to text algorithm
	let result = metadata_to_string(m.speech_to_text_with_metadata(&audio_buf).unwrap());

	// Output the result
	println!("{}", result);
}
