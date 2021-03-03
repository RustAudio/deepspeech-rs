pub type size_t = :: std :: os :: raw :: c_ulong ;
 # [repr (C)] # [repr (align (16))] # [derive (Debug , Copy , Clone)] pub struct max_align_t {
pub __clang_max_align_nonce1 : :: std :: os :: raw :: c_longlong , pub __bindgen_padding_0 : u64 , pub __clang_max_align_nonce2 : u128 ,
}
 # [test] fn bindgen_test_layout_max_align_t () {
assert_eq ! (:: std :: mem :: size_of ::< max_align_t > () , 32usize , concat ! ("Size of: " , stringify ! (max_align_t))) ;
 assert_eq ! (:: std :: mem :: align_of ::< max_align_t > () , 16usize , concat ! ("Alignment of " , stringify ! (max_align_t))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< max_align_t > ())) . __clang_max_align_nonce1 as * const _ as usize } , 0usize , concat ! ("Offset of field: " , stringify ! (max_align_t) , "::" , stringify ! (__clang_max_align_nonce1))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< max_align_t > ())) . __clang_max_align_nonce2 as * const _ as usize } , 16usize , concat ! ("Offset of field: " , stringify ! (max_align_t) , "::" , stringify ! (__clang_max_align_nonce2))) ;

}
 # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct ModelState {
_unused : [u8 ; 0] ,
}
 # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct StreamingState {
_unused : [u8 ; 0] ,
}
 # [doc = " @brief Stores text of an individual token, along with its timing information"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct TokenMetadata {
# [doc = " The text corresponding to this token"] pub text : * const :: std :: os :: raw :: c_char , # [doc = " Position of the token in units of 20ms"] pub timestep : :: std :: os :: raw :: c_uint , # [doc = " Position of the token in seconds"] pub start_time : f32 ,
}
 # [test] fn bindgen_test_layout_TokenMetadata () {
assert_eq ! (:: std :: mem :: size_of ::< TokenMetadata > () , 16usize , concat ! ("Size of: " , stringify ! (TokenMetadata))) ;
 assert_eq ! (:: std :: mem :: align_of ::< TokenMetadata > () , 8usize , concat ! ("Alignment of " , stringify ! (TokenMetadata))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< TokenMetadata > ())) . text as * const _ as usize } , 0usize , concat ! ("Offset of field: " , stringify ! (TokenMetadata) , "::" , stringify ! (text))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< TokenMetadata > ())) . timestep as * const _ as usize } , 8usize , concat ! ("Offset of field: " , stringify ! (TokenMetadata) , "::" , stringify ! (timestep))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< TokenMetadata > ())) . start_time as * const _ as usize } , 12usize , concat ! ("Offset of field: " , stringify ! (TokenMetadata) , "::" , stringify ! (start_time))) ;

}
 # [doc = " @brief A single transcript computed by the model, including a confidence"] # [doc = "        value and the metadata for its constituent tokens."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct CandidateTranscript {
# [doc = " Array of TokenMetadata objects"] pub tokens : * const TokenMetadata , # [doc = " Size of the tokens array"] pub num_tokens : :: std :: os :: raw :: c_uint , # [doc = " Approximated confidence value for this transcript. This is roughly the"] # [doc = " sum of the acoustic model logit values for each timestep/character that"] # [doc = " contributed to the creation of this transcript."] pub confidence : f64 ,
}
 # [test] fn bindgen_test_layout_CandidateTranscript () {
assert_eq ! (:: std :: mem :: size_of ::< CandidateTranscript > () , 24usize , concat ! ("Size of: " , stringify ! (CandidateTranscript))) ;
 assert_eq ! (:: std :: mem :: align_of ::< CandidateTranscript > () , 8usize , concat ! ("Alignment of " , stringify ! (CandidateTranscript))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< CandidateTranscript > ())) . tokens as * const _ as usize } , 0usize , concat ! ("Offset of field: " , stringify ! (CandidateTranscript) , "::" , stringify ! (tokens))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< CandidateTranscript > ())) . num_tokens as * const _ as usize } , 8usize , concat ! ("Offset of field: " , stringify ! (CandidateTranscript) , "::" , stringify ! (num_tokens))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< CandidateTranscript > ())) . confidence as * const _ as usize } , 16usize , concat ! ("Offset of field: " , stringify ! (CandidateTranscript) , "::" , stringify ! (confidence))) ;

}
 # [doc = " @brief An array of CandidateTranscript objects computed by the model."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct Metadata {
