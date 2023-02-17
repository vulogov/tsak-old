extern crate log;
use rhai::{Dynamic, Array, EvalAltResult};
use rhai::plugin::*;

use lexical_core;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Sampler {
    d: VecDeque<f64>,
}

impl Sampler {
    fn new() -> Self {
        Self {
            d: VecDeque::with_capacity(128),
        }
    }
    fn init() -> Sampler {
        let mut res = Sampler::new();
        res.zero();
        res
    }
    fn zero(self: &mut Sampler) {
        for _ in 1..129 {
            self.try_set(0.0 as f64);
        }
    }
    fn try_set(self: &mut Sampler, v: f64) {
        if self.d.len() == self.d.capacity() {
            let _ = self.d.pop_front();
        }
        let _ = self.d.push_back(v);
    }
    fn set(self: &mut Sampler, v: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        if v.is_float() {
            self.try_set(v.clone_cast::<f64>());
            return Result::Ok(Dynamic::from(self.d.len() as i64));
        }
        if v.is_int() {
            self.try_set(v.clone_cast::<i64>() as f64);
            return Result::Ok(Dynamic::from(self.d.len() as i64));
        }
        if v.is_string() {
            match lexical_core::parse::<f64>(v.clone_cast::<String>().as_bytes()) {
                Ok(res) => self.try_set(res),
                _ => {
                    return Err("Error parsing string value for Sampler".into());
                }
            }
            return Result::Ok(Dynamic::from(self.d.len() as i64));
        }
        Err("Value for the Sampler must be numeric".into())
    }
    fn get(self: &mut Sampler) -> Dynamic {
        let mut res = Array::new();
        for v in &self.d {
            res.push(Dynamic::from(v.clone()));
        }
        Dynamic::from(res)
    }
    fn try_downsample(self: &mut Sampler) -> VecDeque<f64> {
        let mut res: VecDeque<f64> = VecDeque::new();
        for i in (0..127).step_by(8) {
            let mut c: f64 = 0.0;
            for j in 0..8 {
                match self.d.get((i+j) as usize) {
                    Some(val) => c += val,
                    None => continue,
                }
            }
            let c = c / 8.0;
            res.push_back(c);
        }
        res
    }
    fn downsample(self: &mut Sampler) -> Dynamic {
        let ds_res = self.try_downsample();
        let mut res = Array::new();
        for v in &ds_res {
            res.push(Dynamic::from(v.clone()));
        }
        Dynamic::from(res)
    }
}

#[export_module]
pub mod sampler_module {

}





pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::sampler init");

    engine.register_type::<Sampler>()
          .register_fn("Sampler", Sampler::init)
          .register_fn("set", Sampler::set)
          .register_fn("get", Sampler::get)
          .register_fn("downsample", Sampler::downsample)
          .register_fn("to_string", |x: &mut Sampler| format!("{:?}", x.d) );

    let module = exported_module!(sampler_module);

    engine.register_static_module("sampler", module.into());


}
