extern crate rsoundio;

use std::thread;
use std::time::Duration;

#[test]
fn test_outstream() {
    let sio = rsoundio::SoundIo::new();
    sio.connect_backend(rsoundio::ffi::SioBackend::Dummy).unwrap();
    let current_backend = sio.current_backend().unwrap();
    assert_eq!(current_backend, rsoundio::ffi::SioBackend::Dummy);
    sio.flush_events();
    let dev_idx = sio.default_output_device_index().unwrap();
    let dev = sio.output_device(dev_idx).unwrap();
    let mut stream = dev.create_outstream().unwrap();
    stream.open().unwrap();
    assert!(stream.sample_rate() > 0);
    let layout = stream.layout();
    assert_eq!(layout.channel_count(), 2);
    let cb = |out: rsoundio::OutStream, min_frame_count: i32, max_frame_count: i32| {
        let l: Vec<f32> = (0..max_frame_count as usize)
                              .map(|i| {
                                  match i % 2 == 0 {
                                      true => -1.0,
                                      false => 1.0,
                                  }
                              })
                              .collect();
        let r = l.clone();
        let frames = vec![l, r];
        let frames_written = out.write_stream_f32(min_frame_count, &frames).unwrap();
        assert!(frames_written > 0);
        assert!(out.latency().is_ok());
    };
    stream.register_write_callback(Box::new(cb));
    let ucb = |_: rsoundio::OutStream| println!("Underflow!");
    stream.register_underflow_callback(Box::new(ucb));
    let ecb = |_: rsoundio::OutStream, err: rsoundio::ffi::SioError| println!("Error: {}", err);
    stream.register_error_callback(Box::new(ecb));
    stream.start().unwrap();
    thread::sleep(Duration::new(1, 0));
    assert!(stream.pause().is_none());
    thread::sleep(Duration::new(1, 0));
    assert!(stream.unpause().is_none());
    assert!(stream.clear_buffer().is_none());
    thread::sleep(Duration::new(1, 0));
    stream.destroy();
    dev.dec_ref();
}
