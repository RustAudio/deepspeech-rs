/*!
Bindings to the DeepSpeech library
*/

extern crate libc;
extern crate deepspeech_sys;

use std::ffi::CStr;
use std::path::Path;
use std::ops::Drop;
use libc::free;
use deepspeech_sys::root::DeepSpeech as ds;

pub struct Model {
	model :ds::Model,
}

fn path_to_buf(p :&Path) -> Vec<u8> {
	let s = p.to_str().unwrap();
	let mut v = Vec::with_capacity(s.len());
	v.extend_from_slice(s.as_bytes());
	v.push(0);
	v
}

impl Model {
	/// Load a DeepSpeech model from the specified model and alphabet file paths
	pub fn load_from_files(model_path :&Path, n_cep :u16, n_context :u16,
			alphabet_path :&Path, beam_width :u16) -> Self {
		let mp = path_to_buf(model_path);
		let ap = path_to_buf(alphabet_path);
		let model = unsafe {
			ds::Model::new(
				mp.as_ptr() as _,
				n_cep as _,
				n_context as _,
				ap.as_ptr() as _,
				beam_width as _)
		};
		Model {
			model,
		}
	}

	/// Load a KenLM language model from a file and enable decoding using beam scoring
	pub fn enable_decoder_with_lm(&mut self, alphabet_path :&Path,
			language_model_path :&Path, trie_path :&Path,
			weight :f32, word_count_weight :f32,
			valid_word_count_weight :f32) {
		let ap = path_to_buf(alphabet_path);
		let lp = path_to_buf(language_model_path);
		let tp = path_to_buf(trie_path);
		unsafe {
			self.model.enableDecoderWithLM(
				ap.as_ptr() as _,
				lp.as_ptr() as _,
				tp.as_ptr() as _,
				weight,
				word_count_weight,
				valid_word_count_weight);
		}
	}

	/// Perform speech-to-text using the model
	///
	/// The input buffer must consist of mono 16-bit samples.
	/// The sample rate is not freely chooseable but a property
	/// of the model files.
	pub fn speech_to_text(&mut self, buffer :&[i16],
			sample_rate :u32) -> Result<String, std::string::FromUtf8Error> {
		let r = unsafe {
			let ptr = self.model.stt(buffer.as_ptr(),
				buffer.len() as _, sample_rate as _);
			let s = CStr::from_ptr(ptr);
			let mut v = Vec::new();
			v.extend_from_slice(s.to_bytes());
			free(ptr as _);
			v
		};
		String::from_utf8(r)
	}
}

impl Drop for Model {
	fn drop(&mut self) {
		unsafe {
			ds::Model_Model_destructor(&mut self.model);
		}
	}
}
