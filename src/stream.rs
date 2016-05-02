use std::os::raw::{c_int, c_double, c_void};
use std::{ptr, slice};
use std::ffi::CString;

use ffi;
use base::*;

macro_rules! write_stream {
    ($name:ident, $t:ty) => (
        /// Expects a vector of `channel_count` channel `buffers`
        /// containing audio data that is written into
        /// the buffer of the output device.
        /// NOTE: This is subject of change.
        /// Passing an iterator gives more flexibility and should be more performant
        /// in most cases.
        ///
        /// Returns the number of actually written frames.
        /// If the provided buffers contain less frames
        /// than `min_frame_count`, or less buffers
        /// as `channel_count` are provided,
        /// then a `ffi::enums::SioError::Invalid` is returned.
        pub fn $name(&self, min_frame_count: u32, buffers: &[Vec<$t>]) -> SioResult<u32> {
            let channel_count = self.layout().channel_count();
            // check if buffer contains frames for all channels
            if buffers.len() < channel_count as usize {
                return Err(ffi::enums::SioError::Invalid);
            }
            // check if there are at least min_frame_count frames for all channels
            if !buffers.iter().map(|c| c.len()).all(|l| l >= min_frame_count as usize) {
                return Err(ffi::enums::SioError::Invalid);
            }

            // assuming that every channel buffer has the same length
            let frame_count = buffers[0].len() as c_int;
            let mut raw_areas: *mut ffi::SoundIoChannelArea = ptr::null_mut();
            let actual_frame_count = try!(self.begin_write(&mut raw_areas, &frame_count));
            let areas = unsafe { slice::from_raw_parts_mut(raw_areas, channel_count as usize) };
            for idx in 0..actual_frame_count as usize {
                for channel in 0..channel_count as usize {
                    let area = areas[channel];
                    let addr = (area.ptr as usize + area.step as usize * idx) as *mut $t;
                    unsafe { *addr = buffers[channel][idx] };
                }
            }
            self.end_write().map_or(Ok(actual_frame_count), |err| Err(err))
        }
    )
}

extern "C" fn write_wrapper(raw_out: *mut ffi::SoundIoOutStream, min: c_int, max: c_int) {
    let mut out = OutStream::new(raw_out);
    out.marker = true;
    let callbacks_ptr = unsafe { (*out.stream).userdata as *mut Box<OutStreamCallbacks> };
    let callbacks: &mut Box<OutStreamCallbacks> = unsafe { &mut *callbacks_ptr };
    callbacks.write.as_mut().map(|f| f(out, min as u32, max as u32));
}

extern "C" fn underflow_wrapper(raw_out: *mut ffi::SoundIoOutStream) {
    let mut out = OutStream::new(raw_out);
    out.marker = true;
    let callbacks_ptr = unsafe { (*out.stream).userdata as *mut Box<OutStreamCallbacks> };
    let callbacks: &mut Box<OutStreamCallbacks> = unsafe { &mut *callbacks_ptr };
    callbacks.underflow.as_mut().map(|f| f(out));
}

extern "C" fn error_wrapper(raw_out: *mut ffi::SoundIoOutStream, error: ffi::enums::SioError) {
    let mut out = OutStream::new(raw_out);
    out.marker = true;
    let callbacks_ptr = unsafe { (*out.stream).userdata as *mut Box<OutStreamCallbacks> };
    let callbacks: &mut Box<OutStreamCallbacks> = unsafe { &mut *callbacks_ptr };
    callbacks.error.as_mut().map(|f| f(out, error));
}

struct OutStreamCallbacks<'a> {
    write: Option<Box<FnMut(OutStream, u32, u32) + 'a>>,
    underflow: Option<Box<FnMut(OutStream) + 'a>>,
    error: Option<Box<FnMut(OutStream, ffi::enums::SioError) + 'a>>,
}
impl<'a> Default for OutStreamCallbacks<'a> {
    fn default() -> Self {
        OutStreamCallbacks {
            write: None,
            underflow: None,
            error: None,
        }
    }
}

/// An audio output stream, returned from a `Device`.
pub struct OutStream<'a> {
    stream: *mut ffi::SoundIoOutStream,
    callbacks: Box<OutStreamCallbacks<'a>>,
    name: CString,
    marker: bool,
}
impl<'a> OutStream<'a> {
    pub fn new(raw_stream: *mut ffi::SoundIoOutStream) -> Self {
        let callbacks = Box::new(OutStreamCallbacks::default());
        OutStream {
            stream: raw_stream,
            callbacks: callbacks,
            name: CString::new("outstream").unwrap(),
            marker: false,
        }
    }

