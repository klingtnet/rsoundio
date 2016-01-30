mod ffi;

use std::os::raw::c_int;
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
struct ChannelLayout {
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
