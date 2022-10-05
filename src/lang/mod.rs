use rhai::{Engine, EvalAltResult, packages::Package};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;
use crate::stdlib;

pub type RhaiResult<T> = std::result::Result<T, Box<EvalAltResult>>;

pub struct LangEngine {
    engine:    Engine,
}

impl LangEngine {
    pub fn new() -> Self {
        Self {
            engine:     Engine::new(),
        }
    }
    pub fn init() -> LangEngine {
        let mut e = LangEngine::new();
        e.engine.register_global_module(RandomPackage::new().as_shared_module());
        e.engine.register_global_module(SciPackage::new().as_shared_module());
        stdlib::initlib(&mut e.engine);
        e
    }
}

impl LangEngine {
    pub fn run(&mut self, c: String) ->  Result<(), Box<EvalAltResult>> {
        self.engine.run(c.as_str())
    }
}
