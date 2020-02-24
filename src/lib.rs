/*!
Bindings to the DeepSpeech library
*/

extern crate libc;
extern crate deepspeech_sys;

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
	/// Load a DeepSpeech model from the specified model file path
	pub fn load_from_files(model_path :&Path) -> Result<Self, ()> {
		let mp = path_to_buf(model_path);
		let mut model = ptr::null_mut();
		let ret = unsafe {
			ds::DS_CreateModel(
				mp.as_ptr() as _,
				&mut model)
		};
		if ret != 0 {
			return Err(());
		}
		Ok(Model {
			model,
		})
	}

	/// Enable decoding using an external scorer
	pub fn enable_external_scorer(&mut self, scorer_path :&Path) {
		let sp = path_to_buf(scorer_path);
		unsafe {
			ds::DS_EnableExternalScorer(
				self.model,
				sp.as_ptr() as _);
		}
	}

	/// Disable decoding using an external scorer
	pub fn disable_external_scorer(&mut self) -> Result<(), ()> {
		let ret = unsafe {
			ds::DS_DisableExternalScorer(self.model)
		};
		if ret != 0 {
			Err(())
		} else {
			Ok(())
		}
	}

	/// Get sample rate expected by a model
	pub fn get_sample_rate(&self) -> i32 {
		unsafe {
			ds::DS_GetModelSampleRate(self.model) as _
		}
	}

	pub fn get_model_beam_width(&self) -> u16 {
		unsafe {
			ds::DS_GetModelBeamWidth(self.model) as _
		}
	}

	pub fn set_model_beam_width(&mut self, bw :u16) -> Result<(), ()> {
		let ret = unsafe {
			ds::DS_SetModelBeamWidth(self.model, bw as _)
		};
		if ret != 0 {
			Err(())
		} else {
			Ok(())
		}
	}

	/// Set hyperparameters alpha and beta of the external scorer
	pub fn set_scorer_alpha_beta(&mut self, alpha :f32, beta :f32) -> Result<(), ()> {
		let ret = unsafe {
			ds::DS_SetScorerAlphaBeta(self.model, alpha, beta)
		};
		if ret != 0 {
			Err(())
		} else {
			Ok(())
		}
	}

	/// Perform speech-to-text using the model
	///
	/// The input buffer must consist of mono 16-bit samples.
	/// The sample rate is not freely chooseable but a property
	/// of the model files.
	pub fn speech_to_text(&mut self, buffer :&[i16]) -> Result<String, std::string::FromUtf8Error> {
		let r = unsafe {
			let ptr = ds::DS_SpeechToText(
				self.model,
				buffer.as_ptr(),
				buffer.len() as _);
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
	pub fn speech_to_text_with_metadata(&mut self, buffer :&[i16]) -> Result<Metadata, ()> {
		let ptr = unsafe {
			ds::DS_SpeechToTextWithMetadata(
				self.model,
				buffer.as_ptr(),
				buffer.len() as _)
		};
		Ok(Metadata {
			metadata : ptr
		})
	}

	/// Set up a state for streaming inference
	pub fn create_stream(&mut self) -> Result<Stream, ()> {
		let mut ptr = ptr::null_mut();
		let ret = unsafe {
			ds::DS_CreateStream(
				self.model,
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

	pub fn confidence(&self) -> f64 {
		unsafe {
			(*self.metadata).confidence
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
