extern crate deepspeech;
extern crate audrey;
extern crate dasp_interpolate;
extern crate dasp_signal;

use std::path::Path;
use std::env::args;
use std::fs::File;
use std::time::Instant;

use deepspeech::Model;
use audrey::read::Reader;
use dasp_interpolate::linear::Linear;
use dasp_signal::{from_iter, Signal, interpolate::Converter};

// The model has been trained on this specific
// sample rate.
const SAMPLE_RATE :u32 = 16_000;

/*
TODO list:
* better resampling (right now it seems that recognition is impaired compared to manual resampling)...
  maybe use sinc?
* channel cropping
* use clap or something to parse the command line arguments
*/
fn main() {
	let start = Instant::now();
	let model_dir_str = args().nth(1)
		.expect("Please specify model dir");
	let audio_file_path = args().nth(2)
		.expect("Please specify an audio file to run STT on");
	let dir_path = Path::new(&model_dir_str);
	let mut graph_name: String = String::from("output_graph.pb");
	let mut scorer_name: Option<String> = None;
	// search for model in model directory
	for file in dir_path.read_dir().expect("Specified model dir is not a dir") {
		if let Ok(f) = file {
			if f.file_type().unwrap().is_file() {
				let file_name = f.file_name().into_string().unwrap();
				if file_name.ends_with(".pb") || file_name.ends_with(".pbmm") {
					graph_name = file_name;
				} else if file_name.ends_with(".scorer") {
					scorer_name = Some(file_name);
				}
			}
		}
	}
	let mut m = Model::load_from_files(&dir_path.join(&graph_name)).unwrap();
	// enable external scorer if found in the model folder
	if let Some(scorer) = scorer_name {
		println!("Using external scorer `{}`", scorer);
		m.enable_external_scorer(&dir_path.join(&scorer)).unwrap();
	}

	let initialized_time = Instant::now();
	println!("Model initialized in {:?}.", initialized_time - start);

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

	let len_seconds = audio_buf.len() as f64 / SAMPLE_RATE as f64;

	let decoded_time = Instant::now();

	println!("Decoding done in {:?}. Sample length {}s. Running STT.",
		decoded_time - initialized_time, len_seconds);

	// Run the speech to text algorithm
	let result = m.speech_to_text(&audio_buf).unwrap();

	let text_time = Instant::now();

	let elapsed = text_time - decoded_time;

	let elapsed_f = elapsed.subsec_micros() as f64 / 1_000_000.0
		+ elapsed.as_secs() as f64;
	println!("STT done in {:?}. Real time factor {:.5}", elapsed, elapsed_f / len_seconds);

	// Output the result
	println!("{}", result);

}
