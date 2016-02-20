extern crate rsoundio;

#[test]
fn test_soundio() {
    let sio = rsoundio::SoundIo::new();
    assert!(sio.backend_count() > 0);
    assert!(sio.backend(0).is_some());
    assert!(sio.backend(-1).is_none());
    sio.connect().unwrap();
    sio.disconnect();
    if sio.have_backend(rsoundio::SioBackend::Alsa) {
        sio.connect_backend(rsoundio::SioBackend::Alsa)
           .unwrap();
        sio.disconnect();
    }
    assert!(rsoundio::SoundIo::channel_layout_builtin_count() >= 0);
    sio.connect().unwrap();
    sio.flush_events();
    assert!(sio.output_device_count().unwrap() > 0);
    assert!(sio.input_device_count().unwrap() > 0);
}

#[test]
fn test_channel_layout() {
    let cnt = rsoundio::SoundIo::channel_layout_builtin_count();
    assert!(cnt > 0);
    assert!(rsoundio::ChannelLayout::builtin(-1).is_none());
    assert_eq!(rsoundio::ChannelLayout::builtin(0),
               rsoundio::ChannelLayout::builtin(0));
    let mut layout = rsoundio::ChannelLayout::default(2).unwrap();
    assert!(layout.detect_builtin());
    assert!(layout.find_channel(rsoundio::SioChannelId::FrontLeft)
                  .is_some());
    assert!(layout.find_channel(rsoundio::SioChannelId::Lfe2)
                  .is_none());
    assert!(cnt > 2);
    let preferred = [rsoundio::ChannelLayout::builtin(0).unwrap(),
                     rsoundio::ChannelLayout::builtin(1).unwrap()];
    let available = [rsoundio::ChannelLayout::builtin(1).unwrap(),
                     rsoundio::ChannelLayout::builtin(2).unwrap()];
    let best_match = rsoundio::ChannelLayout::best_matching_channel_layout(&preferred, &available);
    assert_eq!(rsoundio::ChannelLayout::builtin(1).unwrap(),
               best_match.unwrap());

}

#[test]
fn test_enums() {
    assert_eq!("(no error)", format!("{}", rsoundio::SioError::None));
    assert_eq!("ALSA", format!("{}", rsoundio::SioBackend::Alsa));
    assert_eq!("Front Left",
               format!("{}", rsoundio::SioChannelId::FrontLeft));
    assert_eq!(rsoundio::SioChannelId::from("Front Left".to_string()),
               rsoundio::SioChannelId::FrontLeft);
    assert!(rsoundio::SioChannelId::from("SomeInvalidBoredom".to_string()) !=
            rsoundio::SioChannelId::FrontLeft);
    assert_eq!("unsigned 8-bit",
               format!("{}", rsoundio::SioFormat::U8));
    assert_eq!(1, rsoundio::SioFormat::U8.bytes_per_sample());
    assert_eq!(4, rsoundio::SioFormat::U32LE.bytes_per_sample());
}

#[test]
fn test_device() {
    let sio = rsoundio::SoundIo::new();
    sio.connect().unwrap();
    sio.flush_events();
    let in_dev_idx = sio.default_input_device_index().unwrap();
    let out_dev_idx = sio.default_output_device_index().unwrap();
    let in_dev = sio.input_device(in_dev_idx).unwrap();
    let out_dev = sio.output_device(out_dev_idx).unwrap();
    println!("{}", in_dev);
    println!("{}", out_dev);
    assert!(in_dev != out_dev);
    assert_eq!(in_dev, in_dev);
    out_dev.sort_channel_layouts();
    let stereo_layout = rsoundio::ChannelLayout::default(2).unwrap();
    assert!(in_dev.supports_format(rsoundio::SioFormat::Float32LE));
    assert!(out_dev.supports_format(rsoundio::SioFormat::Float32LE));
    assert!(in_dev.supports_layout(&stereo_layout));
    assert!(out_dev.supports_layout(&stereo_layout));
    assert!(in_dev.supports_sample_rate(48_000));
    assert!(out_dev.supports_sample_rate(48_000));
    assert!(in_dev.nearest_sample_rate(1) > 0);
    assert!(out_dev.nearest_sample_rate(1) > 0);
}
