extern crate log;
use rhai::{NativeCallContext, EvalAltResult};

use crate::stdlib::linguistic::LANG_DETECTOR;

pub fn detect_language(_context: NativeCallContext, d: String) -> Result<String, Box<EvalAltResult>> {
    let detector = LANG_DETECTOR.lock().unwrap();
    let detected_language = detector.d.detect_language_of(d);
    drop(detector);
    match detected_language {
        Some(lang) => return Result::Ok(format!("{}", lang).to_string()),
        _ => return Err("Can not detect language".into()),
    }
}
