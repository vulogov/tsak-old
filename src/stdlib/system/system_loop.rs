extern crate log;
use std::{thread, time};
use rhai::{NativeCallContext, EvalAltResult};

pub fn system_loop(_context: NativeCallContext) -> Result<(), Box<EvalAltResult>> {
    log::info!("TSAK entering into event loop");
    loop {
        thread::sleep(time::Duration::from_millis(250));
    }
}
