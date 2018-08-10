/*!
Bindings to the DeepSpeech library
*/

extern crate libc;
extern crate deepspeech_sys;

use std::ffi::CStr;
use std::path::Path;
use std::ops::Drop;
use std::ptr;
use libc::free;
use deepspeech_sys as ds;

pub struct Model {
	model :* mut ds::ModelState,
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
			alphabet_path :&Path, beam_width :u16) -> Result<Self, ()> {
		let mp = path_to_buf(model_path);
		let ap = path_to_buf(alphabet_path);
		let mut model = ptr::null_mut();
		let ret = unsafe {
			ds::DS_CreateModel(
				mp.as_ptr() as _,
				n_cep as _,
				n_context as _,
				ap.as_ptr() as _,
				beam_width as _,
				&mut model)
		};
		if ret != 0 {
			return Err(());
		}
		Ok(Model {
			model,
		})
	}

	/// Load a KenLM language model from a file and enable decoding using beam scoring
	pub fn enable_decoder_with_lm(&mut self, alphabet_path :&Path,
			language_model_path :&Path, trie_path :&Path,
			weight :f32,
			valid_word_count_weight :f32) {
		let ap = path_to_buf(alphabet_path);
		let lp = path_to_buf(language_model_path);
		let tp = path_to_buf(trie_path);
		unsafe {
			ds::DS_EnableDecoderWithLM(
				self.model,
				ap.as_ptr() as _,
				lp.as_ptr() as _,
				tp.as_ptr() as _,
				weight,
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
			let ptr = ds::DS_SpeechToText(
				self.model,
				buffer.as_ptr(),
				buffer.len() as _,
				sample_rate as _);
			let s = CStr::from_ptr(ptr);
			let mut v = Vec::new();
			v.extend_from_slice(s.to_bytes());
			free(ptr as _);
			v
		};
		String::from_utf8(r)
	}

	/// Set up a state for streaming inference
	pub fn setup_stream(&mut self, pre_alloc_frames :u32, sample_rate :u32) -> Result<Stream, ()> {
		let mut ptr = ptr::null_mut();
		let ret = unsafe {
			ds::DS_SetupStream(
				self.model,
				pre_alloc_frames as _,
				sample_rate as _,
				&mut ptr,
			)
		};
		if ret != 0 {
			return Err(());
		}
		Ok(Stream {
			stream : ptr
		})
	}
}

impl Drop for Model {
	fn drop(&mut self) {
		unsafe {
			ds::DS_DestroyModel(self.model);
		}
	}
}

pub struct Stream {
	stream :* mut ds::StreamingState,
}

impl Stream {
	/// Feed audio samples to the stream
	///
	/// The input buffer must consist of mono 16-bit samples.
	pub fn feed_audio(&mut self, buffer :&[i16]) {
		unsafe {
			ds::DS_FeedAudioContent(self.stream, buffer.as_ptr(), buffer.len() as _);
		}
	}

	/// Decodes the intermediate state of what has been said up until now
	///
	/// Note that as of DeepSpeech version 0.2.0,
	/// this function is non-trivial as the decoder can't do streaming yet.
	pub fn intermediate_decode(&mut self) -> Result<String, std::string::FromUtf8Error> {
		let r = unsafe {
			let ptr = ds::DS_IntermediateDecode(self.stream);
			let s = CStr::from_ptr(ptr);
			let mut v = Vec::new();
			v.extend_from_slice(s.to_bytes());
			free(ptr as _);
			v
		};
		String::from_utf8(r)
	}

	/// Deallocates the stream
	pub fn finish(mut self) -> Result<String, std::string::FromUtf8Error> {
		return self.finish_priv();
	}

	pub fn finish_priv(&mut self) -> Result<String, std::string::FromUtf8Error> {
		let r = unsafe {
			let ptr = ds::DS_FinishStream(self.stream);
			let s = CStr::from_ptr(ptr);
			let mut v = Vec::new();
			v.extend_from_slice(s.to_bytes());
			free(ptr as _);
			v
		};
		String::from_utf8(r)
	}
}

impl Drop for Stream {
	fn drop(&mut self) {
		self.finish_priv().unwrap();
	}
}
