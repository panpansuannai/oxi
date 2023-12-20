use nvim_oxi::{api, api::opts, api::types};
use std::ffi;

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

pub fn nvim_info<T: AsRef<str>>(s: T) {
    let _ = api::notify(s.as_ref(), types::LogLevel::Info, &opts::NotifyOpts {});
}

pub fn nvim_error<T: AsRef<str>>(s: &T) {
    let _ = api::notify(s.as_ref(), types::LogLevel::Error, &opts::NotifyOpts {});
}

pub fn nvim_exec_lua<T: AsRef<str>>(script: T) -> Result<Option<String>, String> {
    let script = format!("lua << EOF\n{}\nEOF\n", script.as_ref());
    #[cfg(feature = "wip")]
    let _ = nvim_info(&script);
    let ret = api::exec(&script, false).map_err(|_| "exec lua err".to_string())?;
    Ok(ret)
}
