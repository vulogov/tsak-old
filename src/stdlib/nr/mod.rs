extern crate log;
use howlong;
use std::io;
use std::io::{Read};
use flate2::read::GzEncoder;
use flate2::Compression;
use rhai::{Engine, Map, Scope, format_map_as_json};
use rhai::plugin::*;

use crate::stdlib::bus::queue::{try_queue_is_empty, try_queue_push};
use crate::stdlib::system::system_module::{sleep_millisecond};

pub mod graphql;
pub mod event;
pub mod metric;
pub mod nrlog;
pub mod security;

// use crate::stdlib::nr::event::event_pipe;

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
        let payload = format!("[{}]", format_map_as_json(&nr_log));
        let t = howlong::HighResolutionTimer::new();
        let res = nrlog::raw::send_log_payload(&url.to_string(), &key.to_string(), &payload);
        log::debug!("{:?} takes to send log", t.elapsed());
        return res;
    }
    pub fn security(url: &str, key: &str, metric: Map) -> bool {
        let payload = format!("[{}]", format_map_as_json(&metric));
        log::debug!("Payload: {}", &payload);
        let t = howlong::HighResolutionTimer::new();
        let res = security::raw::send_security_payload(&url.to_string(), &key.to_string(), &payload);
        log::debug!("{:?} takes to send finding", t.elapsed());
        return res;
    }

    pub mod queue {
        #[rhai_fn(name="event")]
        pub fn event_map(p: Map) -> bool  {
            match try_queue_push("events".to_string(), Dynamic::from(p)) {
                Ok(res) => res,
                Err(_) => false,
            }
        }
        #[rhai_fn(name="metric")]
        pub fn metric_map(p: Map) -> bool {
            match try_queue_push("metrics".to_string(), Dynamic::from(p)) {
                Ok(res) => res,
                Err(_) => false,
            }
        }
        #[rhai_fn(name="log")]
        pub fn log_map(p: Map) -> bool {
            match try_queue_push("logs".to_string(), Dynamic::from(p)) {
                Ok(res) => res,
                Err(_) => false,
            }
        }
        pub fn wait_for_metrics() {
            loop {
                log::trace!("Flushing metrics queue");
                sleep_millisecond(100);
                match try_queue_is_empty("metrics".to_string()) {
                    Ok(res) => {
                        if res {
                            return;
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        pub fn wait_for_events() {
            loop {
                log::trace!("Flushing events queue");
                sleep_millisecond(100);
                match try_queue_is_empty("events".to_string()) {
                    Ok(res) => {
                        if res {
                            return;
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        pub fn wait_for_logs() {
            loop {
                log::trace!("Flushing logs queue");
                sleep_millisecond(100);
                match try_queue_is_empty("logs".to_string()) {
                    Ok(res) => {
                        if res {
                            return;
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        pub fn wait_for() {
            wait_for_metrics();
            wait_for_events();
            wait_for_logs();
            log::debug!("All New Relic queues was flushed");
        }
    }
}

pub fn compress_payload(payload: &String) -> io::Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut z = GzEncoder::new(&payload.as_bytes()[..], Compression::fast());
    z.read_to_end(&mut result)?;
    Ok(result)
}

pub fn init(engine: &mut Engine, scope: &mut Scope) {
    log::trace!("Running STDLIB::nr init");
    let mut module = exported_module!(nr_module);
    module.set_var("NR_EVENT", scope.get_value::<String>("NR_EVENT").unwrap());
    module.set_var("NR_METRIC", scope.get_value::<String>("NR_METRIC").unwrap());
    module.set_var("NR_LOG", scope.get_value::<String>("NR_LOG").unwrap());
    module.set_var("NR_TRACE", scope.get_value::<String>("NR_TRACE").unwrap());
    module.set_var("NR_API", scope.get_value::<String>("NR_API").unwrap());
    module.set_var("NR_SEC_API", scope.get_value::<String>("NR_SEC_API").unwrap());
    module.set_var("NR_ACCOUNT", scope.get_value::<String>("NR_ACCOUNT").unwrap());
    module.set_var("NR_API_KEY", scope.get_value::<String>("NR_API_KEY").unwrap());
    module.set_var("NR_INSERT_KEY", scope.get_value::<String>("NR_INSERT_KEY").unwrap());
    module.set_var("HOSTNAME", scope.get_value::<String>("HOSTNAME").unwrap());
    module.set_var("INSTANCE", scope.get_value::<String>("INSTANCE").unwrap());
    engine.register_static_module("newrelic", module.into());
    event::event_type::init(engine);
    metric::metric_type::init(engine);
    security::security_type::init(engine);
    nrlog::log_type::init(engine);
    graphql::nrql_type::init(engine);
    graphql::result_type::init(engine);
}
