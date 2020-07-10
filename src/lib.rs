/*!
Bindings to the DeepSpeech library
*/

#[cfg(feature = "dynamic")]
mod dynamic_bindings;

#[cfg(feature = "static_bindings")]
extern crate deepspeech_sys;
#[cfg(feature = "dynamic")]
extern crate libloading;

extern crate libc;

#[cfg(any(feature = "static_bindings", feature = "dynamic"))]
pub mod errors;
#[cfg(any(feature = "static_bindings", feature = "dynamic"))]
use errors::DeepspeechError;

fn path_to_buf(p: &std::path::Path) -> Vec<u8> {
	let s = p.to_str().unwrap();
	let mut v = Vec::with_capacity(s.len());
	v.extend_from_slice(s.as_bytes());
	v.push(0);
	v
}

macro_rules! impl_model {
	() => {
		pub struct Model {
			library: Library,
			model: *mut ds::ModelState,
		}
		impl Drop for Model {
			fn drop(&mut self) {
				unsafe {
					do_call!(&self.library, DS_FreeModel, self.model);
				}
			}
		}
		impl Model {
			/// Set hyperparameters alpha and beta of the external scorer
			pub fn set_scorer_alpha_beta(
				&mut self,
				alpha: f32,
				beta: f32,
			) -> Result<(), DeepspeechError> {
				let ret = unsafe {
					do_call_with_res!(
						&self.library,
						DS_SetScorerAlphaBeta,
						self.model,
						alpha,
						beta
					)
				};
				if ret != 0 {
					Err(ret.into())
				} else {
					Ok(())
				}
			}
			/// Enable decoding using an external scorer
			pub fn enable_external_scorer(&mut self, scorer_path: &Path) {
				let sp = path_to_buf(scorer_path);
				unsafe {
					do_call!(
						&self.library,
						DS_EnableExternalScorer,
						self.model,
						sp.as_ptr() as _
					);
				}
			}

			/// Get sample rate expected by a model
			pub fn get_sample_rate(&self) -> i32 {
				unsafe { do_call!(&self.library, DS_GetModelSampleRate, self.model) as _ }
			}

			/// Get beam width value the model is currently configured to use
			pub fn get_model_beam_width(&self) -> u16 {
				unsafe { do_call!(&self.library, DS_GetModelBeamWidth, self.model) as _ }
			}

			/// Set beam width value used by the model
			pub fn set_model_beam_width(&mut self, bw: u16) -> Result<(), DeepspeechError> {
				let ret = unsafe {
					do_call_with_res!(&self.library, DS_SetModelBeamWidth, self.model, bw as _)
				};
				if ret != 0 {
					Err(ret.into())
				} else {
					Ok(())
				}
			}

			/// Disable decoding using an external scorer
			pub fn disable_external_scorer(&mut self) -> Result<(), DeepspeechError> {
				let ret = unsafe {
					do_call_with_res!(&self.library, DS_DisableExternalScorer, self.model)
				};
				if ret != 0 {
					Err(ret.into())
				} else {
					Ok(())
				}
			}

			/// Perform speech-to-text using the model
			///
			/// The input buffer must consist of mono 16-bit samples.
			/// The sample rate is not freely chooseable but a property
			/// of the model files.
			pub fn speech_to_text(&mut self, buffer: &[i16]) -> Result<String, DeepspeechError> {
				let r = unsafe {
					let ptr = do_call!(
						&self.library,
						DS_SpeechToText,
						self.model,
						buffer.as_ptr(),
						buffer.len() as _
					);
					let s = CStr::from_ptr(ptr);
					let mut v = Vec::new();
					v.extend_from_slice(s.to_bytes());
					do_call!(&self.library, DS_FreeString, ptr);
					v
				};
				String::from_utf8(r).map_err(|e| e.into())
			}
			/// Perform speech-to-text using the model, getting extended metadata
			///
			/// The input buffer must consist of mono 16-bit samples.
			/// The sample rate is not freely chooseable but a property
			/// of the model files.
			///
			/// The `num_transcripts` param contains the maximum number of
			/// `CandidateTranscript`s to return. The actually returned number
			/// might be smaller.
			pub fn speech_to_text_with_metadata(
				&mut self,
				buffer: &[i16],
				num_transcripts: u16,
			) -> Result<Metadata, DeepspeechError> {
				let ptr = unsafe {
					do_call_with_res!(
						&self.library,
						DS_SpeechToTextWithMetadata,
						self.model,
						buffer.as_ptr(),
						buffer.len() as _,
						num_transcripts as _
					)
				};
				Ok(Metadata {
					library: self.library.clone(),
					metadata: ptr,
				})
			}

			/// Set up a state for streaming inference
			pub fn create_stream(&mut self) -> Result<Stream, DeepspeechError> {
				let mut ptr = ptr::null_mut();
				let ret = unsafe {
					do_call_with_res!(&self.library, DS_CreateStream, self.model, &mut ptr)
				};
				if ret != 0 {
					return Err(ret.into());
				}
				Ok(Stream {
					library: self.library.clone(),
					stream: ptr,
				})
			}
		}
	};
}

