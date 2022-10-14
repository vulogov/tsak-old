extern crate log;
use rhai::{Engine, Map};
use rhai::plugin::*;
use serde_json::{from_str};
use crate::stdlib::nr::graphql::nrql::{nrql_query};

#[export_module]
pub mod nrql_module {


    pub fn query(url: String, a: String, key: String, q: String) -> Map {
        let data = nrql_query(url, a, key, q);
        match from_str(&data) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error converting from JSON: {}", err);
                return Map::new();
            }
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::nrql init");
    let module = exported_module!(nrql_module);

    engine.register_static_module("nrql", module.into());
}
