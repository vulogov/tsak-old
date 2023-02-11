extern crate log;
use howlong;
use std::{thread, time, env};
use rhai::{Dynamic, Module};
use rhai::plugin::*;
use tokio::sync::mpsc::{unbounded_channel,UnboundedSender,UnboundedReceiver};

use crate::lang::{LangEngine};
pub mod system_metrics;

#[derive(Debug)]
pub struct NRChannels {
    n:       i64,
    pub s:   UnboundedSender<String>,
    pub r:   UnboundedReceiver<String>,
}

impl NRChannels {
    fn new() -> Self {
        let (s,r) = unbounded_channel::<String>();
        Self {
            n: 0,
            s: s,
            r: r,
        }
    }
    fn init() -> NRChannels {
        NRChannels::new()
    }
    fn send(&mut self, v: String) -> Dynamic {
        let _ = self.s.send(v);
        self.n += 1;
        Dynamic::from(self.n as i64)
    }
    fn recv(&mut self) -> String {
        if self.n == 0 {
            return "".to_string();
        }
        match self.r.recv().await {
            Ok(val) => {
                self.n -= 1;
                return val;
            }
            Err(err) => {
                log::error!("Error receiving from channel: {}", err);
                "".to_string()
            }
        }
    }
}

#[export_module]
pub mod system_module {
    pub fn sleep(s: i64) {
        let t = howlong::HighResolutionTimer::new();
        thread::sleep(time::Duration::from_secs(s as u64));
        log::debug!("slept for {:?}", t.elapsed());
    }
    pub fn sleep_millisecond(s: i64) {
        let t = howlong::HighResolutionTimer::new();
        thread::sleep(time::Duration::from_millis(s as u64));
        log::debug!("slept for {:?}", t.elapsed());
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
}


pub fn init(engine: &mut LangEngine) {
    log::trace!("Running STDLIB::system init");
    engine.engine.register_type::<NRChannels>()
          .register_fn("Channels", NRChannels::init)
          .register_fn("send", NRChannels::send)
          .register_fn("recv", NRChannels::recv)
          .register_fn("to_string", |x: &mut NRChannels| format!("{:?} {:?}", x.s, x.r) );

    let mut internal_module = Module::new();
    internal_module.set_id("internal");
    let mut s_chan = NRChannels::init();
    s_chan.s = engine.s.clone();
    s_chan.r = engine.r;
    internal_module.set_var("bus", s_chan);
    let module = exported_module!(system_module);
    engine.engine.register_static_module("system", module.into());
    engine.engine.register_static_module("internal", internal_module.into());
    system_metrics::init(&mut engine.engine);
}
