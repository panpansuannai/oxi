pub mod nvim_scheduler;

use lazy_static::lazy_static;
use std::future::Future;
use std::pin;
use std::sync::{mpsc::channel, mpsc::Receiver, mpsc::Sender, Mutex};
use tokio::runtime;

type Task = pin::Pin<Box<dyn Future<Output = ()> + Send>>;
type TaskSender = Sender<Task>;
type TaskReciever = Receiver<Task>;

lazy_static! {
    static ref SENDER: Mutex<Option<TaskSender>> = Mutex::new(None);
}

pub fn init() {
    nvim_scheduler::init();
    std::thread::spawn(|| {
        let (sender, reciever) = channel::<Task>();
        **SENDER.lock().as_mut().unwrap() = Some(sender);
        let rt = runtime::Builder::new_current_thread().build().unwrap();
        loop {
            let _ = rt.block_on(scheduler(&reciever));
        }
    });
}
pub fn async_task<T: Future<Output = ()> + Send + 'static>(task: T) -> Result<(), String> {
    let sender_lock = SENDER.lock();
    sender_lock
        .map(|mut g| {
            let _ = g.as_mut().unwrap().send(Box::pin(task));
            return;
        })
        .map_err(|e| format!("lock err: {:?}", e.to_string()))
}

async fn scheduler(recv: &TaskReciever) -> Result<(), String> {
    let task = recv.try_recv();
    if let Err(_) = task {
        return Ok(());
    }
    let task = task.unwrap();
    task.await;
    return Ok(());
}

#[test]
fn test() {
    let _ = async_task(Box::pin(async {}));
}
