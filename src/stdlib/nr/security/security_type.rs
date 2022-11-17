extern crate log;
use rhai::{Engine, Map, Dynamic, Identifier};
use crate::stdlib::timestamp::timestamp_module::{timestamp_ms};
use crate::stdlib::nr::nr_module::{security};


#[derive(Debug, Clone)]
pub struct Finding {
    sec: Map,
}

impl Finding {
    fn new() -> Self {
        Self {
            sec: Map::new(),
        }
    }
    pub fn init() -> Finding {
        let mut res = Finding::new();
        res.sec.insert(Identifier::from("detectedAt"), Dynamic::from(timestamp_ms()));
        res
    }
    fn get_field(&mut self, index: String) -> Dynamic {
        let key = Identifier::from(index);
        if self.sec.contains_key(&key) {
            return self.sec.get(&key).unwrap().clone();
        }
        return Dynamic::default();
    }
    pub fn set_field(&mut self, index: String, value: Dynamic) {
        let key = Identifier::from(index);
        self.sec.insert(key, value);
    }
    fn raw(&mut self) -> Map {
        self.sec.clone()
    }
    fn send(&mut self, url: &str, key: &str) -> bool {
        security(url, key, self.sec.clone())
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::Finding type init");
    engine.register_type::<Finding>()
          .register_fn("Finding", Finding::init)
          .register_fn("raw", Finding::raw)
          .register_fn("send", Finding::send)
          .register_indexer_get(Finding::get_field)
          .register_indexer_set(Finding::set_field)
          .register_fn("to_string", |x: &mut Finding| format!("{:?}", x.sec) );
}
