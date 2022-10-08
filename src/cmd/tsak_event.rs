extern crate log;
use std::thread::sleep;
use std::time::Duration;
use rhai::{format_map_as_json};
use crate::cmd;
use crate::lang;
use crate::tsak_lib::io::get_file;
use crate::stdlib::nr::event::raw;

pub fn run_event(c: &cmd::Cli, l: u8, e: u32, s: &String, args: &Vec<String>) {
    log::trace!("run_event() reached");
    let mut engine = lang::LangEngine::init();
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
        calculate_event(c, &mut engine, &script);
    } else {
        loop {
            if ! calculate_event(c, &mut engine, &script) {
                log::error!("Error during event generation");
                break;
            }
            sleep(Duration::from_secs(e.into()));
        }
    }
}

fn calculate_event(c: &cmd::Cli, e: &mut lang::LangEngine, s: &String) -> bool {
    log::debug!("Calculating event");
    let res = e.eval_map_with_scope(s);
    match &res {
        Some(event) => {
            log::debug!("Calculation returns={:?}", &event);
            let payload = format_map_as_json(&event);
            log::debug!("JSON payload={}", &payload);
            raw::send_event_payload(&c.nr_event, &c.nr_account, &c.nr_insert_key, &format!("[{}]",payload));
        }
        _ => {
            log::error!("Calculation script doesn't return suitable result");
            return false;
        }
    }
    return true;
}
