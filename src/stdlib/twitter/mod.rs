extern crate log;
use rhai::{Engine, Map, Identifier, Dynamic};
use rhai::plugin::*;
use crate::tsak_lib::io::get_file::get_file_from_url_with_bearer;

#[export_module]
pub mod twitter_module {
    pub fn search(s: String) {
        let res = get_file_from_url_with_bearer("https://api.twitter.com/2/tweets/search/recent?query=from:twitterdev".to_string(), "".to_string());
        println!("{:?}", &res);
    }
}





pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::twitter init");
    let module = exported_module!(twitter_module);

    engine.register_static_module("twitter", module.into());


}
