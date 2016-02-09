use std::os::raw::{c_int, c_double, c_void, c_char};
use std::mem;
use std::clone::Clone;
use std::default::Default;

use ffi::enums::*;

#[repr(C)]
#[derive(Copy)]
pub struct SoundIoChannelLayout {
    pub name: *const c_char,
    pub channel_count: c_int,
    pub channels: [SioChannelId; 24usize],
}
impl Clone for SoundIoChannelLayout {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for SoundIoChannelLayout {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct SoundIoSampleRateRange {
    pub min: c_int,
    pub max: c_int,
}
impl Clone for SoundIoSampleRateRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for SoundIoSampleRateRange {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct SoundIoChannelArea {
    pub ptr: *mut c_char,
    pub step: c_int,
}
impl Clone for SoundIoChannelArea {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for SoundIoChannelArea {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct SoundIo {
    pub userdata: *mut c_void,
    pub on_devices_change: Option<unsafe extern "C" fn(arg1: *mut SoundIo)>,
    pub on_backend_disconnect: Option<unsafe extern "C" fn(arg1: *mut SoundIo, err: c_int)>,
    pub on_events_signal: Option<unsafe extern "C" fn(arg1: *mut SoundIo)>,
    pub current_backend: SioBackend,
    pub app_name: *const c_char,
    pub emit_rtprio_warning: Option<extern "C" fn()>,
    pub jack_info_callback: Option<unsafe extern "C" fn(msg: *const c_char)>,
    pub jack_error_callback: Option<unsafe extern "C" fn(msg: *const c_char)>,
}
impl Clone for SoundIo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for SoundIo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct SoundIoDevice {
    pub soundio: *mut SoundIo,
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub aim: SioDeviceAim,
    pub layouts: *mut SoundIoChannelLayout,
    pub layout_count: c_int,
    pub current_layout: SoundIoChannelLayout,
    pub formats: *mut SioFormat,
    pub format_count: c_int,
    pub current_format: SioFormat,
    pub sample_rates: *mut SoundIoSampleRateRange,
    pub sample_rate_count: c_int,
    pub sample_rate_current: c_int,
    pub software_latency_min: c_double,
    pub software_latency_max: c_double,
    pub software_latency_current: c_double,
    pub is_raw: u8,
    pub ref_count: c_int,
    pub probe_error: c_int,
}
impl Clone for SoundIoDevice {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for SoundIoDevice {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct SoundIoOutStream {
    pub device: *mut SoundIoDevice,
    pub format: SioFormat,
    pub sample_rate: c_int,
    pub layout: SoundIoChannelLayout,
    pub software_latency: c_double,
    pub userdata: *mut c_void,
    pub write_callback: Option<extern "C" fn(arg1: *mut SoundIoOutStream,
                                             frame_count_min: c_int,
                                             frame_count_max: c_int)
                                            >,
    pub underflow_callback: Option<extern "C" fn(arg1: *mut SoundIoOutStream)>,
    pub error_callback: Option<extern "C" fn(arg1: *mut SoundIoOutStream, err: SioError)>,
    pub name: *const c_char,
    pub non_terminal_hint: u8,
    pub bytes_per_frame: c_int,
    pub bytes_per_sample: c_int,
    pub layout_error: c_int,
}
impl Clone for SoundIoOutStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for SoundIoOutStream {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct SoundIoInStream {
    pub device: *mut SoundIoDevice,
    pub format: SioFormat,
    pub sample_rate: c_int,
    pub layout: SoundIoChannelLayout,
    pub software_latency: c_double,
    pub userdata: *mut c_void,
    pub read_callback: Option<unsafe extern "C" fn(arg1: *mut SoundIoInStream,
                                                   frame_count_min: c_int,
                                                   frame_count_max: c_int)
                                                  >,
    pub overflow_callback: Option<unsafe extern "C" fn(arg1: *mut SoundIoInStream)>,
    pub error_callback: Option<unsafe extern "C" fn(arg1: *mut SoundIoInStream, err: c_int)>,
    pub name: *const c_char,
    pub non_terminal_hint: u8,
    pub bytes_per_frame: c_int,
    pub bytes_per_sample: c_int,
    pub layout_error: c_int,
}
impl Clone for SoundIoInStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for SoundIoInStream {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
