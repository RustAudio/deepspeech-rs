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
 # [doc = " @brief Stores each individual character, along with its timing information"] # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct MetadataItem {
# [doc = " The character generated for transcription"] pub character : * mut :: std :: os :: raw :: c_char , # [doc = " Position of the character in units of 20ms"] pub timestep : :: std :: os :: raw :: c_int , # [doc = " Position of the character in seconds"] pub start_time : f32 ,
}
 # [test] fn bindgen_test_layout_MetadataItem () {
assert_eq ! (:: std :: mem :: size_of ::< MetadataItem > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( MetadataItem ) )) ;
 assert_eq ! (:: std :: mem :: align_of ::< MetadataItem > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( MetadataItem ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< MetadataItem > ( ) ) ) . character as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( MetadataItem ) , "::" , stringify ! ( character ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< MetadataItem > ( ) ) ) . timestep as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( MetadataItem ) , "::" , stringify ! ( timestep ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< MetadataItem > ( ) ) ) . start_time as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( MetadataItem ) , "::" , stringify ! ( start_time ) )) ;

}
 # [doc = " @brief Stores the entire CTC output as an array of character metadata objects"] # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct Metadata {
# [doc = " List of items"] pub items : * mut MetadataItem , # [doc = " Size of the list of items"] pub num_items : :: std :: os :: raw :: c_int , # [doc = " Approximated confidence value for this transcription. This is roughly the"] # [doc = " sum of the acoustic model logit values for each timestep/character that"] # [doc = " contributed to the creation of this transcription."] pub confidence : f64 ,
}
 # [test] fn bindgen_test_layout_Metadata () {
