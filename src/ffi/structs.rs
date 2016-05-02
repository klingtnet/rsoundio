use std::os::raw::{c_int, c_double, c_void, c_char};

use ffi::enums::*;

/// Represents a `ffi::enums::SioChannelLayoutId`. Contains
/// the number of channels in the layout, the channel id for each
/// channel and the name of the layout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoChannelLayout {
    pub name: *const c_char,
    pub channel_count: c_int,
    pub channels: [SioChannelId; 24usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoSampleRateRange {
    pub min: c_int,
    pub max: c_int,
}

/// Stores the base pointer of the sound buffer
/// and the step size to get to the base address
/// of the next sample.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoChannelArea {
    /// Base address of buffer.
    pub ptr: *mut c_char,
    /// How many bytes it takes to get from the beginning of one sample to
    /// the beginning of the next sample.
    pub step: c_int,
}

/// Base struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIo {
    /// Optional. Put whatever you want here. Defaults to NULL.
    pub userdata: *mut c_void,
    /// Optional callback. Called when the list of devices
    /// change. Only called during a call to
    /// If you do not supply a callback, the default will
    /// crash your program with an error message. This callback
    /// is also called when the thread that retrieves device
    /// information runs into an unrecoverable condition
    /// such as running out of memory.
    ///
    /// Possible errors:
    /// * #SoundIoErrorBackendDisconnected
    /// * #SoundIoErrorNoMem
    /// * #SoundIoErrorSystemResources
    /// * #SoundIoErrorOpeningDevice - unexpected problem accessing device
    ///   information
    /// ::soundio_flush_events or ::soundio_wait_events.
    pub on_devices_change: Option<unsafe extern "C" fn(arg1: *mut SoundIo)>,
    /// Optional callback. Called when the backend disconnects.
    /// For example, when the JACK server shuts down. When this
    /// happens, listing devices and opening streams will always
    /// fail with SoundIoErrorBackendDisconnected.
    pub on_backend_disconnect: Option<unsafe extern "C" fn(arg1: *mut SoundIo, err: c_int)>,
    /// Optional callback. Called from an unknown thread that
    /// you should not use to call any soundio functions.
    /// You may use this to signal a condition variable to wake up.
    /// Called when ::soundio_wait_events would be woken up.
    pub on_events_signal: Option<unsafe extern "C" fn(arg1: *mut SoundIo)>,
    /// Read-only. After calling ::soundio_connect or
    /// ::soundio_connect_backend, this field tells which
    /// backend is currently connected.
    pub current_backend: SioBackend,
    /// Optional: Application name.
    /// PulseAudio uses this for "application name".
    /// JACK uses this for `client_name`.
    /// Must not contain a colon (":").
    pub app_name: *const c_char,
    /// Optional: Real time priority warning.
    /// This callback is fired when making thread real-time
    /// priority failed. By default, it will print to stderr
    /// only the first time it is calle a message instructing
    /// the user how to configure their system to allow
    /// real-time priority threads. This must be set to a function
    /// not NULL. To silence the warning, assign this to a
    /// function that does nothing.
    pub emit_rtprio_warning: Option<extern "C" fn()>,
    /// Optional: JACK info callback.
    /// By default, libsoundio sets this to an empty function
    /// in order to silence stdio messages from JACK.
    /// You may override the behavior by setting this to `NULL`
    /// or providing your own function. This is registered with
    /// JACK regardless of whether ::soundio_connect_backend
    /// succeeds.
    pub jack_info_callback: Option<unsafe extern "C" fn(msg: *const c_char)>,
    /// Optional: JACK error callback.
    /// See SoundIo::jack_info_callback
    pub jack_error_callback: Option<unsafe extern "C" fn(msg: *const c_char)>,
}

