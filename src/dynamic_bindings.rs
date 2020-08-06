pub type size_t = :: std :: os :: raw :: c_ulong ;
 # [repr ( C )] # [repr ( align ( 16 ) )] # [derive ( Debug , Copy , Clone )] pub struct max_align_t {
pub __clang_max_align_nonce1 : :: std :: os :: raw :: c_longlong , pub __bindgen_padding_0 : u64 , pub __clang_max_align_nonce2 : u128 ,
}
 # [test] fn bindgen_test_layout_max_align_t () {
assert_eq ! (:: std :: mem :: size_of ::< max_align_t > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( max_align_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of ::< max_align_t > ( ) , 16usize , concat ! ( "Alignment of " , stringify ! ( max_align_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< max_align_t > ( ) ) ) . __clang_max_align_nonce1 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( max_align_t ) , "::" , stringify ! ( __clang_max_align_nonce1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< max_align_t > ( ) ) ) . __clang_max_align_nonce2 as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( max_align_t ) , "::" , stringify ! ( __clang_max_align_nonce2 ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ModelState {
_unused : [u8 ; 0] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct StreamingState {
_unused : [u8 ; 0] ,
}
 # [doc = " @brief Stores text of an individual token, along with its timing information"] # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct TokenMetadata {
# [doc = " The text corresponding to this token"] pub text : * const :: std :: os :: raw :: c_char , # [doc = " Position of the token in units of 20ms"] pub timestep : :: std :: os :: raw :: c_uint , # [doc = " Position of the token in seconds"] pub start_time : f32 ,
}
 # [test] fn bindgen_test_layout_TokenMetadata () {
assert_eq ! (:: std :: mem :: size_of ::< TokenMetadata > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( TokenMetadata ) )) ;
 assert_eq ! (:: std :: mem :: align_of ::< TokenMetadata > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( TokenMetadata ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< TokenMetadata > ( ) ) ) . text as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( TokenMetadata ) , "::" , stringify ! ( text ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< TokenMetadata > ( ) ) ) . timestep as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( TokenMetadata ) , "::" , stringify ! ( timestep ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< TokenMetadata > ( ) ) ) . start_time as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( TokenMetadata ) , "::" , stringify ! ( start_time ) )) ;

}
 # [doc = " @brief A single transcript computed by the model, including a confidence"] # [doc = "        value and the metadata for its constituent tokens."] # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct CandidateTranscript {
# [doc = " Array of TokenMetadata objects"] pub tokens : * const TokenMetadata , # [doc = " Size of the tokens array"] pub num_tokens : :: std :: os :: raw :: c_uint , # [doc = " Approximated confidence value for this transcript. This is roughly the"] # [doc = " sum of the acoustic model logit values for each timestep/character that"] # [doc = " contributed to the creation of this transcript."] pub confidence : f64 ,
}
 # [test] fn bindgen_test_layout_CandidateTranscript () {
assert_eq ! (:: std :: mem :: size_of ::< CandidateTranscript > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( CandidateTranscript ) )) ;
 assert_eq ! (:: std :: mem :: align_of ::< CandidateTranscript > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( CandidateTranscript ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< CandidateTranscript > ( ) ) ) . tokens as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( CandidateTranscript ) , "::" , stringify ! ( tokens ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< CandidateTranscript > ( ) ) ) . num_tokens as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( CandidateTranscript ) , "::" , stringify ! ( num_tokens ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< CandidateTranscript > ( ) ) ) . confidence as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( CandidateTranscript ) , "::" , stringify ! ( confidence ) )) ;

}
 # [doc = " @brief An array of CandidateTranscript objects computed by the model."] # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct Metadata {
