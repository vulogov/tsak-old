extern crate log;
use crate::stdlib::string::Text;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

use crate::cmd::tsak_processors::PROCESSOR;

pub fn txt_run(context: NativeCallContext, t: &mut Text) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    let ast = context.engine().compile(&t.raw())?;
    context.engine().eval_ast::<Dynamic>(&ast)
}

pub fn str_run(context: NativeCallContext, t: String) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    let ast = context.engine().compile(&t)?;
    context.engine().eval_ast::<Dynamic>(&ast)
}

pub fn str_spawn(_context: NativeCallContext, t: String) -> Result<(), Box<EvalAltResult>> {
    let p = PROCESSOR.lock().unwrap();
    p.push(t);
    drop(p);
    Result::Ok(())
}
