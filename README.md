# deepspeech-rs

[![docs](https://docs.rs/deepspeech/badge.svg)](https://docs.rs/crate/deepspeech)
[![crates.io](https://img.shields.io/crates/v/deepspeech.svg)](https://crates.io/crates/deepspeech)

Rust bindings of [Mozilla's DeepSpeech](https://github.com/mozilla/DeepSpeech) library.

The library is open source and performs Speech-To-Text completely offline. They provide pretrained models for English.

## Quickstart

Preparation:

1. Obtain the Deepspeech `native_client` library. The [release announcement] contains precompiled libraries for various targets.
2. Download the pretrained models named like `deepspeech-{version}-models.tar.gz` from the release announcement and extract the zip file to some location.
3. Add the directory where the `native_client` library lies to your `LD_LIBRARY_PATH` and `LIBRARY_PATH` environment variables.

You can now invoke the example via:

```
cargo run --release --example client <path-to-model-dir> <path-to-audio-file>
```

It will print out the recognized sequence on stdout. The format of the audio files is important: only mono files are supported for now.

All codecs that the awesome [audrey](https://github.com/RustAudio/audrey) library supports are supported.

See DeepSpeech's [release announcement] for more.

[release announcement]: https://github.com/mozilla/DeepSpeech/releases/tag/v0.7.0

## Supported versions of DeepSpeech

We currently support version `0.7.0` of the DeepSpeech library.
We will always try to provide compatibility with the most recent release possible.

## License

Licensed under Apache 2 or MIT (at your option). For details, see the [LICENSE](LICENSE) file.

All examples inside the `examples/` folder are licensed under the
[CC-0](https://creativecommons.org/publicdomain/zero/1.0/) license.

The generated bindings (`sys` subdirectory in git, `-sys` crate on crates.io) fall unter the Mozilla Public License, version 2.0.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed / CC-0 licensed as above, without any additional terms or conditions.
