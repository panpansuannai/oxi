mod async_task_center;
#[cfg(feature = "test_function")]
mod debug;
mod git_op;
mod lint;
mod ping;
mod task_center;
mod utils;

#[no_mangle]
pub extern "C" fn init() {
    task_center::init();
    async_task_center::init();
}
