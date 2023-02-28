extern crate log;
use rhai::{Engine};
use polars::prelude::*;

mod sampler;

#[derive(Debug, Clone)]
pub struct NRData {
    df: DataFrame,
}

impl NRData {
    fn new() -> Self {
        Self {
            df:    DataFrame::default(),
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::Data init");
    engine.register_type::<NRData>()
          .register_fn("Data", NRData::new)
          .register_fn("Data", NRData::init_sampler)
          .register_indexer_set(NRData::set_sampler)
          .register_indexer_get(NRData::get_sampler)
          .register_indexer_get(NRData::get_row_sampler)
          .register_fn("to_string", |x: &mut NRData| format!("{:?}", x.df) );
}
