extern crate log;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};
use reachable::*;
use reachable::target::Status;
use std::str::FromStr;
use crate::stdlib::timestamp::timestamp_module::{timestamp_ms};

pub fn ping_icmp(_context: NativeCallContext, d: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let start = timestamp_ms();
    match IcmpTarget::from_str(&d) {
        Ok(target) => {
            match target.check_availability() {
                Ok(status) => {
                    log::trace!("ICMP ping returned {}", status);
                    if status == Status::Available {
                        let stop = timestamp_ms();
                        return Result::Ok(Dynamic::from((stop-start) as i64));
                    } else {
                        return Err("ICMP Ping not available".into());
                    }
                }
                Err(err) => {
                    return Err(format!("ICMP ping error: {}", err).into());
                }
            }
        },
        Err(err) => {
            return Err(format!("ICMP ping error: {}", err).into());
        }
    }
}

pub fn ping_tcp(_context: NativeCallContext, d: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let start = timestamp_ms();
    match TcpTarget::from_str(&d) {
        Ok(target) => {
            match target.check_availability() {
                Ok(status) => {
                    log::trace!("TCP ping returned {}", status);
                    if status == Status::Available {
                        let stop = timestamp_ms();
                        return Result::Ok(Dynamic::from((stop-start) as i64));
                    } else {
                        return Err("TCP Ping not available".into());
                    }
                }
                Err(err) => {
                    return Err(format!("TCP ping error: {}", err).into());
                }
            }
        },
        Err(err) => {
            return Err(format!("TCP ping error: {}", err).into());
        }
    }
}
