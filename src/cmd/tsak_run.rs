extern crate log;
use crate::cmd;
use crate::lang;

pub fn run_run(c: &cmd::Cli, e: &String, args: &Vec<String>) {
    log::trace!("run_run() reached");
    let mut engine = lang::LangEngine::init(c);
    engine.set_extra_scope(args);
    match engine.eval_with_scope(&e.to_string()) {
        Some(res) => log::trace!("Script finished succesfully: {:?}", res),
        None => log::trace!("Script return None"),
    }
}
