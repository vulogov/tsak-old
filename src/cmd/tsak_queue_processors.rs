extern crate log;
use crate::cmd;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use crate::stdlib::system::system_module::{sleep,sleep_millisecond};
use crate::stdlib::bus::queue::queue_read_payloads;



pub async fn event_processor_main(_c: cmd::Cli) -> () {
    log::trace!("event processor reached");

    loop {
        sleep_millisecond(500 as i64);
        let data = queue_read_payloads("event".to_string(), 50);
        if data.len() > 0 {
            println!("{:?}", &data);
        }

    }
}

pub async fn metric_processor_main(_c: cmd::Cli) -> () {
    log::trace!("metric processor reached");

    loop {
        sleep(1 as i64)

    }
}

pub async fn log_processor_main(_c: cmd::Cli) -> () {
    log::trace!("log processor reached");

    loop {
        sleep(1 as i64)

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
