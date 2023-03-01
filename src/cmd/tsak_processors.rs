extern crate log;
use crate::cmd;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use bastion::{spawn};
use rhai::{Map};

use crate::lang::LangEngine;
use crate::stdlib::system::NRBus;
use crossbeam_channel::{Sender, Receiver};

async fn processor_main(c: cmd::Cli, s: Sender<String>, r: Receiver<String>) -> () {
    log::trace!("cmd::tsak_processors() reached");
    let mut bus = NRBus::init();
    bus.s = s;
    bus.r = r;
    loop {
        match bus.recv() {
            Ok(msg) => {
                if msg.is_map() {
                    let data = msg.cast::<Map>();
                    if data.contains_key("action") && data.contains_key("value") {
                        match data.get("action") {
                            Some(aname) => {
                                let action_name = aname.clone_cast::<String>();
                                if action_name == "spawn".to_string() {
                                    let spawn_r = bus.r.clone();
                                    let spawn_s = bus.s.clone();
                                    match data.get("value") {
                                        Some(code) => {
                                            let spawn_c = code.clone_cast::<String>();
                                            let spawn_cli = c.clone();
                                            spawn!  {
                                                log::info!("TSAK processor is in spawn");
                                                let mut engine = LangEngine::init_with_channels(&spawn_cli, spawn_s, spawn_r);
                                                let _ = engine.run(spawn_c);
                                            };
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
            Err(_) => continue,
        }

    }
}

pub fn tsak_processors(c: cmd::Cli, e: &LangEngine) {
    log::trace!("cmd::tsak_processors() reached");
    let _ = spawn! {
        processor_main(c.clone(), e.s.clone(), e.r.clone())
    };
}
