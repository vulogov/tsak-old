extern crate log;
use rhai::{Engine, Scope, Map};
use tokio::task;
use tokio::time::sleep;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::vec::Vec;
use serde_json::{to_string};

use crate::stdlib::json::json_module::{to_map};
use crate::stdlib::nr::security::raw::{send_security_payload};

#[derive(Debug)]
struct Findings {
    inq:  VecDeque<String>,
    c:   i64,
    r:   bool,
}

impl Findings {
    fn new() -> Self {
        Self {
            inq: VecDeque::new(),
            c:   0,
            r:   false,
        }
    }
}

lazy_static! {
    static ref FINDINGS: Mutex<Findings> = {
        let e: Mutex<Findings> = Mutex::new(Findings::new());
        e
    };
}

async fn security_sender(api: String, key: String)  {
    log::debug!("Findng sender will communicate to: {}", &api);
    log::debug!("Finding sender will use: {}", &key);

    loop {
        sleep(Duration::from_millis(500)).await;
        let mut e = FINDINGS.lock().unwrap();
        if e.inq.len() > 0 {
            e.c = 0;
            e.r = false;
            let mut out: Vec<Map> = Vec::new();
            while e.inq.len() > 0 {
                match e.inq.pop_front() {
                    Some(v) => {
                        e.c += 1;
                        let m = to_map(v);
                        out.push(m);
                    }
                    None => {
                        break;
                    }
                }
                if e.c > 50 {
                    break;
                }
            }
            match to_string(&out) {
                Ok(payload) => {
                    log::debug!("Sending {} events, {} bytes to NR", &e.c, &payload.len());
                    send_security_payload(&api, &key, &payload);
                }
                Err(err) => {
                    log::error!("Error generating payload: {}", err);

                }
            }
            drop(out);
        }
        e.r = true;
        drop(e);
    }
}

pub fn queue_json_payload_to_findings(p: String) -> bool {
    let mut e = FINDINGS.lock().unwrap();
    e.inq.push_back(p);
    true
}

pub fn wait_findings_for_complete()  {
    log::debug!("Waiting for findings queue to clear");
    loop {
        let _ = async { sleep(Duration::from_millis(500)).await; };
        let e = FINDINGS.lock().unwrap();
        if e.r && e.inq.len() == 0 {
            log::debug!("Findings queue is empty");
            drop(e);
            return;
        }
        drop(e);
    }
}

pub fn queue_map_payload_to_findings(p: Map) -> bool {
    match to_string(&p) {
        Ok(res) => {
            let mut e = FINDINGS.lock().unwrap();
            e.inq.push_back(res);
            true
        },
        Err(err) => {
            log::error!("Error converting to JSON: {}", err);
            false
        }
    }
}

pub fn init(_engine: &mut Engine, scope: &mut Scope) {
    log::trace!("Running STDLIB::finding::pipe type init");
    let _ = task::spawn(security_sender(
        scope.get_value::<String>("NR_SEC_API").unwrap(),
        scope.get_value::<String>("NR_INSERT_KEY").unwrap()
    ));
}
