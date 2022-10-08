use rhai::{Engine, Scope, EvalAltResult, packages::Package};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;
use crate::stdlib;

pub mod scope;

pub type RhaiResult<T> = std::result::Result<T, Box<EvalAltResult>>;

pub struct LangEngine<'a> {
    engine:    Engine,
    scope:     Scope<'a>,
}

impl LangEngine<'_> {
    pub fn new() -> Self {
        Self {
            engine:     Engine::new(),
            scope:      Scope::new(),
        }
    }
    pub fn init() -> LangEngine<'static> {
        let mut e = LangEngine::new();
        e.engine.register_global_module(RandomPackage::new().as_shared_module());
        e.engine.register_global_module(SciPackage::new().as_shared_module());
        stdlib::initlib(&mut e.engine);
        e.set_default_scope();
        e
    }
}

impl LangEngine<'_> {
    pub fn run(&mut self, c: String) ->  Result<(), Box<EvalAltResult>> {
        self.engine.run_with_scope(&mut self.scope, c.as_str())
    }
}
