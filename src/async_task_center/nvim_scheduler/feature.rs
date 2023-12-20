use super::*;

use std::future::Future;
use std::task::Poll;

pub struct NvimFuture {
    finish: bool,
    task: Option<Box<dyn Fn() + Send>>,
}

impl NvimFuture {
    pub fn new(task: Box<dyn Fn() + Send>) -> NvimFuture {
        NvimFuture {
            finish: false,
            task: Some(task),
        }
    }
}

impl Future for NvimFuture {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let s = self.get_mut();
        if s.finish {
            return Poll::Ready(());
        }
        let task = s.task.take();
        if let None = task {
            return Poll::Ready(());
        }
        if let Err(_) = push_task(NvimTask::new(cx.waker().clone(), task.unwrap())) {
            return Poll::Ready(());
        }
        s.finish = true;
        return Poll::Pending;
    }
}
