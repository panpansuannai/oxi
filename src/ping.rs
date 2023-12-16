use std::ffi;

#[no_mangle]
pub extern "C" fn ping(i: ffi::c_int) -> ffi::c_int {
    return i;
}

