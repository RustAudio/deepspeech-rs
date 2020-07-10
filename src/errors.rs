#[cfg(feature = "static_bindings")]
use deepspeech_sys as ds;

#[cfg(all(feature = "dynamic", not(feature = "static_bindings")))]
use crate::dynamic_bindings as ds;

use self::ds::{
	DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_MODEL,
	DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_SESS,
	DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_STREAM, DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_MMAP,
	DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_SESS, DeepSpeech_Error_Codes_DS_ERR_FAIL_INTERPRETER,
	DeepSpeech_Error_Codes_DS_ERR_FAIL_READ_PROTOBUF, DeepSpeech_Error_Codes_DS_ERR_FAIL_RUN_SESS,
	DeepSpeech_Error_Codes_DS_ERR_INVALID_ALPHABET, DeepSpeech_Error_Codes_DS_ERR_INVALID_SCORER,
	DeepSpeech_Error_Codes_DS_ERR_INVALID_SHAPE, DeepSpeech_Error_Codes_DS_ERR_MODEL_INCOMPATIBLE,
	DeepSpeech_Error_Codes_DS_ERR_NO_MODEL, DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_LM,
	DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_TRIE,
	DeepSpeech_Error_Codes_DS_ERR_SCORER_NOT_ENABLED, DeepSpeech_Error_Codes_DS_ERR_SCORER_NO_TRIE,
	DeepSpeech_Error_Codes_DS_ERR_SCORER_UNREADABLE,
	DeepSpeech_Error_Codes_DS_ERR_SCORER_VERSION_MISMATCH,
};

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

/// An error code returned from `libdeepspeech` itself.
#[repr(u32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LibraryError {
	NoModel = DeepSpeech_Error_Codes_DS_ERR_NO_MODEL,
	InvalidAlphabet = DeepSpeech_Error_Codes_DS_ERR_INVALID_ALPHABET,
	InvalidShape = DeepSpeech_Error_Codes_DS_ERR_INVALID_SHAPE,
	InvalidScorer = DeepSpeech_Error_Codes_DS_ERR_INVALID_SCORER,
	ModelIncompatible = DeepSpeech_Error_Codes_DS_ERR_MODEL_INCOMPATIBLE,
	ScorerNotEnabled = DeepSpeech_Error_Codes_DS_ERR_SCORER_NOT_ENABLED,
	ScorerUnreadable = DeepSpeech_Error_Codes_DS_ERR_SCORER_UNREADABLE,
	ScorerInvalidLm = DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_LM,
	ScorerNoTrie = DeepSpeech_Error_Codes_DS_ERR_SCORER_NO_TRIE,
	ScorerInvalidTrie = DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_TRIE,
	ScorerVersionMismatch = DeepSpeech_Error_Codes_DS_ERR_SCORER_VERSION_MISMATCH,
	FailInitMmap = DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_MMAP,
	FailInitSess = DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_SESS,
	FailInterpreter = DeepSpeech_Error_Codes_DS_ERR_FAIL_INTERPRETER,
	FailRunSess = DeepSpeech_Error_Codes_DS_ERR_FAIL_RUN_SESS,
	FailCreateStream = DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_STREAM,
	FailReadProtobuf = DeepSpeech_Error_Codes_DS_ERR_FAIL_READ_PROTOBUF,
	FailCreateSess = DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_SESS,
	FailCreateModel = DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_MODEL,
}

