use lexical_core;
use rhai::{Dynamic};
use std::time::Duration;
use snmp::{SyncSession, Value};

pub fn snmp_get(addr: &String, oid: &String, community: &String) -> Dynamic {
    let mut noid: Vec<u32> = Vec::new();
    for i in oid.split(".") {
        if i.len() == 0 {
            continue;
        }
        match lexical_core::parse::<u32>(&i.as_bytes()) {
            Ok(nres) => noid.push(nres),
            Err(err) => {
                log::error!("Invalid data in OID[{:?}]: {} ", &i, err);
                return Dynamic::UNIT;
            }
        }
    }
    let timeout = Duration::from_secs(2);
    let mut sess = SyncSession::new(addr, community.as_bytes(), Some(timeout), 0).unwrap();
    let resp = sess.getnext(&noid);
    match resp {
        Ok(mut res) => {
            if let Some((_, value)) = res.varbinds.next() {
                return match value {
                    Value::OctetString(val) => Dynamic::from(String::from_utf8_lossy(val).to_string()),
                    Value::Integer(val) => Dynamic::from(val),
                    Value::Counter64(val) => Dynamic::from(val),
                    Value::Boolean(val) => Dynamic::from(val),
                    Value::Counter32(val) |
                    Value::Unsigned32(val) |
                    Value::Timeticks(val)
                        => Dynamic::from(val),
                    _ => Dynamic::from(format!("{:?}", value))
                };
            }
        }
        Err(err) => {
            log::error!("Error getting OID[{:?}] from {:?}: {:?} ", &oid, &addr, err);
            return Dynamic::UNIT;
        }
    }

    Dynamic::UNIT
}
