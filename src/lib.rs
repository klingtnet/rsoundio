pub mod ffi;

use std::os::raw::{c_int, c_double};
use std::fmt::Display;

pub struct SoundIo {
    context: *mut ffi::SoundIo,
}
impl SoundIo {
    pub fn new() -> Self {
        SoundIo { context: unsafe { ffi::soundio_create() } }
    }

    pub fn channel_layout_builtin_count() -> i32 {
        let cnt = unsafe { ffi::soundio_channel_layout_builtin_count() };
        if cnt < 0 {
            panic!("Negative # of builtin channel layouts!")
        } else {
            cnt as i32
        }
    }

    pub fn connect(&self) -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_connect(self.context) } {
            ffi::SioError::None => None,
            err @ _ => Some(err),
        }
    }

    pub fn connect_backend(&self,
                           backend: ffi::SioBackend)
                           -> Option<ffi::SioError> {
        match unsafe { ffi::soundio_connect_backend(self.context, backend) } {
            ffi::SioError::None => None,
            err @ _ => {
                println!("{:?}", err);
                Some(err)
            }
        }
    }

    pub fn backend_count(&self) -> i32 {
        let cnt = unsafe { ffi::soundio_backend_count(self.context) } as i32;
        if cnt < 0 {
            panic!("Negative backend count!");
        } else {
            cnt
        }
    }

    pub fn get_backend(&self, idx: i32) -> Option<ffi::SioBackend> {
        match unsafe { ffi::soundio_get_backend(self.context, idx) } {
            ffi::SioBackend::None => None,
            backend @ _ => Some(backend),
        }
    }

    pub fn current_backend(&self) -> Option<ffi::SioBackend> {
        match unsafe { (*self.context).current_backend } {
            ffi::SioBackend::None => None,
            backend @ _ => Some(backend),
        }
    }

    pub fn have_backend(&self, backend: ffi::SioBackend) -> bool {
        unsafe { ffi::soundio_have_backend(backend) == 1u8 }
    }

    pub fn flush_events(&self) {
        unsafe { ffi::soundio_flush_events(self.context) }
    }

    pub fn wait_events(&self) {
        unsafe { ffi::soundio_wait_events(self.context) }
    }

    pub fn wakeup(&self) {
        unsafe { ffi::soundio_wakeup(self.context) }
    }

    pub fn force_device_scan(&self) {
        unsafe { ffi::soundio_force_device_scan(self.context) }
    }

    pub fn disconnect(&self) {
        unsafe { ffi::soundio_disconnect(self.context) }
    }

    pub fn input_device_count(&self) -> Option<i32> {
        let cnt = unsafe { ffi::soundio_input_device_count(self.context) };
        if cnt < 0 {
            None
        } else {
            Some(cnt as i32)
        }
    }

    pub fn output_device_count(&self) -> Option<i32> {
        let cnt = unsafe { ffi::soundio_output_device_count(self.context) };
        if cnt < 0 {
            None
        } else {
            Some(cnt as i32)
        }
    }

    pub fn get_input_device(&self, idx: i32) -> Option<Device> {
        let dev_ptr = unsafe { ffi::soundio_get_input_device(self.context, idx) };
        if dev_ptr.is_null() {
            None
        } else {
            Some(Device::new(dev_ptr))
        }
    }

    pub fn get_output_device(&self, idx: i32) -> Option<Device> {
        let dev_ptr = unsafe { ffi::soundio_get_output_device(self.context, idx) };
        if dev_ptr.is_null() {
            None
        } else {
            Some(Device::new(dev_ptr))
        }
    }

    pub fn default_input_device_index(&self) -> Option<i32> {
        match unsafe { ffi::soundio_default_input_device_index(self.context) } {
            -1 => None,
            idx @ _ => Some(idx as i32),
        }
    }

    pub fn default_output_device_index(&self) -> Option<i32> {
        match unsafe { ffi::soundio_default_output_device_index(self.context) } {
            -1 => None,
            idx @ _ => Some(idx as i32),
        }
    }
}
impl Drop for SoundIo {
    fn drop(&mut self) {
        unsafe {
            self.disconnect();
            ffi::soundio_destroy(self.context)
        }
    }
}

#[derive(Debug)]
pub struct ChannelLayout {
    layout: *const ffi::SoundIoChannelLayout,
}
impl ChannelLayout {
    fn new(raw_layout: *const ffi::SoundIoChannelLayout) -> Self {
        ChannelLayout { layout: raw_layout }
    }

    pub fn get_builtin(idx: i32) -> Option<Self> {
        if 0 <= idx && idx < SoundIo::channel_layout_builtin_count() {
            Some(ChannelLayout::new(unsafe {
                ffi::soundio_channel_layout_get_builtin(idx as c_int)
            }))
        } else {
            None
        }
    }

    pub fn get_default(channel_count: i32) -> Option<Self> {
        if channel_count < 0 {
            None
        } else {
            Some(ChannelLayout::new(unsafe {
                ffi::soundio_channel_layout_get_default(channel_count as i32)
            }))
        }
    }


    pub fn find_channel(&self, channel: ffi::SioChannelId) -> Option<i32> {
        match unsafe { ffi::soundio_channel_layout_find_channel(self.layout, channel) } {
            -1 => None,
            idx @ _ => Some(idx),
        }
    }

