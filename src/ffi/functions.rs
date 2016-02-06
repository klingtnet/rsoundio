use std::os::raw::{c_int, c_double, c_void, c_char};

use ffi::enums::*;
use ffi::structs::*;

#[link(name = "soundio")]
extern "C" {
    pub fn soundio_create() -> *mut SoundIo;
    pub fn soundio_destroy(soundio: *mut SoundIo);
    pub fn soundio_connect(soundio: *mut SoundIo) -> SioError;
    pub fn soundio_connect_backend(soundio: *mut SoundIo,
                                   backend: SioBackend)
                                   -> SioError;
    pub fn soundio_disconnect(soundio: *mut SoundIo);
    pub fn soundio_strerror(error: SioError) -> *const c_char;
    pub fn soundio_backend_name(backend: SioBackend) -> *const c_char;
    pub fn soundio_backend_count(soundio: *mut SoundIo) -> c_int;
    pub fn soundio_get_backend(soundio: *mut SoundIo, index: c_int) -> SioBackend;
    pub fn soundio_have_backend(backend: SioBackend) -> u8;
    pub fn soundio_flush_events(soundio: *mut SoundIo);
    pub fn soundio_wait_events(soundio: *mut SoundIo);
    pub fn soundio_wakeup(soundio: *mut SoundIo);
    pub fn soundio_force_device_scan(soundio: *mut SoundIo);
    pub fn soundio_channel_layout_equal(a: *const SoundIoChannelLayout,
                                        b: *const SoundIoChannelLayout)
                                        -> u8;
    pub fn soundio_get_channel_name(id: SioChannelId) -> *const c_char;
    pub fn soundio_parse_channel_id(str: *const c_char, str_len: c_int) -> SioChannelId;
    pub fn soundio_channel_layout_builtin_count() -> c_int;
    pub fn soundio_channel_layout_get_builtin(index: c_int) -> *const SoundIoChannelLayout;
    pub fn soundio_channel_layout_get_default(channel_count: c_int)
                                              -> *const SoundIoChannelLayout;
    pub fn soundio_channel_layout_find_channel(layout: *const SoundIoChannelLayout,
                                               channel: SioChannelId)
                                               -> c_int;
    pub fn soundio_channel_layout_detect_builtin(layout: *mut SoundIoChannelLayout) -> u8;
    pub fn soundio_best_matching_channel_layout(preferred_layouts:
                                                    *const SoundIoChannelLayout,
                                                preferred_layout_count:
                                                    c_int,
                                                available_layouts:
                                                    *const SoundIoChannelLayout,
                                                available_layout_count:
                                                    c_int)
     -> *const SoundIoChannelLayout;
    // TODO: I am not sure if I should implement the sort method. The benefit does not
    // justify the amount of work to get it done. `ChannelLayout` contains only
    // `*const SoundIoChannelLayout` pointer, so `transmute` must be used to
    // make the `*mut` pointer.
    pub fn soundio_sort_channel_layouts(layouts: *mut SoundIoChannelLayout,
                                        layout_count: c_int);
    pub fn soundio_get_bytes_per_sample(format: SioFormat) -> c_int;
    pub fn soundio_format_string(format: SioFormat) -> *const c_char;
    pub fn soundio_input_device_count(soundio: *mut SoundIo) -> c_int;
    pub fn soundio_output_device_count(soundio: *mut SoundIo) -> c_int;
    pub fn soundio_get_input_device(soundio: *mut SoundIo,
                                    index: c_int)
                                    -> *mut SoundIoDevice;
    pub fn soundio_get_output_device(soundio: *mut SoundIo,
                                     index: c_int)
                                     -> *mut SoundIoDevice;
    pub fn soundio_default_input_device_index(soundio: *mut SoundIo) -> c_int;
    pub fn soundio_default_output_device_index(soundio: *mut SoundIo) -> c_int;
    pub fn soundio_device_ref(device: *mut SoundIoDevice);
    pub fn soundio_device_unref(device: *mut SoundIoDevice);
    pub fn soundio_device_equal(a: *const SoundIoDevice,
                                b: *const SoundIoDevice)
                                -> u8;
    pub fn soundio_device_sort_channel_layouts(device: *mut SoundIoDevice);
    pub fn soundio_device_supports_format(device: *mut SoundIoDevice,
                                          format: SioFormat)
                                          -> u8;
    pub fn soundio_device_supports_layout(device: *mut SoundIoDevice,
                                          layout: *const SoundIoChannelLayout)
                                          -> u8;
    pub fn soundio_device_supports_sample_rate(device: *mut SoundIoDevice,
                                               sample_rate: c_int)
                                               -> u8;
    pub fn soundio_device_nearest_sample_rate(device: *mut SoundIoDevice,
                                              sample_rate: c_int)
                                              -> c_int;
    pub fn soundio_outstream_create(device: *mut SoundIoDevice)
                                    -> *mut SoundIoOutStream;
    pub fn soundio_outstream_destroy(outstream: *mut SoundIoOutStream);
    pub fn soundio_outstream_open(outstream: *mut SoundIoOutStream) -> SioError;
    pub fn soundio_outstream_start(outstream: *mut SoundIoOutStream) -> SioError;
    pub fn soundio_outstream_begin_write(outstream: *mut SoundIoOutStream,
                                         areas: *mut *mut SoundIoChannelArea,
                                         frame_count: *mut c_int)
                                         -> SioError;
    pub fn soundio_outstream_end_write(outstream: *mut SoundIoOutStream) -> SioError;
    pub fn soundio_outstream_clear_buffer(outstream: *mut SoundIoOutStream)
                                          -> SioError;
    pub fn soundio_outstream_pause(outstream: *mut SoundIoOutStream,
                                   pause: u8)
                                   -> SioError;
    pub fn soundio_outstream_get_latency(outstream: *mut SoundIoOutStream,
                                         out_latency: *mut c_double)
                                         -> SioError;
    pub fn soundio_instream_create(device: *mut SoundIoDevice) -> *mut SoundIoInStream;
    pub fn soundio_instream_destroy(instream: *mut SoundIoInStream);
    pub fn soundio_instream_open(instream: *mut SoundIoInStream) -> c_int;
    pub fn soundio_instream_start(instream: *mut SoundIoInStream) -> c_int;
    pub fn soundio_instream_begin_read(instream: *mut SoundIoInStream,
                                       areas: *mut *mut SoundIoChannelArea,
                                       frame_count: *mut c_int)
                                       -> c_int;
    pub fn soundio_instream_end_read(instream: *mut SoundIoInStream) -> c_int;
    pub fn soundio_instream_pause(instream: *mut SoundIoInStream, pause: u8) -> c_int;
    pub fn soundio_instream_get_latency(instream: *mut SoundIoInStream,
                                        out_latency: *mut c_double)
                                        -> c_int;
    pub fn soundio_ring_buffer_create(soundio: *mut SoundIo,
                                      requested_capacity: c_int)
                                      -> *mut SoundIoRingBuffer;
    pub fn soundio_ring_buffer_destroy(ring_buffer: *mut SoundIoRingBuffer);
    pub fn soundio_ring_buffer_capacity(ring_buffer: *mut SoundIoRingBuffer) -> c_int;
    pub fn soundio_ring_buffer_write_ptr(ring_buffer: *mut SoundIoRingBuffer) -> *mut c_char;
    pub fn soundio_ring_buffer_advance_write_ptr(ring_buffer: *mut SoundIoRingBuffer,
                                                 count: c_int);
    pub fn soundio_ring_buffer_read_ptr(ring_buffer: *mut SoundIoRingBuffer) -> *mut c_char;
    pub fn soundio_ring_buffer_advance_read_ptr(ring_buffer: *mut SoundIoRingBuffer,
                                                count: c_int);
    pub fn soundio_ring_buffer_fill_count(ring_buffer: *mut SoundIoRingBuffer) -> c_int;
    pub fn soundio_ring_buffer_free_count(ring_buffer: *mut SoundIoRingBuffer) -> c_int;
    pub fn soundio_ring_buffer_clear(ring_buffer: *mut SoundIoRingBuffer);
}
