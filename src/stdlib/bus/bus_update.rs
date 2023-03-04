extern crate log;
use scaproust::*;
use std::time::Duration;
use rhai::{Map, NativeCallContext, EvalAltResult};
use serde_json::{to_string, from_str};



pub fn update_bus_push(_context: NativeCallContext, uri: String, d: Map) -> Result<bool, Box<EvalAltResult>> {
    try_update_bus_push(uri, d)
}

pub fn try_update_bus_push(uri: String, d: Map) -> Result<bool, Box<EvalAltResult>> {
    match SessionBuilder::new().with("tcp", Tcp).build() {
        Ok(mut session) => {
            match session.create_socket::<Push>() {
                Ok(mut socket) => {
                    let timeout = Duration::from_millis(1000);
                    socket.set_send_timeout(Some(timeout)).unwrap();
                    match socket.connect(&uri) {
                        Ok(_) => {
                            match to_string(&d) {
                                Ok(res) => {
                                    match socket.send(res.as_bytes().to_vec()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            let msg = format!("bus::cluster::update send error: {}", err);
                                            log::error!("{}", &msg);
                                            return Err(msg.into())
                                        }
                                    }
                                }
                                Err(err) => {
                                    let msg = format!("bus::cluster::update converting to JSON: {}", err);
                                    log::error!("{}", &msg);
                                    return Err(msg.into())
                                }
                            }
                        }
                        Err(err) => {
                            let msg = format!("bus::cluster::update connect error: {}", err);
                            log::error!("{}", &msg);
                            return Err(msg.into())
                        }
                    }
                }
                Err(err) => {
                    let msg = format!("bus::cluster::update socket error: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into())
                }
            }
        }
        Err(err) => {
            let msg = format!("bus::cluster::update session error: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into())
        }
    }
    Result::Ok(true)
}
