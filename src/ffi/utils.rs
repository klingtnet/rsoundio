use std::os::raw::c_char;
use std::str::Utf8Error;
use std::ffi::CStr;

/// Converts a char pointer to an owned string.
/// If the char pointer is `NULL` or the source string is not valiud UTF8,
/// an error is returned.
pub fn ptr_to_string(str_ptr: *const c_char) -> Result<String, Utf8Error> {
    if !str_ptr.is_null() {
        let str_slice: &str = try!(unsafe { CStr::from_ptr(str_ptr) }.to_str());
        Ok(str_slice.to_string())
    } else {
        Ok("".to_string())
    }
}
