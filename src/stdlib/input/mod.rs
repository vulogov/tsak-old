extern crate log;
use rhai::{Engine};
use rhai::plugin::*;
use crate::tsak_lib::io::get_file;

#[export_module]
pub mod input_module {
    pub fn stdin() -> String {
        get_file::get_file("-".to_string())
    }
    pub fn url(u: &str) -> String {
        get_file::get_file(u.to_string())
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::input init");
    let module = exported_module!(input_module);

    engine.register_static_module("input", module.into());
}
