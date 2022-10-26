extern crate log;
use std::collections::HashMap;
use rhai::{Engine, Dynamic, Scope, Identifier};
use rhai::plugin::*;
use crate::stdlib::nr::event::event_type;


#[derive(Debug, Clone)]
pub struct Problem<'a> {
    name: String,
    ctx:  Scope<'a>,
    cnd:  HashMap<String, String>,
    res:  HashMap<String, bool>,
    errmsg: String,
}

impl Problem<'_> {
    fn new() -> Self {
        Self {
            name: String::new(),
            ctx:  Scope::new(),
            cnd:  HashMap::new(),
            res:  HashMap::new(),
            errmsg: String::new(),
        }
    }
    fn init(n: String) -> Problem<'static> {
        let mut res = Problem::new();
        res.name = n.clone();
        res
    }
    fn get_field(&mut self, index: String) -> Dynamic {
        let key = Identifier::from(index);
        if self.ctx.contains(&key) {
            return self.ctx.get(&key).unwrap().clone();
        }
        return Dynamic::default();
    }
    fn set_field(&mut self, index: String, value: Dynamic) {
        let key = Identifier::from(index);
        self.ctx.push_dynamic(key, value);
    }
    fn condition(&mut self, index: String, value: String) {
        let _ = self.cnd.insert(index, value);
    }
    fn error(&mut self) -> String {
        self.errmsg.clone()
    }
    fn run(&mut self, e: &Engine) {
        self.res = HashMap::new();
        for (k, v) in &self.cnd {
            log::debug!("PROBLEM: {} = {}", &k, &v);
            match e.compile_with_scope(&mut self.ctx, v) {
                Ok(ast) => {
                    match e.eval_ast_with_scope::<bool>(&mut self.ctx, &ast) {
                        Ok(res) => {
                            log::debug!("CONDITION: {}={}", &k, &res);
                            self.res.insert(k.clone(), res);
                        }
                        Err(err) => {
                            self.errmsg = format!("Condition execution error: {}", err);
                            break;
                        }
                    }
                }
                Err(err) => {
                    self.errmsg = format!("Condition compilation error: {}", err);
                    break;
                }
            }
        }
        if ! &self.errmsg.is_empty() {
            log::error!("{}", &self.errmsg);
        }
    }
    fn event(&mut self) -> event_type::Event {
        let mut res = event_type::Event::init();
        res.set_field("host.name".to_string(), Dynamic::from(self.name.clone()));
        res.set_field("eventType".to_string(), Dynamic::from("TSAKProblems"));
        let mut is_failed: i64 = 0;
        let mut is_true: i64 = 0;
        for (k, v) in &self.res {
            if *v {
                is_true += 1;
            } else {
                is_failed += 1;
            }
            log::debug!("EVENT: {}={}", &k, &v);
            res.set_field(k.to_string(), Dynamic::from(v.clone()));
        }
        res.set_field("FAILED".to_string(), Dynamic::from(is_failed));
        res.set_field("SUCCEDED".to_string(), Dynamic::from(is_true));
        if is_failed > 0 {
            res.set_field("SCORE".to_string(), Dynamic::from((is_failed as f64 /(is_true as f64 + is_failed as f64)) * 100.0 as f64));
        } else {
            res.set_field("SCORE".to_string(), Dynamic::from(100.0 as f64));
        }
        res
    }
}

#[export_module]
pub mod problem_module {
    pub fn run(context: NativeCallContext, mut p: Problem) -> event_type::Event {
        let engine = context.engine();
        p.run(engine);
        p.event()
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::problem init");
    let module = exported_module!(problem_module);
    engine.register_static_module("problem", module.into());

    engine.register_type::<Problem>()
          .register_fn("Problem", Problem::init)
          .register_fn("condition", Problem::condition)
          .register_fn("event", Problem::event)
          .register_fn("error", Problem::error)
          .register_indexer_get(Problem::get_field)
          .register_indexer_set(Problem::set_field)
          .register_fn("to_string", |x: &mut Problem| format!("name={} #conditions={}", x.name, x.cnd.len()) );
}
