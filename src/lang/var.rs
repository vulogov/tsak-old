extern crate log;
use rhai::{Dynamic};
use crate::lang::{LangEngine};
use crate::stdlib::system::globals::{NRGlobals, get_global, set_global};

impl LangEngine<'_> {
    pub fn create_var_syntax(&mut self) {
        log::debug!("[{}] creating custom var syntax", self.id());
        let _ = self.engine.register_custom_syntax([ "var", "$ident$", "=", "$expr$" ], true, |context, inputs| {
            let var_name = inputs[0].get_string_value().unwrap().to_string();
            let expr = &inputs[1];

            match context.eval_expression_tree(expr) {
                Ok(value) => {
                    let mut g = NRGlobals::new();
                    g.set_global(var_name, value);
                    drop(g);
                    return Ok(Dynamic::UNIT);
                }
                Err(err) => Err(err),
            }
        });
        self.engine.register_fn("glob", |x: String | get_global(x) );
        self.engine.register_fn("glob", |x: String, v: Dynamic | set_global(x, v) );
    }

}
