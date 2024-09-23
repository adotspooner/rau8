use std::ffi::CStr;
use std::os::raw::c_char;

use unicode_segmentation::UnicodeSegmentation;

#[no_mangle]
pub extern "C" fn rau8_valid(str: *const c_char) -> bool {
    let cstr = unsafe { CStr::from_ptr(str) };
    
    if let Ok(_valid_utf8_string) = String::from_utf8(cstr.to_bytes().to_vec()) {
        return true;
    }

    return false;
}

#[no_mangle]
pub extern "C" fn rau8_bytes(str: *const c_char) -> i64 {
    let cstr = unsafe { CStr::from_ptr(str) };
    
    if let Ok(valid_utf8_string) = String::from_utf8(cstr.to_bytes().to_vec()) {
        return i64::try_from(valid_utf8_string.len()).unwrap();
    }

    return -1;
}

#[no_mangle]
pub extern "C" fn rau8_scalar_values(str: *const c_char) -> i64 {
    let cstr = unsafe { CStr::from_ptr(str) };
    
    if let Ok(valid_utf8_string) = String::from_utf8(cstr.to_bytes().to_vec()) {
        return i64::try_from(valid_utf8_string.chars().count()).unwrap();
    }

    return -1;
}

#[no_mangle]
pub extern "C" fn rau8_grapheme_clusters(str: *const c_char) -> i64 {
    let cstr = unsafe { CStr::from_ptr(str) };
    
    if let Ok(valid_utf8_string) = String::from_utf8(cstr.to_bytes().to_vec()) {
        let g = valid_utf8_string.graphemes(true).collect::<Vec<&str>>();
        let l = g.len();
        return i64::try_from(l).unwrap();
    }

    return -1;
}