extern crate log;
use voca_rs::*;
use rhai::{Engine, Dynamic, Array};
use rhai::plugin::*;
use regex::Regex;
use crate::stdlib::grok;

#[derive(Debug, Clone)]
struct Text {
    t: String,
}

impl Text {
    fn new() -> Self {
        Self {
            t: String::new(),
        }
    }
    fn init() -> Text {
        Text::new()
    }
    fn init_str(t: String) -> Text {
        let mut res = Text::new();
        res.t = t.clone();
        res
    }
    fn raw(&mut self) -> String {
        self.t.clone()
    }
    fn lines(&mut self) -> Array {
        let mut res = Array::new();
        for l in self.t.lines() {
            let line = manipulate::trim(&manipulate::expand_tabs(&l.to_string(), 1), "");
            if line.is_empty() {
                continue;
            }
            res.push(Dynamic::from(line));
        }
        res
    }
    fn lines_grok(&mut self, mut g: grok::NRGrok, p: String) -> Array {
        let mut res = Array::new();

        for l in self.t.lines() {
            let line = manipulate::trim(&manipulate::expand_tabs(&l.to_string(), 1), "");
            res.push(Dynamic::from(g.do_match(line, p.clone())));
        }

        res
    }
    fn lines_match(&mut self, r: String) -> Array {
        let mut res = Array::new();
        match Regex::new(&r) {
            Ok(re) => {
                for l in self.t.lines() {
                    let line = manipulate::trim(&manipulate::expand_tabs(&l.to_string(), 1), "");
                    if re.is_match(&line) {
                        res.push(Dynamic::from(line));
                    }
                }
            }
            Err(err) => {
                log::error!("Regex creation failed: {}", err);
            }
        }
        res
    }
}

#[export_module]
pub mod string_module {
    pub fn trim(s: &str) -> String {
    	s.trim().into()
    }

    pub fn lowercase(s: &str) -> String {
    	s.to_lowercase()
    }

    pub fn uppercase(s: &str) -> String {
    	s.to_uppercase()
    }

    pub fn starts_with(a: &str, b: &str) -> bool {
    	a.starts_with(b)
    }

    pub fn ends_with(a: &str, b: &str) -> bool {
    	a.ends_with(b)
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::str init");
    let module = exported_module!(string_module);
    engine.register_static_module("str", module.into());

    engine.register_type::<Text>()
          .register_fn("Text", Text::init)
          .register_fn("Text", Text::init_str)
          .register_fn("raw", Text::raw)
          .register_fn("lines", Text::lines)
          .register_fn("lines", Text::lines_match)
          .register_fn("lines", Text::lines_grok)
          .register_fn("to_string", |x: &mut Text| format!("{}", x.t) );

}
