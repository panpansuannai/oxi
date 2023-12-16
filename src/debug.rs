use std::ffi;
use nvim_oxi::{api, api::types, api::opts};
use crate::utils;

#[repr(C)]
pub struct StructType {
    s: utils::CharPtr,
}

#[no_mangle]
pub extern "C" fn do_some_thing() {
    let notify_result = api::notify("doing", types::LogLevel::Info, &opts::NotifyOpts {});
    if notify_result.is_err() {
        println!("hello")
    }
}

#[no_mangle]
pub extern "C" fn ffi_type_convert(
    int_type: ffi::c_int,
    str_type: utils::CharPtr,
    struct_type: StructType,
) {
    let int_str = int_type.to_string();
    let s = utils::ffi_to_str(str_type).unwrap();
    let s2 = utils::ffi_to_str(struct_type.s).unwrap();
    let _ = api::notify(
        format!("int is {}, str is {}, struct str is {}", int_str, s, s2).as_str(),
        types::LogLevel::Info,
        &opts::NotifyOpts {},
    );
}