    /// Change settings (e.g. `set_format`) **before** calling `open`.
    /// After you call this function, `OutStream::software_latency` is set to
    /// the correct value.
    ///
    /// The next thing to do is call `start`.
    /// If this function returns an error, the outstream is in an invalid state and
    /// you must call `destroy` on it.
    ///
    /// Possible errors:
    ///
    /// - `ffi::enums::SioErrorInvalid`
    ///     - device is not an *output* device
    ///     - format is not valid
    ///     - `channel_count` is greater than 24
    /// - `ffi::enums::SioError::NoMem`
    /// - `ffi::enums::SioError::OpeningDevice`
    /// - `ffi::enums::SioError::BackendDisconnected`
    /// - `ffi::enums::SioError::SystemResources`
    /// - `ffi::enums::SioError::NoSuchClient` - when JACK returns `JackNoSuchClient`
    /// - `ffi::enums::SioErrorOpeningDevice`
    /// - `ffi::enums::SioErrorIncompatibleBackend` - `OutStream::channel_count` is
    ///   greater than the number of channels the backend can handle.
    /// - `ffi::enums::SioErrorIncompatibleDevice` - stream parameters requested are not
    ///   compatible with the chosen device.
    pub fn open(&self) -> SioResult<()> {
        match unsafe { ffi::soundio_outstream_open(self.stream) } {
            ffi::enums::SioError::None => Ok(()),
            err => Err(err),
        }
    }

    /// After you call this function, the registered `write_callback` will be called.
    ///
    /// This function might directly call the `write_callback`.
    ///
    /// Possible errors:
    ///
    /// - `ffi::enums::SioError::Streaming`
    /// - `ffi::enums::SioError::NoMem`
    /// - `ffi::enums::SioError::SystemResources`
    /// - `ffi::enums::SioError::BackendDisconnected`
    pub fn start(&self) -> SioResult<()> {
        match unsafe { ffi::soundio_outstream_start(self.stream) } {
            ffi::enums::SioError::None => Ok(()),
            err => Err(err),
        }
    }

    /// Registers the given callback as `write_callback` that is called as soon as you call `start`.
    ///
    /// In this callback, you call `OutStream::write_stream_FMT` where `FMT` is one of the supported
    /// format, `u16`, `f32` etc.
    ///
    /// `frame_count_max` will always be greater than 0. Note that you
    /// should write as many frames as you can; `frame_count_min` might be 0 and
    /// you can still get a buffer underflow if you always write
    /// `frame_count_min` frames.
    ///
    /// For Dummy, ALSA, and PulseAudio, `frame_count_min` will be 0. For JACK
    /// and CoreAudio `frame_count_min` will be equal to `frame_count_max`.
    ///
    /// The code in the supplied function must be suitable for real-time
    /// execution. That means that it cannot call functions that might block
    /// for a long time. This includes all I/O functions (disk, TTY, network),
    /// malloc, free, printf, pthread_mutex_lock, sleep, wait, poll, select,
    /// pthread_join, pthread_cond_wait, etc.
    pub fn register_write_callback<W>(&mut self, callback: W)
        where W: FnMut(OutStream, u32, u32) + 'a
    {
        // stored box reference to callback closure
        self.callbacks.write = Some(Box::new(callback));
        unsafe {
            // register wrapper for write_callback
            (*self.stream).write_callback = Some(write_wrapper);
            // store reference to callbacks struct in userdata pointer
            (*self.stream).userdata =
                &self.callbacks as *const Box<OutStreamCallbacks> as *mut c_void
        }
    }

    /// Registers the given callback as `underflow_callback`.
    /// This *optional* callback happens when the sound device runs out of buffered audio data to play.
    /// After this occurs, the outstream waits until the buffer is full to resume playback.
    /// This is called from the `OutStream::write_callback` thread context.
    pub fn register_underflow_callback<U>(&mut self, callback: U)
        where U: FnMut(OutStream) + 'a
    {
        self.callbacks.underflow = Some(Box::new(callback));
        unsafe {
            // register wrapper for write_callback
            (*self.stream).underflow_callback = Some(underflow_wrapper);
            // store reference to callbacks struct in userdata pointer
            (*self.stream).userdata =
                &self.callbacks as *const Box<OutStreamCallbacks> as *mut c_void
        }
    }

