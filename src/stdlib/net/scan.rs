extern crate log;
use netscan::blocking::PortScanner;
use netscan::setting::{ScanType, Destination};
// use netscan::os::{Fingerprinter, ProbeTarget};
use std::time::Duration;
use std::net::{IpAddr};
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn scan_host(_context: NativeCallContext, src: String, dst: String) -> Result<Dynamic, Box<EvalAltResult>> {
    match src.parse::<IpAddr>() {
        Ok(src_addr) => {
            log::info!("net::scan::host src = {}", &src_addr);
            match dst.parse::<IpAddr>() {
                Ok(dst_addr) => {
                    log::info!("net::scan::host dst = {}", &dst_addr);
                    match PortScanner::new(src_addr) {
                        Ok(mut scanner) => {
                            let dst: Destination = Destination::new_with_port_range(dst_addr, 1, 1000);
                            scanner.add_destination(dst);
                            scanner.set_scan_type(ScanType::TcpConnectScan);
                            scanner.set_timeout(Duration::from_millis(10000));
                            scanner.set_wait_time(Duration::from_millis(100));
                            scanner.set_send_rate(Duration::from_millis(1));
                            let res = scanner.scan();
                            println!("{:?}", &res);
                        }
                        Err(err) => {
                            let msg = format!("net::scan::host error: {}", err);
                            log::error!("{}", &msg);
                            return Err(msg.into());
                        }
                    }
                }
                Err(err) => {
                    let msg = format!("net::scan::host error: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => {
            let msg = format!("net::scan::host error: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into());
        }
    }
    Result::Ok(Dynamic::default())
}

// pub fn fingerprint_host(_context: NativeCallContext, src: String, dst: String) -> Result<Dynamic, Box<EvalAltResult>> {
//     match src.parse::<IpAddr>() {
//         Ok(src_addr) => {
//             log::info!("net::scan::fingerprint src = {}", &src_addr);
//             match dst.parse::<IpAddr>() {
//                 Ok(dst_addr) => {
//                     log::info!("net::scan::fingerprint dst = {}", &dst_addr);
//                     match Fingerprinter::new(src_addr) {
//                         Ok(mut scanner) => {
//                             scanner.set_wait_time(Duration::from_millis(200));
//                             let dst: ProbeTarget = ProbeTarget {
//                                 ip_addr: dst_addr,
//                                 open_tcp_ports: vec![22,80],
//                                 closed_tcp_port: 443,
//                                 open_udp_port: 123,
//                                 closed_udp_port: 33455,
//                             };
//                             scanner.add_probe_target(dst);
//                             scanner.set_full_probe();
//                             let results = scanner.probe();
//                             println!("{:?}", &results);
//                         }
//                         Err(err) => {
//                             let msg = format!("net::scan::fingerprint error: {}", err);
//                             log::error!("{}", &msg);
//                             return Err(msg.into());
//                         }
//                     }
//                 }
//                 Err(err) => {
//                     let msg = format!("net::scan::fingerprint error: {}", err);
//                     log::error!("{}", &msg);
//                     return Err(msg.into());
//                 }
//             }
//         }
//         Err(err) => {
//             let msg = format!("net::scan::fingerprint error: {}", err);
//             log::error!("{}", &msg);
//             return Err(msg.into());
//         }
//     }
//     Result::Ok(Dynamic::default())
// }
