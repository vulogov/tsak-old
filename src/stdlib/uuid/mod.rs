extern crate log;

use rhai::{Engine};
use rhai::plugin::*;

#[export_module]
pub mod uuid_module {
    use uuid::Uuid;

    pub fn uuid_v4() -> String {
    	Uuid::new_v4().to_string()
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::uuid init");
    let module = exported_module!(uuid_module);

    engine.register_static_module("uuid", module.into());
}
