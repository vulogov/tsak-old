extern crate log;
use howlong;
use std::io;
use std::io::{Read};
use flate2::read::GzEncoder;
use flate2::Compression;
use rhai::{Engine, Map, format_map_as_json};
use rhai::plugin::*;

pub mod graphql;
pub mod event;
pub mod metric;
pub mod nrlog;

#[export_module]
pub mod nr_module {
    pub fn metric(url: &str, key: &str, metric: Map) -> bool {
        let payload = format!("[{}]", format_map_as_json(&metric));
        log::debug!("Payload: {}", &payload);
        let t = howlong::HighResolutionTimer::new();
        let res = metric::raw::send_metric_payload(&url.to_string(), &key.to_string(), &payload);
        log::debug!("{:?} takes to send metric", t.elapsed());
        return res;
    }
    pub fn event(url: &str, account: &str, key: &str, event: Map) -> bool {
        let payload = format_map_as_json(&event);
        log::debug!("Payload: {}", &payload);
        let t = howlong::HighResolutionTimer::new();
        let res = event::raw::send_event_payload(&url.to_string(), &account.to_string(), &key.to_string(), &payload);
        log::debug!("{:?} takes to send event", t.elapsed());
        return res;
    }
    pub fn log(url: &str, key: &str, nr_log: Map) -> bool {
        let payload = format!("{}", format_map_as_json(&nr_log));
        log::debug!("Payload: {}", &payload);
        let t = howlong::HighResolutionTimer::new();
        let res = nrlog::raw::send_log_payload(&url.to_string(), &key.to_string(), &payload);
        log::debug!("{:?} takes to send log", t.elapsed());
        return res;
    }
}

pub fn compress_payload(payload: &String) -> io::Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut z = GzEncoder::new(&payload.as_bytes()[..], Compression::fast());
    z.read_to_end(&mut result)?;
    Ok(result)
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::nr init");
    let module = exported_module!(nr_module);
    engine.register_static_module("newrelic", module.into());
    event::event_type::init(engine);
    metric::metric_type::init(engine);
}
