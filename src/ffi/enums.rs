use std::fmt;
use std::ffi::CString;
use std::fmt::Display;
use std::str::Utf8Error;

use ffi::functions::*;
use ffi::utils::*;

/// Possible error codes.
#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SioError {
    None = 0,
    /// Out of memory.
    NoMem = 1,
    /// The backend does not appear to be active or running.
    InitAudioBackend = 2,
    /// A system resource other than memory (Dummy) was not available.
    SystemResources = 3,
    /// Attempted to open a device and failed.
    OpeningDevice = 4,
    NoSuchDevice = 5,
    /// The programmer did not comply with the API.
    Invalid = 6,
    /// libsoundio was compiled without support for that backend.
    ///
    /// Currently there is no JACK support in Arch Linux until
    /// `jack2{,-dbus}1.9.10-4` is available. See [this](https://bugs.archlinux.org/task/47839)
    /// bug report for details.
    BackendUnavailable = 7,
    /// An open stream had an error that can only be recovered from by
    /// destroying the stream and creating it again.
    Streaming = 8,
    /// Attempted to use a device with parameters it cannot support.
    IncompatibleDevice = 9,
    /// When JACK returns `JackNoSuchClient`
    NoSuchClient = 10,
    /// Attempted to use parameters that the backend cannot support.
    IncompatibleBackend = 11,
    /// Backend server shutdown or became inactive.
    BackendDisconnected = 12,
    Interrupted = 13,
    /// Buffer underrun occurred.
    Underflow = 14,
    /// Unable to convert to or from UTF-8 to the native string format.
    EncodingString = 15,
}
impl Display for SioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_ptr = unsafe { soundio_strerror(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}

/// Specifies where a channel is physically located.
#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum SioChannelId {
    Invalid = 0,
    /// First of the more commonly supported ids.
    FrontLeft = 1,
    FrontRight = 2,
    FrontCenter = 3,
    Lfe = 4,
    BackLeft = 5,
    BackRight = 6,
    FrontLeftCenter = 7,
    FrontRightCenter = 8,
    BackCenter = 9,
    SideLeft = 10,
    SideRight = 11,
    TopCenter = 12,
    TopFrontLeft = 13,
    TopFrontCenter = 14,
    TopFrontRight = 15,
    TopBackLeft = 16,
    TopBackCenter = 17,
    /// Last of the more commonly supported ids.
    TopBackRight = 18,
    /// First of the less commonly supported ids.
    BackLeftCenter = 19,
    BackRightCenter = 20,
    FrontLeftWide = 21,
    FrontRightWide = 22,
    FrontLeftHigh = 23,
    FrontCenterHigh = 24,
    FrontRightHigh = 25,
    TopFrontLeftCenter = 26,
    TopFrontRightCenter = 27,
    TopSideLeft = 28,
    TopSideRight = 29,
    LeftLfe = 30,
    RightLfe = 31,
    Lfe2 = 32,
    BottomCenter = 33,
    BottomLeftCenter = 34,
    BottomRightCenter = 35,
    /// Mid/side recording
    MsMid = 36,
    MsSide = 37,
    /// first order ambisonic channels
    AmbisonicW = 38,
    AmbisonicX = 39,
    AmbisonicY = 40,
    AmbisonicZ = 41,
    /// X-Y Recording
    XyX = 42,
    XyY = 43,
    /// First of the "other" channel ids
    HeadphonesLeft = 44,
    HeadphonesRight = 45,
    ClickTrack = 46,
    ForeignLanguage = 47,
    HearingImpaired = 48,
    Narration = 49,
    Haptic = 50,
    /// Last of the "other" channel ids
    DialogCentricMix = 51,
    Aux = 52,
    Aux0 = 53,
    Aux1 = 54,
    Aux2 = 55,
    Aux3 = 56,
    Aux4 = 57,
    Aux5 = 58,
    Aux6 = 59,
    Aux7 = 60,
    Aux8 = 61,
    Aux9 = 62,
    Aux10 = 63,
    Aux11 = 64,
    Aux12 = 65,
    Aux13 = 66,
    Aux14 = 67,
    Aux15 = 68,
}
impl Display for SioChannelId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_ptr = unsafe { soundio_get_channel_name(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}
impl From<String> for SioChannelId {
    fn from(id: String) -> Self {
        let str_len = id.len() as i32;
        let cstr = CString::new(id).unwrap();
        unsafe { soundio_parse_channel_id(cstr.as_ptr(), str_len) }
    }
}

/// Built-in channel layouts for convenience.
#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioChannelLayoutId {
    Mono = 0,
    Stereo = 1,
    TwoPointOne = 2,
    ThreePointZero = 3,
    ThreePointZeroBack = 4,
    ThreePointOne = 5,
    FourPointZero = 6,
    Quad = 7,
    QuadSide = 8,
    FourPointOne = 9,
    FivePointZeroBack = 10,
    FivePointZeroSide = 11,
    FivePointOne = 12,
    FivePointOneBack = 13,
    SixPointZeroSide = 14,
    SixPointZeroFront = 15,
    Hexagonal = 16,
    SixPointOne = 17,
    SixPointOneBack = 18,
    SixPointOneFront = 19,
    SevenPointZero = 20,
    SevenPointZeroFront = 21,
    SevenPointOne = 22,
    SevenPointOneWide = 23,
    SevenPointOneWideBack = 24,
    Octagonal = 25,
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SioBackend {
    None = 0,
    Jack = 1,
    PulseAudio = 2,
    Alsa = 3,
    CoreAudio = 4,
    Wasapi = 5,
    Dummy = 6,
}
impl Display for SioBackend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_ptr = unsafe { soundio_backend_name(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioDeviceAim {
    /// capture/recording
    Input = 0,
    /// playback
    Output = 1,
}

/// Supported sound formats, each for little- and big-endian.
#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioFormat {
    Invalid = 0,
    /// Signed 8 bit
    S8 = 1,
    /// Unsigned 8 bit
    U8 = 2,
    /// Signed 16 bit little-endian
    S16LE = 3,
    /// Signed 16 bit big-endian
    S16BE = 4,
    /// Unsigned 16 bit little-endian
    U16LE = 5,
    /// Unsigned 16 bit big-endian
    U16BE = 6,
    /// The 24 bit types are not supported in rsoundio at the moment.
    /// Because there is no native number type that is 24 bits wide,
    /// (at least not in Rusts `std`) I had to use something like
    /// C unions in Rust to support them.
    /// This would probably be unsafe and working with
    /// 3 byte number types is also not too much fun.
    ///
    /// Signed 24 bit little-endian
    S24LE = 7,
    /// Signed 24 bit big-endian
    S24BE = 8,
    /// Unsigned 24 bit little-endian
    U24LE = 9,
    /// Unsigned 24 bit big-endian
    U24BE = 10,
    /// Signed 32 bit little-endian
    S32LE = 11,
    /// Signed 32 bit big-endian
    S32BE = 12,
    /// Unsigned 32 bit little-endian
    U32LE = 13,
    /// Unsigend 32 bit big-endian
    U32BE = 14,
    /// 32 bit float little-endian in [-1.0, 1.0]
    Float32LE = 15,
    /// 32 bit float big-endian in [-1.0, 1.0]
    Float32BE = 16,
    /// 64 bit float little-endian in [-1.0, 1.0]
    Float64LE = 17,
    /// 64 bit float big-endian in [-1.0, 1.0]
    Float64BE = 18,
}
impl Display for SioFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_ptr = unsafe { soundio_format_string(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}
impl SioFormat {
    /// Returns the number of bytes a sample takes in this format.
    pub fn bytes_per_sample(self) -> i32 {
        unsafe { soundio_get_bytes_per_sample(self) as i32 }
    }
}

// #[allow(dead_code,non_camel_case_types)]
// enum SoundIoRingBuffer { }
