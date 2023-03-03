extern crate log;
use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};
use crossbeam_deque::{Worker, Steal};
use serde_json::{to_string, from_str};
use crate::stdlib::bus::QUEUES;


pub fn queue_init() {
    log::debug!("Initializing default queues");
    let mut q = QUEUES.lock().unwrap();
    q.insert("events".to_string(), Worker::new_fifo());
    q.insert("metrics".to_string(), Worker::new_fifo());
    q.insert("logs".to_string(), Worker::new_fifo());
    q.insert("vulnerabilities".to_string(), Worker::new_fifo());
    drop(q);
}

pub fn queue_read_payloads(k: String, n: usize) -> Vec<Map> {
    let mut res: Vec<Map> = Vec::new();
    let mut q = QUEUES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        return res;
    }
    let w = q.get_mut(&k).unwrap();
    let s = w.stealer();
    drop(q);
    let mut c = 0;
    for _ in 0..n {
        if s.is_empty() {
            break;
        }
        match s.steal() {
            Steal::Success(val) => {
                match from_str::<Map>(&val) {
                    Ok(jval) => res.push(jval),
                    Err(_) => { c += 1; },
                }
            }
            _ => { c += 1; },
        }
    }
    if c > 0 {
        log::error!("{} errors in JSON payload acquisition in {}", c, &k);
    }
    res
}

pub fn queue_push(_context: NativeCallContext, k: String, d: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    try_queue_push(k, d)
}

pub fn try_queue_push(k: String, d: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    match to_string(&d) {
        Ok(res) => {
            let mut q = QUEUES.lock().unwrap();
            if ! q.contains_key(&k) {
                log::trace!("new bus::internal::queue : {}", &k);
                let w = Worker::new_fifo();
                w.push(res);
                q.insert(k, w);
            } else {
                let w = q.get_mut(&k).unwrap();
                w.push(res);
            }
            drop(q);
        }
        Err(err) => {
            let msg = format!("Error converting to JSON: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into())
        }
    }
    Result::Ok(true)
}

pub fn queue_pull(_context: NativeCallContext, k: String) -> Result<Dynamic, Box<EvalAltResult>> {
    try_queue_pull(k)
}

pub fn try_queue_pull(k: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut q = QUEUES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        return Err(format!("bus::internal::queue no queue: {}", &k).into());
    }
    let w = q.get_mut(&k).unwrap();
    let s = w.stealer();
    drop(q);
    if s.is_empty() {
        return Err(format!("bus::internal::queue is empty: {}", &k).into());
    }
    match s.steal() {
        Steal::Success(res) => {
            match from_str::<Dynamic>(&res) {
                Ok(val) => Result::Ok(val),
                Err(err) => {
                    let msg = format!("Error converting from JSON: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        _ => Err(format!("bus::internal::queue can not pull: {}", &k).into()),
    }
}

pub fn queue_is_empty(_context: NativeCallContext, k: String) -> Result<bool, Box<EvalAltResult>> {
    try_queue_is_empty(k)
}

pub fn try_queue_is_empty(k: String) -> Result<bool, Box<EvalAltResult>> {
    let mut q = QUEUES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        return Err(format!("bus::internal::queue no queue: {}", &k).into());
    }
    let w = q.get_mut(&k).unwrap();
    let s = w.stealer();
    drop(q);
    if s.is_empty() {
        return Result::Ok(true);
    }
    Result::Ok(false)
}
