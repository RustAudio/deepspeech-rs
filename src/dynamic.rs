#![doc = "\nBindings to the DeepSpeech library\n"]
extern crate deepspeech_sys;
extern crate libc;
use deepspeech_sys as ds;
use std::ffi::CStr;
use std::fmt;
use std::mem::forget;
use std::ops::Drop;
use std::path::Path;
use std::ptr;
use std::slice;

pub struct Model {
    library: std::sync::Arc<ds::dynamic::LibraryWrapper>,
    model: *mut ds::dynamic::ModelState,
}
pub struct Metadata {
    library: std::sync::Arc<ds::dynamic::LibraryWrapper>,
    metadata: *mut ds::dynamic::Metadata,
}
#[repr(transparent)]
pub struct TokenMetadata {
    metadata_item: ds::TokenMetadata,
}
#[repr(transparent)]
pub struct CandidateTranscript {
    transcript_item: ds::CandidateTranscript,
}
fn path_to_buf(p: &Path) -> Vec<u8> {
    let s = p.to_str().unwrap();
    let mut v = Vec::with_capacity(s.len());
    v.extend_from_slice(s.as_bytes());
    v.push(0);
    v
}
impl Model {
    #[doc = " Load a DeepSpeech model from the specified model file path"]
    pub fn load_from_files(
        library_path: impl AsRef<std::ffi::OsStr>,
        model_path: &Path,
    ) -> Result<Self, ()> {
        let library = std::sync::Arc::new(
            ds::dynamic::LibraryWrapper::from_path(library_path).map_err(|_| ())?,
        );
        let mp = path_to_buf(model_path);
        let mut model = ptr::null_mut();
        let ret = unsafe {
            ds::dynamic::LibraryWrapper::DS_CreateModel(&library, mp.as_ptr() as _, &mut model)
                .unwrap()
        };
        if ret != 0 {
            return Err(());
        }
        Ok(Model { library, model })
    }
    #[doc = " Enable decoding using an external scorer"]
    pub fn enable_external_scorer(&mut self, scorer_path: &Path) {
        let sp = path_to_buf(scorer_path);
        unsafe {
            ds::dynamic::LibraryWrapper::DS_EnableExternalScorer(
                &self.library,
                self.model,
                sp.as_ptr() as _,
            )
            .unwrap();
        }
    }
    #[doc = " Disable decoding using an external scorer"]
    pub fn disable_external_scorer(&mut self) -> Result<(), ()> {
        let ret = unsafe {
            ds::dynamic::LibraryWrapper::DS_DisableExternalScorer(&self.library, self.model)
                .unwrap()
        };
        if ret != 0 {
            Err(())
        } else {
            Ok(())
        }
    }
    #[doc = " Get sample rate expected by a model"]
    pub fn get_sample_rate(&self) -> i32 {
        unsafe {
            ds::dynamic::LibraryWrapper::DS_GetModelSampleRate(&self.library, self.model).unwrap()
                as _
        }
    }
    #[doc = " Get beam width value the model is currently configured to use"]
    pub fn get_model_beam_width(&self) -> u16 {
        unsafe {
            ds::dynamic::LibraryWrapper::DS_GetModelBeamWidth(&self.library, self.model).unwrap()
                as _
        }
    }
    #[doc = " Set beam width value used by the model"]
    pub fn set_model_beam_width(&mut self, bw: u16) -> Result<(), ()> {
        let ret = unsafe {
            ds::dynamic::LibraryWrapper::DS_SetModelBeamWidth(&self.library, self.model, bw as _)
                .unwrap()
        };
        if ret != 0 {
            Err(())
        } else {
            Ok(())
        }
    }
    #[doc = " Set hyperparameters alpha and beta of the external scorer"]
    pub fn set_scorer_alpha_beta(&mut self, alpha: f32, beta: f32) -> Result<(), ()> {
        let ret = unsafe {
            ds::dynamic::LibraryWrapper::DS_SetScorerAlphaBeta(
                &self.library,
                self.model,
                alpha,
                beta,
            )
            .unwrap()
        };
        if ret != 0 {
            Err(())
        } else {
            Ok(())
        }
    }
    #[doc = " Perform speech-to-text using the model"]
    #[doc = ""]
    #[doc = " The input buffer must consist of mono 16-bit samples."]
    #[doc = " The sample rate is not freely chooseable but a property"]
    #[doc = " of the model files."]
    pub fn speech_to_text(&mut self, buffer: &[i16]) -> Result<String, std::string::FromUtf8Error> {
        let r = unsafe {
            let ptr = ds::dynamic::LibraryWrapper::DS_SpeechToText(
                &self.library,
                self.model,
                buffer.as_ptr(),
                buffer.len() as _,
            )
            .unwrap();
            let s = CStr::from_ptr(ptr);
            let mut v = Vec::new();
            v.extend_from_slice(s.to_bytes());
            ds::dynamic::LibraryWrapper::DS_FreeString(&self.library, ptr).unwrap();
            v
        };
        String::from_utf8(r)
    }
    #[doc = " Perform speech-to-text using the model, getting extended metadata"]
    #[doc = ""]
    #[doc = " The input buffer must consist of mono 16-bit samples."]
    #[doc = " The sample rate is not freely chooseable but a property"]
    #[doc = " of the model files."]
    #[doc = ""]
    #[doc = " The `num_transcripts` param contains the maximum number of"]
    #[doc = " `CandidateTranscript`s to return. The actually returned number"]
    #[doc = " might be smaller."]
    pub fn speech_to_text_with_metadata(
        &mut self,
        buffer: &[i16],
        num_transcripts: u16,
    ) -> Result<Metadata, ()> {
        let ptr = unsafe {
            ds::dynamic::LibraryWrapper::DS_SpeechToTextWithMetadata(
                &self.library,
                self.model,
                buffer.as_ptr(),
                buffer.len() as _,
                num_transcripts as _,
            )
            .unwrap()
        };
        Ok(Metadata {
            library: std::sync::Arc::clone(&self.library),
            metadata: ptr,
        })
    }
    #[doc = " Set up a state for streaming inference"]
    pub fn create_stream(&mut self) -> Result<Stream, ()> {
        let mut ptr = ptr::null_mut();
        let ret = unsafe {
            ds::dynamic::LibraryWrapper::DS_CreateStream(&self.library, self.model, &mut ptr)
                .unwrap()
        };
        if ret != 0 {
            return Err(());
        }
        Ok(Stream {
            library: std::sync::Arc::clone(&self.library),
            stream: ptr,
        })
    }
}
impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            ds::dynamic::LibraryWrapper::DS_FreeModel(&self.library, self.model).unwrap();
        }
    }
}
impl Drop for Metadata {
    fn drop(&mut self) {
        unsafe {
            ds::dynamic::LibraryWrapper::DS_FreeMetadata(&self.library, self.metadata).unwrap();
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
impl TokenMetadata {
    #[doc = " The text of the token generated for transcription"]
    pub fn text(&self) -> Result<&str, std::str::Utf8Error> {
        unsafe {
            let slice = CStr::from_ptr(self.metadata_item.text);
            slice.to_str()
        }
    }
    #[doc = " Position of the token in units of 20ms"]
    pub fn timestep(&self) -> u32 {
        self.metadata_item.timestep
    }
    #[doc = " Position of the token in seconds"]
    pub fn start_time(&self) -> f32 {
        self.metadata_item.start_time
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
pub struct Stream {
    library: std::sync::Arc<ds::dynamic::LibraryWrapper>,
    stream: *mut ds::dynamic::StreamingState,
}
impl Stream {
    #[doc = " Feed audio samples to the stream"]
    #[doc = ""]
    #[doc = " The input buffer must consist of mono 16-bit samples."]
    pub fn feed_audio(&mut self, buffer: &[i16]) {
        unsafe {
            ds::dynamic::LibraryWrapper::DS_FeedAudioContent(
                &self.library,
                self.stream,
                buffer.as_ptr(),
                buffer.len() as _,
            )
            .unwrap();
        }
    }
    #[doc = " Decodes the intermediate state of what has been spoken up until now"]
    #[doc = ""]
    #[doc = " Note that as of DeepSpeech version 0.2.0,"]
    #[doc = " this function is non-trivial as the decoder can't do streaming yet."]
    pub fn intermediate_decode(&mut self) -> Result<String, std::string::FromUtf8Error> {
        let r = unsafe {
            let ptr =
                ds::dynamic::LibraryWrapper::DS_IntermediateDecode(&self.library, self.stream)
                    .unwrap();
            let s = CStr::from_ptr(ptr);
            let mut v = Vec::new();
            v.extend_from_slice(s.to_bytes());
            ds::dynamic::LibraryWrapper::DS_FreeString(&self.library, ptr).unwrap();
            v
        };
        String::from_utf8(r)
    }
    #[doc = " Deallocates the stream and returns the decoded text"]
    pub fn finish(self) -> Result<String, std::string::FromUtf8Error> {
        let r = unsafe {
            let ptr =
                ds::dynamic::LibraryWrapper::DS_FinishStream(&self.library, self.stream).unwrap();
            let s = CStr::from_ptr(ptr);
            let mut v = Vec::new();
            v.extend_from_slice(s.to_bytes());
            ds::dynamic::LibraryWrapper::DS_FreeString(&self.library, ptr).unwrap();
            v
        };
        forget(self);
        String::from_utf8(r)
    }
    #[doc = " Deallocates the stream and returns the extended metadata"]
    #[doc = ""]
    #[doc = " The `num_transcripts` param contains the maximum number of"]
    #[doc = " `CandidateTranscript`s to return. The actually returned number"]
    #[doc = " might be smaller."]
    pub fn finish_with_metadata(self, num_transcripts: u32) -> Result<Metadata, ()> {
        let ptr = unsafe {
            ds::dynamic::LibraryWrapper::DS_FinishStreamWithMetadata(
                &self.library,
                self.stream,
                num_transcripts as _,
            )
            .unwrap()
        };
        let library = std::sync::Arc::clone(&self.library);
        unsafe {
            std::ptr::drop_in_place::<std::sync::Arc<libloading::Library>>(&self.library as *const _ as *mut _);
        };
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
            ds::dynamic::LibraryWrapper::DS_FreeStream(&self.library, self.stream).unwrap();
        }
    }
}

/*
TODO: How?
pub fn deepspeech_version() -> Result<String, std::string::FromUtf8Error> {
    let r = unsafe {
        let ptr = ds::dynamic::LibraryWrapper::DS_Version();
        let s = CStr::from_ptr(ptr);
        let mut v = Vec::new();
        v.extend_from_slice(s.to_bytes());
        ds::dynamic::LibraryWrapper::DS_FreeString(ptr);
        v
    };
    String::from_utf8(r)
}
*/