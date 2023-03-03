extern crate log;
use std::net::{IpAddr, Ipv4Addr};
use rhai::{Module, Engine};

mod ping;
mod scan;
mod interfaces;

#[derive(Debug, Clone)]
pub struct NRIP {
    ip:     IpAddr,
}

impl NRIP {
    fn new() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
    pub fn init_str(addr: String) -> NRIP {
        let mut res = NRIP::new();
        match addr.parse::<IpAddr>() {
            Ok(a) => {
                res.ip = a;
            }
            Err(_) => {
                log::error!("Invalid IP address to parse: {}", &addr);
            }
        }
        res
    }
    pub fn init_addr(addr: IpAddr) -> NRIP {
        let mut res = NRIP::new();
        res.ip = addr;
        res
    }
    pub fn is_ipv4(&mut self) -> bool {
        self.ip.is_ipv4()
    }
    pub fn is_ipv6(&mut self) -> bool {
        self.ip.is_ipv6()
    }
    pub fn is_loopback(&mut self) -> bool {
        self.ip.is_loopback()
    }
    pub fn raw(&mut self) -> String {
        format!("{}", self.ip)
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::net init");

    engine.register_type::<NRIP>()
          .register_fn("IP", NRIP::new)
          .register_fn("IP", NRIP::init_str)
          .register_fn("is_ipv4", NRIP::is_ipv4)
          .register_fn("is_ipv6", NRIP::is_ipv6)
          .register_fn("is_loopback", NRIP::is_loopback)
          .register_fn("raw", NRIP::raw)
          .register_fn("to_string", |x: &mut NRIP| format!("{:?}", x.ip) );

    let mut module = Module::new();
    module.set_native_fn("ip", interfaces::get_local_ip);
    module.set_native_fn("interfaces", interfaces::get_local_if);

    let mut ping_module = Module::new();
    ping_module.set_native_fn("icmp", ping::ping_icmp);
    ping_module.set_native_fn("tcp", ping::ping_tcp);
    module.set_sub_module("ping", ping_module);

    let mut scan_module = Module::new();
    scan_module.set_native_fn("host", scan::scan_host);
    // scan_module.set_native_fn("fingerprint", scan::fingerprint_host);
    module.set_sub_module("scan", scan_module);

    engine.register_static_module("net", module.into());


}
