extern crate log;
use local_ip_address::{local_ip, list_afinet_netifas};
use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};
use crate::stdlib::net::NRIP;

pub fn get_local_ip(_context: NativeCallContext) -> Result<NRIP, Box<EvalAltResult>> {
    match local_ip() {
        Ok(addr) => Result::Ok(NRIP::init_addr(addr)),
        Err(err) => Err(format!("Local IP address: {}", err).into()),
    }
}

pub fn get_local_if(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut res = Map::new();
    if let Ok(network_interfaces) = list_afinet_netifas() {
        for (name, ip) in network_interfaces.iter() {
            res.insert(name.into(), Dynamic::from(format!("{}", &ip)));
        }
    } else {
        return Err("Error getting network interfaces".into());
    }
    return Result::Ok(Dynamic::from(res))
}
