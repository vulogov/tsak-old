extern crate log;

use crate::lang::{LangEngine};
use crate::cmd::{Cli, Mode};

impl LangEngine<'_> {
    pub fn set_vm_size(&mut self, c: &Cli) {
        match c.vm {
            Mode::Small => {
                log::debug!("Size of TSAK engine: small: [{}]", self.id());
                self.engine.set_max_string_size(1024);
                self.engine.set_max_map_size(64);
                self.engine.set_max_operations(10000);
                self.engine.set_max_call_levels(10);
            }
            Mode::Medium => {
                log::debug!("Size of TSAK engine: medium: [{}]", self.id());
                self.engine.set_max_string_size(10240);
                self.engine.set_max_map_size(512);
                self.engine.set_max_operations(100000);
                self.engine.set_max_call_levels(100);
            }
            Mode::Large => {
                log::debug!("Size of TSAK engine: large: [{}]", self.id());
                self.engine.set_max_string_size(1024000);
                self.engine.set_max_map_size(5000);
                self.engine.set_max_operations(1000000);
                self.engine.set_max_call_levels(1000);
            }
            Mode::Huge => {
                log::debug!("Size of TSAK engine: huge: [{}]", self.id());
                self.engine.set_max_string_size(0);
                self.engine.set_max_map_size(0);
                self.engine.set_max_operations(0);
                self.engine.set_max_call_levels(10000);
            }
        }
    }

}
