extern crate log;
use rhai::{Engine};
use rhai::plugin::*;

use crate::stdlib::nr::graphql::nrql::{nrql_query};

#[export_module]
pub mod nrql_module {


    pub fn query(url: String, a: String, key: String, q: String) {
        nrql_query(url, a, key, q)
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::nrql init");
    let module = exported_module!(nrql_module);

    engine.register_static_module("nrql", module.into());
}
