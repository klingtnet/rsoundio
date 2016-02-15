use std::os::raw::{c_int, c_double, c_char};

use ffi::enums::*;
use ffi::structs::*;

#[link(name = "soundio")]
extern "C" {
    /// Create a SoundIo context. You may create multiple instances of this to
    /// connect to multiple backends. Sets all fields to defaults.
    /// Returns `NULL` if and only if memory could not be allocated.
    /// See also ::soundio_destroy
    pub fn soundio_create() -> *mut SoundIo;
    pub fn soundio_destroy(soundio: *mut SoundIo);
    /// Tries ::soundio_connect_backend on all available backends in order.
    /// Possible errors:
    /// * #SoundIoErrorInvalid - already connected
    /// * #SoundIoErrorNoMem
    /// * #SoundIoErrorSystemResources
    /// * #SoundIoErrorNoSuchClient - when JACK returns `JackNoSuchClient`
    /// See also ::soundio_disconnect
    pub fn soundio_connect(soundio: *mut SoundIo) -> SioError;
    /// Instead of calling ::soundio_connect you may call this function to try a
    /// specific backend.
    /// Possible errors:
    /// * #SoundIoErrorInvalid - already connected or invalid backend parameter
    /// * #SoundIoErrorNoMem
    /// * #SoundIoErrorBackendUnavailable - backend was not compiled in
    /// * #SoundIoErrorSystemResources
    /// * #SoundIoErrorNoSuchClient - when JACK returns `JackNoSuchClient`
    /// * #SoundIoErrorInitAudioBackend - requested `backend` is not active
    /// * #SoundIoErrorBackendDisconnected - backend disconnected while connecting
    /// See also ::soundio_disconnect
    pub fn soundio_connect_backend(soundio: *mut SoundIo, backend: SioBackend) -> SioError;
    pub fn soundio_disconnect(soundio: *mut SoundIo);
    /// Get a string representation of a #SoundIoError
    pub fn soundio_strerror(error: SioError) -> *const c_char;
    /// Get a string representation of a #SoundIoBackend
    pub fn soundio_backend_name(backend: SioBackend) -> *const c_char;
    /// Returns the number of available backends.
    pub fn soundio_backend_count(soundio: *mut SoundIo) -> c_int;
    /// get the available backend at the specified index
    /// (0 <= index < ::soundio_backend_count)
    pub fn soundio_get_backend(soundio: *mut SoundIo, index: c_int) -> SioBackend;
    pub fn soundio_have_backend(backend: SioBackend) -> u8;
    /// Atomically update information for all connected devices. Note that calling
    /// this function merely flips a pointer; the actual work of collecting device
    /// information is done elsewhere. It is performant to call this function many
    /// times per second.
    ///
    /// When you call this, the following callbacks might be called:
    /// * SoundIo::on_devices_change
    /// * SoundIo::on_backend_disconnect
    /// This is the only time those callbacks can be called.
    ///
    /// This must be called from the same thread as the thread in which you call
    /// these functions:
    /// * ::soundio_input_device_count
    /// * ::soundio_output_device_count
    /// * ::soundio_get_input_device
    /// * ::soundio_get_output_device
    /// * ::soundio_default_input_device_index
    /// * ::soundio_default_output_device_index
    ///
    /// Note that if you do not care about learning about updated devices, you
    /// might call this function only once ever and never call
    /// ::soundio_wait_events.
    pub fn soundio_flush_events(soundio: *mut SoundIo);
    /// This function calls ::soundio_flush_events then blocks until another event
    /// is ready or you call ::soundio_wakeup. Be ready for spurious wakeups.
    pub fn soundio_wait_events(soundio: *mut SoundIo);
    /// Makes ::soundio_wait_events stop blocking.
    pub fn soundio_wakeup(soundio: *mut SoundIo);
    /// If necessary you can manually trigger a device rescan. Normally you will
    /// not ever have to call this function, as libsoundio listens to system events
    /// for device changes and responds to them by rescanning devices and preparing
    /// the new device information for you to be atomically replaced when you call
    /// ::soundio_flush_events. However you might run into cases where you want to
    /// force trigger a device rescan, for example if an ALSA device has a
    /// SoundIoDevice::probe_error.
    ///
    /// After you call this you still have to use ::soundio_flush_events or
    /// ::soundio_wait_events and then wait for the
    /// SoundIo::on_devices_change callback.
    ///
    /// This can be called from any thread context except for
    /// SoundIoOutStream::write_callback and SoundIoInStream::read_callback
    pub fn soundio_force_device_scan(soundio: *mut SoundIo);
    /// Returns whether the channel count field and each channel id matches in
    /// the supplied channel layouts.
    pub fn soundio_channel_layout_equal(a: *const SoundIoChannelLayout,
                                        b: *const SoundIoChannelLayout)
                                        -> u8;
    pub fn soundio_get_channel_name(id: SioChannelId) -> *const c_char;
    /// Given UTF-8 encoded text which is the name of a channel such as
    /// "Front Left", "FL", or "front-left", return the corresponding
    /// SoundIoChannelId. Returns SoundIoChannelIdInvalid for no match.
    pub fn soundio_parse_channel_id(str: *const c_char, str_len: c_int) -> SioChannelId;
    /// Returns the number of builtin channel layouts.
    pub fn soundio_channel_layout_builtin_count() -> c_int;
    /// Returns a builtin channel layout. 0 <= `index` < ::soundio_channel_layout_builtin_count
    ///
    /// Although `index` is of type `int`, it should be a valid
    /// #SoundIoChannelLayoutId enum value.
    pub fn soundio_channel_layout_get_builtin(index: c_int) -> *const SoundIoChannelLayout;
    /// Get the default builtin channel layout for the given number of channels.
    pub fn soundio_channel_layout_get_default(channel_count: c_int) -> *const SoundIoChannelLayout;
    /// Return the index of `channel` in `layout`, or `-1` if not found.
    pub fn soundio_channel_layout_find_channel(layout: *const SoundIoChannelLayout,
                                               channel: SioChannelId)
                                               -> c_int;
    /// Populates the name field of layout if it matches a builtin one.
    /// returns whether it found a match
    pub fn soundio_channel_layout_detect_builtin(layout: *mut SoundIoChannelLayout) -> u8;
    /// Iterates over preferred_layouts. Returns the first channel layout in
    /// preferred_layouts which matches one of the channel layouts in
    /// available_layouts. Returns NULL if none matches.
    pub fn soundio_best_matching_channel_layout(preferred_layouts: *const SoundIoChannelLayout,
                                                preferred_layout_count: c_int,
                                                available_layouts: *const SoundIoChannelLayout,
                                                available_layout_count: c_int)
                                                -> *const SoundIoChannelLayout;
    /// Sorts by channel count, descending.
    /// TODO: I am not sure if I should implement the sort method. The benefit does not
    /// justify the amount of work to get it done. `ChannelLayout` contains only
    /// `*const SoundIoChannelLayout` pointer, so `transmute` must be used to
    /// make the `*mut` pointer.
    pub fn soundio_sort_channel_layouts(layouts: *mut SoundIoChannelLayout, layout_count: c_int);
    /// Returns -1 on invalid format.
    pub fn soundio_get_bytes_per_sample(format: SioFormat) -> c_int;
    /// Returns string representation of `format`.
    pub fn soundio_format_string(format: SioFormat) -> *const c_char;
    /// When you call ::soundio_flush_events, a snapshot of all device state is
    /// saved and these functions merely access the snapshot data. When you want
    /// to check for new devices, call ::soundio_flush_events. Or you can call
    /// ::soundio_wait_events to block until devices change. If an error occurs
    /// scanning devices in a background thread, SoundIo::on_backend_disconnect is called
    /// with the error code.

    /// Get the number of input devices.
    /// Returns -1 if you never called ::soundio_flush_events.
    pub fn soundio_input_device_count(soundio: *mut SoundIo) -> c_int;
    /// Get the number of output devices.
    /// Returns -1 if you never called ::soundio_flush_events.
    pub fn soundio_output_device_count(soundio: *mut SoundIo) -> c_int;
    /// Always returns a device. Call ::soundio_device_unref when done.
    /// `index` must be 0 <= index < ::soundio_input_device_count
    /// Returns NULL if you never called ::soundio_flush_events or if you provide
    /// invalid parameter values.
    pub fn soundio_get_input_device(soundio: *mut SoundIo, index: c_int) -> *mut SoundIoDevice;
    /// Always returns a device. Call ::soundio_device_unref when done.
    /// `index` must be 0 <= index < ::soundio_output_device_count
    /// Returns NULL if you never called ::soundio_flush_events or if you provide
    /// invalid parameter values.
    pub fn soundio_get_output_device(soundio: *mut SoundIo, index: c_int) -> *mut SoundIoDevice;
    /// returns the index of the default input device
    /// returns -1 if there are no devices or if you never called
    /// ::soundio_flush_events.
    pub fn soundio_default_input_device_index(soundio: *mut SoundIo) -> c_int;
    /// returns the index of the default output device
    /// returns -1 if there are no devices or if you never called
    /// ::soundio_flush_events.
    pub fn soundio_default_output_device_index(soundio: *mut SoundIo) -> c_int;
    /// Add 1 to the reference count of `device`.
    pub fn soundio_device_ref(device: *mut SoundIoDevice);
    /// Remove 1 to the reference count of `device`. Clean up if it was the last
    /// reference.
    pub fn soundio_device_unref(device: *mut SoundIoDevice);
    /// Return `true` if and only if the devices have the same SoundIoDevice::id,
    /// SoundIoDevice::is_raw, and SoundIoDevice::aim are the same.
    pub fn soundio_device_equal(a: *const SoundIoDevice, b: *const SoundIoDevice) -> u8;
    /// Sorts channel layouts by channel count, descending.
    pub fn soundio_device_sort_channel_layouts(device: *mut SoundIoDevice);
    /// Convenience function. Returns whether `format` is included in the device's
    /// supported formats.
    pub fn soundio_device_supports_format(device: *mut SoundIoDevice, format: SioFormat) -> u8;
    /// Convenience function. Returns whether `layout` is included in the device's
    /// supported channel layouts.
    pub fn soundio_device_supports_layout(device: *mut SoundIoDevice,
                                          layout: *const SoundIoChannelLayout)
                                          -> u8;
    /// Convenience function. Returns whether `sample_rate` is included in the
    /// device's supported sample rates.
    pub fn soundio_device_supports_sample_rate(device: *mut SoundIoDevice,
                                               sample_rate: c_int)
                                               -> u8;
    /// Convenience function. Returns the available sample rate nearest to
    /// `sample_rate`, rounding up.
    pub fn soundio_device_nearest_sample_rate(device: *mut SoundIoDevice,
                                              sample_rate: c_int)
                                              -> c_int;
    /// Allocates memory and sets defaults. Next you should fill out the struct fields
    /// and then call ::soundio_outstream_open. Sets all fields to defaults.
    /// Returns `NULL` if and only if memory could not be allocated.
    /// See also ::soundio_outstream_destroy
    pub fn soundio_outstream_create(device: *mut SoundIoDevice) -> *mut SoundIoOutStream;
    /// You may not call this function from the SoundIoOutStream::write_callback thread context.
    pub fn soundio_outstream_destroy(outstream: *mut SoundIoOutStream);
    /// After you call this function, SoundIoOutStream::software_latency is set to
    /// the correct value.
    ///
    /// The next thing to do is call ::soundio_instream_start.
    /// If this function returns an error, the outstream is in an invalid state and
    /// you must call ::soundio_outstream_destroy on it.
    ///
    /// Possible errors:
    /// * #SoundIoErrorInvalid
    ///   * SoundIoDevice::aim is not #SoundIoDeviceAimOutput
    ///   * SoundIoOutStream::format is not valid
    ///   * SoundIoOutStream::channel_count is greater than #SOUNDIO_MAX_CHANNELS
    /// * #SoundIoErrorNoMem
    /// * #SoundIoErrorOpeningDevice
    /// * #SoundIoErrorBackendDisconnected
    /// * #SoundIoErrorSystemResources
    /// * #SoundIoErrorNoSuchClient - when JACK returns `JackNoSuchClient`
    /// * #SoundIoErrorOpeningDevice
    /// * #SoundIoErrorIncompatibleBackend - SoundIoOutStream::channel_count is
    ///   greater than the number of channels the backend can handle.
    /// * SoundIoErrorIncompatibleDevice - stream parameters requested are not
    ///   compatible with the chosen device.
    pub fn soundio_outstream_open(outstream: *mut SoundIoOutStream) -> SioError;
    /// After you call this function, SoundIoOutStream::write_callback will be called.
    ///
    /// This function might directly call SoundIoOutStream::write_callback.
    ///
    /// Possible errors:
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorNoMem
    /// * #SoundIoErrorSystemResources
    /// * #SoundIoErrorBackendDisconnected
    pub fn soundio_outstream_start(outstream: *mut SoundIoOutStream) -> SioError;
    /// Call this function when you are ready to begin writing to the device buffer.
    ///  * `outstream` - (in) The output stream you want to write to.
    ///  * `areas` - (out) The memory addresses you can write data to, one per
    ///    channel. It is OK to modify the pointers if that helps you iterate.
    ///  * `frame_count` - (in/out) Provide the number of frames you want to write.
    ///    Returned will be the number of frames you can actually write, which is
    ///    also the number of frames that will be written when you call
    ///    ::soundio_outstream_end_write. The value returned will always be less
    ///    than or equal to the value provided.
    /// It is your responsibility to call this function exactly as many times as
    /// necessary to meet the `frame_count_min` and `frame_count_max` criteria from
    /// SoundIoOutStream::write_callback.
    /// You must call this function only from the SoundIoOutStream::write_callback thread context.
    /// After calling this function, write data to `areas` and then call
    /// ::soundio_outstream_end_write.
    /// If this function returns an error, do not call ::soundio_outstream_end_write.
    ///
    /// Possible errors:
    /// * #SoundIoErrorInvalid
    ///   * `*frame_count` <= 0
    ///   * `*frame_count` < `frame_count_min` or `*frame_count` > `frame_count_max`
    ///   * function called too many times without respecting `frame_count_max`
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorUnderflow - an underflow caused this call to fail. You might
    ///   also get a SoundIoOutStream::underflow_callback, and you might not get
    ///   this error code when an underflow occurs. Unlike #SoundIoErrorStreaming,
    ///   the outstream is still in a valid state and streaming can continue.
    /// * SoundIoErrorIncompatibleDevice - in rare cases it might just now
    ///   be discovered that the device uses non-byte-aligned access, in which
    ///   case this error code is returned.
    pub fn soundio_outstream_begin_write(outstream: *mut SoundIoOutStream,
                                         areas: *mut *mut SoundIoChannelArea,
                                         frame_count: *mut c_int)
                                         -> SioError;
    /// Commits the write that you began with ::soundio_outstream_begin_write.
    /// You must call this function only from the SoundIoOutStream::write_callback thread context.
    ///
    /// Possible errors:
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorUnderflow - an underflow caused this call to fail. You might
    ///   also get a SoundIoOutStream::underflow_callback, and you might not get
    ///   this error code when an underflow occurs. Unlike #SoundIoErrorStreaming,
    ///   the outstream is still in a valid state and streaming can continue.
    pub fn soundio_outstream_end_write(outstream: *mut SoundIoOutStream) -> SioError;
    /// Clears the output stream buffer.
    /// This function can be called from any thread.
    /// This function can be called regardless of whether the outstream is paused
    /// or not.
    /// Some backends do not support clearing the buffer. On these backends this
    /// function will return SoundIoErrorIncompatibleBackend.
    /// Some devices do not support clearing the buffer. On these devices this
    /// function might return SoundIoErrorIncompatibleDevice.
    /// Possible errors:
    ///
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorIncompatibleBackend
    /// * #SoundIoErrorIncompatibleDevice
    pub fn soundio_outstream_clear_buffer(outstream: *mut SoundIoOutStream) -> SioError;
    /// If the underlying backend and device support pausing, this pauses the
    /// stream. SoundIoOutStream::write_callback may be called a few more times if
    /// the buffer is not full.
    /// Pausing might put the hardware into a low power state which is ideal if your
    /// software is silent for some time.
    /// This function may be called from any thread context, including
    /// SoundIoOutStream::write_callback.
    /// Pausing when already paused or unpausing when already unpaused has no
    /// effect and returns #SoundIoErrorNone.
    ///
    /// Possible errors:
    /// * #SoundIoErrorBackendDisconnected
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorIncompatibleDevice - device does not support
    ///   pausing/unpausing. This error code might not be returned even if the
    ///   device does not support pausing/unpausing.
    /// * #SoundIoErrorIncompatibleBackend - backend does not support
    ///   pausing/unpausing.
    /// * #SoundIoErrorInvalid - outstream not opened and started
    pub fn soundio_outstream_pause(outstream: *mut SoundIoOutStream, pause: u8) -> SioError;
    /// Obtain the total number of seconds that the next frame written after the
    /// last frame written with ::soundio_outstream_end_write will take to become
    /// audible. This includes both software and hardware latency. In other words,
    /// if you call this function directly after calling ::soundio_outstream_end_write,
    /// this gives you the number of seconds that the next frame written will take
    /// to become audible.
    ///
    /// This function must be called only from within SoundIoOutStream::write_callback.
    ///
    /// Possible errors:
    /// * #SoundIoErrorStreaming
    pub fn soundio_outstream_get_latency(outstream: *mut SoundIoOutStream,
                                         out_latency: *mut c_double)
                                         -> SioError;
    /// Allocates memory and sets defaults. Next you should fill out the struct fields
    /// and then call ::soundio_instream_open. Sets all fields to defaults.
    /// Returns `NULL` if and only if memory could not be allocated.
    /// See also ::soundio_instream_destroy
    pub fn soundio_instream_create(device: *mut SoundIoDevice) -> *mut SoundIoInStream;
    /// You may not call this function from SoundIoInStream::read_callback.
    pub fn soundio_instream_destroy(instream: *mut SoundIoInStream);
    /// After you call this function, SoundIoInStream::software_latency is set to the correct
    /// value.
    /// The next thing to do is call ::soundio_instream_start.
    /// If this function returns an error, the instream is in an invalid state and
    /// you must call ::soundio_instream_destroy on it.
    ///
    /// Possible errors:
    /// * #SoundIoErrorInvalid
    ///   * device aim is not #SoundIoDeviceAimInput
    ///   * format is not valid
    ///   * requested layout channel count > #SOUNDIO_MAX_CHANNELS
    /// * #SoundIoErrorOpeningDevice
    /// * #SoundIoErrorNoMem
    /// * #SoundIoErrorBackendDisconnected
    /// * #SoundIoErrorSystemResources
    /// * #SoundIoErrorNoSuchClient
    /// * #SoundIoErrorIncompatibleBackend
    /// * #SoundIoErrorIncompatibleDevice
    pub fn soundio_instream_open(instream: *mut SoundIoInStream) -> c_int;
    /// After you call this function, SoundIoInStream::read_callback will be called.
    ///
    /// Possible errors:
    /// * #SoundIoErrorBackendDisconnected
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorOpeningDevice
    /// * #SoundIoErrorSystemResources
    pub fn soundio_instream_start(instream: *mut SoundIoInStream) -> c_int;
    /// Call this function when you are ready to begin reading from the device
    /// buffer.
    /// * `instream` - (in) The input stream you want to read from.
    /// * `areas` - (out) The memory addresses you can read data from. It is OK
    ///   to modify the pointers if that helps you iterate. There might be a "hole"
    ///   in the buffer. To indicate this, `areas` will be `NULL` and `frame_count`
    ///   tells how big the hole is in frames.
    /// * `frame_count` - (in/out) - Provide the number of frames you want to read;
    ///   returns the number of frames you can actually read. The returned value
    ///   will always be less than or equal to the provided value. If the provided
    ///   value is less than `frame_count_min` from SoundIoInStream::read_callback this function
    ///   returns with #SoundIoErrorInvalid.
    /// It is your responsibility to call this function no more and no fewer than the
    /// correct number of times according to the `frame_count_min` and
    /// `frame_count_max` criteria from SoundIoInStream::read_callback.
    /// You must call this function only from the SoundIoInStream::read_callback thread context.
    /// After calling this function, read data from `areas` and then use
    /// ::soundio_instream_end_read` to actually remove the data from the buffer
    /// and move the read index forward. ::soundio_instream_end_read should not be
    /// called if the buffer is empty (`frame_count` == 0), but it should be called
    /// if there is a hole.
    ///
    /// Possible errors:
    /// * #SoundIoErrorInvalid
    ///   * `*frame_count` < `frame_count_min` or `*frame_count` > `frame_count_max`
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorIncompatibleDevice - in rare cases it might just now
    ///   be discovered that the device uses non-byte-aligned access, in which
    ///   case this error code is returned.
    pub fn soundio_instream_begin_read(instream: *mut SoundIoInStream,
                                       areas: *mut *mut SoundIoChannelArea,
                                       frame_count: *mut c_int)
                                       -> c_int;
    /// This will drop all of the frames from when you called
    /// ::soundio_instream_begin_read.
    /// You must call this function only from the SoundIoInStream::read_callback thread context.
    /// You must call this function only after a successful call to
    /// ::soundio_instream_begin_read.
    ///
    /// Possible errors:
    /// * #SoundIoErrorStreaming
    pub fn soundio_instream_end_read(instream: *mut SoundIoInStream) -> c_int;
    /// If the underyling device supports pausing, this pauses the stream and
    /// prevents SoundIoInStream::read_callback from being called. Otherwise this returns
    /// #SoundIoErrorIncompatibleDevice.
    /// This function may be called from any thread.
    /// Pausing when already paused or unpausing when already unpaused has no
    /// effect and always returns #SoundIoErrorNone.
    ///
    /// Possible errors:
    /// * #SoundIoErrorBackendDisconnected
    /// * #SoundIoErrorStreaming
    /// * #SoundIoErrorIncompatibleDevice - device does not support pausing/unpausing
    pub fn soundio_instream_pause(instream: *mut SoundIoInStream, pause: u8) -> c_int;
    /// Obtain the number of seconds that the next frame of sound being
    /// captured will take to arrive in the buffer, plus the amount of time that is
    /// represented in the buffer. This includes both software and hardware latency.
    ///
    /// This function must be called only from within SoundIoInStream::read_callback.
    ///
    /// Possible errors:
    /// * #SoundIoErrorStreaming
    pub fn soundio_instream_get_latency(instream: *mut SoundIoInStream,
                                        out_latency: *mut c_double)
                                        -> c_int;
}
