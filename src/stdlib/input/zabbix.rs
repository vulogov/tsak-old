extern crate log;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum ZabbixError {
    NotSupported(String),
}

impl std::fmt::Display for ZabbixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZabbixError::NotSupported(e) => f.write_fmt(format_args!("ZabbixNotSupported ({})", e)),
        }
    }
}
impl std::error::Error for ZabbixError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Run a passive check on a specified host and return the result
pub fn get_metric(
    addr: std::net::SocketAddr,
    name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = vec![];
    log::trace!("input::zabbix[{}:{}", &addr.ip(), &addr.port());
    let mut sock = std::net::TcpStream::connect(addr)?;
    log::trace!("input::zabbix connected");
    sock.write(
        &[
            "ZBXD\x01".as_bytes(),
            (name.len() as u32).to_le_bytes().as_ref(),
            0u32.to_le_bytes().as_ref(),
            name.as_bytes(),
        ]
        .concat(),
    )?;
    sock.read_to_end(&mut buffer)?;
    log::trace!("input::zabbix received");
    let _ = sock.shutdown(std::net::Shutdown::Both);
    log::trace!("input::zabbix closed");
    assert_eq!(&buffer[0..5], &[90, 66, 88, 68, 1]);
    let len = u32::from_le_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]) as usize;
    let response = String::from_utf8_lossy(&buffer[13..13 + len]).to_string();
    if response.starts_with("ZBX_NOTSUPPORTED\0") {
        Err(Box::new(ZabbixError::NotSupported(
            response.split('\0').nth(1).unwrap_or_default().to_owned(),
        )))
    } else {
        Ok(response)
    }
}

pub fn zabbix_get(addr: String, key: String) -> String {
    match addr.parse::<std::net::SocketAddr>() {
        Ok(address) => {
            match get_metric(address, &key) {
                Ok(res) => {
                    log::trace!("Get from Zabbix {}: {}]", &addr, &res);
                    return res;
                }
                Err(err) => {
                    log::error!("Error getting data from Zabbix agent: {}", err);
                }
            }
        }
        Err(err) => {
            log::error!("Error parsing address for input::zabbix: {}", err);
        }
    }

    "".to_string()
}
