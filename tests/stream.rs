extern crate rsoundio;

use std::os::raw::c_int;

#[test]
fn test_outstream() {
    println!("test: OutStream");
    let sio = rsoundio::SoundIo::new();
    assert!(sio.connect().is_none());
    // assert!(sio.connect_backend(rsoundio::ffi::SioBackend::Alsa).is_none());
    println!("current backend: {}", sio.current_backend().unwrap());
    sio.flush_events();
    let mut fmt;
    let dev_idx = sio.default_output_device_index().unwrap();
    let dev = sio.get_output_device(dev_idx).unwrap();
    println!("device: {}, ref_count: {}", dev, dev.ref_count());
    let mut stream = dev.create_outstream().unwrap();
    assert!(stream.open().is_none());
    fmt = stream.current_format().unwrap();
    println!("current format: {}", fmt);
    let cb = |out: rsoundio::OutStream, min: i32, max: i32| {
        println!("Closure: {}, {} using format: {}", min, max, fmt);
    };
    let boxed_cb = Box::new(cb);
    stream.register_write_callback(boxed_cb);
    let ucb = | out: rsoundio::OutStream | {
        println!("Underflow!")
    };
    stream.register_underflow_callback(Box::new(ucb));
    let ecb = | out: rsoundio::OutStream, err: rsoundio::ffi::SioError| {
        println!("Error: {}", err)
    };
    stream.register_error_callback(Box::new(ecb));
    assert!(stream.start().is_none());
    loop {
        sio.wait_events();
    }
    assert!(stream.clear_buffer().is_none());
    assert!(stream.pause(true).is_none());
    assert!(stream.pause(false).is_none());
    println!("latency: {}", stream.get_latency().unwrap());
    stream.destroy();
}

unsafe extern "C" fn write_callback(raw_out: *mut rsoundio::ffi::SoundIoOutStream,
                                    min_frame_count: c_int,
                                    max_frame_count: c_int) {
    let out = rsoundio::OutStream::new(raw_out);
    let sr = out.get_sample_rate();
    let layout = out.get_layout();
    let channels = layout.channel_count();
    // osc stuff:
    let mut f = 8_000.0f32;
    // let phi = 2.0 * f * ::std::f32::consts::PI / (sr as f32);
    let mut sample = 0.0f32;

    let mut raw_areas: *mut rsoundio::ffi::SoundIoChannelArea = ::std::ptr::null_mut();
    let mut frames_left = max_frame_count;
    let mut block_size = frames_left;
    while frames_left > 0 {
        if let Some(err) = out.begin_write(&mut raw_areas, &mut block_size) {
            panic!("{}", err);
        }
        if block_size <= 0 {
            break;
        }

        let areas = unsafe { ::std::slice::from_raw_parts_mut(raw_areas, channels as usize) };
        let mut dir = 1.0;
        for idx in 0..block_size {
            if f >= 8_000.0 || f <= 60.0 {
                dir *= -1.0
            };
            f += dir * 1.0;
            let phi = 2.0 * f * ::std::f32::consts::PI / (sr as f32);
            sample = (phi * (idx as f32)).sin();
            assert!(sample.abs() < 1.001);
            for ch_idx in 0..channels {
                let addr = (areas[ch_idx as usize].ptr as usize +
                            areas[ch_idx as usize].step as usize *
                            idx as usize) as *mut f32;
                unsafe { *addr = sample };
            }
        }
        assert!(out.end_write().is_none());
        frames_left -= block_size;
    }
}
