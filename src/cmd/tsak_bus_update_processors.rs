extern crate log;
use crate::cmd;
use std::{thread, time};
use std::time::Duration;
use scaproust::*;
use voca_rs::*;
use rhai::{Dynamic, Map};
use crate::stdlib::bus::bus_update::try_update_bus_push;
use crate::stdlib::timestamp::timestamp_module::timestamp_ms;
use serde_json::{from_str};

pub async fn bus_update_server_processor_main(c: cmd::Cli) -> () {
    log::debug!("update server reached");
    match SessionBuilder::new().with("tcp", Tcp).build() {
        Ok(mut session) => {
            match session.create_socket::<Pull>() {
                Ok(mut socket) => {
                    // let timeout = Duration::from_millis(5000);
                    // socket.set_recv_timeout(Some(timeout)).unwrap();
                    match socket.bind(&c.update_uri) {
                        Ok(_) => {
                            loop {
                                match socket.recv() {
                                    Ok(data) => {
                                        let buffer: String = std::str::from_utf8(&data).unwrap().to_string();
                                        match from_str::<Map>(&buffer) {
                                            Ok(pkt) => {
                                                println!("{:?}", &pkt);
                                            }
                                            Err(err) => {
                                                println!("{}", err);
                                                continue;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        log::error!("error {}", err);
                                        thread::sleep(time::Duration::from_millis(1000));
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            log::error!("Error bind to update server socket: {}", err);
                        }
                    }
                }
                Err(err) => {
                    log::error!("Error creating update server socket: {}", err);
                }
            }
        }
        Err(err) => {
            log::error!("Error creating update server session: {}", err);
        }
    }
}

pub async fn bus_update_client_processor_main(c: cmd::Cli) -> () {
    let mut upd_pkt = Map::new();
    upd_pkt.insert("name".into(), Dynamic::from(c.name));
    upd_pkt.insert("capability".into(), Dynamic::from(c.capability));
    loop {
        upd_pkt.insert("timestamp".into(), Dynamic::from(timestamp_ms()));
        for s in c.update_server.split(",") {
            let srv = manipulate::trim(&manipulate::expand_tabs(&s.to_string(), 1), "");
            log::debug!("Will send heartbeat to {}", &srv);
            match try_update_bus_push(srv, upd_pkt.clone()) {
                Ok(_) => {},
                Err(err) => {
                    log::error!("update error: {}", err);
                    break;
                }
            }
        }
        thread::sleep(time::Duration::from_millis(15000));
    }
}
