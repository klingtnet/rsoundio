use std::fmt::Display;
use std::os::raw::c_int;
use std::ffi::{CString, NulError};

use ffi;
use stream::OutStream;

pub type SioResult<T> = Result<T, ffi::SioError>;

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

    pub fn connect_backend(&self, backend: ffi::SioBackend) -> Option<ffi::SioError> {
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

    /// Returns the default output `Device` of the backend.
    /// `None` if you aren't connected to a backend.
    pub fn default_output_device(&self) -> Option<Device> {
        self.default_output_device_index().and_then(|idx| self.get_output_device(idx))
    }

    /// Returns the default input `Device` of the backend.
    /// `None` if you aren't connected to a backend.
    pub fn default_input_device(&self) -> Option<Device> {
        self.default_input_device_index().and_then(|idx| self.get_input_device(idx))
    }

    /// Sets the application name that is shown in the
    /// system audio mixer.
    /// If the `name` contains a null byte, a `NulError` is returned.
    /// The `:` characters in the `name` will be replaced by `_`.
    pub fn set_app_name<T: Into<String>>(&self, name: T) -> Result<(), NulError>{
        let s = name.into().s.replace(":", "_");
        let c_str = try!(CString::new(s));
        unsafe { (*self.context).app_name = c_str.as_ptr() };
        Ok(())
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
    pub fn new(raw_layout: *const ffi::SoundIoChannelLayout) -> Self {
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
            let mut_layout: *mut ffi::SoundIoChannelLayout = ::std::mem::transmute(self.layout);
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
    pub fn inc_ref(&self) {
        unsafe { ffi::soundio_device_ref(self.device) }
    }

    pub fn dec_ref(&self) {
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
