extern crate log;
use crate::stdlib::string::Text;
use crate::stdlib::string::tokens::{split_str};
use rhai::{Array, Dynamic};

impl Text {
    pub fn tokenize(&mut self) -> Dynamic {
        let mut res = Array::new();
        let r = split_str(self.raw().as_str());
        for token in r {
            res.push(Dynamic::from(token.clone()));
        }
        return Dynamic::from(res);
    }
}
