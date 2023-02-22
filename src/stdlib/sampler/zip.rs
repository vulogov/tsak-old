extern crate log;
use crate::stdlib::sampler::Sampler;
use rhai::{Dynamic, Array, FnPtr, NativeCallContext, EvalAltResult};

pub fn sampler_zip(context: NativeCallContext, t: &mut Sampler, f: FnPtr) -> Result<Vec<rhai::Dynamic>, Box<EvalAltResult>> {
    let mut res = Array::new();
    for v in t.raw() {
        let r: Result<Dynamic, Box<EvalAltResult>> = f.call_within_context(&context, (v,));
        match r {
            Ok(val) => res.push(val),
            Err(_) => continue,
        }
    }
    return Result::Ok(res);
}
