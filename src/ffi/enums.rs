use std::os::raw::c_char;
use std::fmt;
use std::ffi::{CStr,CString};
use std::fmt::Display;
use std::str::Utf8Error;

use ffi::functions::*;

pub fn ptr_to_string(str_ptr: *const c_char) -> Result<String, Utf8Error> {
    let str_slice: &str = try!(unsafe { CStr::from_ptr(str_ptr) }.to_str());
    Ok(str_slice.to_string())
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SioError {
    None = 0,
    NoMem = 1,
    InitAudioBackend = 2,
    SystemResources = 3,
    OpeningDevice = 4,
    NoSuchDevice = 5,
    Invalid = 6,
    BackendUnavailable = 7,
    Streaming = 8,
    IncompatibleDevice = 9,
    NoSuchClient = 10,
    IncompatibleBackend = 11,
    BackendDisconnected = 12,
    Interrupted = 13,
    Underflow = 14,
    EncodingString = 15,
}
impl Display for SioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_ptr = unsafe { soundio_strerror(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum SioChannelId {
    Invalid = 0,
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
    TopBackRight = 18,
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
    MsMid = 36,
    MsSide = 37,
    AmbisonicW = 38,
    AmbisonicX = 39,
    AmbisonicY = 40,
    AmbisonicZ = 41,
    XyX = 42,
    XyY = 43,
    HeadphonesLeft = 44,
    HeadphonesRight = 45,
    ClickTrack = 46,
    ForeignLanguage = 47,
    HearingImpaired = 48,
    Narration = 49,
    Haptic = 50,
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
#[derive(Clone, Copy)]
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
    Input = 0,
    Output = 1,
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioFormat {
    Invalid = 0,
    S8 = 1,
    U8 = 2,
    S16LE = 3,
    S16BE = 4,
    U16LE = 5,
    U16BE = 6,
    S24LE = 7,
    S24BE = 8,
    U24LE = 9,
    U24BE = 10,
    S32LE = 11,
    S32BE = 12,
    U32LE = 13,
    U32BE = 14,
    Float32LE = 15,
    Float32BE = 16,
    Float64LE = 17,
    Float64BE = 18,
}
impl Display for SioFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_ptr = unsafe { soundio_format_string(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}
impl SioFormat {
    pub fn get_bytes_per_sample(self) -> i32 {
        unsafe { soundio_get_bytes_per_sample(self) as i32 }
    }
}

#[allow(dead_code,non_camel_case_types)]
pub enum SoundIoRingBuffer { }
