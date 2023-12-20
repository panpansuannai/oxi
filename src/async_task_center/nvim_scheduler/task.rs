use std::task::Waker;

pub struct NvimTask {
    pub(super) waker: Waker,
    pub(super) exec: Box<dyn Fn() + Send>,
}

impl NvimTask {
    pub fn new(waker: Waker, exec: Box<dyn Fn() + Send>) -> NvimTask {
        NvimTask { waker, exec }
    }
}
