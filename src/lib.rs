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
//! out.register_write_callback(Box::new(|out: rsoundio::OutStream, min_frame_count: i32,
//! max_frame_count: i32| {
//!     let frames = vec![vec![], vec![]];
//!     // frames should contain audio data for each channel
//!     out.write_stream_f32(min_frame_count, &frames).unwrap();
//! }));
//! out.open().unwrap();
//! // start the audio stream and wait for events
//! // out.start().unwrap();
//! // loop { sio.wait_events(); }
//! out.destroy();
//! ```

pub mod ffi;
mod base;
mod stream;

pub use base::*;
pub use stream::*;
