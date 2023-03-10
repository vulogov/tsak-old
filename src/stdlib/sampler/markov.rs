extern crate log;
use crate::stdlib::sampler::Sampler;
use rhai::{Dynamic};
use decorum::{R64};
use markovr::MarkovChain;

impl Sampler {
    pub fn markov(&mut self) -> Dynamic {
        let source = self.raw();
        let mut dst: Vec<R64> = Vec::new();
        for v in source {
            dst.push(v.into());
        }
        let palanteer = MarkovChain::<R64>::new(1, &[]);
        let v = palanteer.generate(&dst);
        println!("{:?}", &v);
        Dynamic::default()
    }
}
