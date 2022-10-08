extern crate log;
use rustyline::error::ReadlineError;
use rustyline::{Editor};
use crate::stdlib::banner;
use crate::cmd;
use crate::lang;

pub fn run_shell(c: &cmd::Cli, args: &Vec<String>)  {
    log::trace!("run_shell() reached");
    println!("{}", banner::bund_banner());
    let mut engine = lang::LangEngine::init();
    engine.set_cli_scope(c);
    engine.set_extra_scope(args);
    let mut line = Editor::<()>::new().unwrap();
    loop {
        let readline = line.readline("[TSAK > ");
        match readline {
            Ok(line) => {
                match engine.run(line.to_string()) {
                    Ok(_) => log::trace!("Script finished succesfully"),
                    Err(err) => log::error!("Error running script: {}", err),
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
    println!("{}", banner::banner(&"Zay Gezunt".to_string()));
}
