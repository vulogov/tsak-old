extern crate log;
use rhai::{Engine, Map, Dynamic, Identifier};
use sysinfo::{System, SystemExt};
use crate::stdlib::timestamp::timestamp_module::{timestamp_ms};
use crate::stdlib::nr::nr_module::{metric};


#[derive(Debug, Clone)]
struct Metric {
    met: Map,
}

impl Metric {
    fn new() -> Self {
        Self {
            met: Map::new(),
        }
    }
    fn init(n: String, v: Dynamic) -> Metric {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut res = Metric::new();
        let mut attr = Map::new();
        res.met.insert(Identifier::from("timestamp"), Dynamic::from(timestamp_ms()));
        res.met.insert(Identifier::from("name"), Dynamic::from(n));
        res.met.insert(Identifier::from("type"), Dynamic::from("gauge"));
        res.met.insert(Identifier::from("value"), v);
        attr.insert(Identifier::from("host.name"), Dynamic::from(sys.host_name().unwrap()));
        res.met.insert(Identifier::from("attributes"), Dynamic::from(attr));
        res
    }
    fn get_field(&mut self, index: String) -> Dynamic {
        let key = Identifier::from(&index);
        let attr = self.met.get(&Identifier::from("attributes")).unwrap().clone().cast::<Map>();
        if attr.contains_key(&key) {
            return attr.get(&key).unwrap().clone();
        }
        return Dynamic::default();
    }
    fn set_field(&mut self, index: String, value: Dynamic) {
        let key = Identifier::from(&index);
        let mut attr = self.met.get(&Identifier::from("attributes")).unwrap().clone().cast::<Map>();
        attr.insert(key, value);
        self.met.insert(Identifier::from("attributes"), Dynamic::from(attr));
    }
    fn get_metric(&mut self, index: String) -> Dynamic {
        let key = Identifier::from(index);
        if self.met.contains_key(&key) {
            return self.met.get(&key).unwrap().clone();
        }
        return Dynamic::default();
    }
    fn set_metric(&mut self, index: String, value: Dynamic) {
        let key = Identifier::from(index);
        self.met.insert(key, value);
    }
    fn raw(&mut self) -> Map {
        self.met.clone()
    }
    fn send(&mut self, url: &str, key: &str) -> bool {
        metric(url, key, self.met.clone())
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::Metric type init");
    engine.register_type::<Metric>()
          .register_fn("Metric", Metric::init)
          .register_fn("raw", Metric::raw)
          .register_fn("send", Metric::send)
          .register_fn("get", Metric::get_metric)
          .register_fn("set", Metric::set_metric)
          .register_indexer_get(Metric::get_field)
          .register_indexer_set(Metric::set_field)
          .register_fn("to_string", |x: &mut Metric| format!("{:?}", x.met) );
}