/// Represents a sound device.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoDevice {
    /// Read-only. Set automatically.
    pub soundio: *mut SoundIo,
    /// A string of bytes that uniquely identifies this device.
    /// If the same physical device supports both input and output,
    /// that makes one SoundIoDevice for the input and one
    /// SoundIoDevice for the output.
    /// In this case, the id of each SoundIoDevice will be the same,
    /// and SoundIoDevice::aim will be different.
    /// Additionally, if the device supports raw mode,
    /// there may be up to four devices with the same id:
    /// one for each value of SoundIoDevice::is_raw and
    /// one for each value of SoundIoDevice::aim.
    pub id: *mut c_char,
    /// User-friendly UTF-8 encoded text to describe the device.
    pub name: *mut c_char,
    /// Tells whether this device is an input device or an output device.
    pub aim: SioDeviceAim,
    /// Channel layouts are handled similarly
    /// to SoundIoDevice::formats.
    /// If this information is missing due to
    /// a SoundIoDevice::probe_error, layouts will be NULL.
    /// It's OK to modify this data, for example calling
    /// ::soundio_sort_channel_layouts on it.
    /// Devices are guaranteed to have at least 1 channel layout.
    pub layouts: *mut SoundIoChannelLayout,
    /// See SoundIoDevice::current_format
    pub layout_count: c_int,
    pub current_layout: SoundIoChannelLayout,
    /// List of formats this device supports. See also
    /// SoundIoDevice::current_format.
    pub formats: *mut SioFormat,
    /// How many formats are available in SoundIoDevice::formats.
    pub format_count: c_int,
    /// A device is either a raw device or it is a virtual device that is
    /// provided by a software mixing service such as dmix or PulseAudio (see
    /// SoundIoDevice::is_raw). If it is a raw device,
    /// current_format is meaningless;
    /// the device has no current format until you open it. On the other hand,
    /// if it is a virtual device, current_format describes the
    /// destination sample format that your audio will be converted to. Or,
    /// if you're the lucky first application to open the device, you might
    /// cause the current_format to change to your format.
    /// Generally, you want to ignore current_format and use
    /// whatever format is most convenient
    /// for you which is supported by the device, because when you are the only
    /// application left, the mixer might decide to switch
    /// current_format to yours. You can learn the supported formats via
    /// formats and SoundIoDevice::format_count. If this information is missing
    /// due to a probe error, formats will be `NULL`. If current_format is
    /// unavailable, it will be set to #SoundIoFormatInvalid.
    /// Devices are guaranteed to have at least 1 format available.
    pub current_format: SioFormat,
    /// Sample rate is the number of frames per second.
    /// Sample rate is handled very similar to SoundIoDevice::formats.
    /// If sample rate information is missing due to a probe error, the field
    /// will be set to NULL.
    /// Devices which have SoundIoDevice::probe_error set to #SoundIoErrorNone are
    /// guaranteed to have at least 1 sample rate available.
    pub sample_rates: *mut SoundIoSampleRateRange,
    /// How many sample rate ranges are available in
    /// SoundIoDevice::sample_rates. 0 if sample rate information is missing
    /// due to a probe error.
    pub sample_rate_count: c_int,
    /// See SoundIoDevice::current_format
    /// 0 if sample rate information is missing due to a probe error.
    pub sample_rate_current: c_int,
    /// Software latency minimum in seconds. If this value is unknown or
    /// irrelevant, it is set to 0.0.
    /// For PulseAudio and WASAPI this value is unknown until you open a
    /// stream.
    pub software_latency_min: c_double,
    /// Software latency maximum in seconds. If this value is unknown or
    /// irrelevant, it is set to 0.0.
    /// For PulseAudio and WASAPI this value is unknown until you open a
    /// stream.
    pub software_latency_max: c_double,
    /// Software latency in seconds. If this value is unknown or
    /// irrelevant, it is set to 0.0.
    /// For PulseAudio and WASAPI this value is unknown until you open a
    /// stream.
    /// See SoundIoDevice::current_format
    pub software_latency_current: c_double,
    /// Raw means that you are directly opening the hardware device and not
    /// going through a proxy such as dmix, PulseAudio, or JACK. When you open a
    /// raw device, other applications on the computer are not able to
    /// simultaneously access the device. Raw devices do not perform automatic
    /// resampling and thus tend to have fewer formats available.
    pub is_raw: u8,
    /// Devices are reference counted. See ::soundio_device_ref and
    /// ::soundio_device_unref.
    pub ref_count: c_int,
    /// This is set to a SoundIoError representing the result of the device
    /// probe. Ideally this will be SoundIoErrorNone in which case all the
    /// fields of the device will be populated. If there is an error code here
    /// then information about formats, sample rates, and channel layouts might
    /// be missing.
    ///
    /// Possible errors:
    /// * #SoundIoErrorOpeningDevice
    /// * #SoundIoErrorNoMem
    pub probe_error: SioError,
}

