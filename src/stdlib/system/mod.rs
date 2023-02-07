extern crate log;
use howlong;
use std::{thread, time, env};
use rhai::{Engine};
use rhai::plugin::*;

pub mod system_metrics;

#[export_module]
pub mod system_module {
    pub fn sleep(s: i64) {
        let t = howlong::HighResolutionTimer::new();
        thread::sleep(time::Duration::from_secs(s as u64));
        log::debug!("slept for {:?}", t.elapsed());
    }
    pub fn sleep_millisecond(s: i64) {
        let t = howlong::HighResolutionTimer::new();
        thread::sleep(time::Duration::from_millis(s as u64));
        log::debug!("slept for {:?}", t.elapsed());
    }
    pub fn env(n: String) -> String {
        match env::var(&n) {
            Ok(val) => return val,
            Err(e) => {
                log::error!("Error fetching environment variable {}: {:?}", &n, e);
            }
        }
        return "".to_string();
    }
}


pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::system init");
    let module = exported_module!(system_module);
    engine.register_static_module("system", module.into());
    system_metrics::init(engine);
}
