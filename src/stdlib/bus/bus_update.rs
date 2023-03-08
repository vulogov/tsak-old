extern crate log;

use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};
use crate::stdlib::bus::queue::{try_queue_push, try_queue_pull};



pub fn update_bus_push(_context: NativeCallContext, d: Map) -> Result<bool, Box<EvalAltResult>> {
    try_update_bus_push(d)
}

pub fn update_bus_pull(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    try_update_bus_pull()
}

pub fn update_bus_push_dynamic(_context: NativeCallContext, d: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    try_queue_push("bus_publish".to_string(), Dynamic::from(d))
}

pub fn try_update_bus_push(d: Map) -> Result<bool, Box<EvalAltResult>> {
    try_queue_push("bus_publish".to_string(), Dynamic::from(d))
}

pub fn try_update_bus_pull() -> Result<Dynamic, Box<EvalAltResult>> {
    try_queue_pull("bus_receive".to_string())
}
