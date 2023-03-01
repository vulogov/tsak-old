extern crate log;
use rhai::{Engine, Module};

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::BUS init");
    let mut module = Module::new();
    let mut internal_module = Module::new();
    module.set_id("bus");
    internal_module.set_id("internal");
    module.set_sub_module("internal", internal_module);
    engine.register_static_module("bus", module.into());
}
