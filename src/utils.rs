use std::{ffi::CStr, os::raw::c_char};

pub fn vk_str_to_string(vk_str_buffer: &[c_char]) -> String {
    let raw_string = unsafe { CStr::from_ptr(vk_str_buffer.as_ptr()) };

    raw_string.to_str().expect("Cannot convert string").to_owned()
}