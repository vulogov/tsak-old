extern crate log;
use crate::lang::{LangEngine};
use crate::cmd::{Cli};
pub mod genid;
pub mod banner;

pub mod uuid;
pub mod env;
pub mod string;
pub mod nr;
pub mod nrql;
pub mod timestamp;
pub mod system;
pub mod tsak_log;
pub mod input;
pub mod json;
pub mod grok;
pub mod problem;
pub mod nn;
pub mod fake;
pub mod csv;

#[macro_export]
macro_rules! err {
	($fmt:literal $($t:tt)*) => (
		Box::new(rhai::EvalAltResult::ErrorRuntime(
			format!($fmt $($t)*).into(),
			rhai::Position::NONE
		))
	)
}

pub fn initlib(engine: &mut LangEngine, _c: &Cli) {
    log::trace!("Running STDLIB init");
    uuid::init(&mut engine.engine);
    env::init(&mut engine.engine);
    string::init(&mut engine.engine);
    timestamp::init(&mut engine.engine);
    nrql::init(&mut engine.engine);
    nr::init(&mut engine.engine, &mut engine.scope);
    system::init(&mut engine.engine);
    tsak_log::init(&mut engine.engine);
    input::init(&mut engine.engine);
    json::init(&mut engine.engine);
    grok::init(&mut engine.engine);
	problem::init(&mut engine.engine);
	nn::init(&mut engine.engine);
	fake::init(&mut engine.engine);
	csv::init(&mut engine.engine);
}
