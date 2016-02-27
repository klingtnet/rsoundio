# A Rusty Wrapper for [libsoundio](http://libsound.io)

[![Circle CI](https://circleci.com/gh/klingtnet/rsoundio.svg?style=svg)](https://circleci.com/gh/klingtnet/rsoundio) [![Crates.io](https://img.shields.io/crates/v/rustc-serialize.svg)](https://crates.io/crates/rsoundio)

Rsoundio is a wrapper for [libsoundio](https://github.com/andrewrk/libsoundio), a cross-platform realtime audio in- and output library.

The following backens are supported:

- JACK
- PulseAudio
- ALSA
- CoreAudio
- WASAPI

For a comparison of libsoundio with other audio libaries, take a look at the [wiki](https://github.com/andrewrk/libsoundio/wiki).

---

This is a *work in progress* and there are still some things that don't work, especially recording via input streams is not supported yet (see [TODOs](#todos).
Most of the input stream implementation can be copied from the output stream though.

- [documentation](https://docs.klingt.net/rustdoc/rsoundio/)

## Usage

Add it to the `dependencies` section of your projects `Cargo.toml`

```toml
[dependencies]
rsoundio = "0.1.*"
```

## Example

`cargo run --example sine`

## TODOs

- [x] add documentation
- [ ] implement `InStream`
- [ ] implement remaining callback registrations for `SoundIo` struct
- [ ] let `write_stream_FMT` accept an iterator instead of a `Vec<Vec<FMT>>`
- [x] make `rsoundio::ffi` private and only export the enums
- [ ] publish on crates.io
- [x] write examples
