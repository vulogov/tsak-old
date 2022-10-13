extern crate log;
use rhai::{Engine};
use rhai::plugin::*;
use fsio::{file};
use crate::tsak_lib::io::get_file;

pub mod command;

#[export_module]
pub mod input_module {
    pub fn stdin() -> String {
        get_file::get_file("-".to_string())
    }
    pub fn url(u: &str) -> String {
        get_file::get_file(u.to_string())
    }
    pub fn file(u: &str) -> String {
        match file::read_text_file(u) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error reading {}", err);
                return "".to_string();
            }
        }
    }
    pub fn command(c: &str, a: String) -> String {
        command::os_command(c, &a)
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::input init");
    let module = exported_module!(input_module);

    engine.register_static_module("input", module.into());
}
