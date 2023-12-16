use lazy_static::lazy_static;
use std::panic::catch_unwind;
use std::sync::{mpsc::channel, mpsc::Receiver, mpsc::Sender, Mutex};

type Task = Box<dyn Fn() + Send>;
type TaskSender = Sender<Task>;
type TaskReciever = Receiver<Task>;

lazy_static! {
    static ref SENDER: Mutex<Option<TaskSender>> = Mutex::new(None);
    static ref REVIEVER: Mutex<Option<TaskReciever>> = Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn schedule_task() {
    if let Err(sche_err) = scheduler() {
        crate::utils::nvim_error(&sche_err)
    }
}

pub fn init() {
    let (sender, reciever) = channel::<Box<dyn Fn() + Send>>();
    **SENDER.lock().as_mut().unwrap() = Some(sender);
    **REVIEVER.lock().as_mut().unwrap() = Some(reciever);
}

pub fn push_task(task: Task) -> Result<(), String> {
    let sender_lock = SENDER.lock();
    sender_lock
        .map(|mut g| {
            let _ = g.as_mut().unwrap().send(task);
            return;
        })
        .map_err(|e| { format!("lock err: {:?}", e.to_string())})
}

fn scheduler() -> Result<(), String> {
    let mut reciever_lock = REVIEVER.try_lock();
    if let Err(e) = reciever_lock {
        return Err(e.to_string());
    }
    let reciever = reciever_lock.as_mut().unwrap().as_ref().unwrap();
    let p = catch_unwind(|| match reciever.try_recv() {
        Ok(task) => {
            task();
        }
        Err(_) => return,
    });
    p.map_err(|e| format!("panic: {:?}", e))
}
