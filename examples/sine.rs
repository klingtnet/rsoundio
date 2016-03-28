extern crate rsoundio;
extern crate rb;

use rb::{RB, SpscRb, RbProducer, RbConsumer};
use std::f32::consts::PI as PI32;
use std::thread;
use std::time::Duration;

fn main() {
    let rb = SpscRb::new(4096);
    let (producer, consumer) = (rb.producer(), rb.consumer());
    // create an audio context
    let mut sio = rsoundio::SoundIo::new();
    sio.set_name("rsoundio-example").unwrap();
    // connect to the default audio backend
    sio.connect().unwrap();
    let backend = sio.current_backend().unwrap();
    println!("Connected to backend: {}", backend);
    sio.flush_events();
    // get default output device
    let dev = sio.default_output_device().unwrap();
    assert!(dev.probe_error().is_none());
    println!("Using output device: {}", dev);
    // create output stream
    let mut out = dev.create_outstream().unwrap();
    assert!(out.set_name("sine").is_ok());
    out.set_format(rsoundio::SioFormat::Float32LE).unwrap();
    println!("Output format: {}", out.format().unwrap());

    thread::spawn(move || {
        const LEN: usize = 512;
        let mut pos = 0;
        loop {
            let f = 440f32;
            let w = 2.0 * f * PI32 / 48_000.0;
            let cycle =  (48_000f32 / f) as usize;

            let samples: Vec<f32> = (0..LEN)
                                    .map(|i| (w * (i+pos) as f32).sin())
                                    .collect();
            producer.write_blocking(&samples).unwrap();
            pos = (pos + LEN) % cycle;
        }
    });

    // register callbacks
    out.register_write_callback(|out: rsoundio::OutStream,
                                 min_frame_count: u32,
                                 max_frame_count: u32| {
        const BUF_SIZE: usize = 2048;
        let mut data = vec![0.0f32; BUF_SIZE];
        let len = if backend != rsoundio::SioBackend::PulseAudio {
            ::std::cmp::min(BUF_SIZE, max_frame_count as usize)
        } else {
            BUF_SIZE
        };
        consumer.read_blocking(&mut data[..len]);
        let frames = vec![data[..len].iter().cloned().collect(),
                          data[..len].iter().cloned().collect()];
        out.write_stream_f32(min_frame_count, &frames).unwrap();
    });
    out.register_underflow_callback(|out: rsoundio::OutStream| {
        println!("Underflow in {} occured!", out.name().unwrap())
    });
    out.register_error_callback(|out: rsoundio::OutStream, err: rsoundio::SioError| {
        println!("{} error: {}", out.name().unwrap(), err)
    });

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
}
