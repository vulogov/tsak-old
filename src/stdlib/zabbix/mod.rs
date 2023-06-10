extern crate log;
use rhai::{Engine};
use rhai::plugin::*;

#[derive(Debug, Clone)]
pub struct ZabbixTrapper {
    host: String,
    port: u16,
}

impl ZabbixTrapper {
    fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 10051,
        }
    }
    fn init(host: String, port: u16) -> Self {
        Self {
            host: host,
            port: port,
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::zabbix init");
    engine.register_type::<ZabbixTrapper>()
        .register_fn("ZabbixTrapper", ZabbixTrapper::new)
        .register_fn("ZabbixTrapper", ZabbixTrapper::init)
        .register_fn("to_string", |x: &mut ZabbixTrapper| format!("ZabbixTrapper({}:{})", x.host, x.port) );
}
