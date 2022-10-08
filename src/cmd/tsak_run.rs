extern crate log;
use crate::cmd;
use crate::lang;

pub fn run_run(c: &cmd::Cli, args: &Vec<String>) {
    log::trace!("run_run() reached");
    let mut engine = lang::LangEngine::init();
    engine.set_cli_scope(c);
    for code in args {
        match engine.run(code.to_string()) {
            Ok(_) => log::trace!("Script finished succesfully"),
            Err(err) => log::error!("Error running script: {}", err),
        }
    }
}