/// Represents an audio output stream.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoOutStream {
    /// Populated automatically when you call ::soundio_outstream_create.
    pub device: *mut SoundIoDevice,
    /// Defaults to #SoundIoFormatFloat32NE, followed by the first one
    /// supported.
    pub format: SioFormat,
    /// Sample rate is the number of frames per second.
    /// Defaults to 48000 (and then clamped into range).
    pub sample_rate: c_int,
    /// Defaults to Stereo, if available, followed by the first layout
    /// supported.
    pub layout: SoundIoChannelLayout,
    /// Ignoring hardware latency, this is the number of seconds it takes for
    /// the last sample in a full buffer to be played.
    /// After you call ::soundio_outstream_open, this value is replaced with the
    /// actual software latency, as near to this value as possible.
    /// On systems that support clearing the buffer, this defaults to a large
    /// latency, potentially upwards of 2 seconds, with the understanding that
    /// you will call ::soundio_outstream_clear_buffer when you want to reduce
    /// the latency to 0. On systems that do not support clearing the buffer,
    /// this defaults to a reasonable lower latency value.
    ///
    /// On backends with high latencies (such as 2 seconds), `frame_count_min`
    /// will be 0, meaning you don't have to fill the entire buffer. In this
    /// case, the large buffer is there if you want it; you only have to fill
    /// as much as you want. On backends like JACK, `frame_count_min` will be
    /// equal to `frame_count_max` and if you don't fill that many frames, you
    /// will get glitches.
    ///
    /// If the device has unknown software latency min and max values, you may
    /// still set this, but you might not get the value you requested.
    /// For PulseAudio, if you set this value to non-default, it sets
    /// `PA_STREAM_ADJUST_LATENCY` and is the value used for `maxlength` and
    /// `tlength`.
    ///
    /// For JACK, this value is always equal to
    /// SoundIoDevice::software_latency_current of the device.
    pub software_latency: c_double,
    /// Defaults to NULL. Put whatever you want here.
    pub userdata: *mut c_void,
    /// In this callback, you call ::soundio_outstream_begin_write and
    /// ::soundio_outstream_end_write as many times as necessary to write
    /// at minimum `frame_count_min` frames and at maximum `frame_count_max`
    /// frames. `frame_count_max` will always be greater than 0. Note that you
    /// should write as many frames as you can; `frame_count_min` might be 0 and
    /// you can still get a buffer underflow if you always write
    /// `frame_count_min` frames.
    ///
    /// For Dummy, ALSA, and PulseAudio, `frame_count_min` will be 0. For JACK
    /// and CoreAudio `frame_count_min` will be equal to `frame_count_max`.
    ///
    /// The code in the supplied function must be suitable for real-time
    /// execution. That means that it cannot call functions that might block
    /// for a long time. This includes all I/O functions (disk, TTY, network),
    /// malloc, free, printf, pthread_mutex_lock, sleep, wait, poll, select,
    /// pthread_join, pthread_cond_wait, etc.
    pub write_callback: Option<extern "C" fn(arg1: *mut SoundIoOutStream,
                                                 frame_count_min: c_int,
                                                 frame_count_max: c_int)
                                                >,
    /// This optional callback happens when the sound device runs out of
    /// buffered audio data to play. After this occurs, the outstream waits
    /// until the buffer is full to resume playback.
    /// This is called from the SoundIoOutStream::write_callback thread context.
    pub underflow_callback: Option<extern "C" fn(arg1: *mut SoundIoOutStream)>,
    /// Optional callback. `err` is always SoundIoErrorStreaming.
    /// SoundIoErrorStreaming is an unrecoverable error. The stream is in an
    /// invalid state and must be destroyed.
    /// If you do not supply error_callback, the default callback will print
    /// a message to stderr and then call `abort`.
    /// This is called from the SoundIoOutStream::write_callback thread context.
    pub error_callback: Option<extern "C" fn(arg1: *mut SoundIoOutStream, err: SioError)>,
    /// Optional: Name of the stream. Defaults to "SoundIoOutStream"
    /// PulseAudio uses this for the stream name.
    /// JACK uses this for the client name of the client that connects when you
    /// open the stream.
    /// WASAPI uses this for the session display name.
    /// Must not contain a colon (":").
    pub name: *const c_char,
    /// Optional: Hint that this output stream is nonterminal. This is used by
    /// JACK and it means that the output stream data originates from an input
    /// stream. Defaults to `false`.
    pub non_terminal_hint: u8,
    /// computed automatically when you call ::soundio_outstream_open
    pub bytes_per_frame: c_int,
    /// computed automatically when you call ::soundio_outstream_open
    pub bytes_per_sample: c_int,
    /// If setting the channel layout fails for some reason, this field is set
    /// to an error code. Possible error codes are:
    /// * #SoundIoErrorIncompatibleDevice
    pub layout_error: c_int,
}

