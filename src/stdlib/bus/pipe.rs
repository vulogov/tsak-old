extern crate log;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};
use crossbeam_channel::{unbounded};
use serde_json::{to_string, from_str};
use crate::stdlib::bus::PIPES;

pub fn pipes_init() {
    log::debug!("Initializing default pipes");
    let mut q = PIPES.lock().unwrap();
    q.insert("bus".to_string(), unbounded::<String>());
    drop(q);
}

pub fn create_pipe(n: String) {
    log::debug!("Create pipe: {}", &n);
    let mut q = PIPES.lock().unwrap();
    q.insert(n.to_string(), unbounded::<String>());
    drop(q);
}

pub fn pipe_push_raw(k: String, d: String) {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        log::trace!("new bus::internal::pipe : {}", &k);
        let (s,r) = unbounded::<String>();
        match s.send(d) {
            Ok(_) => {
                q.insert(k, (s,r));
            }
            Err(_) => {
                drop(q);
            }
        };
    } else {
        let (s,_) = q.get_mut(&k).unwrap();
        match s.send(d) {
            Ok(_) => {},
            Err(_) => {
                drop(q);
            }
        }
    }
}
pub fn pipe_push(_context: NativeCallContext, k: String, d: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    try_pipe_push(k,d)
}

pub fn try_pipe_push(k: String, d: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    match to_string(&d) {
        Ok(res) => {
            let mut q = PIPES.lock().unwrap();
            if ! q.contains_key(&k) {
                log::trace!("new bus::internal::pipe : {}", &k);
                let (s,r) = unbounded::<String>();
                match s.send(res) {
                    Ok(_) => q.insert(k, (s,r)),
                    Err(err) => {
                        drop(q);
                        return Err(format!("bus::internal::pipe error: {}", err).into());
                    }
                };
            } else {
                let (s,_) = q.get_mut(&k).unwrap();
                match s.send(res) {
                    Ok(_) => {},
                    Err(err) => {
                        drop(q);
                        return Err(format!("bus::internal::pipe error: {}", err).into());
                    }
                }
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

pub fn pipe_pull(_context: NativeCallContext, k: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        return Err(format!("bus::internal::pipe no pipe: {}", &k).into());
    }
    let (_, r) = q.get_mut(&k).unwrap();
    if r.is_empty() {
        return Err(format!("bus::internal::pipe is empty: {}", &k).into());
    }
    match r.recv() {
        Ok(res) => {
            match from_str::<Dynamic>(&res) {
                Ok(val) => {
                    drop(q);
                    return Result::Ok(val);
                }
                Err(err) => {
                    let msg = format!("Error converting from JSON: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => Err(format!("bus::internal::pipe {} can not recv: {}", &k, &err).into()),
    }
}

pub fn pipe_log_pull(_context: NativeCallContext, logname: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let pname = format!("logfile:{}", &logname);
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&pname) {
        drop(q);
        return Err(format!("bus::internal::pipe no pipe: {}", &pname).into());
    }
    let (_, r) = q.get_mut(&pname).unwrap();
    if r.is_empty() {
        return Err(format!("bus::internal::pipe is empty: {}", &pname).into());
    }
    match r.recv() {
        Ok(res) => {
            return Result::Ok(Dynamic::from(res as String));
        }
        Err(err) => Err(format!("bus::internal::pipe {} can not recv: {}", &pname, &err).into()),
    }
}

pub fn pipe_is_empty(_context: NativeCallContext, k: String) -> Result<bool, Box<EvalAltResult>> {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        return Err(format!("bus::internal::pipe no pipe: {}", &k).into());
    }
    let (_, r) = q.get_mut(&k).unwrap();
    if r.is_empty() {
        drop(q);
        return Result::Ok(true);
    }
    drop(q);
    Result::Ok(false)
}

pub fn pipe_have_no_logs(context: NativeCallContext, logname: String) -> Result<bool, Box<EvalAltResult>> {
    let pname = format!("logfile:{}", &logname);
    pipe_is_empty(context, pname)
}