    /// *Optional* callback. `err` is always `ffi::enums::SioError::ErrorStreaming`.
    /// This is an unrecoverable error. The stream is in an
    /// invalid state and must be destroyed, call `OutStream::destroy`.
    /// If you do not supply `error_callback`, the default callback will print
    /// a message to stderr and then call `abort`.
    /// This is called from the `OutStream::write_callback` thread context.
    pub fn register_error_callback<E>(&mut self, callback: E)
        where E: FnMut(OutStream, ffi::enums::SioError) + 'a
    {
        self.callbacks.error = Some(Box::new(callback));
        unsafe {
            // register wrapper for write_callback
            (*self.stream).error_callback = Some(error_wrapper);
            // store reference to callbacks struct in userdata pointer
            (*self.stream).userdata =
                &self.callbacks as *const Box<OutStreamCallbacks> as *mut c_void
        }
    }

    write_stream!(write_stream_i8, i8);
    write_stream!(write_stream_u8, u8);
    write_stream!(write_stream_i16, i16);
    write_stream!(write_stream_u16, u16);
    write_stream!(write_stream_i32, i32);
    write_stream!(write_stream_u32, u32);
    write_stream!(write_stream_f32, f32);
    write_stream!(write_stream_f64, f64);

    fn begin_write(&self,
                   areas: *mut *mut ffi::SoundIoChannelArea,
                   frame_count: &c_int)
                   -> SioResult<u32> {
        let mut actual_frame_count = *frame_count as c_int;
        match unsafe {
            ffi::soundio_outstream_begin_write(self.stream,
                                               areas,
                                               &mut actual_frame_count as *mut c_int)
        } {
            ffi::enums::SioError::None => Ok(actual_frame_count as u32),
            err => Err(err),
        }
    }

    fn end_write(&self) -> Option<ffi::enums::SioError> {
        match unsafe { ffi::soundio_outstream_end_write(self.stream) } {
            ffi::enums::SioError::None => None,
            err => Some(err),
        }
    }

    /// Clears the output stream buffer.
    /// This function can be called from any thread.
    /// This function can be called regardless of whether the outstream is paused
    /// or not.
    /// Some backends do not support clearing the buffer. On these backends this
    /// function will return `ffi::enums::SioError::IncompatibleBackend`.
    /// Some devices do not support clearing the buffer. On these devices this
    /// function might return `ffi::enums::SioError::IncompatibleDevice`.
    /// Possible errors:
    ///
    /// - `ffi::enums::SioError::Streaming`
    /// - `ffi::enums::SioError::IncompatibleBackend`
    /// - `ffi::enums::SioError::IncompatibleDevice`
    pub fn clear_buffer(&self) -> Option<ffi::enums::SioError> {
        match unsafe { ffi::soundio_outstream_clear_buffer(self.stream) } {
            ffi::enums::SioError::None => None,
            err => Some(err),
        }
    }

    /// If the underlying backend and device support pausing, this pauses the
    /// stream. `OutStream::write_callback` may be called a few more times if
    /// the buffer is not full.
    /// Pausing might put the hardware into a low power state which is ideal if your
    /// software is silent for some time.
    /// This function may be called from any thread context, including
    /// `OutStream::write_callback`.
    /// Pausing when already paused or unpausing when already unpaused has no
    /// effect and returns `None`.
    ///
    /// Possible errors:
    ///
    /// - `ffi::enums::SioError::BackendDisconnected`
    /// - `ffi::enums::SioError::Streaming`
    /// - `ffi::enums::SioError::IncompatibleDevice` - device does not support
    ///   pausing/unpausing. This error code might not be returned even if the
    ///   device does not support pausing/unpausing.
    /// - `ffi::enums::SioError::IncompatibleBackend` - backend does not support
    ///   pausing/unpausing.
    /// - `ffi::enums::SioError::Invalid` - outstream not opened and started
    pub fn pause(&self) -> Option<ffi::enums::SioError> {
        self.stream_pause(true)
    }

    /// Unpauses the stream. See `pause` for more details.
    ///
    /// Possible errors:
    ///
    /// - `ffi::enums::SioError::BackendDisconnected`
    /// - `ffi::enums::SioError::Streaming`
    /// - `ffi::enums::SioError::IncompatibleDevice` - device does not support
    ///   pausing/unpausing. This error code might not be returned even if the
    ///   device does not support pausing/unpausing.
    /// - `ffi::enums::SioError::IncompatibleBackend` - backend does not support
    ///   pausing/unpausing.
    /// - `ffi::enums::SioError::Invalid` - outstream not opened and started
    pub fn unpause(&self) -> Option<ffi::enums::SioError> {
        self.stream_pause(false)
    }

