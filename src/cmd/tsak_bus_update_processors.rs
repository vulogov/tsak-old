extern crate log;
use crate::cmd;
use std::{thread, time};
use std::time::Duration;
use voca_rs::*;
use rhai::{Dynamic, Map};
use crate::stdlib::bus::bus_update::try_update_bus_push;
use crate::stdlib::timestamp::timestamp_module::timestamp_ms;
use serde_json::{from_str, to_string};

pub async fn bus_update_server_processor_main(c: cmd::Cli) -> () {
    log::debug!("TSAK BUS server reached with binding to {}", &c.bus);
    loop {
    }
}

pub async fn bus_update_client_processor_main(c: cmd::Cli) -> () {
    log::debug!("TSAK BUS client reached with connect to {}", &c.bus_connect);
    loop {
        
    }
}
