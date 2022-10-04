extern crate log;
use crate::cmd;
use crate::lang;

pub fn run_run(_c: &cmd::Cli, a: &Vec<String>) {
    log::trace!("run_run() reached");
    let mut engine = lang::LangEngine::init();
    for code in a {
        match engine.run(code.to_string()) {
            Ok(_) => log::trace!("Script finished succesfully"),
            Err(err) => log::error!("Error running script: {}", err),
        }
    }
}
