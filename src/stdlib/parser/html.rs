extern crate log;
use html_parser::Dom;
use serde_json::{from_str};
use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};

pub fn html_parse(_context: NativeCallContext, t: String) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    match Dom::parse(&t) {
        Ok(doc) => {
            let errors = &doc.errors;
            if errors.len() == 0 {
                match doc.to_json() {
                    Ok(document) => {
                        match from_str::<Map>(&document) {
                            Ok(res) => {
                                return Result::Ok(Dynamic::from(res));
                            }
                            Err(err) => {
                                return Err(format!("parse::html() json parsing: {}", err).into());
                            }
                        }
                    }
                    Err(err) => {
                        return Err(format!("parse::html() json parsing: {}", err).into());
                    }
                }
            } else {
                for m in errors {
                    log::error!("parse::html() parse error: {}", &m);
                }
                return Err("parse::html() error parsing".into());
            }
        }
        Err(err) => {
            return Err(format!("parse::html() error parsing: {}", err).into());
        }
    }
}
