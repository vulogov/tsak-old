extern crate log;
use lazy_static::lazy_static;
use rhai::{Engine, Map, Array, Dynamic, Identifier};
use rhai::plugin::*;
use std::{thread, time};
// Later add NetworkExt, NetworksExt, ProcessExt,
use sysinfo::{System, SystemExt, CpuExt};
use crate::stdlib::nr::metric::metric_type;
use crate::stdlib::system::system_module::sleep;
use std::sync::Mutex;

lazy_static! {
    static ref METRIC_SYS: Mutex<System> = {
        let sys: Mutex<System> = Mutex::new(System::new_all());
        sys.lock().unwrap().refresh_all();
        sys
    };
}

pub async fn update_sysinfo() {
    log::debug!("Initiated sysinfo update thread");
    loop {
        let mut sys = METRIC_SYS.lock().unwrap();
        sys.refresh_all();
        drop(sys);
        sleep(5 as i64);
    }
}

#[export_module]
pub mod metrics_module {
    pub fn refresh() {
        log::debug!("Refrsheing sysinfo");
        let mut sys = METRIC_SYS.lock().unwrap();
        sys.refresh_all();
        thread::sleep(time::Duration::from_millis(100));
    }
    pub mod memory {
        pub fn total_memory() -> i64 {
            METRIC_SYS.lock().unwrap().total_memory() as i64
        }
        pub fn used_memory() -> i64 {
            METRIC_SYS.lock().unwrap().used_memory() as i64
        }

        pub mod m {
            pub fn total_memory() -> metric_type::Metric {
                metric_type::Metric::init("total.memory".to_string(), Dynamic::from(METRIC_SYS.lock().unwrap().total_memory() as i64))
            }
            pub fn used_memory() -> metric_type::Metric {
                metric_type::Metric::init("used.memory".to_string(), Dynamic::from(METRIC_SYS.lock().unwrap().used_memory() as i64))
            }
        }
    }

    pub mod cpu {
        pub fn cpus() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_cpu();
            for cpu in METRIC_SYS.lock().unwrap().cpus() {
                let key = Identifier::from(cpu.name());
                res.insert(key, Dynamic::from(cpu.cpu_usage() as f64));
            }
            res
        }
        pub fn usage() -> Array {
            let mut res = Array::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_cpu();
            for cpu in METRIC_SYS.lock().unwrap().cpus() {
                res.push(Dynamic::from(cpu.cpu_usage() as f64));
            }
            res
        }

        pub mod m {
            pub fn usage() -> metric_type::Metric {
                thread::sleep(time::Duration::from_millis(100));
                METRIC_SYS.lock().unwrap().refresh_cpu();
                let mut c: f64 = 0.0;
                let mut a: f64 = 0.0;
                for cpu in METRIC_SYS.lock().unwrap().cpus() {
                    a += cpu.cpu_usage() as f64;
                    c += 1.0;
                }
                metric_type::Metric::init("used.cpu".to_string(), Dynamic::from((a / c) as f64))
            }
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::System functions init");
    let module = exported_module!(metrics_module);
    engine.register_static_module("metrics", module.into());
}