    fn stream_pause(&self, pause: bool) -> Option<ffi::enums::SioError> {
        let pause_c_bool = match pause {
            true => 1u8,
            false => 0u8,
        };

        match unsafe { ffi::soundio_outstream_pause(self.stream, pause_c_bool) } {
            ffi::enums::SioError::None => None,
            err => Some(err),
        }
    }

    /// Obtain the total number of seconds that the next frame written after the
    /// last frame written will take to become
    /// audible.
    /// This includes both software and hardware latency.
    ///
    /// This function must be called only from within `OutStream::write_callback`.
    ///
    /// Possible errors:
    ///
    /// - `ffi::enums::SioError::Streaming`
    pub fn latency(&self) -> SioResult<f64> {
        let mut latency = 0.0f64;
        match unsafe {
            ffi::soundio_outstream_get_latency(self.stream, &mut latency as *mut c_double)
        } {
            ffi::enums::SioError::None => Ok(latency),
            err => Err(err),
        }

    }

    /// Returns the current `format` or a `ffi::enums::SioError::Invalid` if
    /// the format is not set.
    pub fn format(&self) -> SioResult<ffi::enums::SioFormat> {
        match unsafe { (*self.stream).format } {
            ffi::enums::SioFormat::Invalid => Err(ffi::enums::SioError::Invalid),
            fmt => Ok(fmt),
        }
    }

    /// Sets the stream format to `format`.
    /// **Must** be called before `open`ing the stream.
    ///
    /// If the device doesn't support the format
    /// `ffi::enums::SioError::IncompatibleDevice` is returned.
    pub fn set_format(&self, format: ffi::enums::SioFormat) -> SioResult<()> {
        let dev = self.device();
        if dev.supports_format(format) {
            unsafe { (*self.stream).format = format };
            Ok(())
        } else {
            Err(ffi::enums::SioError::IncompatibleDevice)
        }
    }

    /// Returns the channel layout of the output stream.
    pub fn layout(&self) -> ChannelLayout {
        ChannelLayout::new(unsafe { &(*self.stream).layout })
    }

    /// Returns the sample rate of the output stream.
    pub fn sample_rate(&self) -> u32 {
        unsafe { (*self.stream).sample_rate as u32 }
    }

    /// Sets the stream sample rate.
    /// Make sure that the device supports the given sample rate to avoid
    /// sample rate conversions. A `Device` provides `supports_sample_rate` and
    /// `nearest_sample_rate` methods for this purpose.
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        unsafe { (*self.stream).sample_rate = sample_rate as c_int }
    }

    /// Returns the underlying device of the output stream.
    pub fn device(&self) -> Device {
        Device::new(unsafe { (*self.stream).device })
    }

    /// Sets the stream name to `name`.
    /// PulseAudio uses this for the stream name.
    /// JACK uses this for the client name of the client that connects when you
    /// open the stream.
    /// WASAPI uses this for the session display name.
    /// Colons (`:`) contained in `name` will be replaced with `_`.
    /// If the `name` contains a `NULL` byte, `SioError::EncodingString` is returned.
    pub fn set_name<T: Into<String>>(&mut self, name: T) -> SioResult<()> {
        let s = name.into().replace(":", "_");
        self.name = try!(CString::new(s).map_err(|_| ffi::enums::SioError::EncodingString));
        unsafe { (*self.stream).name = self.name.as_ptr() };
        Ok(())
    }

    /// Returns the stream name or `None` if the name wasn't set.
    pub fn name(&self) -> Option<String> {
        let s_ptr = unsafe { (*self.stream).name };
        if !s_ptr.is_null() {
            match ffi::utils::ptr_to_string(s_ptr) {
                Ok(s) => Some(s),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns an `ffi::enums::SioError` if the layout is incompatible
    /// with the audio output device.
    /// If the layout is compatible `()` is returned.
    pub fn layout_error(&self) -> SioResult<()> {
        match unsafe { (*self.stream).layout_error } {
            0 => Ok(()),
            e => {
                println!("layout error: {}", e);
                Ok(())
            }
        }
    }

    /// Destroys the output stream.
    /// Calls this when your application shuts down.
    fn destroy(&self) {
        unsafe { ffi::soundio_outstream_destroy(self.stream) }
    }
}
impl<'a> Drop for OutStream<'a> {
    fn drop(&mut self) {
        // Only drop if usage `marker` is false.
        // The usage marker is set by the callback function to prevent the
        // source stream from dropping on the context switch of the callback function.
        if !self.marker {
            self.destroy()
        } else {
            // reset usage marker.
            self.marker = false
        }
    }
}
