extern crate log;
use rhai::{Engine, Map, EvalAltResult};

use crate::stdlib::nrql::{query};
use crate::stdlib::nr::graphql::nrql::{nrql_query};
use crate::stdlib::nr::graphql::result_type::{NRResult};


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
    fn query_map(&mut self, q: String) -> Result<Map, Box<EvalAltResult>> {
        query(self.uri.clone(), self.account.clone(), self.key.clone(), q)
    }
    fn query_str(&mut self, q: String) -> String {
        nrql_query(self.uri.clone(), self.account.clone(), self.key.clone(), q)
    }
    fn query(&mut self, q: String) -> NRResult {
        NRResult::init(nrql_query(self.uri.clone(), self.account.clone(), self.key.clone(), q))
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::NRQL type init");
    engine.register_type::<NRQL>()
          .register_fn("NRQL", NRQL::init)
          .register_fn("query_map", NRQL::query_map)
          .register_fn("query_str", NRQL::query_str)
          .register_fn("query", NRQL::query)
          .register_fn("to_string", |x: &mut NRQL| format!("{:?}", x) );
}
