use rhai::{Engine, Dynamic, Scope, EvalAltResult, packages::Package};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;
use howlong::{HighResolutionTimer};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
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
    pub s:          UnboundedSender<String>,
    pub r:          UnboundedReceiver<String>,
}

impl LangEngine<'_> {
    pub fn new() -> Self {
        let (s,r) = unbounded_channel::<String>();
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
        e.set_cli_scope(&c);
        stdlib::initlib(&mut e, c);
        e.elapsed("Init finished");
        e
    }
    pub fn init_with_channels(c: &Cli, s: UnboundedSender<String>, r: UnboundedReceiver<String>) -> LangEngine<'static> {
        let mut e = LangEngine::new();
        e.name = c.name.clone();
        if c.debug > 0 {
            log::debug!("Debug is enabled");
            e.is_debug = true;
        }
        e.s = s.clone();
        e.r = r;
        e.engine.register_global_module(RandomPackage::new().as_shared_module());
        e.engine.register_global_module(SciPackage::new().as_shared_module());
        e.set_default_scope();
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
        e.r = self.r;
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
