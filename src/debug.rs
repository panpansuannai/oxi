use crate::utils;
use nvim_oxi::{api, api::opts, api::types};
use std::ffi;

#[repr(C)]
pub struct StructType {
    s: utils::CharPtr,
}

#[no_mangle]
pub extern "C" fn do_some_thing() {
    if let Err(e) = utils::nvim_exec_lua(
        r#"
    print('hello')
    print('world')
    local i = "hello"
    print(i.." pan")
    "#,
    ) {
        utils::nvim_error(&e);
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
