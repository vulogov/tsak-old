extern crate log;
use rhai::{Engine, Dynamic, Array};
use rhai::plugin::*;
use rustsv::prelude::*;
use rhai::serde::to_dynamic;

#[export_module]
pub mod csv_module {
    pub fn dynamic(d: String) -> Array {
        let mut res = Array::new();
        let content = parse(d, ',', false);
        for entry in content {
            let mut row = Array::new();
            for (_, mut v) in entry {
                if v.chars().nth(0) == Some('\"') {
                    v.pop();
                    v.remove(0);
                }
                match to_dynamic(&v) {
                    Ok(value) => row.push(value),
                    Err(_) => continue,
                }
            }
            res.push(Dynamic::from(row));
        }
        return res;
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::CSV init");
    let module = exported_module!(csv_module);

    engine.register_static_module("csv", module.into());
}
