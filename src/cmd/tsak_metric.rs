extern crate log;
use std::thread::sleep;
use std::time::Duration;
use rhai::{format_map_as_json};
use crate::cmd;
use crate::lang;
use crate::tsak_lib::io::get_file;
use crate::stdlib::nr::metric::raw;

pub fn run_metric(c: &cmd::Cli, l: u8, e: u32, s: &String, args: &Vec<String>) {
    log::trace!("run_metric() reached");
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
    if l == 0 {
        calculate_metric(c, &mut engine, &script);
    } else {
        loop {
            let t = howlong::HighResolutionTimer::new();
            if ! calculate_metric(c, &mut engine, &script) {
                log::error!("Error during metric generation");
                break;
            }
            log::debug!("{:?} takes to calculate and send metric", t.elapsed());
            sleep(Duration::from_secs(e.into()));
        }
    }
    log::debug!("Submission is finished");
}

fn calculate_metric(c: &cmd::Cli, e: &mut lang::LangEngine, s: &String) -> bool {
    log::debug!("Calculating metric");
    let res = e.eval_map_with_scope(s);
    match &res {
        Some(metric) => {
            log::debug!("Calculation returns={:?}", &metric);
            let payload = format_map_as_json(&metric);
            log::debug!("JSON payload={}", &payload);
            raw::send_metric_payload(&c.nr_event, &c.nr_insert_key, &format!("[{}]",payload));
        }
        _ => {
            log::error!("Calculation script doesn't return suitable result");
            return false;
        }
    }
    return true;
}
