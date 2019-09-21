/*!
Bindings to the DeepSpeech library
*/

use std::ffi::CStr;
use std::fmt;
use std::path::Path;
use std::ops::Drop;
use std::ptr;
use std::mem::forget;
use std::slice;
use deepspeech_sys as ds;

pub struct Model {
	model :* mut ds::ModelState,
}

pub struct Metadata {
	metadata :* mut ds::Metadata,
}

#[repr(transparent)]
pub struct MetadataItem {
	metadata_item :ds::MetadataItem,
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
	pub fn load_from_files(model_path :&Path, alphabet_path :&Path, beam_width :u16) -> Result<Self, ()> {
		let mp = path_to_buf(model_path);
		let ap = path_to_buf(alphabet_path);
		let mut model = ptr::null_mut();
		let ret = unsafe {
			ds::DS_CreateModel(
				mp.as_ptr() as _,
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
	pub fn enable_decoder_with_lm(&mut self, language_model_path :&Path,
			trie_path :&Path, weight :f32, valid_word_count_weight :f32) {
		let lp = path_to_buf(language_model_path);
		let tp = path_to_buf(trie_path);
		unsafe {
			ds::DS_EnableDecoderWithLM(
				self.model,
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
			ds::DS_FreeString(ptr);
			v
		};
		String::from_utf8(r)
	}

	/// Perform speech-to-text using the model, getting extended metadata
	///
	/// The input buffer must consist of mono 16-bit samples.
	/// The sample rate is not freely chooseable but a property
	/// of the model files.
	pub fn speech_to_text_with_metadata(&mut self, buffer :&[i16],
			sample_rate :u32) -> Result<Metadata, ()> {
		let ptr = unsafe {
			ds::DS_SpeechToTextWithMetadata(
				self.model,
				buffer.as_ptr(),
				buffer.len() as _,
				sample_rate as _)
		};
		Ok(Metadata {
			metadata : ptr
		})
	}

	/// Set up a state for streaming inference
	pub fn create_stream(&mut self, sample_rate :u32) -> Result<Stream, ()> {
		let mut ptr = ptr::null_mut();
		let ret = unsafe {
			ds::DS_CreateStream(
				self.model,
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
			ds::DS_FreeModel(self.model);
		}
	}
}

impl Drop for Metadata {
	fn drop(&mut self) {
		unsafe {
			ds::DS_FreeMetadata(self.metadata);
		}
	}
}

impl Metadata {
	pub fn items(&self) -> &[MetadataItem] {
		unsafe {
			let ptr = (*self.metadata).items as * const MetadataItem;
			slice::from_raw_parts(ptr, self.num_items() as usize)
		}
	}

	pub fn num_items(&self) -> i32 {
		unsafe {
			(*self.metadata).num_items
		}
	}

	pub fn probability(&self) -> f64 {
		unsafe {
			(*self.metadata).probability
		}
	}
}

impl MetadataItem {
	pub fn character(&self) -> Result<&str, std::str::Utf8Error> {
		unsafe {
			let slice = CStr::from_ptr(self.metadata_item.character);
			slice.to_str()
		}
	}

	pub fn timestep(&self) -> i32 {
		self.metadata_item.timestep
	}

	pub fn start_time(&self) -> f32 {
		self.metadata_item.start_time
	}
}

impl fmt::Display for Metadata {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();
		for item in self.items() {
			s += item.character().unwrap();
		}
		write!(f, "{}", s)
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

	/// Decodes the intermediate state of what has been spoken up until now
	///
	/// Note that as of DeepSpeech version 0.2.0,
	/// this function is non-trivial as the decoder can't do streaming yet.
	pub fn intermediate_decode(&mut self) -> Result<String, std::string::FromUtf8Error> {
		let r = unsafe {
			let ptr = ds::DS_IntermediateDecode(self.stream);
			let s = CStr::from_ptr(ptr);
			let mut v = Vec::new();
			v.extend_from_slice(s.to_bytes());
			ds::DS_FreeString(ptr);
			v
		};
		String::from_utf8(r)
	}

	/// Deallocates the stream and returns the decoded text
	pub fn finish(self) -> Result<String, std::string::FromUtf8Error> {
		let r = unsafe {
			let ptr = ds::DS_FinishStream(self.stream);
			let s = CStr::from_ptr(ptr);
			let mut v = Vec::new();
			v.extend_from_slice(s.to_bytes());
			ds::DS_FreeString(ptr);
			v
		};
		// Don't run the destructor for self,
		// as DS_FinishStream already does it for us
		forget(self);
		String::from_utf8(r)
	}

	/// Deallocates the stream and returns the extended metadata
	pub fn finish_with_metadata(self) -> Result<Metadata, ()> {
		let ptr = unsafe {
			ds::DS_FinishStreamWithMetadata(self.stream)
		};
		// Don't run the destructor for self,
		// as DS_FinishStream already does it for us
		forget(self);
		Ok(Metadata {
			metadata : ptr
		})
	}
}

impl Drop for Stream {
	fn drop(&mut self) {
		unsafe {
			ds::DS_FreeStream(self.stream);
		}
	}
}
