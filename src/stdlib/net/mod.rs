extern crate log;
use rhai::{Module, Engine};

mod ping;

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::net init");
    let mut module = Module::new();

    let mut ping_module = Module::new();
    ping_module.set_native_fn("icmp", ping::ping_icmp);
    ping_module.set_native_fn("tcp", ping::ping_tcp);
    module.set_sub_module("ping", ping_module);

    engine.register_static_module("net", module.into());


}
