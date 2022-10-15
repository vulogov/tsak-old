extern crate log;
use rhai::{Engine, Map, Dynamic, Identifier};
use sysinfo::{System, SystemExt};
use crate::stdlib::timestamp::timestamp_module::{timestamp_ms};
use crate::stdlib::nr::nr_module::{event};


#[derive(Debug, Clone)]
struct Event {
    evt: Map,
}

impl Event {
    fn new() -> Self {
        Self {
            evt: Map::new(),
        }
    }
    fn init() -> Event {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut res = Event::new();
        res.evt.insert(Identifier::from("timestamp"), Dynamic::from(timestamp_ms()));
        res.evt.insert(Identifier::from("eventType"), Dynamic::from("TSAKEvents"));
        res.evt.insert(Identifier::from("host.name"), Dynamic::from(sys.host_name().unwrap()));
        res
    }
    fn get_field(&mut self, index: String) -> Dynamic {
        let key = Identifier::from(index);
        if self.evt.contains_key(&key) {
            return self.evt.get(&key).unwrap().clone();
        }
        return Dynamic::default();
    }
    fn set_field(&mut self, index: String, value: Dynamic) {
        let key = Identifier::from(index);
        self.evt.insert(key, value);
    }
    fn raw(&mut self) -> Map {
        self.evt.clone()
    }
    fn send(&mut self, url: &str, account: &str, key: &str) -> bool {
        event(url, account, key, self.evt.clone())
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::Event type init");
    engine.register_type::<Event>()
          .register_fn("Event", Event::init)
          .register_fn("raw", Event::raw)
          .register_fn("send", Event::send)
          .register_indexer_get(Event::get_field)
          .register_indexer_set(Event::set_field)
          .register_fn("to_string", |x: &mut Event| format!("{:?}", x.evt) );
}
