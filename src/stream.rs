use std::os::raw::{c_int, c_double, c_void};

use ffi;
use base::*;

extern "C" fn wrapper<W>(raw_out: *mut ffi::SoundIoOutStream, min: c_int, max: c_int)
    where W: Fn(OutStream, i32, i32),
{
    let out = OutStream::new(raw_out);
    let cb_ptr = unsafe { (*out.stream).userdata as *const Box<W> };
    let cb: &W = unsafe { &*cb_ptr };
    cb(out, min, max);
}

struct OutStreamCallbacks
{
    write: Option<Box<Fn(OutStream, i32, i32)>>,
    underflow: Option<Box<Fn(OutStream)>>,
    error: Option<Box<Fn(OutStream, ffi::SioError)>>,
}
impl Default for OutStreamCallbacks
{
    fn default() -> Self {
        OutStreamCallbacks {
            write: None,
            underflow: None,
            error: None,
        }
    }
}

pub struct OutStream
{
    // TODO: make this private again
    pub stream: *mut ffi::SoundIoOutStream,
    callbacks: OutStreamCallbacks,
}
impl OutStream
{
    pub fn new(raw_stream: *mut ffi::SoundIoOutStream) -> Self {
        OutStream {
            stream: raw_stream,
            callbacks: OutStreamCallbacks::default(),
        }
    }

    pub fn open(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_open(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

    pub fn start(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_start(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

    pub fn register_write_callback<W>(&mut self, callback: Box<W>)
        where W: Fn(OutStream, i32, i32)
    {
        let userdata = &*callback as *const W as *mut c_void;
        unsafe {
            (*self.stream).userdata = userdata;
            (*self.stream).write_callback = Some(wrapper::<W>);
        }
    }

    pub fn begin_write(&self,
                       areas: *mut *mut ffi::SoundIoChannelArea,
                       frame_count: *mut c_int)
                       -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_begin_write(self.stream, areas, frame_count) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

    pub fn end_write(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_end_write(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

    pub fn clear_buffer(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_outstream_clear_buffer(self.stream) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

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

    pub fn get_latency(&self) -> Result<f64, ffi::SioError> {
        let mut latency = 0.0f64;
        match unsafe {
            ffi::soundio_outstream_get_latency(self.stream, &mut latency as *mut c_double)
        } {
            ffi::SioError::None => Ok(latency),
            err @ _ => Err(err),
        }

    }

    pub fn current_format(&self) -> Result<ffi::SioFormat, ffi::SioError> {
        match unsafe { (*self.stream).format } {
            ffi::SioFormat::Invalid => Err(ffi::SioError::Invalid),
            fmt @ _ => Ok(fmt),
        }
    }

    pub fn get_layout(&self) -> ChannelLayout {
        ChannelLayout::new(unsafe { &(*self.stream).layout })
    }

    pub fn get_sample_rate(&self) -> i32 {
        unsafe { (*self.stream).sample_rate as i32 }
    }

    pub fn get_device(&self) -> Device {
        let dev = Device::new(unsafe { (*self.stream).device });
        dev.inc_ref();
        dev
    }

    pub fn destroy(&self) {
        unsafe { ffi::soundio_outstream_destroy(self.stream) }
    }
}
impl Drop for OutStream
{
    fn drop(&mut self) {
        // TODO: call destroy manually.
        // OutStream will get dropped each time a new
        // struct is created from the same *mut pointer.
    }
}