macro_rules! impl_stream {
	() => {
		pub struct Stream {
			library: Library,
			stream: *mut ds::StreamingState,
		}
		impl Stream {
			/// Feed audio samples to the stream
			///
			/// The input buffer must consist of mono 16-bit samples.
			pub fn feed_audio(&mut self, buffer: &[i16]) {
				unsafe {
					do_call!(
						&self.library,
						DS_FeedAudioContent,
						self.stream,
						buffer.as_ptr(),
						buffer.len() as _
					);
				}
			}

			/// Decodes the intermediate state of what has been spoken up until now
			///
			/// Note that as of DeepSpeech version 0.2.0,
			/// this function is non-trivial as the decoder can't do streaming yet.
			pub fn intermediate_decode(&mut self) -> Result<String, DeepspeechError> {
				let r = unsafe {
					let ptr = do_call!(&self.library, DS_IntermediateDecode, self.stream);
					let s = CStr::from_ptr(ptr);
					let mut v = Vec::new();
					v.extend_from_slice(s.to_bytes());
					do_call!(&self.library, DS_FreeString, ptr);
					v
				};
				String::from_utf8(r).map_err(|e| e.into())
			}

			/// Deallocates the stream and returns the decoded text
			pub fn finish(self) -> Result<String, DeepspeechError> {
				let r = unsafe {
					let ptr = do_call!(&self.library, DS_FinishStream, self.stream);
					let s = CStr::from_ptr(ptr);
					let mut v = Vec::new();
					v.extend_from_slice(s.to_bytes());
					do_call!(&self.library, DS_FreeString, ptr);
					v
				};
				// Don't run the destructor for self,
				// as DS_FinishStream already does it for us
				forget(self);
				String::from_utf8(r).map_err(|e| e.into())
			}

			/// Deallocates the stream and returns the extended metadata
			///
			/// The `num_transcripts` param contains the maximum number of
			/// `CandidateTranscript`s to return. The actually returned number
			/// might be smaller.
			pub fn finish_with_metadata(
				mut self,
				num_transcripts: u32,
			) -> Result<Metadata, DeepspeechError> {
				let ptr = unsafe {
					do_call_with_res!(
						&self.library,
						DS_FinishStreamWithMetadata,
						self.stream,
						num_transcripts as _
					)
				};
				let library = self.library.clone();
				unsafe {
					std::ptr::drop_in_place(&mut self.library);
				}
				// Don't run the destructor for self,
				// as DS_FinishStream already does it for us
				forget(self);
				Ok(Metadata {
					library,
					metadata: ptr,
				})
			}
		}

		impl Drop for Stream {
			fn drop(&mut self) {
				unsafe {
					do_call!(&self.library, DS_FreeStream, self.stream);
				}
			}
		}
	};
}

macro_rules! impl_metadata {
	() => {
		pub struct Metadata {
			library: Library,
			metadata: *mut ds::Metadata,
		}
		impl Drop for Metadata {
			fn drop(&mut self) {
				unsafe {
					do_call!(&self.library, DS_FreeMetadata, self.metadata);
				}
			}
		}

		impl Metadata {
			pub fn transcripts(&self) -> &[CandidateTranscript] {
				unsafe {
					let ptr = (*self.metadata).transcripts as *const CandidateTranscript;
					slice::from_raw_parts(ptr, self.num_transcripts() as usize)
				}
			}

			pub fn num_transcripts(&self) -> u32 {
				unsafe { (*self.metadata).num_transcripts }
			}
		}
	};
}

macro_rules! impl_token_metadata {
	() => {
		#[repr(transparent)]
		pub struct TokenMetadata {
			metadata_item: ds::TokenMetadata,
		}
		impl TokenMetadata {
			/// The text of the token generated for transcription
			pub fn text(&self) -> Result<&str, std::str::Utf8Error> {
				unsafe {
					let slice = CStr::from_ptr(self.metadata_item.text);
					slice.to_str()
				}
			}

			/// Position of the token in units of 20ms
			pub fn timestep(&self) -> u32 {
				self.metadata_item.timestep
			}

			/// Position of the token in seconds
			pub fn start_time(&self) -> f32 {
				self.metadata_item.start_time
			}
		}
	};
}

