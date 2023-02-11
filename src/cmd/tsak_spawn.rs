extern crate log;
use tokio::task;
use crate::cmd;
use crate::lang;
use crate::tsak_lib::io::get_file;


pub fn run_spawn(c: &cmd::Cli, e: &String, args: &Vec<String>)  {
    log::trace!("run_spawn() reached");
    let t = howlong::HighResolutionTimer::new();
    let mut engine = lang::LangEngine::init(c);

    let mut n = 0;
    for script in args {
        let run_script = get_file::get_file(script.to_string());
        if run_script.is_empty() {
            log::warn!("Spawn script is empty. Skipping");
            continue;
        }
        n += 1;
        let thr_c = c.clone();
        let thr_script = script.clone();
        let thr_run_script = run_script.clone();
        let thr_s = engine.s.clone();
        let thr_r = engine.r;
        task::spawn(
            async move {
                log::trace!("Spawned for {}", &thr_script);
                let mut thr_engine = lang::LangEngine::init_with_channels(&thr_c, thr_s, thr_r);
                let _ = thr_engine.eval_with_scope(&thr_run_script.to_string());
            }
        );
    }
    log::debug!("{} tasks spawned. Continue to main script", &n);
    let main_script = get_file::get_file(e.to_string());
    if main_script.is_empty() {
        log::warn!("Main script file have a zero length which is incorrect");
    } else {
        match engine.eval_with_scope(&main_script.to_string()) {
            Some(res) => log::trace!("Script finished succesfully: {:?}", res),
            None => log::trace!("Script return None"),
        }
    }
    log::debug!("{:?} takes to spawn tasks and run script", t.elapsed());

}
