extern crate log;

use rhai::{Dynamic, Map, Array, NativeCallContext, EvalAltResult};
use std::fs::{OpenOptions};
use std::io::BufReader;
use bindet;

pub fn filetype_detect(_context: NativeCallContext, fname: String) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    match OpenOptions::new().read(true).open(&fname) {
        Ok(file) => {
            let buf = BufReader::new(file);
            match bindet::detect(buf).map_err(|e| e.kind()) {
                Ok(res) => {
                    match res {
                        Some(ft) => {
                            let mut out = Array::new();
                            for v in ft.all_matches {
                                let mut row = Map::new();
                                row.insert("full_match".into(), Dynamic::from(v.full_match));
                                row.insert("type".into(), Dynamic::from(format!("{:?}", v.file_type)));
                                out.push(Dynamic::from(row));
                            }
                            return Result::Ok(Dynamic::from(out));
                        }
                        None => return Err("parser::filetype() did not detected type of file".into()),
                    }
                }
                Err(err) => {
                    return Err(format!("parser::filetype() error detection: {}", err).into());
                }
            }
        }
        Err(err) => {
            return Err(format!("parser::filetype() error parsing: {}", err).into());
        }
    }
}