impl LibraryError {
	/// Attempts to convert a raw error flag into a known error code.
	/// Returns `None` if `err_code` does not match a known error code value.
	pub fn from_code(err_code: ds::DeepSpeech_Error_Codes) -> Option<Self> {
		match err_code {
			ds::DeepSpeech_Error_Codes_DS_ERR_NO_MODEL => Some(LibraryError::NoModel),
			ds::DeepSpeech_Error_Codes_DS_ERR_INVALID_ALPHABET => {
				Some(LibraryError::InvalidAlphabet)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_INVALID_SHAPE => Some(LibraryError::InvalidShape),
			ds::DeepSpeech_Error_Codes_DS_ERR_INVALID_SCORER => Some(LibraryError::InvalidScorer),
			ds::DeepSpeech_Error_Codes_DS_ERR_MODEL_INCOMPATIBLE => {
				Some(LibraryError::ModelIncompatible)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_SCORER_NOT_ENABLED => {
				Some(LibraryError::ScorerNotEnabled)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_SCORER_UNREADABLE => {
				Some(LibraryError::ScorerUnreadable)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_LM => {
				Some(LibraryError::ScorerInvalidLm)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_SCORER_NO_TRIE => Some(LibraryError::ScorerNoTrie),
			ds::DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_TRIE => {
				Some(LibraryError::ScorerInvalidTrie)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_SCORER_VERSION_MISMATCH => {
				Some(LibraryError::ScorerVersionMismatch)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_MMAP => Some(LibraryError::FailInitMmap),
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_SESS => Some(LibraryError::FailInitSess),
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_INTERPRETER => {
				Some(LibraryError::FailInterpreter)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_RUN_SESS => Some(LibraryError::FailRunSess),
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_STREAM => {
				Some(LibraryError::FailCreateStream)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_READ_PROTOBUF => {
				Some(LibraryError::FailReadProtobuf)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_SESS => {
				Some(LibraryError::FailCreateSess)
			}
			ds::DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_MODEL => {
				Some(LibraryError::FailCreateModel)
			}
			_ => None,
		}
	}

	/// Converts this item into its error flag number.
	pub fn as_code(self) -> u32 {
		self as u32
	}
}

impl From<LibraryError> for u32 {
	fn from(outer: LibraryError) -> Self {
		outer.as_code()
	}
}

impl Display for LibraryError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		// Taken from the `DS_FOR_EACH_ERROR` macro in deepspeech/native_client/deepspeech.h
		let msg = match self {
			LibraryError::NoModel => "Missing model information.",
			LibraryError::InvalidAlphabet => {
				"Invalid alphabet embedded in model. (Data corruption?"
			}
			LibraryError::InvalidShape => "Invalid model shape.",
			LibraryError::InvalidScorer => "Invalid scorer file.",
			LibraryError::ModelIncompatible => "Incompatible model.",
			LibraryError::ScorerNotEnabled => "External scorer is not enabled.",
			LibraryError::ScorerUnreadable => "Could not read scorer file.",
			LibraryError::ScorerInvalidLm => "Could not recognize language model header in scorer.",
			LibraryError::ScorerNoTrie => {
				"Reached end of scorer file before loading vocabulary trie."
			}
			LibraryError::ScorerInvalidTrie => "Invalid magic in trie header.",
			LibraryError::ScorerVersionMismatch => {
				"Scorer file version does not match expected version."
			}
			LibraryError::FailInitMmap => "Failed to initialize memory mapped model.",
			LibraryError::FailInitSess => "Failed to initialize the session.",
			LibraryError::FailInterpreter => "Interpreter failed.",
			LibraryError::FailRunSess => "Failed to run the session.",
			LibraryError::FailCreateStream => "Error creating the stream.",
			LibraryError::FailReadProtobuf => "Error reading the proto buffer model file.",
			LibraryError::FailCreateSess => "Failed to create session.",
			LibraryError::FailCreateModel => "Could not allocate model state.",
		};
		f.write_str("DeepspeechError: ")?;
		f.write_str(msg)
	}
}

impl Error for LibraryError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum DeepspeechError {

	/// A library call returned an error code.
	LibraryError(LibraryError),
	
	/// A library call returned an error flag that cannot be matched to an error code.
	UnknownLibraryError(u32),

	/// The library returned a string that could not be parsed as UTF8.
	ParseError(Utf8Error),

	#[cfg(feature = "dynamic")]
	/// `libloading` returned an error attempting to use the dynamic library.
	///
	/// Only available with the `dynamic` feature.
	DynamicError(libloading::Error),
}

impl Display for DeepspeechError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::LibraryError(inner) => Display::fmt(inner, f),
			Self::ParseError(inner) => Display::fmt(inner, f),
			Self::UnknownLibraryError(inner) => write!(f, "Unknown library error code: {}", inner),
			#[cfg(feature = "dynamic")]
			Self::DynamicError(inner) => Display::fmt(inner, f),
		}
	}
}

impl Error for DeepspeechError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::LibraryError(inner) => Some(inner),
			Self::ParseError(inner) => Some(inner),
			Self::UnknownLibraryError(_) => None,
			#[cfg(feature = "dynamic")]
			Self::DynamicError(inner) => Some(inner),
		}
	}
}

impl From<Utf8Error> for DeepspeechError {
	fn from(inner: Utf8Error) -> Self {
		Self::ParseError(inner)
	}
}
impl From<std::string::FromUtf8Error> for DeepspeechError {
	fn from(inner: std::string::FromUtf8Error) -> Self {
		inner.utf8_error().into()
	}
}

impl From<u32> for DeepspeechError {
	fn from(code: u32) -> Self {
		match LibraryError::from_code(code) {
			Some(err) => Self::LibraryError(err),
			None => Self::UnknownLibraryError(code),
		}
	}
}
impl From<i32> for DeepspeechError {
	fn from(code: i32) -> Self {
		let code = code.abs();
		code.into()
	}
}

#[cfg(feature = "dynamic")]
impl From<libloading::Error> for DeepspeechError {
	fn from(inner: libloading::Error) -> Self {
		Self::DynamicError(inner)
	}
}
