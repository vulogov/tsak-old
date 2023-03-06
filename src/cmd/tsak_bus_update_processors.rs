extern crate log;
use crate::cmd;
use std::{thread, time};
use std::time::Duration;
use scaproust::*;
use voca_rs::*;
use rhai::{Dynamic, Map};
use crate::stdlib::bus::bus_update::try_update_bus_push;
use crate::stdlib::timestamp::timestamp_module::timestamp_ms;
use serde_json::{from_str, to_string};

pub async fn bus_update_server_processor_main(c: cmd::Cli) -> () {
    log::debug!("TSAK BUS server reached with binding to {}", &c.bus);
    loop {
        match SessionBuilder::new().with("tcp", Tcp).build() {
            Ok(mut session) => {
                match session.create_socket::<Bus>() {
                    Ok(mut socket) => {
                        let timeout = Duration::from_millis(100);
                        // socket.set_recv_timeout(Some(timeout)).unwrap();
                        match socket.bind(&c.bus) {
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
                                            thread::sleep(time::Duration::from_millis(100));
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
}

pub async fn bus_update_client_processor_main(c: cmd::Cli) -> () {
    log::debug!("TSAK BUS server reached with binding to {}", &c.bus);
    loop {
        match SessionBuilder::new().with("tcp", Tcp).build() {
            Ok(mut session) => {
                match session.create_socket::<Bus>() {
                    Ok(mut socket) => {
                        let timeout = Duration::from_millis(100);
                        // socket.set_recv_timeout(Some(timeout)).unwrap();
                        match socket.connect("tcp://127.0.0.1:20010") {
                            Ok(_) => {
                                loop {
                                    let m = Map::new();
                                    socket.send(to_string::<Map>(&m).unwrap().as_bytes().to_vec());
                                    thread::sleep(time::Duration::from_millis(15000));
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
}
