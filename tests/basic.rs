extern crate rsoundio;

#[test]
fn test_soundio() {
    let sio = rsoundio::SoundIo::new();
    assert!(sio.backend_count() > 0);
    assert!(sio.get_backend(0).is_some());
    assert!(sio.get_backend(-1).is_none());
    assert!(sio.connect().is_none());
    sio.disconnect();
    if sio.have_backend(rsoundio::ffi::SioBackend::SoundIoBackendAlsa) {
        assert!(sio.connect_backend(rsoundio::ffi::SioBackend::SoundIoBackendAlsa)
                   .is_none());
        sio.disconnect();
    }
    assert!(rsoundio::SoundIo::channel_layout_builtin_count() >= 0);
    assert!(sio.connect().is_none());
    sio.flush_events();
    assert!(sio.output_device_count().unwrap() > 0);
    assert!(sio.input_device_count().unwrap() > 0);
}

#[test]
fn test_channel_layout() {
    let cnt = rsoundio::SoundIo::channel_layout_builtin_count();
    assert!(cnt > 0);
    assert!(rsoundio::ChannelLayout::get_builtin(-1).is_none());
    assert_eq!(rsoundio::ChannelLayout::get_builtin(0),
               rsoundio::ChannelLayout::get_builtin(0));
    let mut layout = rsoundio::ChannelLayout::get_default(2).unwrap();
    assert!(layout.detect_builtin());
    assert!(layout.find_channel(rsoundio::ffi::SioChannelId::SoundIoChannelIdFrontLeft)
                  .is_some());
    assert!(layout.find_channel(rsoundio::ffi::SioChannelId::SoundIoChannelIdLfe2)
                  .is_none());
    assert!(cnt > 2);
    let preferred = [rsoundio::ChannelLayout::get_builtin(0).unwrap(),
                     rsoundio::ChannelLayout::get_builtin(1).unwrap()];
    let available = [rsoundio::ChannelLayout::get_builtin(1).unwrap(),
                     rsoundio::ChannelLayout::get_builtin(2).unwrap()];
    let best_match = rsoundio::ChannelLayout::best_matching_channel_layout(&preferred, &available);
    assert_eq!(rsoundio::ChannelLayout::get_builtin(1).unwrap(),
               best_match.unwrap());

}

#[test]
fn test_enums() {
    assert_eq!("(no error)",
               format!("{}", rsoundio::ffi::SioError::SoundIoErrorNone));
    assert_eq!("ALSA",
               format!("{}", rsoundio::ffi::SioBackend::SoundIoBackendAlsa));
    assert_eq!("Front Left",
               format!("{}",
                       rsoundio::ffi::SioChannelId::SoundIoChannelIdFrontLeft));
    assert_eq!(rsoundio::ffi::SioChannelId::from("Front Left".to_string()),
               rsoundio::ffi::SioChannelId::SoundIoChannelIdFrontLeft);
    assert!(rsoundio::ffi::SioChannelId::from("SomeInvalidBoredom".to_string()) !=
            rsoundio::ffi::SioChannelId::SoundIoChannelIdFrontLeft);
    assert_eq!("unsigned 8-bit",
               format!("{}", rsoundio::ffi::SioFormat::SoundIoFormatU8));
    assert_eq!(1,
               rsoundio::ffi::SioFormat::SoundIoFormatU8.get_bytes_per_sample());
    assert_eq!(4,
               rsoundio::ffi::SioFormat::SoundIoFormatU32LE.get_bytes_per_sample());
}

#[test]
fn test_device() {
    let sio = rsoundio::SoundIo::new();
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
    let stereo_layout = rsoundio::ChannelLayout::get_default(2).unwrap();
    assert!(in_dev.supports_format(rsoundio::ffi::SioFormat::SoundIoFormatFloat32LE));
    assert!(out_dev.supports_format(rsoundio::ffi::SioFormat::SoundIoFormatFloat32LE));
    assert!(in_dev.supports_layout(&stereo_layout));
    assert!(out_dev.supports_layout(&stereo_layout));
    assert!(in_dev.supports_sample_rate(48_000));
    assert!(out_dev.supports_sample_rate(48_000));
    assert!(in_dev.nearest_sample_rate(1) > 0);
    assert!(out_dev.nearest_sample_rate(1) > 0);
}
