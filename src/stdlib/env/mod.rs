extern crate log;

use rhai::{Engine};
use rhai::plugin::*;


#[export_module]
pub mod env_module {
    #[rhai_fn(name="variable")]
    pub fn env_var(key: &str) -> String {
    	std::env::var(key).unwrap_or_else(|_| String::new())
    }

}


pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::env init");
    let module = exported_module!(env_module);

    engine.register_static_module("env", module.into());
}
