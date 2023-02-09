extern crate log;
use rhai::{Dynamic};
use rhai::plugin::*;
use crate::tsak_lib::io::get_file;
use urlencoding::encode;
use serde_json::{from_str};

#[export_module]
pub mod internetsearch_module {
    pub fn ddg(s: String) -> Dynamic {
        let q = format!("http://api.duckduckgo.com/?q={}&format=json", encode(&s));
        let r = get_file::get_file(q.to_string());
        match from_str(&r) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error converting from JSON: {}", err);
                return Dynamic::default();
            }
        }
    }
}





pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::internetsearch init");
    let module = exported_module!(internetsearch_module);

    engine.register_static_module("internetsearch", module.into());


}
