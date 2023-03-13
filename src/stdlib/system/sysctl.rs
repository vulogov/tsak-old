extern crate log;
use sysctl;
use sysctl::{Sysctl, CtlValue};
use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};

pub fn sysctl_get(_context: NativeCallContext, t: String) -> Result<Dynamic, Box<EvalAltResult>> {
    try_sysctl_get(t)
}

pub fn try_sysctl_get(t: String) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    match sysctl::Ctl::new(&t) {
        Ok(ctl) => {
            let mut res = Map::new();
            match ctl.value() {
                Ok(value) => {
                    match value {
                        CtlValue::Int(v) => res.insert("value".into(), Dynamic::from(v as i32)),
                        CtlValue::String(v) => res.insert("value".into(), Dynamic::from(v as String)),
                        CtlValue::S64(v) => res.insert("value".into(), Dynamic::from(v as i64)),
                        CtlValue::Uint(v) => res.insert("value".into(), Dynamic::from(v as u32)),
                        CtlValue::Long(v) => res.insert("value".into(), Dynamic::from(v as i64)),
                        CtlValue::Ulong(v) => res.insert("value".into(), Dynamic::from(v as u64)),
                        CtlValue::U64(v) => res.insert("value".into(), Dynamic::from(v as u64)),
                        CtlValue::U32(v) => res.insert("value".into(), Dynamic::from(v as u32)),
                        _ => {
                            let v: String = value.into_string().unwrap();
                            res.insert("value".into(), Dynamic::from(v.clone()))
                        }
                    }
                }
                Err(_) => res.insert("value".into(), Dynamic::from("Unavailable")),
            };
            match ctl.description() {
                Ok(d) => res.insert("description".into(), Dynamic::from(d)),
                Err(_) => res.insert("description".into(), Dynamic::from("Unavailable")),
            };
            return Result::Ok(Dynamic::from(res));
        }
        Err(err) => return Err(format!("{}", err).into()),
    }
}
