extern crate log;
use rhai::{Engine, Map, EvalAltResult, Module, NativeCallContext};
use serde_json::{from_str};
use crate::stdlib::nr::graphql::nrql::{nrql_query};


fn do_query(_context: NativeCallContext, url: String, a: String, key: String, q: String) -> Result<Map, Box<EvalAltResult>> {
    query(url, a, key, q)
}

pub fn query(url: String, a: String, key: String, q: String) -> Result<Map, Box<EvalAltResult>> {
    let data = nrql_query(url, a, key, q);
    match from_str(&data) {
        Ok(res) => Result::Ok(res),
        Err(err) => {
            let msg = format!("Error converting from JSON: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into());
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::nrql init");
    let mut module = Module::new();
    module.set_native_fn("query", do_query);
    engine.register_static_module("nrql", module.into());
}
