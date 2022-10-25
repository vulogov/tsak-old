use lazy_static::lazy_static;
extern crate log;
use rhai::{Engine, Dynamic};
use rhai::plugin::*;
// Later add NetworkExt, NetworksExt, ProcessExt,
use sysinfo::{System, SystemExt};
use crate::stdlib::nr::metric::metric_type;

lazy_static! {
    static ref METRIC_SYS: System = {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys
    };
}

#[export_module]
pub mod metrics_module {
    pub mod memory {
        pub fn total_memory() -> i64 {
            METRIC_SYS.total_memory() as i64
        }
        pub fn used_memory() -> i64 {
            METRIC_SYS.used_memory() as i64
        }

        pub mod m {
            pub fn total_memory() -> metric_type::Metric {
                metric_type::Metric::init("total.memory".to_string(), Dynamic::from(METRIC_SYS.total_memory() as i64))
            }
            pub fn used_memory() -> metric_type::Metric {
                metric_type::Metric::init("used.memory".to_string(), Dynamic::from(METRIC_SYS.used_memory() as i64))
            }
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::System functions init");
    let module = exported_module!(metrics_module);
    engine.register_static_module("metrics", module.into());
}
