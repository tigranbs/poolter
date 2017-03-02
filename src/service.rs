#![allow(dead_code)]
use std::sync::mpsc::{channel, Sender, Receiver};

pub type Callback = Box<Fn() + Send>;

pub struct PoolService {
    task_channel: Sender<Callback>,
    receiver: Receiver<Callback>
}

impl PoolService {
    pub fn new() -> PoolService {
        let (s, r) = channel::<Callback>();

        PoolService {
            task_channel: s,
            receiver: r
        }
    }

    pub fn channel(&self) -> Sender<Callback> {
        self.task_channel.clone()
    }

    pub fn listen(&self) {
        loop {
            match self.receiver.recv() {
                Ok(cb) => cb(),
                Err(_) => continue
            }
        }
    }
}
