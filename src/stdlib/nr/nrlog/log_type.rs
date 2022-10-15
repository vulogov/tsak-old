extern crate log;
use rhai::{Engine, Map, Dynamic, Identifier};
use sysinfo::{System, SystemExt};
use crate::stdlib::timestamp::timestamp_module::{timestamp_ms};
use crate::stdlib::nr::nr_module::{log};


#[derive(Debug, Clone)]
struct Log {
    log: Map,
}

impl Log {
    fn new() -> Self {
        Self {
            log: Map::new(),
        }
    }
    fn init(m: String) -> Log {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut res = Log::new();
        let mut attr = Map::new();
        res.log.insert(Identifier::from("timestamp"), Dynamic::from(timestamp_ms()));
        res.log.insert(Identifier::from("message"), Dynamic::from(m));
        attr.insert(Identifier::from("host.name"), Dynamic::from(sys.host_name().unwrap()));
        attr.insert(Identifier::from("logtype"), Dynamic::from("tsaklog"));
        attr.insert(Identifier::from("service"), Dynamic::from("tsakservice"));
        res.log.insert(Identifier::from("attributes"), Dynamic::from(attr));
        res
    }
    fn get_field(&mut self, index: String) -> Dynamic {
        let key = Identifier::from(&index);
        let attr = self.log.get(&Identifier::from("attributes")).unwrap().clone().cast::<Map>();
        if attr.contains_key(&key) {
            return attr.get(&key).unwrap().clone();
        }
        return Dynamic::default();
    }
    fn set_field(&mut self, index: String, value: Dynamic) {
        let key = Identifier::from(&index);
        let mut attr = self.log.get(&Identifier::from("attributes")).unwrap().clone().cast::<Map>();
        attr.insert(key, value);
        self.log.insert(Identifier::from("attributes"), Dynamic::from(attr));
    }
    fn get_log(&mut self, index: String) -> Dynamic {
        let key = Identifier::from(index);
        if self.log.contains_key(&key) {
            return self.log.get(&key).unwrap().clone();
        }
        return Dynamic::default();
    }
    fn set_log(&mut self, index: String, value: Dynamic) {
        let key = Identifier::from(index);
        self.log.insert(key, value);
    }
    fn raw(&mut self) -> Map {
        self.log.clone()
    }
    fn send(&mut self, url: &str, key: &str) -> bool {
        log(url, key, self.log.clone())
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::Log type init");
    engine.register_type::<Log>()
          .register_fn("Log", Log::init)
          .register_fn("raw", Log::raw)
          .register_fn("send", Log::send)
          .register_fn("get", Log::get_log)
          .register_fn("set", Log::set_log)
          .register_indexer_get(Log::get_field)
          .register_indexer_set(Log::set_field)
          .register_fn("to_string", |x: &mut Log| format!("{:?}", x.log) );
}
