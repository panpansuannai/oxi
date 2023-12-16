use std::ffi;
use nvim_oxi::{api, api::types, api::opts};

pub type CharPtr = *const ffi::c_char;

pub fn ffi_to_str(p: CharPtr) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let mut ptr = p as *const u8;
    let mut buf: Vec<u8> = vec![];
    unsafe {
        loop {
            let c = *ptr;
            if c == 0 {
                break;
            }
            buf.push(c);
            ptr = ptr.add(1);
        }
        Some(String::from_utf8_unchecked(buf))
    }
}

pub fn nvim_info(s: std::string::String) {
    let _ = api::notify(s.as_str(), types::LogLevel::Info, &opts::NotifyOpts {});
}

pub fn nvim_error(s: std::string::String) {
    let _ = api::notify(s.as_str(), types::LogLevel::Error, &opts::NotifyOpts {});
}
