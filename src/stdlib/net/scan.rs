extern crate log;
use netscan::blocking::PortScanner;
use netscan::setting::{ScanType, Destination};
use netscan::os::{Fingerprinter, ProbeTarget};
use netscan::service::{ServiceDetector, PortDatabase};
use std::time::Duration;
use std::net::{IpAddr};
use sudo;
use rhai::{Dynamic, Array, Map, NativeCallContext, EvalAltResult};


pub fn scan_host(_context: NativeCallContext, src: String, dst: String) -> Result<Dynamic, Box<EvalAltResult>> {
    match sudo::check() {
        sudo::RunningAs::User => {
            return Err("Host scanning require TSAK to be in privilege mode".into());
        }
        _ => {}
    }
    match src.parse::<IpAddr>() {
        Ok(src_addr) => {
            log::debug!("net::scan::host src = {}", &src_addr);
            match dst.parse::<IpAddr>() {
                Ok(dst_addr) => {
                    log::debug!("net::scan::host dst = {}", &dst_addr);
                    match PortScanner::new(src_addr) {
                        Ok(mut scanner) => {
                            let dst: Destination = Destination::new_with_port_range(dst_addr, 1, 1000);
                            scanner.add_destination(dst);
                            scanner.set_scan_type(ScanType::TcpConnectScan);
                            scanner.set_timeout(Duration::from_millis(10000));
                            scanner.set_wait_time(Duration::from_millis(100));
                            scanner.set_send_rate(Duration::from_millis(1));
                            let res = scanner.scan();
                            let mut out = Array::new();
                            for (_, ports) in res.result_map {
                                for port in ports {
                                    let mut row = Array::new();
                                    row.push(Dynamic::from(port.port as i64));
                                    row.push(Dynamic::from(format!("{:?}", port.status)));
                                    out.push(Dynamic::from(row));
                                }
                            }
                            return Result::Ok(Dynamic::from(out));
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
}

pub fn fingerprint_host(_context: NativeCallContext, src: String, dst: String) -> Result<Dynamic, Box<EvalAltResult>> {
    match sudo::check() {
        sudo::RunningAs::User => {
            return Err("Host fingerprinting require TSAK to be in privilege mode".into());
        }
        _ => {}
    }
    match src.parse::<IpAddr>() {
        Ok(src_addr) => {
            log::debug!("net::scan::fingerprint src = {}", &src_addr);
            match dst.parse::<IpAddr>() {
                Ok(dst_addr) => {
                    log::debug!("net::scan::fingerprint dst = {}", &dst_addr);
                    match Fingerprinter::new(src_addr) {
                        Ok(mut scanner) => {
                            scanner.set_wait_time(Duration::from_millis(200));
                            let dst: ProbeTarget = ProbeTarget {
                                ip_addr: dst_addr,
                                open_tcp_ports: vec![22,80],
                                closed_tcp_port: 443,
                                open_udp_port: 123,
                                closed_udp_port: 33455,
                            };
                            scanner.add_probe_target(dst);
                            scanner.set_full_probe();
                            let results = scanner.probe();
                            let mut out = Map::new();
                            for result in results {
                                let mut row = Map::new();
                                row.insert("icmp_echo_reply".into(), Dynamic::from(result.icmp_echo_result.unwrap().icmp_echo_reply));
                                row.insert("icmp_echo_code".into(), Dynamic::from(result.icmp_echo_result.unwrap().icmp_echo_code));
                                row.insert("ip_id".into(), Dynamic::from(result.icmp_echo_result.unwrap().ip_id));
                                row.insert("ip_df".into(), Dynamic::from(result.icmp_echo_result.unwrap().ip_df));
                                row.insert("ip_ttl".into(), Dynamic::from(result.icmp_echo_result.unwrap().ip_ttl));
                                row.insert("icmp_timestamp_reply".into(), Dynamic::from(result.icmp_timestamp_result.unwrap().icmp_timestamp_reply));
                                row.insert("icmp_address_mask_reply".into(), Dynamic::from(result.icmp_address_mask_result.unwrap().icmp_address_mask_reply));
                                row.insert("icmp_information_reply".into(), Dynamic::from(result.icmp_information_result.unwrap().icmp_information_reply));
                                out.insert(format!("{}", result.ip_addr).into(), Dynamic::from(row));
                            }
                            return Result::Ok(Dynamic::from(out));
                        }
                        Err(err) => {
                            let msg = format!("net::scan::fingerprint error: {}", err);
                            log::error!("{}", &msg);
                            return Err(msg.into());
                        }
                    }
                }
                Err(err) => {
                    let msg = format!("net::scan::fingerprint error: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => {
            let msg = format!("net::scan::fingerprint error: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into());
        }
    }
}

pub fn services_host(_context: NativeCallContext, src: String, dst: String, p_low: i64, p_high: i64) -> Result<Dynamic, Box<EvalAltResult>> {
    match sudo::check() {
        sudo::RunningAs::User => {
            return Err("Host services scan require TSAK to be in privilege mode".into());
        }
        _ => {}
    }
    match src.parse::<IpAddr>() {
        Ok(src_addr) => {
            log::debug!("net::scan::services src = {}", &src_addr);
            match dst.parse::<IpAddr>() {
                Ok(dst_addr) => {
                    log::debug!("net::scan::services dst = {}", &dst_addr);
                    match PortScanner::new(src_addr) {
                        Ok(mut scanner) => {
                            let dst: Destination = Destination::new_with_port_range(dst_addr, p_low as u16, p_high as u16);
                            scanner.add_destination(dst);
                            scanner.set_scan_type(ScanType::TcpSynScan);
                            scanner.set_timeout(Duration::from_millis(10000));
                            scanner.set_wait_time(Duration::from_millis(100));
                            scanner.set_send_rate(Duration::from_millis(1));
                            let res = scanner.scan();
                            let mut out = Map::new();
                            for (ip, _) in res.result_map.clone() {
                                let mut row = Map::new();
                                let mut service_detector = ServiceDetector::new();
                                service_detector.set_dst_ip(ip);
                                service_detector.set_ports(res.get_open_ports(ip));
                                let service_map = service_detector.detect(Some(PortDatabase::default()));
                                for (k, v) in service_map {
                                    row.insert(format!("{}", k).into(), Dynamic::from(v));
                                }
                                out.insert(format!("{}", &ip).into(), Dynamic::from(row));
                            }
                            return Result::Ok(Dynamic::from(out));
                        }
                        Err(err) => {
                            let msg = format!("net::scan::host error: {}", err);
                            log::error!("{}", &msg);
                            return Err(msg.into());
                        }
                    }
                }
                Err(err) => {
                    let msg = format!("net::scan::services error: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => {
            let msg = format!("net::scan::services error: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into());
        }
    }
}
