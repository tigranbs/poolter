#![allow(dead_code)]

extern crate num_cpus;
use std::thread;
use service::{PoolService, Callback};
use std::sync::mpsc::Sender;

pub struct PoolTer {
    service_channels: Vec<Sender<Callback>>,
    channel_index: usize
}

pub struct ExecChan {
    channel: Sender<Callback>
}

impl PoolTer {
    pub fn init() -> PoolTer {
        let mut poolter = PoolTer{
            service_channels: vec![],
            channel_index: 0
        };

        for _ in 0..num_cpus::get() {
            let sv = PoolService::new();
            poolter.service_channels.push(sv.channel());
            thread::spawn(move || {
                sv.listen();
            });
        }

        poolter
    }

    pub fn exec(&mut self, cb: Callback) -> ExecChan {
        if self.channel_index >= self.service_channels.len() {
            self.channel_index = 0;
        }

        let chan = self.service_channels[self.channel_index].clone();
        self.channel_index += 1;
        let _ = chan.send(cb);
        ExecChan {
            channel: chan
        }
    }
}


impl ExecChan {
    pub fn then(&self, cb: Callback) -> ExecChan {
        let _ = self.channel.send(cb);
        ExecChan {
            channel: self.channel.clone()
        }
    }
}
