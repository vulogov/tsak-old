extern crate log;
use lazy_static::lazy_static;
use std::collections::btree_map::BTreeMap;
use rhai::{Dynamic, EvalAltResult};
use serde_json::{to_string, from_str};
use std::sync::Mutex;

lazy_static! {
    static ref GLOBALS: Mutex<BTreeMap<String, String>> = {
        let g: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());
        g
    };
}

#[derive(Debug, Clone)]
pub struct NRGlobals {

}

impl NRGlobals {
    pub fn new() -> Self {
        log::trace!("Initialize globals");
        let g = GLOBALS.lock().unwrap();
        drop(g);
        Self {

        }
    }
    pub fn set_global(&mut self, name: String, value: Dynamic) {
        match to_string(&value) {
            Ok(res) => {
                let mut g = GLOBALS.lock().unwrap();
                g.insert(name, res);
                drop(g);
            }
            Err(err) => {
                log::error!("set global error: {}", err);
            }
        }
    }
    pub fn get_global(&mut self, name: String) -> Result<Dynamic, Box<EvalAltResult>> {
        let g = GLOBALS.lock().unwrap();
        if g.contains_key(&name) {
            let res = g.get(&name).unwrap();
            match from_str::<Dynamic>(res) {
                Ok(out) => return Result::Ok(out),
                Err(err) => return Err(format!("get global error: {}", err).into()),
            }
        }
        Err(format!("global key error: {}", &name).into())
    }
}

pub fn get_global(x: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut g = NRGlobals::new();
    let res = g.get_global(x);
    drop(g);
    res
}
