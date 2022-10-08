extern crate log;
use std::time::{SystemTime, UNIX_EPOCH};
use rhai::{Engine};
use rhai::plugin::*;

#[export_module]
pub mod timestamp_module {


    pub fn timestamp_ms() -> u64 {
    	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::timestamp init");
    let module = exported_module!(timestamp_module);

    engine.register_static_module("timestamp", module.into());
}
