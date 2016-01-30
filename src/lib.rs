mod ffi;

pub struct SoundIo {
    context: *mut ffi::Struct_SoundIo,
}
impl SoundIo {
    pub fn new() -> Self {
        SoundIo { context: unsafe { ffi::soundio_create() } }
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
        let cnt: i32 = unsafe { ffi::soundio_backend_count(self.context) };
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
}
impl Drop for SoundIo {
    fn drop(&mut self) {
        unsafe {
            self.disconnect();
            ffi::soundio_destroy(self.context)
        }
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
}

#[test]
fn test_enums() {
    assert_eq!("(no error)",
               format!("{}", ffi::Enum_SoundIoError::SoundIoErrorNone));
    assert_eq!("ALSA",
               format!("{}", ffi::Enum_SoundIoBackend::SoundIoBackendAlsa));
    assert_eq!("Front Left",
               format!("{}", ffi::Enum_SoundIoChannelId::SoundIoChannelIdFrontLeft));
}