# [doc = " Array of CandidateTranscript objects"] pub transcripts : * const CandidateTranscript , # [doc = " Size of the transcripts array"] pub num_transcripts : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_Metadata () {
assert_eq ! (:: std :: mem :: size_of ::< Metadata > () , 16usize , concat ! ("Size of: " , stringify ! (Metadata))) ;
 assert_eq ! (:: std :: mem :: align_of ::< Metadata > () , 8usize , concat ! ("Alignment of " , stringify ! (Metadata))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< Metadata > ())) . transcripts as * const _ as usize } , 0usize , concat ! ("Offset of field: " , stringify ! (Metadata) , "::" , stringify ! (transcripts))) ;
 assert_eq ! (unsafe { & (* (:: std :: ptr :: null ::< Metadata > ())) . num_transcripts as * const _ as usize } , 8usize , concat ! ("Offset of field: " , stringify ! (Metadata) , "::" , stringify ! (num_transcripts))) ;

}
 pub const DeepSpeech_Error_Codes_DS_ERR_OK : DeepSpeech_Error_Codes = 0 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_NO_MODEL : DeepSpeech_Error_Codes = 4096 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_INVALID_ALPHABET : DeepSpeech_Error_Codes = 8192 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_INVALID_SHAPE : DeepSpeech_Error_Codes = 8193 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_INVALID_SCORER : DeepSpeech_Error_Codes = 8194 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_MODEL_INCOMPATIBLE : DeepSpeech_Error_Codes = 8195 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_SCORER_NOT_ENABLED : DeepSpeech_Error_Codes = 8196 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_SCORER_UNREADABLE : DeepSpeech_Error_Codes = 8197 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_LM : DeepSpeech_Error_Codes = 8198 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_SCORER_NO_TRIE : DeepSpeech_Error_Codes = 8199 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_SCORER_INVALID_TRIE : DeepSpeech_Error_Codes = 8200 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_SCORER_VERSION_MISMATCH : DeepSpeech_Error_Codes = 8201 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_MMAP : DeepSpeech_Error_Codes = 12288 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_INIT_SESS : DeepSpeech_Error_Codes = 12289 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_INTERPRETER : DeepSpeech_Error_Codes = 12290 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_RUN_SESS : DeepSpeech_Error_Codes = 12291 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_STREAM : DeepSpeech_Error_Codes = 12292 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_READ_PROTOBUF : DeepSpeech_Error_Codes = 12293 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_SESS : DeepSpeech_Error_Codes = 12294 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_CREATE_MODEL : DeepSpeech_Error_Codes = 12295 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_INSERT_HOTWORD : DeepSpeech_Error_Codes = 12296 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_CLEAR_HOTWORD : DeepSpeech_Error_Codes = 12297 ;
 pub const DeepSpeech_Error_Codes_DS_ERR_FAIL_ERASE_HOTWORD : DeepSpeech_Error_Codes = 12304 ;
 pub type DeepSpeech_Error_Codes = :: std :: os :: raw :: c_uint ;
 extern "C" {
# [doc = " @brief Returns a textual description corresponding to an error code."] # [doc = "        The string returned must be freed with @{link DS_FreeString()}."] # [doc = ""] # [doc = " @return The error description."] pub fn DS_ErrorCodeToErrorMessage (aErrorCode : :: std :: os :: raw :: c_int ,) -> * mut :: std :: os :: raw :: c_char ;
 # [doc = " @brief Returns the version of this library. The returned version is a semantic"] # [doc = "        version (SemVer 2.0.0). The string returned must be freed with {@link DS_FreeString()}."] # [doc = ""] # [doc = " @return The version string."] pub fn DS_Version () -> * mut :: std :: os :: raw :: c_char ;
 # [doc = " @brief Free a char* string returned by the DeepSpeech API."] pub fn DS_FreeString (str_ : * mut :: std :: os :: raw :: c_char) ;
 # [doc = " @brief Free memory allocated for metadata information."] pub fn DS_FreeMetadata (m : * mut Metadata) ;
 # [doc = " @brief Destroy a streaming state without decoding the computed logits. This"] # [doc = "        can be used if you no longer need the result of an ongoing streaming"] # [doc = "        inference and don't want to perform a costly decode operation."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = ""] # [doc = " @note This method will free the state pointer (@p aSctx)."] pub fn DS_FreeStream (aSctx : * mut StreamingState) ;
 # [doc = " @brief Compute the final decoding of an ongoing streaming inference and return"] # [doc = "        results including metadata. Signals the end of an ongoing streaming"] # [doc = "        inference."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = " @param aNumResults The number of candidate transcripts to return."] # [doc = ""] # [doc = " @return Metadata struct containing multiple candidate transcripts. Each transcript"] # [doc = "         has per-token metadata including timing information. The user is"] # [doc = "         responsible for freeing Metadata by calling {@link DS_FreeMetadata()}."] # [doc = "         Returns NULL on error."] # [doc = ""] # [doc = " @note This method will free the state pointer (@p aSctx)."] pub fn DS_FinishStreamWithMetadata (aSctx : * mut StreamingState , aNumResults : :: std :: os :: raw :: c_uint ,) -> * mut Metadata ;
 # [doc = " @brief Compute the final decoding of an ongoing streaming inference and return"] # [doc = "        the result. Signals the end of an ongoing streaming inference."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = ""] # [doc = " @return The STT result. The user is responsible for freeing the string using"] # [doc = "         {@link DS_FreeString()}."] # [doc = ""] # [doc = " @note This method will free the state pointer (@p aSctx)."] pub fn DS_FinishStream (aSctx : * mut StreamingState) -> * mut :: std :: os :: raw :: c_char ;
 # [doc = " @brief Compute the intermediate decoding of an ongoing streaming inference,"] # [doc = "        return results including metadata."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = " @param aNumResults The number of candidate transcripts to return."] # [doc = ""] # [doc = " @return Metadata struct containing multiple candidate transcripts. Each transcript"] # [doc = "         has per-token metadata including timing information. The user is"] # [doc = "         responsible for freeing Metadata by calling {@link DS_FreeMetadata()}."] # [doc = "         Returns NULL on error."] pub fn DS_IntermediateDecodeWithMetadata (aSctx : * const StreamingState , aNumResults : :: std :: os :: raw :: c_uint ,) -> * mut Metadata ;
 # [doc = " @brief Compute the intermediate decoding of an ongoing streaming inference."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = ""] # [doc = " @return The STT intermediate result. The user is responsible for freeing the"] # [doc = "         string using {@link DS_FreeString()}."] pub fn DS_IntermediateDecode (aSctx : * const StreamingState) -> * mut :: std :: os :: raw :: c_char ;
 # [doc = " @brief Feed audio samples to an ongoing streaming inference."] # [doc = ""] # [doc = " @param aSctx A streaming state pointer returned by {@link DS_CreateStream()}."] # [doc = " @param aBuffer An array of 16-bit, mono raw audio samples at the"] # [doc = "                appropriate sample rate (matching what the model was trained on)."] # [doc = " @param aBufferSize The number of samples in @p aBuffer."] pub fn DS_FeedAudioContent (aSctx : * mut StreamingState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint ,) ;
 # [doc = " @brief Create a new streaming inference state. The streaming state returned"] # [doc = "        by this function can then be passed to {@link DS_FeedAudioContent()}"] # [doc = "        and {@link DS_FinishStream()}."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model to use."] # [doc = " @param[out] retval an opaque pointer that represents the streaming state. Can"] # [doc = "                    be NULL if an error occurs."] # [doc = ""] # [doc = " @return Zero for success, non-zero on failure."] pub fn DS_CreateStream (aCtx : * mut ModelState , retval : * mut * mut StreamingState ,) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Use the DeepSpeech model to convert speech to text and output results"] # [doc = " including metadata."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model to use."] # [doc = " @param aBuffer A 16-bit, mono raw audio signal at the appropriate"] # [doc = "                sample rate (matching what the model was trained on)."] # [doc = " @param aBufferSize The number of samples in the audio signal."] # [doc = " @param aNumResults The maximum number of CandidateTranscript structs to return. Returned value might be smaller than this."] # [doc = ""] # [doc = " @return Metadata struct containing multiple CandidateTranscript structs. Each"] # [doc = "         transcript has per-token metadata including timing information. The"] # [doc = "         user is responsible for freeing Metadata by calling {@link DS_FreeMetadata()}."] # [doc = "         Returns NULL on error."] pub fn DS_SpeechToTextWithMetadata (aCtx : * mut ModelState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint , aNumResults : :: std :: os :: raw :: c_uint ,) -> * mut Metadata ;
 # [doc = " @brief Use the DeepSpeech model to convert speech to text."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model to use."] # [doc = " @param aBuffer A 16-bit, mono raw audio signal at the appropriate"] # [doc = "                sample rate (matching what the model was trained on)."] # [doc = " @param aBufferSize The number of samples in the audio signal."] # [doc = ""] # [doc = " @return The STT result. The user is responsible for freeing the string using"] # [doc = "         {@link DS_FreeString()}. Returns NULL on error."] pub fn DS_SpeechToText (aCtx : * mut ModelState , aBuffer : * const :: std :: os :: raw :: c_short , aBufferSize : :: std :: os :: raw :: c_uint ,) -> * mut :: std :: os :: raw :: c_char ;
 # [doc = " @brief Set hyperparameters alpha and beta of the external scorer."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = " @param aAlpha The alpha hyperparameter of the decoder. Language model weight."] # [doc = " @param aLMBeta The beta hyperparameter of the decoder. Word insertion weight."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_SetScorerAlphaBeta (aCtx : * mut ModelState , aAlpha : f32 , aBeta : f32 ,) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Disable decoding using an external scorer."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_DisableExternalScorer (aCtx : * mut ModelState) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Removes all elements from the hot-words map."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure (invalid arguments)."] pub fn DS_ClearHotWords (aCtx : * mut ModelState) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Remove entry for a hot-word from the hot-words map."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = " @param word The hot-word."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure (invalid arguments)."] pub fn DS_EraseHotWord (aCtx : * mut ModelState , word : * const :: std :: os :: raw :: c_char ,) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Add a hot-word and its boost."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = " @param word The hot-word."] # [doc = " @param boost The boost."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure (invalid arguments)."] pub fn DS_AddHotWord (aCtx : * mut ModelState , word : * const :: std :: os :: raw :: c_char , boost : f32 ,) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Enable decoding using an external scorer."] # [doc = ""] # [doc = " @param aCtx The ModelState pointer for the model being changed."] # [doc = " @param aScorerPath The path to the external scorer file."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure (invalid arguments)."] pub fn DS_EnableExternalScorer (aCtx : * mut ModelState , aScorerPath : * const :: std :: os :: raw :: c_char ,) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Frees associated resources and destroys model object."] pub fn DS_FreeModel (ctx : * mut ModelState) ;
 # [doc = " @brief Return the sample rate expected by a model."] # [doc = ""] # [doc = " @param aCtx A ModelState pointer created with {@link DS_CreateModel}."] # [doc = ""] # [doc = " @return Sample rate expected by the model for its input."] pub fn DS_GetModelSampleRate (aCtx : * const ModelState) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Set beam width value used by the model."] # [doc = ""] # [doc = " @param aCtx A ModelState pointer created with {@link DS_CreateModel}."] # [doc = " @param aBeamWidth The beam width used by the model. A larger beam width value"] # [doc = "                   generates better results at the cost of decoding time."] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_SetModelBeamWidth (aCtx : * mut ModelState , aBeamWidth : :: std :: os :: raw :: c_uint ,) -> :: std :: os :: raw :: c_int ;
 # [doc = " @brief Get beam width value used by the model. If {@link DS_SetModelBeamWidth}"] # [doc = "        was not called before, will return the default value loaded from the"] # [doc = "        model file."] # [doc = ""] # [doc = " @param aCtx A ModelState pointer created with {@link DS_CreateModel}."] # [doc = ""] # [doc = " @return Beam width value used by the model."] pub fn DS_GetModelBeamWidth (aCtx : * const ModelState) -> :: std :: os :: raw :: c_uint ;
 # [doc = " @brief An object providing an interface to a trained DeepSpeech model."] # [doc = ""] # [doc = " @param aModelPath The path to the frozen model graph."] # [doc = " @param[out] retval a ModelState pointer"] # [doc = ""] # [doc = " @return Zero on success, non-zero on failure."] pub fn DS_CreateModel (aModelPath : * const :: std :: os :: raw :: c_char , retval : * mut * mut ModelState ,) -> :: std :: os :: raw :: c_int ;

}