# [doc = " Array of CandidateTranscript objects"] pub transcripts : * const CandidateTranscript , # [doc = " Size of the transcripts array"] pub num_transcripts : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_Metadata () {
assert_eq ! (:: std :: mem :: size_of ::< Metadata > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( Metadata ) )) ;
 assert_eq ! (:: std :: mem :: align_of ::< Metadata > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( Metadata ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< Metadata > ( ) ) ) . transcripts as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( Metadata ) , "::" , stringify ! ( transcripts ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< Metadata > ( ) ) ) . num_transcripts as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( Metadata ) , "::" , stringify ! ( num_transcripts ) )) ;

}
 pub const STT_Error_Codes_STT_ERR_OK : STT_Error_Codes = 0 ;
 pub const STT_Error_Codes_STT_ERR_NO_MODEL : STT_Error_Codes = 4096 ;
 pub const STT_Error_Codes_STT_ERR_INVALID_ALPHABET : STT_Error_Codes = 8192 ;
 pub const STT_Error_Codes_STT_ERR_INVALID_SHAPE : STT_Error_Codes = 8193 ;
 pub const STT_Error_Codes_STT_ERR_INVALID_SCORER : STT_Error_Codes = 8194 ;
 pub const STT_Error_Codes_STT_ERR_MODEL_INCOMPATIBLE : STT_Error_Codes = 8195 ;
 pub const STT_Error_Codes_STT_ERR_SCORER_NOT_ENABLED : STT_Error_Codes = 8196 ;
 pub const STT_Error_Codes_STT_ERR_SCORER_UNREADABLE : STT_Error_Codes = 8197 ;
 pub const STT_Error_Codes_STT_ERR_SCORER_INVALID_LM : STT_Error_Codes = 8198 ;
 pub const STT_Error_Codes_STT_ERR_SCORER_NO_TRIE : STT_Error_Codes = 8199 ;
 pub const STT_Error_Codes_STT_ERR_SCORER_INVALID_TRIE : STT_Error_Codes = 8200 ;
 pub const STT_Error_Codes_STT_ERR_SCORER_VERSION_MISMATCH : STT_Error_Codes = 8201 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_INIT_MMAP : STT_Error_Codes = 12288 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_INIT_SESS : STT_Error_Codes = 12289 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_INTERPRETER : STT_Error_Codes = 12290 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_RUN_SESS : STT_Error_Codes = 12291 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_CREATE_STREAM : STT_Error_Codes = 12292 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_READ_PROTOBUF : STT_Error_Codes = 12293 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_CREATE_SESS : STT_Error_Codes = 12294 ;
 pub const STT_Error_Codes_STT_ERR_FAIL_CREATE_MODEL : STT_Error_Codes = 12295 ;
 pub type STT_Error_Codes = u32 ;
 # [derive ( Clone )] pub struct LibraryWrapper {
