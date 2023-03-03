extern crate log;
use yansi::Paint;
use rustyline::error::ReadlineError;
use rustyline::{Editor};
use crate::stdlib::banner;
use crate::cmd;
use crate::lang;

use crate::cmd::tsak_processors;

pub fn run_shell(c: &cmd::Cli, nocolor: u8, args: &Vec<String>)  {
    log::trace!("run_shell() reached");
    if nocolor == 0 {
        log::debug!("Enable colors in shell");
        Paint::enable_windows_ascii();
        Paint::enable();
    } else {
        log::debug!("Disable colors in shell");
        Paint::disable();
    }
    println!("{}", banner::bund_banner());
    let mut engine = lang::LangEngine::init(c);
    engine.set_extra_scope(args);
    tsak_processors::tsak_processors(c.clone(), &engine);
    let mut line = Editor::<()>::new().unwrap();
    if line.load_history(".tsak_history").is_err() {
        log::warn!("No previous history discovered");
    }
    loop {
        let prompt = format!("{}{}{}{}{} {} ", Paint::yellow("["), Paint::red("T"), Paint::blue("S").bold(), Paint::white("A"), Paint::cyan("K"), Paint::green(">").bold());
        let readline = line.readline(&prompt);
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
