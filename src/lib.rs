//! [libsoundio](http://libsound.io) bindings for Rust.
//!
//! # Example
//!
//! ```
//! extern crate rsoundio;
//!
//! // create new context
//! let sio = rsoundio::SoundIo::new();
//! // connect to default audio backend
//! sio.connect().unwrap();
//! sio.flush_events();
//! let dev = sio.default_output_device().unwrap();
//! let mut out = dev.create_outstream().unwrap();
//! // register write_callback
//! out.register_write_callback(|out: rsoundio::OutStream, min_frame_count: u32,
//! max_frame_count: u32| {
//!     let frames = vec![vec![], vec![]];
//!     // frames must contain audio data for each channel
//!     out.write_stream_f32(min_frame_count, &frames).unwrap();
//! });
//! out.open().unwrap();
//! // start the audio stream and wait for events
//! // out.start().unwrap();
//! // loop { sio.wait_events(); }
//! out.destroy();
//! ```

mod ffi;
mod base;
mod stream;

pub use ffi::enums::*;
pub use base::*;
pub use stream::*;