inner : std :: sync :: Arc < libloading :: Library > ,
}
 impl LibraryWrapper {
pub fn from_path (path : impl AsRef < std :: ffi :: OsStr >) -> Result < Self , libloading :: Error > {
let inner = std :: sync :: Arc :: new (libloading :: Library :: new ( & path ) ?) ;
 Ok (Self { inner })
}
 pub unsafe fn STT_ErrorCodeToErrorMessage (& self , aErrorCode : :: std :: os :: raw :: c_int ,) -> Result < * mut :: std :: os :: raw :: c_char , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (:: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char > (b"STT_ErrorCodeToErrorMessage\0") ? ;
 Ok (dyn_symbol ( aErrorCode ))
}
 pub unsafe fn STT_Version (& self ,) -> Result < * mut :: std :: os :: raw :: c_char , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn () -> * mut :: std :: os :: raw :: c_char > (b"STT_Version\0") ? ;
 Ok (dyn_symbol ( ))
}
 pub unsafe fn STT_FreeString (& self , str_ : * mut :: std :: os :: raw :: c_char) -> Result < () , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut :: std :: os :: raw :: c_char) -> () > (b"STT_FreeString\0") ? ;
 Ok (dyn_symbol ( str_ ))
}
 pub unsafe fn STT_FreeMetadata (& self , m : * mut Metadata) -> Result < () , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut Metadata) -> () > (b"STT_FreeMetadata\0") ? ;
 Ok (dyn_symbol ( m ))
}
 pub unsafe fn STT_FreeStream (& self , aSctx : * mut StreamingState) -> Result < () , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut StreamingState) -> () > (b"STT_FreeStream\0") ? ;
 Ok (dyn_symbol ( aSctx ))
}
 pub unsafe fn STT_FinishStreamWithMetadata (& self , aSctx : * mut StreamingState , aNumResults : :: std :: os :: raw :: c_uint ,) -> Result < * mut Metadata , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut StreamingState , :: std :: os :: raw :: c_uint) -> * mut Metadata > (b"STT_FinishStreamWithMetadata\0") ? ;
 Ok (dyn_symbol ( aSctx , aNumResults ))
}
 pub unsafe fn STT_FinishStream (& self , aSctx : * mut StreamingState) -> Result < * mut :: std :: os :: raw :: c_char , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut StreamingState) -> * mut :: std :: os :: raw :: c_char > (b"STT_FinishStream\0") ? ;
 Ok (dyn_symbol ( aSctx ))
}
 pub unsafe fn STT_IntermediateDecodeWithMetadata (& self , aSctx : * const StreamingState , aNumResults : :: std :: os :: raw :: c_uint ,) -> Result < * mut Metadata , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* const StreamingState , :: std :: os :: raw :: c_uint) -> * mut Metadata > (b"STT_IntermediateDecodeWithMetadata\0") ? ;
 Ok (dyn_symbol ( aSctx , aNumResults ))
}
 pub unsafe fn STT_IntermediateDecode (& self , aSctx : * const StreamingState) -> Result < * mut :: std :: os :: raw :: c_char , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* const StreamingState) -> * mut :: std :: os :: raw :: c_char > (b"STT_IntermediateDecode\0") ? ;
 Ok (dyn_symbol ( aSctx ))
}
 pub unsafe fn STT_FeedAudioContent (& self , aSctx : * mut StreamingState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint ,) -> Result < () , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut StreamingState , * const :: std :: os :: raw :: c_short , :: std :: os :: raw :: c_uint) -> () > (b"STT_FeedAudioContent\0") ? ;
 Ok (dyn_symbol ( aSctx , aBuffer , aBufferSize ))
}
 pub unsafe fn STT_CreateStream (& self , aCtx : * mut ModelState , retval : * mut * mut StreamingState ,) -> Result < :: std :: os :: raw :: c_int , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState , * mut * mut StreamingState) -> :: std :: os :: raw :: c_int > (b"STT_CreateStream\0") ? ;
 Ok (dyn_symbol ( aCtx , retval ))
}
 pub unsafe fn STT_SpeechToTextWithMetadata (& self , aCtx : * mut ModelState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint , aNumResults : :: std :: os :: raw :: c_uint ,) -> Result < * mut Metadata , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState , * const :: std :: os :: raw :: c_short , :: std :: os :: raw :: c_uint , :: std :: os :: raw :: c_uint) -> * mut Metadata > (b"STT_SpeechToTextWithMetadata\0") ? ;
 Ok (dyn_symbol ( aCtx , aBuffer , aBufferSize , aNumResults ))
}
 pub unsafe fn STT_SpeechToText (& self , aCtx : * mut ModelState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint ,) -> Result < * mut :: std :: os :: raw :: c_char , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState , * const :: std :: os :: raw :: c_short , :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_char > (b"STT_SpeechToText\0") ? ;
 Ok (dyn_symbol ( aCtx , aBuffer , aBufferSize ))
}
 pub unsafe fn STT_SetScorerAlphaBeta (& self , aCtx : * mut ModelState , aAlpha : f32 , aBeta : f32 ,) -> Result < :: std :: os :: raw :: c_int , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState , f32 , f32) -> :: std :: os :: raw :: c_int > (b"STT_SetScorerAlphaBeta\0") ? ;
 Ok (dyn_symbol ( aCtx , aAlpha , aBeta ))
}
 pub unsafe fn STT_DisableExternalScorer (& self , aCtx : * mut ModelState) -> Result < :: std :: os :: raw :: c_int , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState) -> :: std :: os :: raw :: c_int > (b"STT_DisableExternalScorer\0") ? ;
 Ok (dyn_symbol ( aCtx ))
}
 pub unsafe fn STT_EnableExternalScorer (& self , aCtx : * mut ModelState , aScorerPath : * const :: std :: os :: raw :: c_char ,) -> Result < :: std :: os :: raw :: c_int , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState , * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int > (b"STT_EnableExternalScorer\0") ? ;
 Ok (dyn_symbol ( aCtx , aScorerPath ))
}
 pub unsafe fn STT_FreeModel (& self , ctx : * mut ModelState) -> Result < () , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState) -> () > (b"STT_FreeModel\0") ? ;
 Ok (dyn_symbol ( ctx ))
}
 pub unsafe fn STT_GetModelSampleRate (& self , aCtx : * const ModelState) -> Result < :: std :: os :: raw :: c_int , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* const ModelState) -> :: std :: os :: raw :: c_int > (b"STT_GetModelSampleRate\0") ? ;
 Ok (dyn_symbol ( aCtx ))
}
 pub unsafe fn STT_SetModelBeamWidth (& self , aCtx : * mut ModelState , aBeamWidth : :: std :: os :: raw :: c_uint ,) -> Result < :: std :: os :: raw :: c_int , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* mut ModelState , :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int > (b"STT_SetModelBeamWidth\0") ? ;
 Ok (dyn_symbol ( aCtx , aBeamWidth ))
}
 pub unsafe fn STT_GetModelBeamWidth (& self , aCtx : * const ModelState) -> Result < :: std :: os :: raw :: c_uint , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* const ModelState) -> :: std :: os :: raw :: c_uint > (b"STT_GetModelBeamWidth\0") ? ;
 Ok (dyn_symbol ( aCtx ))
}
 pub unsafe fn STT_CreateModel (& self , aModelPath : * const :: std :: os :: raw :: c_char , retval : * mut * mut ModelState ,) -> Result < :: std :: os :: raw :: c_int , libloading :: Error > {
let dyn_symbol = self . inner . get :: < unsafe extern fn (* const :: std :: os :: raw :: c_char , * mut * mut ModelState) -> :: std :: os :: raw :: c_int > (b"STT_CreateModel\0") ? ;
 Ok (dyn_symbol ( aModelPath , retval ))
}

}
