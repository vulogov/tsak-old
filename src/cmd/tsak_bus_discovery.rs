extern crate log;
use crate::cmd;
use std::time::Duration;
use crate::tsak_lib::discovery;
use portpicker::pick_unused_port;
use crate::stdlib::bus::queue::{queue_push_raw};
use crate::stdlib::system::system_module::sleep_millisecond;


pub async fn bus_discovery_main(c: cmd::Cli) -> () {
    log::debug!("TSAK BUS discovery reached binding to {}", &c.discovery_port);
    let service_port = pick_unused_port().expect("Failure to get a free port for beakon");
    log::debug!("TSAK BUS discovery service port {}", &service_port);
    let service_name = "TSAKBUS".as_bytes();
    match discovery::BeaconSender::new(service_port, service_name, c.discovery_port) {
        Ok(beacon) => {
            match beacon.send_loop(Duration::from_secs(15)) {
                Ok(_) => {},
                Err(err) => {
                    log::error!("Error sending beakon: {}", err);
                }
            }
        }
        Err(_) => {
            log::error!("Failed to create beacon");
        }
    }
}

pub async fn bus_discovery_client_main(c: cmd::Cli) -> () {
    log::debug!("TSAK discovery client is listening on {}", &c.discovery_port);
    let listener = discovery::BeaconListener::new("TSAKBUS".as_bytes(), c.discovery_port).expect("Failure to create client beacon");
    loop {
        let beacon = listener.wait(None).expect("Failed to receive beacon");
        println!("{:?}", &beacon.service_ip);
    }
}
