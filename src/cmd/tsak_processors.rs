extern crate log;
use crate::cmd;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use bastion::{spawn};
use crossbeam_deque::{Worker, Steal};
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::lang::LangEngine;
use crate::stdlib::system::system_module::sleep;

lazy_static! {
    pub static ref PROCESSOR: Mutex<Worker<String>> = {
        let e: Mutex<Worker<String>> = Mutex::new(Worker::new_fifo());
        e
    };
}

async fn processor_main(n: u64, c: cmd::Cli) -> () {
    log::trace!("cmd::tsak_processors({}) reached", &n);
    let p = PROCESSOR.lock().unwrap();
    let s = p.stealer();
    drop(p);
    let mut engine = LangEngine::child_init(&c.clone());
    loop {
        match s.steal() {
            Steal::Success(job) => {
                log::info!("Job picked up at #{}", &n);
                match engine.run(job) {
                    Ok(_) => continue,
                    Err(err) => {
                        log::error!("Background script retured error: {}", err);
                    }
                }
            }
            _ => sleep(1 as i64),
        }
    }
}

pub fn tsak_processors(c: cmd::Cli, _e: &LangEngine) {
    log::trace!("cmd::tsak_processors() reached");
    let p = PROCESSOR.lock().unwrap();
    drop(p);
    log::debug!("{} pre-spawned children requested", &c.proc);
    for n in 0..c.proc {
        let _ = spawn! {
            processor_main(n.into(), c.clone())
        };
    }
}
