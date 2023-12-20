use crate::async_task_center::nvim_scheduler;
use crate::utils;
use nvim_oxi::{api, api::opts, api::types};
use std::ffi;

#[repr(C)]
pub struct StructType {
    s: utils::CharPtr,
}

#[no_mangle]
pub extern "C" fn do_some_thing() {
    if let Err(e) = crate::async_task_center::async_task(async {
        nvim_scheduler::api::nvim_info("hello".to_string()).await;
    }) {
        utils::nvim_error(&format!("push task err: {}", e));
    }
    /*
    match utils::nvim_exec_lua(
        r#"
        return "hello"
    "#,
    ) {
        Err(e) => {
            utils::nvim_error(&e);
            return;
        }
        Ok(s) => {
            utils::nvim_info(&format!("ret is {:#?}", s));
        }
    }
    */
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
