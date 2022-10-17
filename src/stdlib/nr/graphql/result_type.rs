extern crate log;
use rhai::{Engine};
use jsonpath_rust::JsonPathFinder;
use polars::prelude::*;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct NRResult {
    df: DataFrame,
}

impl NRResult {
    fn new() -> Self {
        Self {
            df:    DataFrame::default(),
        }
    }
    pub fn init(d: String) -> NRResult {
        let mut res = NRResult::new();
        res.load_data(d);
        res
    }
    fn load_data(&mut self, d: String) {
        let mut jsons = String::new();
        match JsonPathFinder::from_str(&d, "$.data.actor.account.nrql.results[*]") {
            Ok(res)  => {
                for r in res.find_slice() {
                    jsons += &r.to_string();
                    jsons += &"\n".to_string();
                }
                let file = Cursor::new(jsons);
                match JsonReader::new(file)
                    .with_json_format(JsonFormat::JsonLines)
                    .finish() {
                    Ok(df) => self.df = df,
                    Err(err) => log::error!("Error creating DF: {}", err),
                }

            }
            Err(err) => {
                log::error!("Error converting NRQL JSON: {}", err);
            }
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::Result type init");
    engine.register_type::<NRResult>()
          .register_fn("Result", NRResult::init)
          .register_fn("to_string", |x: &mut NRResult| format!("{:?}", x) );
}
