extern crate log;
use crate::cmd;
use zeromq::prelude::*;
use crate::stdlib::bus::queue::{queue_read_string_payloads, queue_push_raw};
use crate::stdlib::system::system_module::sleep_millisecond;


pub async fn bus_update_server_processor_main(c: cmd::Cli) -> () {
    log::debug!("TSAK BUS server reached with binding to {}", &c.bus);
    let mut socket = zeromq::PubSocket::new();
    match socket.bind(&c.bus).await {
        Ok(_) => {
            loop {
                let q = queue_read_string_payloads("bus_publish".to_string(), 50);
                if q.len() > 0 {
                    for data in q {
                        if data.len() > 0 {
                            match socket.send(data.into()).await {
                                _ => continue,
                            }
                        }
                    }
                } else {
                    sleep_millisecond(100);
                }
            }
        }
        Err(err) => {
            log::error!("Error binding TSAK bus to {}: {}", &c.bus, err);
            return;
        }
    }
}

pub async fn bus_update_client_processor_main(_c: cmd::Cli, srv: String) -> () {
    log::info!("TSAK BUS client reached with connect to {}", &srv);
    let mut socket = zeromq::SubSocket::new();
    match socket.connect(&srv).await {
        Ok(_) => {
            match socket.subscribe("").await {
                Ok(_) => {
                    loop {
                        match socket.recv().await {
                            Ok(res) => {
                                match res.get(0) {
                                    Some(data) => {
                                        match std::str::from_utf8(&data.to_vec()) {
                                            Ok(val) => {
                                                queue_push_raw("bus_receive".into(), val.to_string());
                                            }
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                }
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                }
                Err(err) => {
                    log::error!("Error subscribing to TSAK bus at {}: {}", &srv, err);
                }
            }
        }
        Err(err) => {
            log::error!("Error connecting TSAK bus to {}: {}", &srv, err);
            return;
        }
    }
}
