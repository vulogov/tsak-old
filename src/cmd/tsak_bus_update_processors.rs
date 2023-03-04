extern crate log;
use crate::cmd;
use std::{thread, time};
use std::time::Duration;
use scaproust::*;
use voca_rs::*;

pub async fn bus_update_server_processor_main(c: cmd::Cli) -> () {
    log::debug!("update server reached");
    match SessionBuilder::new().with("tcp", Tcp).build() {
        Ok(mut session) => {
            match session.create_socket::<Pull>() {
                Ok(mut socket) => {
                    let timeout = Duration::from_millis(5000);
                    socket.set_recv_timeout(Some(timeout)).unwrap();
                    match socket.bind(&c.update_uri) {
                        Ok(_) => {
                            loop {
                                match socket.recv() {
                                    Ok(data) => {
                                        println!("{:?}", &data);
                                    }
                                    Err(_) => {
                                        continue;
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
    loop {
        for s in c.update_server.split(",") {
            let srv = manipulate::trim(&manipulate::expand_tabs(&s.to_string(), 1), "");
            log::debug!("Will send heartbeat to {}", &srv);
        }
        thread::sleep(time::Duration::from_millis(5000));
    }
}
