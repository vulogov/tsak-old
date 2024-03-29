extern crate log;
use std::{env};
use std::{thread, time};
use rhai::{Dynamic, Module, EvalAltResult};
use rhai::plugin::*;
use crossbeam_channel::{unbounded, Sender, Receiver};
use proctitle::set_title;
use rhai::serde::{to_dynamic};
use serde_json::{to_string, from_str};

use sudo;

use crate::lang::{LangEngine};
pub mod system_metrics;


mod run;
mod system_loop;
pub mod globals;
pub mod dmesg;
pub mod sysctl;

#[export_module]
pub mod system_module {
    pub fn sleep(s: i64) {
        thread::sleep(time::Duration::from_secs(s as u64));
    }
    pub fn sleep_millisecond(s: i64) {
        thread::sleep(time::Duration::from_millis(s as u64));
    }
    pub fn env(n: String) -> String {
        match env::var(&n) {
            Ok(val) => return val,
            Err(e) => {
                log::error!("Error fetching environment variable {}: {:?}", &n, e);
            }
        }
        return "".to_string();
    }
    pub fn running_as() -> String {
        match sudo::check() {
            sudo::RunningAs::Root => "root".to_string(),
            sudo::RunningAs::User => "user".to_string(),
            sudo::RunningAs::Suid => "suid".to_string(),
        }
    }
    pub fn title(t: String) {
        set_title(t);
    }
}

#[derive(Debug, Clone)]
pub struct NRBus {
    pub s: Sender<String>,
    pub r: Receiver<String>,
}

impl NRBus {
    fn new() -> Self {
        let (s,r) = unbounded::<String>();
        Self {
            s: s,
            r: r,
        }
    }
    pub fn init() -> NRBus {
        NRBus::new()
    }

    pub fn send(&mut self, d: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        match to_string(&d) {
            Ok(res) => {
                match self.s.send(res) {
                    Ok(_) => {},
                    Err(_) => {
                        return Err("Error sending to bus".into());
                    }
                }
            },
            Err(_) => {
                return Err("Error converting to JSON".into());
            }
        };
        return Ok(Dynamic::from(self.s.len() as i64));
    }
    pub fn try_recv_raw(&mut self) -> Result<String, Box<EvalAltResult>> {
        if self.r.len() == 0 {
            return Err("Bus is empty".into());
        }
        match self.r.recv() {
            Ok(res) => {
                return Ok(res);
            },
            Err(_) => {
                return Err("Error receiving from bus".into());
            }
        }
    }
    pub fn try_recv(&mut self) -> Result<Dynamic, Box<EvalAltResult>> {
        if self.r.len() == 0 {
            return Ok(Dynamic::default());
        }
        match self.r.recv() {
            Ok(res) => {
                match to_dynamic(&res) {
                    Ok(val) => {
                        return Ok(val);
                    },
                    Err(_) => {
                        return Err("Error converting from JSON".into());
                    }
                };
            },
            Err(_) => {
                return Err("Error receiving from bus".into());
            }
        }
    }
    pub fn recv(&mut self) -> Result<Dynamic, Box<EvalAltResult>> {
        if self.r.len() == 0 {
            return Err("Bus is empty".into());
        }
        match self.r.recv() {
            Ok(res) => {
                match from_str(&res) {
                    Ok(val) => {
                        return Ok(val);
                    },
                    Err(_) => {
                        return Err("Error converting from JSON".into());
                    }
                };
            },
            Err(_) => {
                return Err("Error receiving from bus".into());
            }
        }
    }
}


pub fn init(engine: &mut LangEngine) {
    log::trace!("Running STDLIB::system init");

    engine.engine.register_type::<NRBus>()
          .register_fn("Bus", NRBus::init)
          .register_fn("send", NRBus::send)
          .register_fn("recv", NRBus::recv)
          .register_fn("try_recv", NRBus::try_recv)
          .register_fn("to_string", |x: &mut NRBus| format!("Message bus len={}", x.s.len()) );

    engine.engine.register_type::<globals::NRGlobals>()
          .register_fn("Globals", globals::NRGlobals::new)
          .register_fn("get", globals::NRGlobals::get_global)
          .register_indexer_get(globals::NRGlobals::get_global)
          .register_fn("set", globals::NRGlobals::set_global)
          .register_indexer_set(globals::NRGlobals::set_global)
          .register_fn("to_string", |x: &mut globals::NRGlobals| format!("{:?}", x) );

    let mut internal_module = Module::new();
    internal_module.set_id("internal");
    internal_module.set_native_fn("run", run::str_run);
    internal_module.set_native_fn("run", run::txt_run);
    internal_module.set_native_fn("background_run", run::str_spawn);
    let mut default_bus = NRBus::init();
    default_bus.s = engine.s.clone();
    default_bus.r = engine.r.clone();
    internal_module.set_var("bus", default_bus);
    let mut module = exported_module!(system_module);
    module.set_var("globals", globals::NRGlobals::new());
    module.set_native_fn("eventloop", system_loop::system_loop);
    module.set_native_fn("sysctl", sysctl::sysctl_get);
    engine.engine.register_static_module("system", module.into());
    engine.engine.register_static_module("internal", internal_module.into());
    system_metrics::init(&mut engine.engine);
}
