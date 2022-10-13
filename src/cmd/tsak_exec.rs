extern crate log;
use crate::cmd;
use crate::lang;
use crate::tsak_lib::io::get_file;

pub fn run_exec(c: &cmd::Cli, s: &String, args: &Vec<String>) {
    log::trace!("run_exec() reached");
    let mut engine = lang::LangEngine::init(c);
    engine.set_cli_scope(c);
    engine.set_extra_scope(args);
    log::trace!("Engine established");
    let script = get_file::get_file(s.to_string());
    if script.is_empty() {
        log::warn!("Script file have a zero length which is incorrect");
        return;
    } else {
        log::trace!("Obtained script fname={} len={}", &s, &script.len());
    }

    let t = howlong::HighResolutionTimer::new();
    let _ = engine.run_with_scope(&script);
    log::debug!("{:?} takes to run script", t.elapsed());
}
