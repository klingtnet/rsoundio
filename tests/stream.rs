extern crate rsoundio;

use std::f32::consts;
use std::thread;
use std::time::Duration;

#[test]
fn test_outstream() {
    let sio = rsoundio::SoundIo::new();
    sio.connect_backend(rsoundio::ffi::SioBackend::Dummy).unwrap();
    let current_backend = sio.current_backend().unwrap();
    assert_eq!(current_backend, rsoundio::ffi::SioBackend::Dummy);
    sio.flush_events();
    let fmt;
    let dev_idx = sio.default_output_device_index().unwrap();
    let mut frames: Vec<Vec<f32>> = vec![vec![], vec![]];
    let dev = sio.output_device(dev_idx).unwrap();
    let mut stream = dev.create_outstream().unwrap();
    stream.open().unwrap();
    fmt = stream.format().unwrap();
    let f = 4400f32;
    let sr = stream.sample_rate();
    let layout = stream.layout();
    let channels = layout.channel_count();
    let phi = 2.0 * f * consts::PI / (sr as f32);
    let l: Vec<f32> = (0..4096).map(|i| f32::sin(i as f32 * phi)).collect();
    let r = l.clone();
    frames = vec![l, r];
    let cb = |out: rsoundio::OutStream, min_frame_count: i32, max_frame_count: i32| {
        let frames_written = out.write_stream_f32(min_frame_count, &frames).unwrap();
        assert!(frames_written > 0);
        out.latency().map(|latency| assert!(latency >= 0.0));
    };
    stream.register_write_callback(Box::new(cb));
    let ucb = |out: rsoundio::OutStream| println!("Underflow!");
    stream.register_underflow_callback(Box::new(ucb));
    let ecb = |out: rsoundio::OutStream, err: rsoundio::ffi::SioError| println!("Error: {}", err);
    stream.register_error_callback(Box::new(ecb));
    //sio.wait_events();
    stream.start().unwrap();
    thread::sleep(Duration::new(1,0));
    assert!(stream.pause().is_none());
    thread::sleep(Duration::new(1,0));
    assert!(stream.unpause().is_none());
    assert!(stream.clear_buffer().is_none());
    thread::sleep(Duration::new(1,0));
    stream.destroy();
    dev.dec_ref();
}
