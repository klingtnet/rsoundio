extern crate rsoundio;
extern crate rci;

use rci::Ci;

use std::thread;
use std::time::Duration;

#[test]
fn test_outstream() {
    if Ci::new().is_some() {
        ::std::process::exit(0)
    }
    let sio = rsoundio::SoundIo::default();
    sio.connect_backend(rsoundio::SioBackend::Dummy).unwrap();
    let current_backend = sio.current_backend().unwrap();
    assert_eq!(current_backend, rsoundio::SioBackend::Dummy);
    sio.flush_events();
    let dev_idx = sio.default_output_device_index().unwrap();
    let dev = sio.output_device(dev_idx).unwrap();
    let mut stream = dev.create_outstream().unwrap();
    stream.open().unwrap();
    let sample_rate: u32 = if dev.supports_sample_rate(96_000) { 96_000 } else { 48_000 };
    stream.set_sample_rate(sample_rate);
    assert_eq!(stream.sample_rate(), sample_rate);
    let layout = stream.layout();
    assert_eq!(layout.channel_count(), 2);
    let cb = move |out: rsoundio::OutStream, min_frame_count: u32, max_frame_count: u32| {
        let l: Vec<f32> = (0..max_frame_count as usize)
                              .map(|i| {
                                  (i as f32 * ((2.0 * ::std::f32::consts::PI * 440.0) / sample_rate as f32))
                                      .sin()
                              })
                              .collect();
        let r = l.clone();
        let frames = vec![l, r];
        let frames_written = out.write_stream_f32(min_frame_count, &frames).unwrap();
        assert!(frames_written > 0);
    };
    stream.register_write_callback(cb);
    let ucb = |_: rsoundio::OutStream| println!("Underflow!");
    stream.register_underflow_callback(ucb);
    let ecb = |_: rsoundio::OutStream, err: rsoundio::SioError| println!("Error: {}", err);
    stream.register_error_callback(ecb);
    stream.start().unwrap();
    thread::sleep(Duration::new(1, 0));
    assert!(stream.pause().is_none());
    thread::sleep(Duration::new(1, 0));
    assert!(stream.unpause().is_none());
    assert!(stream.latency().is_ok());
    assert!(stream.clear_buffer().is_none());
    thread::sleep(Duration::new(1, 0));
}
