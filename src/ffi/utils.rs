use std::os::raw::c_char;
use std::ffi::CStr;
use ffi::enums::SioError;

/// Converts a char pointer to an owned string.
/// If the char pointer is `NULL` or the source string is not valid UTF8,
/// an error is returned.
pub fn ptr_to_string(str_ptr: *const c_char) -> Result<String, SioError> {
    if !str_ptr.is_null() {
        let str_slice: &str = try!(unsafe { CStr::from_ptr(str_ptr) }
                                       .to_str()
                                       .map_err(|_| SioError::EncodingString));
        Ok(str_slice.to_string())
    } else {
        Err(SioError::EncodingString)
    }
}
