extern crate log;
use crate::cmd;
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
    log::debug!("cmd::tsak_processors({}) reached", &n);
    let p = PROCESSOR.lock().unwrap();
    let s = p.stealer();
    drop(p);
    let mut engine = LangEngine::init(&c.clone());
    loop {
        match s.steal() {
            Steal::Success(job) => {
                log::debug!("Job picked up at #{}", &n);
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
    log::debug!("cmd::tsak_processors() reached");
    let p = PROCESSOR.lock().unwrap();
    drop(p);
    log::debug!("{} pre-spawned children requested", &c.proc);
    for n in 0..c.proc {
        let spawn_c = c.clone();
        tokio::spawn(async move{
            processor_main(n.into(), spawn_c).await
        });
    }
}
