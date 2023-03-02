extern crate log;
use crate::cmd;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use crate::stdlib::system::system_module::{sleep,sleep_millisecond};
use crate::stdlib::bus::queue::queue_read_payloads;
use serde_json::{to_string};
use crate::stdlib::nr::event::raw::{send_event_payload};
use crate::stdlib::nr::metric::raw::{send_metric_payload};
use crate::stdlib::nr::nrlog::raw::{send_log_payload};



pub async fn event_processor_main(c: cmd::Cli) -> () {
    log::trace!("event processor reached");

    loop {
        sleep_millisecond(500 as i64);
        let data = queue_read_payloads("events".to_string(), 50);
        if data.len() > 0 {
            match to_string(&data) {
               Ok(payload) => {
                   log::debug!("Sending {} events, {} bytes to New Relic", &data.len(), &payload.len());
                   send_event_payload(&c.nr_event, &c.nr_account, &c.nr_insert_key, &payload);
               }
               Err(err) => {
                   log::error!("Error generating payload: {}", err);
               }
           }
        }
    }
}

pub async fn metric_processor_main(c: cmd::Cli) -> () {
    log::trace!("metric processor reached");

    loop {
        sleep_millisecond(1000 as i64);
        let data = queue_read_payloads("metrics".to_string(), 50);
        if data.len() > 0 {
            match to_string(&data) {
               Ok(payload) => {
                   log::debug!("Sending {} metrics, {} bytes to New Relic", &data.len(), &payload.len());
                   send_metric_payload(&c.nr_metric, &c.nr_insert_key, &payload);
               }
               Err(err) => {
                   log::error!("Error generating payload: {}", err);
               }
           }
        }
    }
}

pub async fn log_processor_main(c: cmd::Cli) -> () {
    log::trace!("log processor reached");

    loop {
        sleep_millisecond(1000 as i64);
        let data = queue_read_payloads("logs".to_string(), 50);
        if data.len() > 0 {
            match to_string(&data) {
               Ok(payload) => {
                   log::debug!("Sending {} log entries, {} bytes to New Relic", &data.len(), &payload.len());
                   send_log_payload(&c.nr_log, &c.nr_insert_key, &payload);
               }
               Err(err) => {
                   log::error!("Error generating payload: {}", err);
               }
           }
        }
    }
}

pub async fn vulnerability_processor_main(_c: cmd::Cli) -> () {
    log::trace!("vulnerability processor reached");

    loop {
        sleep(1 as i64)

    }
}

pub async fn zabbix_out_processor_main(_c: cmd::Cli) -> () {
    log::trace!("zabbix out processor reached");

    loop {
        sleep(1 as i64)

    }
}
