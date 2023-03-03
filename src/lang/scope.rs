extern crate log;
use std::time::{SystemTime, UNIX_EPOCH};
use rhai::{Map, Dynamic};

use crate::lang::{LangEngine};
use crate::cmd::{Cli};

impl LangEngine<'_> {
    pub fn set_default_scope(&mut self) {
        log::debug!("Setting TSAK default scope");
        self.scope.push("TSAK_STARTED", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64);
    }
    pub fn set_cli_scope(&mut self, c: &Cli) {
        log::debug!("Setting TSAK shell scope");
        self.scope.push("NR_EVENT", c.nr_event.clone());
        self.scope.push("NR_METRIC", c.nr_metric.clone());
        self.scope.push("NR_LOG", c.nr_log.clone());
        self.scope.push("NR_TRACE", c.nr_trace.clone());
        self.scope.push("NR_API", c.nr_api.clone());
        self.scope.push("NR_SEC_API", c.nr_sec_api.clone());
        // And keys
        self.scope.push("NR_ACCOUNT", c.nr_account.clone());
        self.scope.push("NR_API_KEY", c.nr_api_key.clone());
        self.scope.push("NR_INSERT_KEY", c.nr_insert_key.clone());
        // And environment
        self.scope.push("HOSTNAME", c.hostname.clone());
        self.scope.push("INSTANCE", c.name.clone());
    }
    pub fn set_channels_to_scope(&mut self) {
        log::debug!("Setting TSAK default channels to scope");
        self.scope.push("SEND_CHANNEL", self.s.clone());
        self.scope.push("RECV_CHANNEL", self.r.clone());
    }
    pub fn set_extra_scope(&mut self, args: &Vec<String>) {
        for arg in args {
            let pair: Vec<_> = arg.splitn(2, "=").collect();
            if pair.len() != 2 {
                continue;
            }
            let key = pair[0];
            let value = pair[1];
            match self.engine.eval_expression_with_scope::<f64>(&mut self.scope, value) {
                Ok(res) => self.scope.push(key, res),
                Err(_)  => self.scope.push(key, format!("{}", &value)),
            };
            log::debug!("Detected extra scope var {}={}", &key, &value);
        }
    }
    pub fn eval_int_with_scope(&mut self, c: &String) -> i64 {
        return self.engine.eval_expression_with_scope::<i64>(&mut self.scope, c).unwrap()
    }
    pub fn eval_map_with_scope(&mut self, c: &String) -> Option<Map> {
        match self.engine.compile_with_scope(&mut self.scope, c) {
            Ok(ast) => {
                match self.engine.eval_ast_with_scope::<Map>(&mut self.scope, &ast) {
                    Ok(ret) => return Some(ret),
                    Err(err)  => {
                        log::error!("Script evaluating error: {}", err);
                        return None;
                    }
                }
            }
            Err(err) => {
                log::error!("Script parsing error: {}", err);
                return None;
            }
        }
    }
    pub fn run_with_scope(&mut self, c: &String) -> bool {
        match self.engine.compile_with_scope(&mut self.scope, c) {
            Ok(ast) => {
                match self.engine.run_ast_with_scope(&mut self.scope, &ast) {
                    Ok(ret) => {
                        log::debug!("Script returned {:?}", ret);
                        return true;
                    }
                    Err(err)  => {
                        log::error!("Script evaluating error: {}", err);
                        return false;
                    }
                }
            }
            Err(err) => {
                log::error!("Script parsing error: {}", err);
                return true;
            }
        }
    }
}

impl LangEngine<'_> {
    pub fn eval_with_scope(&mut self, c: &String) -> Option<Dynamic> {
        match self.engine.compile_with_scope(&mut self.scope, c) {
            Ok(ast) => {
                match self.engine.eval_ast_with_scope::<Dynamic>(&mut self.scope, &ast) {
                    Ok(ret) => return Some(ret),
                    Err(err)  => {
                        log::error!("Script evaluating error: {}", err);
                        return None;
                    }
                }
            }
            Err(err) => {
                log::error!("Script parsing error: {}", err);
                return None;
            }
        }
    }
}
