extern crate log;
use bastion::prelude::*;
use rhai::{NativeCallContext, EvalAltResult};

pub fn system_loop(_context: NativeCallContext) -> Result<(), Box<EvalAltResult>> {
    log::info!("TSAK entering into event loop");
    Bastion::block_until_stopped();
    Result::Ok(())
}
