extern crate log;
use rhai::{Dynamic};
use rhai::plugin::*;
use crate::tsak_lib::io::get_file;
use serde_json::{from_str};

#[export_module]
pub mod github_module {
    pub fn search(s: String, t: String) -> Dynamic {
        let q = format!("https://api.github.com/search/code?q={}", &s);
        log::trace!("github::search {:?}", &q);
        let r = get_file::get_file_from_url_with_token(q.to_string(), t);
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
    log::trace!("Running STDLIB::github init");
    let module = exported_module!(github_module);

    engine.register_static_module("github", module.into());


}
