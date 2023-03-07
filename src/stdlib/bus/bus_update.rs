extern crate log;

use rhai::{Map, NativeCallContext, EvalAltResult};
use serde_json::{to_string};



pub fn update_bus_push(_context: NativeCallContext, uri: String, d: Map) -> Result<bool, Box<EvalAltResult>> {
    try_update_bus_push(uri, d)
}

pub fn try_update_bus_push(uri: String, d: Map) -> Result<bool, Box<EvalAltResult>> {
    
    Result::Ok(true)
}
