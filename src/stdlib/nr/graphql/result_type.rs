extern crate log;
use rhai::{Engine, Array};
use jsonpath_rust::JsonPathFinder;
use polars::prelude::*;
use std::io::Cursor;
use std::vec::Vec;

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
                if jsons.is_empty() {
                    log::info!("There is no data to fill Result()");
                    return;
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
    fn select(&mut self, c: Array) -> NRResult {
        let mut i: Vec<String> = Vec::new();
        for k in c {
            match k.into_string() {
                Ok(key) => i.push(key),
                Err(_) => continue,
            }
        }
        let mut res = NRResult::new();
        match self.df.select(i) {
            Ok(df) => res.df = df,
            Err(_) => {}
        }
        return res;
    }
    fn get_field(&mut self, c: String) -> Array {
        let mut res = Array::new();
        match self.df.column(&c) {
            Ok(s) => {
                log::debug!("Got the column: {}", s);
            }
            Err(err) => {
                log::error!("Result selection error: {}", err);
            }
        }
        return res;
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::Result type init");
    engine.register_type::<NRResult>()
          .register_fn("Result", NRResult::init)
          .register_fn("select", NRResult::select)
          .register_indexer_get(NRResult::get_field)
          .register_fn("to_string", |x: &mut NRResult| format!("{:?}", x.df) );
}