macro_rules! impl_candidate_transcript {
	() => {
		#[repr(transparent)]
		pub struct CandidateTranscript {
			transcript_item: ds::CandidateTranscript,
		}
		impl CandidateTranscript {
			pub fn tokens(&self) -> &[TokenMetadata] {
				unsafe {
					let ptr = self.transcript_item.tokens as *const TokenMetadata;
					slice::from_raw_parts(ptr, self.num_tokens() as usize)
				}
			}

			pub fn confidence(&self) -> f64 {
				self.transcript_item.confidence
			}

			pub fn num_tokens(&self) -> u32 {
				self.transcript_item.num_tokens
			}
		}

		impl fmt::Display for CandidateTranscript {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				let mut s = String::new();
				for token in self.tokens() {
					s += token.text().unwrap();
				}
				write!(f, "{}", s)
			}
		}
	};
}
macro_rules! impl_deepspeech_version {
	($($libname:ident, $library:path)?) => {
		pub fn deepspeech_version($($libname : $library),*) -> Result<String, DeepspeechError> {
			let r = unsafe {
				let ptr = do_call!( (( $($libname)* )), DS_Version, );
				let s = CStr::from_ptr(ptr);
				let mut v = Vec::new();
				v.extend_from_slice(s.to_bytes());
				do_call!((($($libname)*)), DS_FreeString, ptr);
				v
			};
			String::from_utf8(r).map_err(|e| e.into())
		}
	};
}

#[cfg(feature = "static_bindings")]
pub use self::static_bindings::*;
#[cfg(feature = "static_bindings")]
mod static_bindings {

	use deepspeech_sys as ds;
	use std::ffi::CStr;
	use std::fmt;
	use std::mem::forget;
	use std::ops::Drop;
	use std::path::Path;
	use std::ptr;
	use std::slice;

	use super::{path_to_buf, DeepspeechError};

	type Library = ();

	macro_rules! do_call {
		($this:expr, $f:ident, $($params:expr),*) => {{
			ds::$f($($params),*)
		}}
	}
	macro_rules! do_call_with_res {
		($this:expr, $f:ident, $($params:expr),*) => {{
			ds::$f($($params),*)
		}}

	}
	impl_model!();
	impl_metadata!();
	impl_stream!();
	impl_token_metadata!();
	impl_candidate_transcript!();
	impl_deepspeech_version!();

	impl Model {
		/// Load a DeepSpeech model from the specified model file path
		pub fn load_from_files(model_path: &Path) -> Result<Self, DeepspeechError> {
			let mp = path_to_buf(model_path);
			let mut model = ptr::null_mut();
			let ret = unsafe { ds::DS_CreateModel(mp.as_ptr() as _, &mut model) };
			if ret != 0 {
				return Err(ret.into());
			}
			Ok(Model { library: (), model })
		}
	}
}

#[cfg(feature = "dynamic")]
/// Wrappers that allow for using an arbitrary `libdeepspeech.so` file at runtime via the `libloading` crate.
pub mod dynamic {
	use super::{path_to_buf, DeepspeechError};
	use crate::dynamic_bindings as ds;
	pub use crate::dynamic_bindings::LibraryWrapper as Library;
	use std::ffi::CStr;
	use std::fmt;
	use std::mem::forget;
	use std::ops::Drop;
	use std::path::Path;
	use std::ptr;
	use std::slice;

	macro_rules! do_call {
		($this:expr, $f:ident, $($params:expr),*) => {{
			ds::LibraryWrapper::$f(&$this, $($params),*).unwrap()
		}}
	}
	macro_rules! do_call_with_res {
		($this:expr, $f:ident, $($params:expr),*) => {{
			ds::LibraryWrapper::$f(&$this, $($params),*)?
		}}

	}

	impl_model!();
	impl_stream!();
	impl_metadata!();
	impl_token_metadata!();
	impl_candidate_transcript!();
	impl_deepspeech_version!(library, Library);
	impl Model {
		pub fn library(&self) -> Library {
			self.library.clone()
		}

		/// Load a DeepSpeech model from the specified shared library and model file paths
		pub fn load_from_files(
			library_path: &Path,
			model_path: &Path,
		) -> Result<Self, DeepspeechError> {
			let library = ds::LibraryWrapper::from_path(library_path)?;
			let mp = path_to_buf(model_path);
			let mut model = ptr::null_mut();
			let ret = unsafe {
				do_call_with_res!(&library, DS_CreateModel, mp.as_ptr() as _, &mut model)
			};
			if ret != 0 {
				return Err(ret.into());
			}
			Ok(Model { library, model })
		}
	}
}
