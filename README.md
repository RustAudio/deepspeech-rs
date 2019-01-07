# deepspeech-rs

[crates.io](https://crates.io/crates/deepspeech)

[docs.rs](https://docs.rs/deepspeech)

Rust bindings of [Mozilla's DeepSpeech](https://github.com/mozilla/DeepSpeech) library.

The library is open source and performs Speech-To-Text completely offline. They provide pretrained models for English.

## Quickstart

Preparation:

1. Obtain the Deepspeech `native_client` library. The [0.4.0 release announcement](https://github.com/mozilla/DeepSpeech/releases/tag/v0.4.0) contains precompiled libraries for various targets.
2. Download the pretrained models from the URL `https://github.com/mozilla/DeepSpeech/releases/download/v0.4.0/deepspeech-0.4.0-models.tar.gz` and extract the zip file to some location.
3. Add the directory where the precompiled components lie (the DeepSpeech checkout) to your `LD_LIBRARY_PATH` and `LIBRARY_PATH` environment variables.

You can now invoke the example via:

```
cargo run --release --example client <path-to-model-dir> <path-to-audio-file>
```

It will print out the recognized sequence on stdout. The format of the audio files is important: only mono files are supported for now.

All codecs that the awesome [audrey](https://github.com/RustAudio/audrey) library supports are supported.

See DeepSpeech's [0.4.0 release announcement](https://github.com/mozilla/DeepSpeech/releases/tag/v0.4.0) for more.

## Supported versions of DeepSpeech

As of writing this, only version `0.3` of the DeepSpeech library is supported.
We will always try to provide compatibility with the most recent release possible.

## License

Licensed under Apache 2 or MIT (at your option). For details, see the [LICENSE](LICENSE) file.

All examples inside the `examples/` folder are licensed under the
[CC-0](https://creativecommons.org/publicdomain/zero/1.0/) license.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed / CC-0 licensed as above, without any additional terms or conditions.
