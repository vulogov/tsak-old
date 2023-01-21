use lexical_core;
use std::time::Duration;
use snmp::{SyncSession, Value};

pub fn snmp_get(addr: &String, oid: &String, community: &String) -> String {
    let mut noid: Vec<u32> = Vec::new();
    for i in oid.split(".") {
        noid.push(lexical_core::parse::<u32>(&i.as_bytes()).unwrap());
    }
    let timeout = Duration::from_secs(2);
    let mut sess = SyncSession::new(addr, community.as_bytes(), Some(timeout), 0).unwrap();
    let mut resp = sess.getnext(&noid).unwrap();
    if let Some((_, value)) = resp.varbinds.next() {
        return match value {
            Value::OctetString(val) => String::from_utf8_lossy(val).to_string(),
            Value::Integer(val) => format!("{}", val),
            Value::Counter32(val) |
            Value::Unsigned32(val)
                => format!("{}", val),
            _ => format!("{:?}", value)
        };
    }
    String::from_utf8_lossy(b"").to_string()
}
