extern crate log;
use rhai::{Engine, Map};

use crate::stdlib::nrql::nrql_module::{query};

#[derive(Debug, Clone)]
struct NRQL {
    uri:        String,
    account:    String,
    key:        String,
}

impl NRQL {
    fn new() -> Self {
        Self {
            uri:        "".to_string(),
            account:    "".to_string(),
            key:        "".to_string(),
        }
    }
    fn init(url: String, a: String, key: String) -> NRQL {
        let mut res = NRQL::new();
        res.uri = url;
        res.account = a;
        res.key = key;
        res
    }
    fn query_raw(&mut self, q: String) -> Map {
        query(self.uri.clone(), self.account.clone(), self.key.clone(), q)
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::NRQL type init");
    engine.register_type::<NRQL>()
          .register_fn("NRQL", NRQL::init)
          .register_fn("query_raw", NRQL::query_raw)
          .register_fn("to_string", |x: &mut NRQL| format!("{:?}", x) );
}
