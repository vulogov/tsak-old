extern crate log;
use rhai::{Engine};
pub mod genid;
pub mod banner;

pub mod uuid;
pub mod env;
pub mod string;
pub mod nr;
pub mod timestamp;


#[macro_export]
macro_rules! err {
	($fmt:literal $($t:tt)*) => (
		Box::new(rhai::EvalAltResult::ErrorRuntime(
			format!($fmt $($t)*).into(),
			rhai::Position::NONE
		))
	)
}

pub fn initlib(engine: &mut Engine) {
    log::trace!("Running STDLIB init");
    uuid::init(engine);
    env::init(engine);
    string::init(engine);
    timestamp::init(engine);
}
