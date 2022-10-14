extern crate log;
use rustyline::error::ReadlineError;
use rustyline::{Editor};
use crate::stdlib::banner;
use crate::cmd;
use crate::lang;

pub fn run_shell(c: &cmd::Cli, args: &Vec<String>)  {
    log::trace!("run_shell() reached");
    println!("{}", banner::bund_banner());
    let mut engine = lang::LangEngine::init(c);
    engine.set_cli_scope(c);
    engine.set_extra_scope(args);
    let mut line = Editor::<()>::new().unwrap();
    if line.load_history(".tsak_history").is_err() {
        log::warn!("No previous history discovered");
    }
    loop {
        let readline = line.readline("[TSAK > ");
        match readline {
            Ok(l) => {
                match engine.eval_with_scope(&l.to_string()) {
                    Some(res) => {
                        line.add_history_entry(l.as_str());
                        log::info!("Return={:?}", res)
                    },
                    _ => log::debug!("Script return None"),
                }
            },
            Err(ReadlineError::Interrupted) => {
                log::info!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                log::info!("CTRL-D");
                break
            },
            Err(err) => {
                log::error!("Error: {:?}", err);
                break
            }
        }
    }
    let _ = line.save_history(".tsak_history");
    println!("{}", banner::banner(&"Zay Gezunt".to_string()));
}
