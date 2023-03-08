extern crate log;
use crate::stdlib::string::Text;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn num_extract_text(context: NativeCallContext, t: &mut Text) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    num_extract_str(context, t.raw())
}

pub fn num_extract_str(_context: NativeCallContext, _t: String) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    Result::Ok(Dynamic::default())
}
