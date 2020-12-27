extern crate deepspeech;
extern crate audrey;

use std::path::Path;
use std::env::args;
use std::fs::File;

use deepspeech::Model;
use deepspeech::CandidateTranscript;
use audrey::read::Reader;
use audrey::sample::interpolate::{Converter, Linear};
use audrey::sample::signal::{from_iter, Signal};

// The model has been trained on this specific
// sample rate.
const SAMPLE_RATE :u32 = 16_000;

fn transcript_to_string(tr :&CandidateTranscript) -> String {
	let mut s = String::new();
	for token in tr.tokens() {
		s += token.text().unwrap();
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
	let mut graph_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
	let mut scorer_name: Option<Box<Path>> = None;
	// search for model in model directory
	for file in dir_path.read_dir().expect("Specified model dir is not a dir") {
		if let Ok(f) = file {
			let file_path = f.path();
			if file_path.is_file() {
				if let Some(ext) = file_path.extension() {
					if ext == "pb" || ext == "pbmm" {
						graph_name = file_path.into_boxed_path();
					} else if ext == "scorer" {
						scorer_name = Some(file_path.into_boxed_path());
					}
				}
			}
		}
	}
	let mut m = Model::load_from_files(&graph_name).unwrap();
	// enable external scorer if found in the model folder
	if let Some(scorer) = scorer_name {
		println!("Using external scorer `{}`", scorer.to_str().unwrap());
		m.enable_external_scorer(&scorer).unwrap();
	}

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
	let metadata = m.speech_to_text_with_metadata(&audio_buf, 1).unwrap();
	let result = transcript_to_string(&metadata.transcripts()[0]);

	// Output the result
	println!("{}", result);
}
