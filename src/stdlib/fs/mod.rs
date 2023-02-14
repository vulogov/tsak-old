extern crate log;
use rhai::{Engine};
use rhai::plugin::*;

mod search;

#[export_module]
pub mod fs_module {

}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::fs init");
    let mut module = exported_module!(fs_module);
    let mut search_module = Module::new();
    search_module.set_native_fn("find", search::search_find);
    search_module.set_native_fn("find_ext", search::search_find_ext);
    search_module.set_native_fn("find_zero", search::search_find_zero);
    search_module.set_native_fn("find_json", search::search_find_json);
    search_module.set_native_fn("zip", search::search_find_zip);
    module.set_sub_module("search", search_module);
    engine.register_static_module("fs", module.into());
}
