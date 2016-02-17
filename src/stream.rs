use std::os::raw::{c_int, c_double, c_void};
use std::{ptr, slice};

use ffi;
use base::*;

macro_rules! write_stream {
    ($name:ident, $t:ty) => (
        /// Expects a vector of `channel_count` channel `buffers`
        /// containing audio data that are written into
        /// the output device buffer.
        /// Returns the number of actually written frames.
        /// If the provided buffers contain less frames
        /// than `min_frame_count`, or less buffers
        /// as `channel_count` are provided,
        /// then a `ffi::SioError::Invalid` is returned.
        pub fn $name(&self, min_frame_count: i32, buffers: &Vec<Vec<$t>>) -> SioResult<i32> {
            let channel_count = self.get_layout().channel_count();
            // check if buffer contains frames for all channels
            if buffers.len() < channel_count as usize {
                return Err(ffi::SioError::Invalid);
            }
            // check if there are at least min_frame_count frames for all channels
            if !buffers.iter().map(|c| c.len()).all(|l| l >= min_frame_count as usize) {
                return Err(ffi::SioError::Invalid);
            }

            // assuming that every channel buffer has the same length
            let mut frame_count = buffers[0].len() as c_int;
            let mut raw_areas: *mut ffi::SoundIoChannelArea = ptr::null_mut();
            let actual_frame_count = try!(self.begin_write(&mut raw_areas, &frame_count));
            let areas = unsafe { slice::from_raw_parts_mut(raw_areas, channel_count as usize) };
            for idx in 0..actual_frame_count as usize {
                for channel in 0..channel_count as usize {
                    let area = areas[channel];
                    let addr = (area.ptr as usize + area.step as usize * idx) as *mut $t;
                    unsafe { *addr = buffers[channel][idx] }
                }
            }
            self.end_write().map_or(Ok(actual_frame_count), |err| Err(err))
        }
    )
}

extern "C" fn write_wrapper<W>(raw_out: *mut ffi::SoundIoOutStream, min: c_int, max: c_int)
    where W: Fn(OutStream, i32, i32)
{
    let out = OutStream::new(raw_out);
    let callbacks_ptr = unsafe { (*out.stream).userdata as *const Box<OutStreamCallbacks> };
    let callbacks: &Box<OutStreamCallbacks> = unsafe { &*callbacks_ptr };
    callbacks.write.as_ref().map(|ref f| f(out, min as i32, max as i32));
}

extern "C" fn underflow_wrapper<U>(raw_out: *mut ffi::SoundIoOutStream)
    where U: Fn(OutStream)
{
    let out = OutStream::new(raw_out);
    let callbacks_ptr = unsafe { (*out.stream).userdata as *const Box<OutStreamCallbacks> };
    let callbacks: &Box<OutStreamCallbacks> = unsafe { &*callbacks_ptr };
    callbacks.underflow.as_ref().map(|ref f| f(out));
}

extern "C" fn error_wrapper<E>(raw_out: *mut ffi::SoundIoOutStream, error: ffi::SioError)
    where E: Fn(OutStream, ffi::SioError)
{
    let out = OutStream::new(raw_out);
    let callbacks_ptr = unsafe { (*out.stream).userdata as *const Box<OutStreamCallbacks> };
    let callbacks: &Box<OutStreamCallbacks> = unsafe { &*callbacks_ptr };
    callbacks.error.as_ref().map(|ref f| f(out, error));
}

struct OutStreamCallbacks<'a> {
    write: Option<Box<Fn(OutStream, i32, i32) + 'a>>,
    underflow: Option<Box<Fn(OutStream) + 'a>>,
    error: Option<Box<Fn(OutStream, ffi::SioError) + 'a>>,
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
impl<'a> Drop for OutStreamCallbacks<'a> {
    fn drop(&mut self) {}
}

