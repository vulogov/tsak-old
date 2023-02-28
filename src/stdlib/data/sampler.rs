extern crate log;
use polars::prelude::*;
use polars::datatypes::AnyValue;
use crate::stdlib::data::NRData;
use crate::stdlib::sampler::Sampler;


impl NRData {
    pub fn init_sampler(n: String, mut s: Sampler) -> NRData {
        let mut res = NRData::new();
        let s = Series::new(&n, s.raw());
        match res.df.hstack(&[s]) {
            Ok(ndf) => {
                res.df = ndf;
            }
            Err(err) => {
                log::error!("Error creating Data from Sampler: {}", err);
            }
        }
        return res;
    }
    pub fn set_sampler(&mut self, n: String, mut s: Sampler)  {
        let (_, w) = self.df.shape();
        if w >= 128 {
            log::error!("No more than 128 samples to Data() !");
            return;
        }
        let s = Series::new(&n, s.raw());
        match self.df.hstack(&[s]) {
            Ok(ndf) => {
                self.df = ndf;
            }
            Err(err) => {
                log::error!("Error creating Data from Sampler: {}", err);
            }
        }
    }
    pub fn get_sampler(&mut self, n: String) -> Sampler  {
        match self.df.column(&n) {
            Ok(c) => {
                let mut s = Sampler::init();
                for v in c.iter() {
                    match v {
                        AnyValue::Int64(val) => s.try_set(val as f64),
                        AnyValue::Float64(val) => s.try_set(val),
                        _ => continue,
                    }
                }
                s
            }
            Err(err) => {
                log::error!("Error get Sampler from Data: {}", err);
                Sampler::init()
            }
        }
    }
    pub fn get_row_sampler(&mut self, n: i64) -> Sampler  {
        match self.df.get(n as usize) {
            Some(row) => {
                let mut s = Sampler::init();
                for v in row {
                    match v {
                        AnyValue::Int64(val) => s.try_set(val as f64),
                        AnyValue::Float64(val) => s.try_set(val),
                        _ => continue,
                    }
                }
                s
            }
            None => {
                log::error!("Error get row Sampler from Data");
                Sampler::init()
            }
        }
    }
}