/// Represents an audio input stream.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoInStream {
    /// Populated automatically when you call ::soundio_outstream_create.
    pub device: *mut SoundIoDevice,
    /// Defaults to #SoundIoFormatFloat32NE, followed by the first one
    /// supported.
    pub format: SioFormat,
    /// Sample rate is the number of frames per second.
    /// Defaults to max(sample_rate_min, min(sample_rate_max, 48000))
    pub sample_rate: c_int,
    /// Defaults to Stereo, if available, followed by the first layout
    /// supported.
    pub layout: SoundIoChannelLayout,
    /// Ignoring hardware latency, this is the number of seconds it takes for a
    /// captured sample to become available for reading.
    /// After you call ::soundio_instream_open, this value is replaced with the
    /// actual software latency, as near to this value as possible.
    /// A higher value means less CPU usage. Defaults to a large value,
    /// potentially upwards of 2 seconds.
    /// If the device has unknown software latency min and max values, you may
    /// still set this, but you might not get the value you requested.
    /// For PulseAudio, if you set this value to non-default, it sets
    /// `PA_STREAM_ADJUST_LATENCY` and is the value used for `fragsize`.
    /// For JACK, this value is always equal to
    /// SoundIoDevice::software_latency_current
    pub software_latency: c_double,
    /// Defaults to NULL. Put whatever you want here.
    pub userdata: *mut c_void,
    /// In this function call ::soundio_instream_begin_read and
    /// ::soundio_instream_end_read as many times as necessary to read at
    /// minimum `frame_count_min` frames and at maximum `frame_count_max`
    /// frames. If you return from read_callback without having read
    /// `frame_count_min`, the frames will be dropped. `frame_count_max` is how
    /// many frames are available to read.
    ///
    /// The code in the supplied function must be suitable for real-time
    /// execution. That means that it cannot call functions that might block
    /// for a long time. This includes all I/O functions (disk, TTY, network),
    /// malloc, free, printf, pthread_mutex_lock, sleep, wait, poll, select,
    /// pthread_join, pthread_cond_wait, etc.
    pub read_callback: Option<unsafe extern "C" fn(arg1: *mut SoundIoInStream,
                                                       frame_count_min: c_int,
                                                       frame_count_max: c_int)
                                                      >,
    /// This optional callback happens when the sound device buffer is full,
    /// yet there is more captured audio to put in it.
    /// This is never fired for PulseAudio.
    /// This is called from the SoundIoInStream::read_callback thread context.
    pub overflow_callback: Option<unsafe extern "C" fn(arg1: *mut SoundIoInStream)>,
    /// Optional callback. `err` is always SoundIoErrorStreaming.
    /// SoundIoErrorStreaming is an unrecoverable error. The stream is in an
    /// invalid state and must be destroyed.
    /// If you do not supply `error_callback`, the default callback will print
    /// a message to stderr and then abort().
    /// This is called from the SoundIoInStream::read_callback thread context.
    pub error_callback: Option<unsafe extern "C" fn(arg1: *mut SoundIoInStream, err: c_int)>,
    /// Optional: Name of the stream. Defaults to "SoundIoInStream";
    /// PulseAudio uses this for the stream name.
    /// JACK uses this for the client name of the client that connects when you
    /// open the stream.
    /// WASAPI uses this for the session display name.
    /// Must not contain a colon (":").
    pub name: *const c_char,
    /// Optional: Hint that this input stream is nonterminal. This is used by
    /// JACK and it means that the data received by the stream will be
    /// passed on or made available to another stream. Defaults to `false`.
    pub non_terminal_hint: u8,
    /// computed automatically when you call ::soundio_instream_open
    pub bytes_per_frame: c_int,
    /// computed automatically when you call ::soundio_instream_open
    pub bytes_per_sample: c_int,
    /// If setting the channel layout fails for some reason, this field is set
    /// to an error code. Possible error codes are: #SoundIoErrorIncompatibleDevice
    pub layout_error: SioError,
}
