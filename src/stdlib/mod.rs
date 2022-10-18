extern crate log;
use crate::lang::{LangEngine};
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


#[macro_export]
macro_rules! err {
	($fmt:literal $($t:tt)*) => (
		Box::new(rhai::EvalAltResult::ErrorRuntime(
			format!($fmt $($t)*).into(),
			rhai::Position::NONE
		))
	)
}

pub fn initlib(engine: &mut LangEngine) {
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
}
