extern crate log;
use rhai::{Engine, Module};

pub mod html;
pub mod filetype;

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::parser init");
    let mut module = Module::new();
    let mut html_module = Module::new();
    module.set_id("parser");
    html_module.set_id("html");

    // HTML parser
    html_module.set_native_fn("parse", html::html_parse);

    module.set_sub_module("html", html_module);
    module.set_native_fn("filetype", filetype::filetype_detect);
    engine.register_static_module("parser", module.into());
}
