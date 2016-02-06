mod ffi;

use std::os::raw::{c_int, c_double};
use std::fmt::Display;

pub struct SoundIo {
    context: *mut ffi::Struct_SoundIo,
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

    pub fn connect(&self) -> Option<ffi::Enum_SoundIoError> {
        match unsafe { ffi::soundio_connect(self.context) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
            err @ _ => Some(err),
        }
    }

    pub fn connect_backend(&self,
                           backend: ffi::Enum_SoundIoBackend)
                           -> Option<ffi::Enum_SoundIoError> {
        match unsafe { ffi::soundio_connect_backend(self.context, backend) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
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

    pub fn get_backend(&self, idx: i32) -> Option<ffi::Enum_SoundIoBackend> {
        match unsafe { ffi::soundio_get_backend(self.context, idx) } {
            ffi::Enum_SoundIoBackend::SoundIoBackendNone => None,
            backend @ _ => Some(backend),
        }
    }

    pub fn current_backend(&self) -> Option<ffi::Enum_SoundIoBackend> {
        match unsafe { (*self.context).current_backend } {
            ffi::Enum_SoundIoBackend::SoundIoBackendNone => None,
            backend @ _ => Some(backend),
        }
    }

    pub fn have_backend(&self, backend: ffi::Enum_SoundIoBackend) -> bool {
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
    layout: *const ffi::Struct_SoundIoChannelLayout,
}
impl ChannelLayout {
    fn new(raw_layout: *const ffi::Struct_SoundIoChannelLayout) -> Self {
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


    pub fn find_channel(&self, channel: ffi::Enum_SoundIoChannelId) -> Option<i32> {
        match unsafe { ffi::soundio_channel_layout_find_channel(self.layout, channel) } {
            -1 => None,
            idx @ _ => Some(idx),
        }
    }

    pub fn detect_builtin(&mut self) -> bool {
        // This is a hack because of the transmute.
        unsafe {
            let mut_layout: *mut ffi::Struct_SoundIoChannelLayout =
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
    device: *mut ffi::Struct_SoundIoDevice,
}
impl Device {
    pub fn new(dev_ptr: *mut ffi::Struct_SoundIoDevice) -> Self {
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

    pub fn supports_format(&self, format: ffi::Enum_SoundIoFormat) -> bool {
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
    stream: *mut ffi::Struct_SoundIoOutStream,
}
impl OutStream {
    pub fn new(raw_stream: *mut ffi::Struct_SoundIoOutStream) -> Self {
        OutStream { stream: raw_stream }
    }

    pub fn open(&self) -> Option<ffi::Enum_SoundIoError> {
        match unsafe { ffi::soundio_outstream_open(self.stream) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
            err @ _ => Some(err),
        }
    }

    pub fn start(&self) -> Option<ffi::Enum_SoundIoError> {
        match unsafe { ffi::soundio_outstream_start(self.stream) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
            err @ _ => Some(err),
        }
    }

    pub fn register_callback<F>(&self, callback: F)
        where F: Fn(OutStream, i32, i32)
    {
        // TODO: set wrapper inside the constructor
        unsafe extern "C" fn wrapper(out: *mut ffi::Struct_SoundIoOutStream,
                                     min: c_int,
                                     max: c_int) {
            !unimplemented();
        };
        unsafe {
            (*self.stream).write_callback = Some(wrapper);
        }
    }

    pub fn begin_write(&self,
                       areas: *mut *mut ffi::Struct_SoundIoChannelArea,
                       frame_count: *mut c_int)
                       -> Option<ffi::Enum_SoundIoError> {
        unimplemented!();
        match unsafe { ffi::soundio_outstream_begin_write(self.stream, areas, frame_count) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
            err @ _ => Some(err),
        }
    }

    pub fn end_write(&self) -> Option<ffi::Enum_SoundIoError> {
        match unsafe { ffi::soundio_outstream_end_write(self.stream) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
            err @ _ => Some(err),
        }
    }

    pub fn clear_buffer(&self) -> Option<ffi::Enum_SoundIoError> {
        match unsafe { ffi::soundio_outstream_clear_buffer(self.stream) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
            err @ _ => Some(err),
        }
    }

    pub fn pause(&self, pause: bool) -> Option<ffi::Enum_SoundIoError> {
        let pause_c_bool = match pause {
            true => 1u8,
            false => 0u8,
        };
        match unsafe { ffi::soundio_outstream_pause(self.stream, pause_c_bool) } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => None,
            err @ _ => Some(err),
        }
    }

    pub fn get_latency(&self) -> Result<f64, ffi::Enum_SoundIoError> {
        let mut latency = 0.0f64;
        match unsafe {
            ffi::soundio_outstream_get_latency(self.stream, &mut latency as *mut c_double)
        } {
            ffi::Enum_SoundIoError::SoundIoErrorNone => Ok(latency),
            err @ _ => Err(err),
        }

    }

    pub fn current_format(&self) -> Result<ffi::Enum_SoundIoFormat, ffi::Enum_SoundIoError> {
        match unsafe { (*self.stream).format } {
            ffi::Enum_SoundIoFormat::SoundIoFormatInvalid => {
                Err(ffi::Enum_SoundIoError::SoundIoErrorInvalid)
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

    pub fn destroy(&mut self) {
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

#[test]
fn test_soundio() {
    let sio = SoundIo::new();
    assert!(sio.backend_count() > 0);
    assert!(sio.get_backend(0).is_some());
    assert!(sio.get_backend(-1).is_none());
    assert!(sio.connect().is_none());
    sio.disconnect();
    if sio.have_backend(ffi::Enum_SoundIoBackend::SoundIoBackendAlsa) {
        assert!(sio.connect_backend(ffi::Enum_SoundIoBackend::SoundIoBackendAlsa).is_none());
        sio.disconnect();
    }
    assert!(SoundIo::channel_layout_builtin_count() >= 0);
    assert!(sio.connect().is_none());
    sio.flush_events();
    assert!(sio.output_device_count().unwrap() > 0);
    assert!(sio.input_device_count().unwrap() > 0);
}

#[test]
fn test_channel_layout() {
    let cnt = SoundIo::channel_layout_builtin_count();
    assert!(cnt > 0);
    assert!(ChannelLayout::get_builtin(-1).is_none());
    assert_eq!(ChannelLayout::get_builtin(0), ChannelLayout::get_builtin(0));
    let mut layout = ChannelLayout::get_default(2).unwrap();
    assert!(layout.detect_builtin());
    assert!(layout.find_channel(ffi::Enum_SoundIoChannelId::SoundIoChannelIdFrontLeft).is_some());
    assert!(layout.find_channel(ffi::Enum_SoundIoChannelId::SoundIoChannelIdLfe2).is_none());
    assert!(cnt > 2);
    let preferred = [ChannelLayout::get_builtin(0).unwrap(),
                     ChannelLayout::get_builtin(1).unwrap()];
    let available = [ChannelLayout::get_builtin(1).unwrap(),
                     ChannelLayout::get_builtin(2).unwrap()];
    let best_match = ChannelLayout::best_matching_channel_layout(&preferred, &available);
    assert_eq!(ChannelLayout::get_builtin(1).unwrap(), best_match.unwrap());

}

#[test]
fn test_enums() {
    assert_eq!("(no error)",
               format!("{}", ffi::Enum_SoundIoError::SoundIoErrorNone));
    assert_eq!("ALSA",
               format!("{}", ffi::Enum_SoundIoBackend::SoundIoBackendAlsa));
    assert_eq!("Front Left",
               format!("{}", ffi::Enum_SoundIoChannelId::SoundIoChannelIdFrontLeft));
    assert_eq!(ffi::Enum_SoundIoChannelId::from("Front Left".to_string()),
               ffi::Enum_SoundIoChannelId::SoundIoChannelIdFrontLeft);
    assert!(ffi::Enum_SoundIoChannelId::from("SomeInvalidBoredom".to_string()) !=
            ffi::Enum_SoundIoChannelId::SoundIoChannelIdFrontLeft);
    assert_eq!("unsigned 8-bit",
               format!("{}", ffi::Enum_SoundIoFormat::SoundIoFormatU8));
    assert_eq!(1,
               ffi::Enum_SoundIoFormat::SoundIoFormatU8.get_bytes_per_sample());
    assert_eq!(4,
               ffi::Enum_SoundIoFormat::SoundIoFormatU32LE.get_bytes_per_sample());
}

#[test]
fn test_device() {
    let sio = SoundIo::new();
    assert!(sio.connect().is_none());
    sio.flush_events();
    let in_dev_idx = sio.default_input_device_index().unwrap();
    let out_dev_idx = sio.default_output_device_index().unwrap();
    let in_dev = sio.get_input_device(in_dev_idx).unwrap();
    let out_dev = sio.get_output_device(out_dev_idx).unwrap();
    println!("{}", in_dev);
    println!("{}", out_dev);
    assert!(in_dev != out_dev);
    assert_eq!(in_dev, in_dev);
    out_dev.sort_channel_layouts();
    let stereo_layout = ChannelLayout::get_default(2).unwrap();
    assert!(in_dev.supports_format(ffi::Enum_SoundIoFormat::SoundIoFormatFloat32LE));
    assert!(out_dev.supports_format(ffi::Enum_SoundIoFormat::SoundIoFormatFloat32LE));
    assert!(in_dev.supports_layout(&stereo_layout));
    assert!(out_dev.supports_layout(&stereo_layout));
    assert!(in_dev.supports_sample_rate(48_000));
    assert!(out_dev.supports_sample_rate(48_000));
    assert!(in_dev.nearest_sample_rate(1) > 0);
    assert!(out_dev.nearest_sample_rate(1) > 0);
}

#[test]
fn test_outstream() {
    let sio = SoundIo::new();
    assert!(sio.connect().is_none());
    sio.flush_events();
    let dev_idx = sio.default_output_device_index().unwrap();
    let dev = sio.get_output_device(dev_idx).unwrap();
    let stream = dev.create_outstream().unwrap();
    assert!(stream.open().is_none());
    assert!(stream.clear_buffer().is_none());
    assert!(stream.pause(true).is_none());
    assert!(stream.pause(false).is_none());
    println!("latency: {}", stream.get_latency().unwrap());
}
