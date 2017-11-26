# deepspeech-rs

[crates.io](https://crates.io/crates/deepspeech)

[docs.rs](https://docs.rs/deepspeech)

Rust bindings of [Mozilla's DeepSpeech](https://github.com/mozilla/DeepSpeech) library.

The library is open source and performs Speech-To-Text completely offline. They provide pretrained models for English.

## Quickstart

Preparation:

1. `git clone https://github.com/mozilla/DeepSpeech`
2. cd into that directory
3. `git checkout v0.1.0`
4. Run `util/taskcluster.py` to download precompiled components. See the [native_client README](https://github.com/mozilla/DeepSpeech/tree/v0.1.0/native_client) for further options for that script and how to compile DeepSpeech yourself.
5. Download the pretrained models from the URL `https://github.com/mozilla/DeepSpeech/releases/download/v0.1.0/deepspeech-0.1.0-models.tar.gz` and extract the zip file to some location.
6. Add the directory where the precompiled components lie (the DeepSpeech checkout) to your `LD_LIBRARY_PATH` and `LIBRARY_PATH` environment variables.

You can now invoke the example via:

```
cargo run --release --example client <path-to-model-dir> <path-to-audio-file>
```

It will print out the recognized sequence on stdout. The format of the audio files is important: they need to have a sample rate of 16 kHz and the stream must be mono.
All codecs that the awesome [audrey](https://github.com/RustAudio/audrey) library supports are supported.
The [0.1.0 release announcement](https://github.com/mozilla/DeepSpeech/releases/tag/v0.1.0) has a detailed list of requirements.

Note: [Right now](https://github.com/mozilla/DeepSpeech/issues/1023), there are no Linux x64 binaries for older CPU platforms without AVX2 support like Ivy Bridge.
See DeepSpeech's [0.1.0 release announcement](https://github.com/mozilla/DeepSpeech/releases/tag/v0.1.0) for a list of supported platforms.

## Supported versions of DeepSpeech

As of writing this, there has been only been one release of the DeepSpeech library yet, version `0.1.0`. We only claim compatibility with that release.
We will try to provide compatibility with the most recent usable release possible.

## License

Licensed under Apache 2 or MIT (at your option). For details, see the [LICENSE](LICENSE) file.

All examples inside the `examples/` folder are licensed under the
[CC-0](https://creativecommons.org/publicdomain/zero/1.0/) license.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed / CC-0 licensed as above, without any additional terms or conditions.
