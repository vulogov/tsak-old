extern crate log;
use crate::stdlib::sampler::Sampler;
use rhai::{EvalAltResult};

use compute::statistics;

impl Sampler {
    pub fn smooth(&mut self) -> Result<Sampler, Box<EvalAltResult>> {
        let mut res = Sampler::init();
        let mut y: Vec<f64> = Vec::new();
        for i in 0..128 {
            y.push(*self.d.get(i).unwrap());
        }
        let std_y  = statistics::mean(&y);
        if std_y == 0.0 {
            return Err("Can not smooth this sample".into());
        }
        let mean_y = statistics::mean(&y);
        for v in y {
            let val = ((v-mean_y)/std_y) as f64;
            if val == -1.0 {
                res.try_set(0.0 as f64);
            } else {
                res.try_set(val);
            }
        }
        Result::Ok(res)
    }
}