assert_eq ! (:: std :: mem :: size_of ::< Metadata > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( Metadata ) )) ;
 assert_eq ! (:: std :: mem :: align_of ::< Metadata > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( Metadata ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< Metadata > ( ) ) ) . items as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( Metadata ) , "::" , stringify ! ( items ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< Metadata > ( ) ) ) . num_items as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( Metadata ) , "::" , stringify ! ( num_items ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null ::< Metadata > ( ) ) ) . confidence as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( Metadata ) , "::" , stringify ! ( confidence ) )) ;

}
 pub const DeepSpeech_Error_Codes_DS_ERR_OK : DeepSpeech_Error_Codes = 0 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_NO_MODEL : DeepSpeech_Error_Codes = 4096 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_INVALID_ALPHABET : DeepSpeech_Error_Codes = 8192 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_INVALID_SHAPE : DeepSpeech_Error_Codes = 8193 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_INVALID_SCORER : DeepSpeech_Error_Codes = 8194 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_MODEL_INCOMPATIBLE : DeepSpeech_Error_Codes = 8195 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_SCORER_NOT_ENABLED : DeepSpeech_Error_Codes = 8196 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_MMAP : DeepSpeech_Error_Codes = 12288 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_SESS : DeepSpeech_Error_Codes = 12289 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_INTERPRETER : DeepSpeech_Error_Codes = 12290 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_RUN_SESS : DeepSpeech_Error_Codes = 12291 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_STREAM : DeepSpeech_Error_Codes = 12292 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_READ_PROTOBUF : DeepSpeech_Error_Codes = 12293 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_SESS : DeepSpeech_Error_Codes = 12294 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_MODEL : DeepSpeech_Error_Codes = 12295 ;
 pub type DeepSpeech_Error_Codes = u32 ;
 extern "C" {
# [doc = " @brief An object providing an interface to a trained DeepSpeech model."] # [doc = ""] # [doc = " @param aModelPath The path to the frozen model graph."] # [doc = " @param[out] retval a ModelState pointer"] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_CreateModel (aModelPath : * const :: std :: os :: raw :: c_char , retval : * mut * mut ModelState ,) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [doc = " @brief Get beam width value used by the model. If {@link DS_SetModelBeamWidth}"] # [doc = "        was not called before, will return the default value loaded from the"] # [doc = "        model file."] # [doc = ""] # [doc = " @param aCtx A ModelState pointer created with {@link DS_CreateModel}."] # [doc = ""] # [doc = " @return Beam width value used by the model."] pub fn DS_GetModelBeamWidth (aCtx : * mut ModelState) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [doc = " @brief Set beam width value used by the model."] # [doc = ""] # [doc = " @param aCtx A ModelState pointer created with {@link DS_CreateModel}."] # [doc = " @param aBeamWidth The beam width used by the model. A larger beam width value"] # [doc = "                   generates better results at the cost of decoding time."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_SetModelBeamWidth (aCtx : * mut ModelState , aBeamWidth : :: std :: os :: raw :: c_uint ,) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [doc = " @brief Return the sample rate expected by a model."] # [doc = ""] # [doc = " @param aCtx A ModelState pointer created with {@link DS_CreateModel}."] # [doc = ""] # [doc = " @return Sample rate expected by the model for its input."] pub fn DS_GetModelSampleRate (aCtx : * mut ModelState) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [doc = " @brief Frees associated resources and destroys model object."] pub fn DS_FreeModel (ctx : * mut ModelState) ;

}
 extern "C" {
# [doc = " @brief Enable decoding using an external scorer."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = " @param aScorerPath The path to the external scorer file."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure (invalid arguments)."] pub fn DS_EnableExternalScorer (aCtx : * mut ModelState , aScorerPath : * const :: std :: os :: raw :: c_char ,) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [doc = " @brief Disable decoding using an external scorer."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_DisableExternalScorer (aCtx : * mut ModelState) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [doc = " @brief Set hyperparameters alpha and beta of the external scorer."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = " @param aAlpha The alpha hyperparameter of the decoder. Language model weight."] # [doc = " @param aLMBeta The beta hyperparameter of the decoder. Word insertion weight."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_SetScorerAlphaBeta (aCtx : * mut ModelState , aAlpha : f32 , aBeta : f32 ,) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [doc = " @brief Use the DeepSpeech model to perform Speech-To-Text."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model to use."] # [doc = " @param aBuffer A 16-bit, mono raw audio signal at the appropriate"] # [doc = "                sample rate (matching what the model was trained on)."] # [doc = " @param aBufferSize The number of samples in the audio signal."] # [doc = ""] # [doc = " @return The STT result. The user is responsible for freeing the string using"] # [doc = "         {@link DS_FreeString()}. Returns NULL on error."] pub fn DS_SpeechToText (aCtx : * mut ModelState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint ,) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [doc = " @brief Use the DeepSpeech model to perform Speech-To-Text and output metadata"] # [doc = " about the results."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model to use."] # [doc = " @param aBuffer A 16-bit, mono raw audio signal at the appropriate"] # [doc = "                sample rate (matching what the model was trained on)."] # [doc = " @param aBufferSize The number of samples in the audio signal."] # [doc = ""] # [doc = " @return Outputs a struct of individual letters along with their timing information."] # [doc = "         The user is responsible for freeing Metadata by calling {@link DS_FreeMetadata()}. Returns NULL on error."] pub fn DS_SpeechToTextWithMetadata (aCtx : * mut ModelState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint ,) -> * mut Metadata ;

}
 extern "C" {
# [doc = " @brief Create a new streaming inference state. The streaming state returned"] # [doc = "        by this function can then be passed to {@link DS_FeedAudioContent()}"] # [doc = "        and {@link DS_FinishStream()}."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model to use."] # [doc = " @param[out] retval an opaque pointer that represents the streaming state. Can"] # [doc = "                    be NULL if an error occurs."] # [doc = ""] # [doc = " @return Zero for success, non-zero on failure."] pub fn DS_CreateStream (aCtx : * mut ModelState , retval : * mut * mut StreamingState ,) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [doc = " @brief Feed audio samples to an ongoing streaming inference."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = " @param aBuffer An array of 16-bit, mono raw audio samples at the"] # [doc = "                appropriate sample rate (matching what the model was trained on)."] # [doc = " @param aBufferSize The number of samples in @p aBuffer."] pub fn DS_FeedAudioContent (aSctx : * mut StreamingState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint ,) ;

}
 extern "C" {
# [doc = " @brief Compute the intermediate decoding of an ongoing streaming inference."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = ""] # [doc = " @return The STT intermediate result. The user is responsible for freeing the"] # [doc = "         string using {@link DS_FreeString()}."] pub fn DS_IntermediateDecode (aSctx : * mut StreamingState) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [doc = " @brief Signal the end of an audio signal to an ongoing streaming"] # [doc = "        inference, returns the STT result over the whole audio signal."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = ""] # [doc = " @return The STT result. The user is responsible for freeing the string using"] # [doc = "         {@link DS_FreeString()}."] # [doc = ""] # [doc = " @note This method will free the state pointer (@p aSctx)."] pub fn DS_FinishStream (aSctx : * mut StreamingState) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [doc = " @brief Signal the end of an audio signal to an ongoing streaming"] # [doc = "        inference, returns per-letter metadata."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = ""] # [doc = " @return Outputs a struct of individual letters along with their timing information."] # [doc = "         The user is responsible for freeing Metadata by calling {@link DS_FreeMetadata()}. Returns NULL on error."] # [doc = ""] # [doc = " @note This method will free the state pointer (@p aSctx)."] pub fn DS_FinishStreamWithMetadata (aSctx : * mut StreamingState) -> * mut Metadata ;

}
 extern "C" {
# [doc = " @brief Destroy a streaming state without decoding the computed logits. This"] # [doc = "        can be used if you no longer need the result of an ongoing streaming"] # [doc = "        inference and don't want to perform a costly decode operation."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = ""] # [doc = " @note This method will free the state pointer (@p aSctx)."] pub fn DS_FreeStream (aSctx : * mut StreamingState) ;

}
 extern "C" {
# [doc = " @brief Free memory allocated for metadata information."] pub fn DS_FreeMetadata (m : * mut Metadata) ;

}
 extern "C" {
# [doc = " @brief Free a char* string returned by the DeepSpeech API."] pub fn DS_FreeString (str : * mut :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [doc = " @brief Return version of this library. The returned version is a semantic version"] # [doc = "        (SemVer 2.0.0). The string returned must be freed with {@link DS_FreeString()}."] # [doc = ""] # [doc = " @return The version string."] pub fn DS_Version () -> * mut :: std :: os :: raw :: c_char ;

}
