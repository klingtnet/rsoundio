extern crate rsoundio;

use std::os::raw::c_int;
use std::f32::consts;

#[test]
fn test_outstream() {
    println!("test: OutStream");
    let sio = rsoundio::SoundIo::new();
    assert!(sio.connect().is_none());
    // assert!(sio.connect_backend(rsoundio::ffi::SioBackend::Alsa).is_none());
    println!("current backend: {}", sio.current_backend().unwrap());
    sio.flush_events();
    let fmt;
    let dev_idx = sio.default_output_device_index().unwrap();
    let mut frames: Vec<Vec<f32>> = vec![vec![],vec![]];
    let dev = sio.get_output_device(dev_idx).unwrap();
    println!("device: {}, ref_count: {}", dev, dev.ref_count());
    let mut stream = dev.create_outstream().unwrap();
    assert!(stream.open().is_none());
    fmt = stream.current_format().unwrap();
    println!("current format: {}", fmt);
    let f  = 440f32;
    let sr = stream.get_sample_rate();
    let layout = stream.get_layout();
    let channels = layout.channel_count();
    let phi = 2.0 * f * consts::PI / (sr as f32);
    let l: Vec<f32> = (0..4800).map(|i| f32::sin(i as f32*phi)).collect();
    let r = l.clone();
    frames = vec![l, r];
    let cb = |out: rsoundio::OutStream, min_frame_count: i32, max_frame_count: i32| {
        out.write_stream(min_frame_count, &frames).unwrap();
    };
    stream.register_write_callback(Box::new(cb));
    let ucb = |out: rsoundio::OutStream| println!("Underflow!");
    stream.register_underflow_callback(Box::new(ucb));
    let ecb = |out: rsoundio::OutStream, err: rsoundio::ffi::SioError| println!("Error: {}", err);
    stream.register_error_callback(Box::new(ecb));
    assert!(stream.start().is_none());
    sio.wait_events();
    println!("Received event!");
    assert!(stream.clear_buffer().is_none());
    assert!(stream.pause(true).is_none());
    assert!(stream.pause(false).is_none());
    println!("latency: {}", stream.get_latency().unwrap());
    stream.destroy();
}
