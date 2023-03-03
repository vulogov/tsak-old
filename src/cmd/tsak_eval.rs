extern crate log;
use crate::cmd;
use crate::lang;
use crate::cmd::tsak_processors;

pub fn run_eval(c: &cmd::Cli, args: &Vec<String>)  {
    log::trace!("run_eval() reached");
    let mut engine = lang::LangEngine::init(c);
    engine.set_extra_scope(args);
    tsak_processors::tsak_processors(c.clone(), &engine);
    log::trace!("Engine established");
    let t = howlong::HighResolutionTimer::new();
    for script in args {
        if script.is_empty() {
            log::warn!("Script file have a zero length which is incorrect");
            continue;
        } else {
            log::trace!("Obtained script {:?} len={}", &script, &script.len());
        }
        match engine.eval_with_scope(&script) {
            Some(res) => log::info!("Script returned: {:?}", &res),
            _ => log::info!("Script returned None"),
        }
    }
    log::debug!("{:?} takes to run script", t.elapsed());
}
