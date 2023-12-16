mod git_op;
mod ping;
mod task_center;
mod utils;
#[cfg(feature="test_function")]
mod debug;

#[no_mangle]
pub extern "C" fn init() {
    task_center::init();
}
