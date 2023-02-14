extern crate log;
use crate::stdlib::string::Text;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn txt_run(context: NativeCallContext, t: &mut Text) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    let ast = context.engine().compile(&t.raw())?;
    context.engine().eval_ast::<Dynamic>(&ast)
}

pub fn str_run(context: NativeCallContext, t: String) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    let ast = context.engine().compile(&t)?;
    context.engine().eval_ast::<Dynamic>(&ast)
}