/// An audio output stream, returned from a `Device`.
pub struct OutStream<'a> {
    stream: *mut ffi::SoundIoOutStream,
    callbacks: Box<OutStreamCallbacks<'a>>,
}
impl<'a> OutStream<'a> {
    pub fn new(raw_stream: *mut ffi::SoundIoOutStream) -> Self {
        let callbacks = Box::new(OutStreamCallbacks::default());
        OutStream {
            stream: raw_stream,
            callbacks: callbacks,
        }
    }

    /// Change settings (e.g. `set_format`) **before** calling `open`.
    /// After you call this function, `OutStream::software_latency` is set to
    /// the correct value.
    ///
    /// The next thing to do is call `outstream_start`.
    /// If this function returns an error, the outstream is in an invalid state and
    /// you must call `destroy` on it.
    ///
    /// Possible errors:
    ///
    /// - `ffi::SioErrorInvalid`
    ///     - TODO: implement format setting
    ///     - SoundIoDevice::aim is not #SoundIoDeviceAimOutput
    ///     - SoundIoOutStream::format is not valid
    ///     - SoundIoOutStream::channel_count is greater than #SOUNDIO_MAX_CHANNELS
    /// - `ffi::SioError::NoMem`
    /// - `ffi::SioError::OpeningDevice`
    /// - `ffi::SioError::BackendDisconnected`
    /// - `ffi::SioError::SystemResources`
    /// - `ffi::SioError::NoSuchClient` - when JACK returns `JackNoSuchClient`
    /// - `ffi::SioErrorOpeningDevice`
    /// - `ffi::SioErrorIncompatibleBackend` - `OutStream::channel_count` is
    ///   greater than the number of channels the backend can handle.
    /// - `ffi::SioErrorIncompatibleDevice` - stream parameters requested are not
    ///   compatible with the chosen device.
    pub fn open(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_open(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

    /// After you call this function, the registered `write_callback` will be called.
    ///
    /// This function might directly call the `write_callback`.
    ///
    /// Possible errors:
    ///
    /// - `ffi::SioError::Streaming`
    /// - `ffi::SioError::NoMem`
    /// - `ffi::SioError::SystemResources`
    /// - `ffi::SioError::BackendDisconnected`
    pub fn start(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_start(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
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
    pub fn register_write_callback<W>(&mut self, callback: Box<W>)
        where W: Fn(OutStream, i32, i32) + 'a
    {
        // stored box reference to callback closure
        self.callbacks.write = Some(callback);
        unsafe {
            // register wrapper for write_callback
            (*self.stream).write_callback = Some(write_wrapper::<W>);
            // store reference to callbacks struct in userdata pointer
            (*self.stream).userdata =
                &self.callbacks as *const Box<OutStreamCallbacks> as *mut c_void
        }
    }

    /// Registers the given callback as `underflow_callback`.
    /// This *optional* callback happens when the sound device runs out of buffered audio data to play.
    /// After this occurs, the outstream waits until the buffer is full to resume playback.
    /// This is called from the `OutStream::write_callback` thread context.
    pub fn register_underflow_callback<U>(&mut self, callback: Box<U>)
        where U: Fn(OutStream) + 'a
    {
        self.callbacks.underflow = Some(callback);
        unsafe {
            // register wrapper for write_callback
            (*self.stream).underflow_callback = Some(underflow_wrapper::<U>);
            // store reference to callbacks struct in userdata pointer
            (*self.stream).userdata =
                &self.callbacks as *const Box<OutStreamCallbacks> as *mut c_void
        }
    }

    /// *Optional* callback. `err` is always `ffi::SioError::ErrorStreaming`.
    /// This is an unrecoverable error. The stream is in an
    /// invalid state and must be destroyed, call `OutStream::destroy`.
    /// If you do not supply `error_callback`, the default callback will print
    /// a message to stderr and then call `abort`.
    /// This is called from the `OutStream::write_callback` thread context.
    pub fn register_error_callback<E>(&mut self, callback: Box<E>)
        where E: Fn(OutStream, ffi::SioError) + 'a
    {
        self.callbacks.error = Some(callback);
        unsafe {
            // register wrapper for write_callback
            (*self.stream).error_callback = Some(error_wrapper::<E>);
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
                   frame_count: &i32)
                   -> SioResult<i32> {
        let mut actual_frame_count = *frame_count as c_int;
        match unsafe {
            ffi::soundio_outstream_begin_write(self.stream,
                                               areas,
                                               &mut actual_frame_count as *mut c_int)
        } {
            ffi::SioError::None => Ok(actual_frame_count),
            err @ _ => Err(err),
        }
    }

    fn end_write(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_end_write(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

    /// Clears the output stream buffer.
    /// This function can be called from any thread.
    /// This function can be called regardless of whether the outstream is paused
    /// or not.
    /// Some backends do not support clearing the buffer. On these backends this
    /// function will return `ffi::SioError::IncompatibleBackend`.
    /// Some devices do not support clearing the buffer. On these devices this
    /// function might return `ffi::SioError::IncompatibleDevice`.
    /// Possible errors:
    ///
    /// - `ffi::SioError::Streaming`
    /// - `ffi::SioError::IncompatibleBackend`
    /// - `ffi::SioError::IncompatibleDevice`
    pub fn clear_buffer(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_clear_buffer(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
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
    /// - `ffi::SioError::BackendDisconnected`
    /// - `ffi::SioError::Streaming`
    /// - `ffi::SioError::IncompatibleDevice` - device does not support
    ///   pausing/unpausing. This error code might not be returned even if the
    ///   device does not support pausing/unpausing.
    /// - `ffi::SioError::IncompatibleBackend` - backend does not support
    ///   pausing/unpausing.
    /// - `ffi::SioError::Invalid` - outstream not opened and started
    pub fn pause(&self, pause: bool) -> Option<ffi::SioError> {
        let pause_c_bool = match pause {
            true => 1u8,
            false => 0u8,
        };
        match unsafe { ffi::soundio_outstream_pause(self.stream, pause_c_bool) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
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
    /// - `ffi::SioError::Streaming`
    pub fn get_latency(&self) -> SioResult<f64> {
        let mut latency = 0.0f64;
        match unsafe {
            ffi::soundio_outstream_get_latency(self.stream, &mut latency as *mut c_double)
        } {
            ffi::SioError::None => Ok(latency),
            err @ _ => Err(err),
        }

    }

    /// Returns the current `format` or a `ffi::SioError::Invalid` if
    /// the format is not set.
    pub fn current_format(&self) -> SioResult<ffi::SioFormat> {
        match unsafe { (*self.stream).format } {
            ffi::SioFormat::Invalid => Err(ffi::SioError::Invalid),
            fmt @ _ => Ok(fmt),
        }
    }

    /// Returns the channel layout of the output stream.
    pub fn get_layout(&self) -> ChannelLayout {
        ChannelLayout::new(unsafe { &(*self.stream).layout })
    }

    /// Returns the sample rate of the output stream.
    pub fn get_sample_rate(&self) -> i32 {
        unsafe { (*self.stream).sample_rate as i32 }
    }

    /// Returns the underlying device of the output stream.
    pub fn get_device(&self) -> Device {
        let dev = Device::new(unsafe { (*self.stream).device });
        dev.inc_ref();
        dev
    }

    /// Destroys the output stream.
    /// Calls this when your application shuts down.
    /// NOTE: This can break if a callback is still active.
    pub fn destroy(&self) {
        unsafe { ffi::soundio_outstream_destroy(self.stream) }
    }
}
impl<'a> Drop for OutStream<'a> {
    fn drop(&mut self) {
        // TODO: call destroy manually.
        // OutStream will get dropped each time a new
        // struct is created from the same *mut pointer.
    }
}
