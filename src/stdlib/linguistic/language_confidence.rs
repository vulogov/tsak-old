extern crate log;
use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};
use smartstring::SmartString;

use crate::stdlib::linguistic::LANG_DETECTOR;

pub fn detect_confidence(_context: NativeCallContext, d: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let detector = LANG_DETECTOR.lock().unwrap();
    let detected_languages = detector.d.compute_language_confidence_values(d);
    drop(detector);
    let mut res = Map::new();
    for (l,v) in detected_languages {
        let mut key = SmartString::new();
        key.push_str(format!("{}", &l).as_str());
        res.insert(key, Dynamic::from(v.clone()));
    }
    return Result::Ok(Dynamic::from(res));
}
