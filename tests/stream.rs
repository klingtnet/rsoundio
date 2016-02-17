extern crate rsoundio;

use std::f32::consts;
use std::thread;
use std::time::Duration;

#[test]
fn test_outstream() {
    println!("test: OutStream");
    let sio = rsoundio::SoundIo::new();
    sio.connect_backend(rsoundio::ffi::SioBackend::Dummy).unwrap();
    println!("current backend: {}", sio.current_backend().unwrap());
    sio.flush_events();
    let fmt;
    let dev_idx = sio.default_output_device_index().unwrap();
    let mut frames: Vec<Vec<f32>> = vec![vec![], vec![]];
    let dev = sio.get_output_device(dev_idx).unwrap();
    println!("device: {}, ref_count: {}", dev, dev.ref_count());
    let mut stream = dev.create_outstream().unwrap();
    stream.open().unwrap();
    fmt = stream.current_format().unwrap();
    println!("current format: {}", fmt);
    let f = 4400f32;
    let sr = stream.get_sample_rate();
    let layout = stream.get_layout();
    let channels = layout.channel_count();
    let phi = 2.0 * f * consts::PI / (sr as f32);
    let l: Vec<f32> = (0..4096).map(|i| f32::sin(i as f32 * phi)).collect();
    let r = l.clone();
    frames = vec![l, r];
    let cb = |out: rsoundio::OutStream, min_frame_count: i32, max_frame_count: i32| {
        let frames_written = out.write_stream_f32(min_frame_count, &frames).unwrap();
        assert!(frames_written > 0);
        out.get_latency().map(|latency| assert!(latency >= 0.0));
    };
    stream.register_write_callback(Box::new(cb));
    let ucb = |out: rsoundio::OutStream| println!("Underflow!");
    stream.register_underflow_callback(Box::new(ucb));
    let ecb = |out: rsoundio::OutStream, err: rsoundio::ffi::SioError| println!("Error: {}", err);
    stream.register_error_callback(Box::new(ecb));
    assert!(stream.start().is_none());
    //sio.wait_events();
    thread::sleep(Duration::new(1,0));
    assert!(stream.pause(true).is_none());
    thread::sleep(Duration::new(1,0));
    assert!(stream.pause(false).is_none());
    assert!(stream.clear_buffer().is_none());
    thread::sleep(Duration::new(1,0));
    stream.destroy();
    dev.dec_ref();
}
