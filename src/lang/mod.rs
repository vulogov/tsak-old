use rhai::{Engine, Scope, EvalAltResult, packages::Package};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;
use howlong::{HighResolutionTimer};
use crate::stdlib;
use crate::cmd::{Cli};

pub mod scope;

pub type RhaiResult<T> = std::result::Result<T, Box<EvalAltResult>>;

pub struct LangEngine<'a> {
    pub engine:    Engine,
    pub scope:     Scope<'a>,
    name:      String,
    pub is_debug:   bool,
    pub timer:      HighResolutionTimer,
}

impl LangEngine<'_> {
    pub fn new() -> Self {
        Self {
            engine:     Engine::new(),
            scope:      Scope::new(),
            timer:      HighResolutionTimer::new(),
            name:       "".to_string(),
            is_debug:   false,
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
        stdlib::initlib(&mut e);
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
}

impl Drop for LangEngine<'_> {
    fn drop(&mut self) {
        log::debug!("{} takes {:?} to execute", self.name, self.timer.elapsed());
        log::debug!("Engine is finished");
    }
}
