extern crate log;
use howlong;
use std::{thread, time};
use rhai::{Engine};
use rhai::plugin::*;

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
}


pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::system init");
    let module = exported_module!(system_module);
    engine.register_static_module("system", module.into());
}
