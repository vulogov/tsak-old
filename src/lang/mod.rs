use rhai::{Engine, Scope, EvalAltResult, packages::Package};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;
use howlong::{HighResolutionTimer};
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::stdlib;
use crate::cmd::{Cli};

use crate::stdlib::nr::event::event_pipe::{wait_events_for_complete};
use crate::stdlib::nr::metric::metric_pipe::{wait_metrics_for_complete};


pub mod scope;

pub type RhaiResult<T> = std::result::Result<T, Box<EvalAltResult>>;


pub struct LangEngine<'a> {
    pub engine:    Engine,
    pub scope:     Scope<'a>,
    name:           String,
    pub is_debug:   bool,
    pub timer:      HighResolutionTimer,
    pub s:          Sender<String>,
    pub r:          Receiver<String>,
}

impl LangEngine<'_> {
    pub fn new() -> Self {
        let (s,r) = unbounded::<String>();
        Self {
            engine:     Engine::new(),
            scope:      Scope::new(),
            timer:      HighResolutionTimer::new(),
            name:       "".to_string(),
            is_debug:   false,
            s: s,
            r: r,
        }
    }
    pub fn init(c: &Cli) -> LangEngine<'static> {
        let mut e = LangEngine::new();
        e.name = c.name.clone();
        if c.debug > 0 {
            log::debug!("Debug is enabled");
            e.is_debug = true;
        }
        e.engine.register_global_module(RandomPackage::new().as_shared_module());
        e.engine.register_global_module(SciPackage::new().as_shared_module());
        e.set_default_scope();
        e.set_channels_to_scope();
        e.set_cli_scope(&c);
        stdlib::initlib(&mut e, c);
        e.elapsed("Init finished");
        e
    }
    pub fn child_init(c: &Cli) -> LangEngine<'static> {
        let mut e = LangEngine::new();
        e.name = c.name.clone();
        if c.debug > 0 {
            log::debug!("Debug is enabled");
            e.is_debug = true;
        }
        e.engine.register_global_module(RandomPackage::new().as_shared_module());
        e.engine.register_global_module(SciPackage::new().as_shared_module());
        e.set_default_scope();
        e.set_channels_to_scope();
        e.set_cli_scope(&c);
        stdlib::child_initlib(&mut e, c);
        e.elapsed("Init finished");
        e
    }
    pub fn init_with_channels(c: &Cli, s: Sender<String>, r: Receiver<String>) -> LangEngine<'static> {
        let mut e = LangEngine::new();
        e.name = c.name.clone();
        if c.debug > 0 {
            log::debug!("Debug is enabled");
            e.is_debug = true;
        }
        e.s = s.clone();
        e.r = r.clone();
        e.engine.register_global_module(RandomPackage::new().as_shared_module());
        e.engine.register_global_module(SciPackage::new().as_shared_module());
        e.set_default_scope();
        e.set_channels_to_scope();
        e.set_cli_scope(&c);
        stdlib::initlib(&mut e, c);
        e.elapsed("Init finished");
        e
    }
    pub fn elapsed(&mut self, m: &str) {
        log::debug!("{} takes: {:?} to execute", m, self.timer.elapsed());
    }
}

impl LangEngine<'_> {
    pub fn run(&mut self, c: String) ->  Result<(), Box<EvalAltResult>> {
        self.engine.run_with_scope(&mut self.scope, c.as_str())
    }
    pub fn derive(&mut self, c: &Cli) -> LangEngine<'static> {
        let mut e = LangEngine::init(c);
        for (k, _, v) in  &self.scope {
            e.scope.set_or_push(k.clone(), v.clone());
        }
        e.s = self.s.clone();
        e.r = self.r.clone();
        e.set_channels_to_scope();
        e
    }
}

impl Drop for LangEngine<'_> {
    fn drop(&mut self) {
        log::debug!("Flushing queues");
        wait_events_for_complete();
        wait_metrics_for_complete();
        log::debug!("{} takes {:?} to execute", self.name, self.timer.elapsed());
        log::debug!("Engine is finished");
    }
}
