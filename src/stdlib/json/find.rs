extern crate log;
use rhai::{Dynamic, Array, NativeCallContext, EvalAltResult};
use rhai::serde::{to_dynamic};
use jsonpath_rust::JsonPathFinder;

pub fn find_in_json(_context: NativeCallContext, json_str: String, q: String) -> Result<Dynamic, Box<EvalAltResult>> {
    match JsonPathFinder::from_str(&json_str, &q) {
        Ok(res)  => {
            let mut out = Array::new();
            for v in &res.find_slice() {
                out.push(to_dynamic(&v).unwrap())
            }
            return Result::Ok(Dynamic::from(out));
        }
        Err(err) => {
            let msg = format!("Error converting JSON: {}", err);
            log::error!("{}", msg);
            return Err(msg.into());
        }
    }
}
