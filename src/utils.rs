use std::{ffi::{CStr, CString}, fs, os::raw::c_char};
use anyhow::Result;


pub fn vk_str_to_string(vk_str_buffer: &[c_char]) -> String {
    let raw_string = unsafe { CStr::from_ptr(vk_str_buffer.as_ptr()) };

    raw_string.to_str().expect("Cannot convert string").to_owned()
}

pub struct VkStringArray {
    _storage: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl VkStringArray {
    pub fn new<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let storage: Vec<CString> = names
            .into_iter()
            .map(|s| CString::new(s.as_ref()).expect("interior NUL"))
            .collect();
        let ptrs = storage.iter().map(|s| s.as_ptr()).collect();
        Self { _storage: storage, ptrs }
    }

    pub fn as_ptrs(&self) -> *const *const c_char { self.ptrs.as_ptr() }
    // pub fn len(&self) -> u32 { self.ptrs.len() as u32 }
}

pub fn read_file(path: &str) -> Result<Vec<u8>> {
    Ok(fs::read(path)?)
}