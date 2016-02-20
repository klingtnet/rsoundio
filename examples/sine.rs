extern crate rsoundio;

use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

fn main() {
    let f = 440f32;
    let cycle_len = (48_000 as f32 / f) as usize;
    let phi = 2.0 * f * PI / 48_000 as f32;
    let mut pos = 0;
    let samples: Vec<f32> = (pos..cycle_len)
                                .map(|i| (phi * i as f32).sin())
                                .collect();
    // create an audio context
    let sio = rsoundio::SoundIo::new();
    sio.set_app_name("rsoundio").unwrap();
    // connect to the default audio backend
    sio.connect().unwrap();
    println!("Connected to backend: {}", sio.current_backend().unwrap());
    sio.flush_events();
    // get default output device
    let dev = sio.default_output_device().unwrap();
    assert!(dev.probe_error().is_none());
    println!("Using output device: {}", dev);
    // create output stream
    let mut out = dev.create_outstream().unwrap();
    assert!(out.set_name("rsoundio-example-sine").is_ok());
    out.set_format(rsoundio::ffi::SioFormat::Float32LE).unwrap();
    println!("Output format: {}", out.format().unwrap());

    // register callbacks
    out.register_write_callback(Box::new(|out: rsoundio::OutStream,
                                          min_frame_count: i32,
                                          max_frame_count: i32| {
        let l: Vec<f32> = samples.iter()
                                 .cycle()
                                 .take(max_frame_count as usize + pos)
                                 .skip(pos)
                                 .map(|s| *s)
                                 .collect();
        pos = (max_frame_count as usize + pos) % cycle_len;
        let r = l.clone();
        let frames = vec![l, r];
        out.write_stream_f32(min_frame_count, &frames).unwrap();
    }));
    out.register_underflow_callback(Box::new(|out: rsoundio::OutStream| {
        println!("Underflow in {} occured!", out.name().unwrap())
    }));
    out.register_error_callback(Box::new(|out: rsoundio::OutStream,
                                          err: rsoundio::ffi::SioError| {
        println!("{} error: {}", out.name().unwrap(), err)
    }));

    // open output stream
    out.open().unwrap();
    let sr = out.sample_rate();
    println!("Sample rate: {}", sr);

    let layout = out.layout();
    println!("Output channel layout: {}", layout);
    // start audio output (now the `write_callback` will be called periodically)
    out.start().unwrap();
    thread::sleep(Duration::new(3, 0));
    println!("Pause for 1s");
    out.pause();
    thread::sleep(Duration::new(1, 0));
    println!("Unpausing");
    out.unpause();
    thread::sleep(Duration::new(3, 0));
    out.destroy()
}
