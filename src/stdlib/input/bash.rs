extern crate log;
use duct_sh;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn run_bash(_context: NativeCallContext, c: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let cmd = c.clone();
    match duct_sh::sh_dangerous(cmd).read() {
        Ok(res) => {
            return Result::Ok(Dynamic::from(res));
        }
        Err(err) => {
            return Err(format!("input::bash() error: {}", err).into());
        }
    }
}

pub fn disabled_run_bash(_context: NativeCallContext, _c: String, _a: String) -> Result<Dynamic, Box<EvalAltResult>> {
    Err("TSAK is in sandbox mode. input::bash() is disabled".into())
}