    pub fn detect_builtin(&mut self) -> bool {
        // This is a hack because of the transmute.
        unsafe {
            let mut_layout: *mut ffi::SoundIoChannelLayout =
                ::std::mem::transmute(self.layout);
            ffi::soundio_channel_layout_detect_builtin(mut_layout) == 1
        }
    }

    pub fn best_matching_channel_layout(preferred_layouts: &[ChannelLayout],
                                        available_layouts: &[ChannelLayout])
                                        -> Option<ChannelLayout> {
        // do some magic with the slices
        let raw_preferred_layouts: Vec<_> = preferred_layouts.iter()
                                                             .map(|l| unsafe { (*l.layout) })
                                                             .collect();
        let raw_available_layouts: Vec<_> = available_layouts.iter()
                                                             .map(|l| unsafe { (*l.layout) })
                                                             .collect();
        let layout_ptr = unsafe {
            ffi::soundio_best_matching_channel_layout(raw_preferred_layouts.as_ptr(),
                                                      preferred_layouts.len() as c_int,
                                                      raw_available_layouts.as_ptr(),
                                                      available_layouts.len() as c_int)
        };
        if layout_ptr.is_null() {
            None
        } else {
            Some(ChannelLayout::new(layout_ptr))
        }
    }

    pub fn channel_count(&self) -> i32 {
        unsafe { (*self.layout).channel_count as i32 }
    }
}
impl PartialEq for ChannelLayout {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::soundio_channel_layout_equal(self.layout, other.layout) == 1u8 }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl Display for ChannelLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let str_ptr = unsafe { (*self.layout).name };
        write!(f, "{}", ffi::ptr_to_string(str_ptr).unwrap())
    }
}

#[derive(Debug)]
pub struct Device {
    device: *mut ffi::SoundIoDevice,
}
impl Device {
    pub fn new(dev_ptr: *mut ffi::SoundIoDevice) -> Self {
        Device { device: dev_ptr }
    }

    // ref is a keyword
    fn inc_ref(&self) {
        unsafe { ffi::soundio_device_ref(self.device) }
    }

    fn dec_ref(&self) {
        unsafe { ffi::soundio_device_unref(self.device) }
    }

    pub fn sort_channel_layouts(&self) {
        unsafe { ffi::soundio_device_sort_channel_layouts(self.device) }
    }

    pub fn supports_format(&self, format: ffi::SioFormat) -> bool {
        unsafe { ffi::soundio_device_supports_format(self.device, format) == 1u8 }
    }

    pub fn supports_layout(&self, layout: &ChannelLayout) -> bool {
        unsafe { ffi::soundio_device_supports_layout(self.device, layout.layout) == 1u8 }
    }

    pub fn supports_sample_rate(&self, sample_rate: i32) -> bool {
        unsafe {
            ffi::soundio_device_supports_sample_rate(self.device, sample_rate as c_int) == 1u8
        }
    }

    pub fn nearest_sample_rate(&self, sample_rate: i32) -> i32 {
        unsafe { ffi::soundio_device_nearest_sample_rate(self.device, sample_rate) as i32 }
    }

    pub fn create_outstream(&self) -> Option<OutStream> {
        let stream_ptr = unsafe { ffi::soundio_outstream_create(self.device) };
        if stream_ptr.is_null() {
            None
        } else {
            Some(OutStream::new(stream_ptr))
        }
    }

    pub fn ref_count(&self) -> i32 {
        unsafe { (*self.device).ref_count as i32 }
    }
}
impl Display for Device {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let str_ptr = unsafe { (*self.device).name };
        write!(f, "{}", ffi::ptr_to_string(str_ptr).unwrap())
    }
}
impl Drop for Device {
    fn drop(&mut self) {
        self.dec_ref()
    }
}
impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::soundio_device_equal(self.device, other.device) == 1u8 }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub struct OutStream {
    // TODO: make this private again after
    // implementint the callback register methods
    pub stream: *mut ffi::SoundIoOutStream,
}
impl OutStream {
    pub fn new(raw_stream: *mut ffi::SoundIoOutStream) -> Self {
        OutStream { stream: raw_stream }
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

    pub fn register_callback<F>(&self, callback: F)
        where F: Fn(OutStream, i32, i32)
    {
        // TODO: set wrapper inside the constructor
        unsafe extern "C" fn wrapper(out: *mut ffi::SoundIoOutStream,
                                     min: c_int,
                                     max: c_int) {
            unimplemented!();
        };
        unsafe {
            (*self.stream).write_callback = Some(wrapper);
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
            ffi::SioFormat::Invalid => {
                Err(ffi::SioError::Invalid)
            }
            fmt @ _ => Ok(fmt),
        }
    }

    pub fn get_layout(&self) -> ChannelLayout {
        ChannelLayout { layout: unsafe { &(*self.stream).layout } }
    }

    pub fn get_sample_rate(&self) -> i32 {
        unsafe { (*self.stream).sample_rate as i32 }
    }

    pub fn get_device(&self) -> Device {
        let dev = Device { device: unsafe { (*self.stream).device } };
        dev.inc_ref();
        dev
    }

    pub fn destroy(&self) {
        unsafe { ffi::soundio_outstream_destroy(self.stream) }
    }
}
impl Drop for OutStream {
    fn drop(&mut self) {
        // TODO: call destroy manually.
        // OutStream will get dropped each time a new
        // struct is created from the same *mut pointer.
    }
}
