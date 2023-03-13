extern crate log;

use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};

pub fn run_bash(_context: NativeCallContext, _c: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let res = Map::new();
    return Result::Ok(Dynamic::from(res));
}

pub fn disabled_run_bash(_context: NativeCallContext, _c: String, _a: String) -> Result<Dynamic, Box<EvalAltResult>> {
    Err("TSAK is in sandbox mode. input::bash() is disabled".into())
}
