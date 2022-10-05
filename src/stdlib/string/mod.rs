extern crate log;

use rhai::{Engine};
use rhai::plugin::*;

#[export_module]
pub mod string_module {
    pub fn trim(s: &str) -> String {
    	s.trim().into()
    }

    pub fn lowercase(s: &str) -> String {
    	s.to_lowercase()
    }

    pub fn uppercase(s: &str) -> String {
    	s.to_uppercase()
    }

    pub fn starts_with(a: &str, b: &str) -> bool {
    	a.starts_with(b)
    }

    pub fn ends_with(a: &str, b: &str) -> bool {
    	a.ends_with(b)
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::str init");
    let module = exported_module!(string_module);

    engine.register_static_module("str", module.into());
}
