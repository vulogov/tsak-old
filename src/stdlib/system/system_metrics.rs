extern crate log;
use lazy_static::lazy_static;
use rhai::{Engine, Map, Array, Dynamic, Identifier};
use rhai::plugin::*;
use std::{thread, time};
// Later add NetworkExt, NetworksExt, ProcessExt,
use sysinfo::{System, SystemExt, UserExt, CpuExt, ComponentExt, DiskExt, ProcessExt, NetworkExt};
use crate::stdlib::nr::metric::metric_type;
use crate::stdlib::system::system_module::sleep;
use crate::stdlib::system::dmesg;

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

    pub mod system {
        pub fn uptime() -> u64 {
            METRIC_SYS.lock().unwrap().uptime() as u64
        }
        pub fn boottime() -> u64 {
            METRIC_SYS.lock().unwrap().uptime() as u64
        }
        pub fn name() -> String {
            METRIC_SYS.lock().unwrap().name().unwrap()
        }
        pub fn kernel_version() -> String {
            METRIC_SYS.lock().unwrap().kernel_version().unwrap()
        }
        pub fn os_version() -> String {
            METRIC_SYS.lock().unwrap().os_version().unwrap()
        }
        pub fn long_os_version() -> String {
            METRIC_SYS.lock().unwrap().long_os_version().unwrap()
        }
        pub fn distribution() -> String {
            METRIC_SYS.lock().unwrap().distribution_id()
        }
        pub fn hostname() -> String {
            METRIC_SYS.lock().unwrap().host_name().unwrap()
        }

        pub fn lavg() -> Dynamic {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_all();

            let lavg = METRIC_SYS.lock().unwrap().load_average();
            res.insert("1".into(), Dynamic::from(lavg.one as f64));
            res.insert("5".into(), Dynamic::from(lavg.five as f64));
            res.insert("15".into(), Dynamic::from(lavg.fifteen as f64));
            Dynamic::from(res)
        }
        pub fn users() -> Dynamic {
            let mut res = Array::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_all();
            for u in METRIC_SYS.lock().unwrap().users() {
                let name = Dynamic::from(u.name().to_string());
                res.push(name);
            }
            Dynamic::from(res)
        }

    }

    pub mod net {
        pub fn recv() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_networks();
            for (i,n) in METRIC_SYS.lock().unwrap().networks() {
                let key = Identifier::from(i);
                res.insert(key, Dynamic::from(n.received() as u64));
            }
            res
        }
        pub fn transmitted() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_networks();
            for (i,n) in METRIC_SYS.lock().unwrap().networks() {
                let key = Identifier::from(i);
                res.insert(key, Dynamic::from(n.transmitted() as u64));
            }
            res
        }
        pub fn total() -> Dynamic {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_networks();
            for (i, n) in METRIC_SYS.lock().unwrap().networks() {
                let key = Identifier::from(i);
                let mut val = Map::new();
                val.insert("received".into(), Dynamic::from(n.total_received() as u64));
                val.insert("transmitted".into(), Dynamic::from(n.total_transmitted() as u64));
                res.insert(key, Dynamic::from(val.clone()));
            }
            Dynamic::from(res)
        }
        pub fn packets() -> Dynamic {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_networks();
            for (i, n) in METRIC_SYS.lock().unwrap().networks() {
                let key = Identifier::from(i);
                let mut val = Map::new();
                val.insert("total_received".into(), Dynamic::from(n.total_packets_received() as u64));
                val.insert("total_transmitted".into(), Dynamic::from(n.total_packets_transmitted() as u64));
                val.insert("received".into(), Dynamic::from(n.packets_received() as u64));
                val.insert("transmitted".into(), Dynamic::from(n.packets_transmitted() as u64));
                res.insert(key, Dynamic::from(val.clone()));
            }
            Dynamic::from(res)
        }
        pub fn errors() -> Dynamic {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_networks();
            for (i, n) in METRIC_SYS.lock().unwrap().networks() {
                let key = Identifier::from(i);
                let mut val = Map::new();
                val.insert("total_received".into(), Dynamic::from(n.total_errors_on_received() as u64));
                val.insert("total_transmitted".into(), Dynamic::from(n.total_errors_on_transmitted() as u64));
                val.insert("received".into(), Dynamic::from(n.errors_on_received() as u64));
                val.insert("transmitted".into(), Dynamic::from(n.errors_on_transmitted() as u64));
                res.insert(key, Dynamic::from(val.clone()));
            }
            Dynamic::from(res)
        }
    }

    pub mod memory {
        pub fn total_memory() -> i64 {
            METRIC_SYS.lock().unwrap().total_memory() as i64
        }
        pub fn used_memory() -> i64 {
            METRIC_SYS.lock().unwrap().used_memory() as i64
        }
        pub fn available_memory() -> i64 {
            METRIC_SYS.lock().unwrap().available_memory() as i64
        }
        pub fn free_memory() -> i64 {
            METRIC_SYS.lock().unwrap().free_memory() as i64
        }
        pub fn total_swap() -> i64 {
            METRIC_SYS.lock().unwrap().total_swap() as i64
        }
        pub fn used_swap() -> i64 {
            METRIC_SYS.lock().unwrap().used_swap() as i64
        }
        pub fn free_swap() -> i64 {
            METRIC_SYS.lock().unwrap().free_swap() as i64
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
        pub fn vendor() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_cpu();
            for cpu in METRIC_SYS.lock().unwrap().cpus() {
                let key = Identifier::from(cpu.name());
                let val = cpu.vendor_id().to_string();
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn brand() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_cpu();
            for cpu in METRIC_SYS.lock().unwrap().cpus() {
                let key = Identifier::from(cpu.name());
                let val = cpu.brand().to_string();
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn frequency() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_cpu();
            for cpu in METRIC_SYS.lock().unwrap().cpus() {
                let key = Identifier::from(cpu.name());
                res.insert(key, Dynamic::from(cpu.frequency() as i64));
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

    pub mod components {
        pub fn temperature() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_components();
            for c in METRIC_SYS.lock().unwrap().components() {
                let key = Identifier::from(c.label());
                res.insert(key, Dynamic::from(c.temperature() as f64));
            }
            res
        }
    }

    pub mod disks {
        pub fn name() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_disks();
            for d in METRIC_SYS.lock().unwrap().disks() {
                let key = Identifier::from(format!("{}", d.mount_point().display()));
                let val = d.name().to_str().unwrap().to_string();
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn filesystem() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_disks();
            for d in METRIC_SYS.lock().unwrap().disks() {
                let key = Identifier::from(format!("{}", d.mount_point().display()));
                let val = std::str::from_utf8(d.file_system()).unwrap().to_string();
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn space() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_disks();
            for d in METRIC_SYS.lock().unwrap().disks() {
                let key = Identifier::from(format!("{}", d.mount_point().display()));
                res.insert(key, Dynamic::from(d.total_space() as u64));
            }
            res
        }
        pub fn available() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_disks();
            for d in METRIC_SYS.lock().unwrap().disks() {
                let key = Identifier::from(format!("{}", d.mount_point().display()));
                res.insert(key, Dynamic::from(d.available_space() as u64));
            }
            res
        }
    }

    pub mod proc {
        pub fn name() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_processes();
            for (_pid, p) in METRIC_SYS.lock().unwrap().processes() {
                let key = Identifier::from(format!("{}", p.pid()));
                let val = p.name().to_string();
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn memory() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_processes();
            for (_pid, p) in METRIC_SYS.lock().unwrap().processes() {
                let key = Identifier::from(format!("{}", p.pid()));
                let val = p.memory() as u64;
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn virtual_memory() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_processes();
            for (_pid, p) in METRIC_SYS.lock().unwrap().processes() {
                let key = Identifier::from(format!("{}", p.pid()));
                let val = p.virtual_memory() as u64;
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn cpu() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_processes();
            for (_pid, p) in METRIC_SYS.lock().unwrap().processes() {
                let key = Identifier::from(format!("{}", p.pid()));
                let val = p.cpu_usage() as f64;
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn started() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_processes();
            for (_pid, p) in METRIC_SYS.lock().unwrap().processes() {
                let key = Identifier::from(format!("{}", p.pid()));
                let val = p.start_time() as u64;
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn running() -> Map {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_processes();
            for (_pid, p) in METRIC_SYS.lock().unwrap().processes() {
                let key = Identifier::from(format!("{}", p.pid()));
                let val = p.run_time() as u64;
                res.insert(key, Dynamic::from(val.clone()));
            }
            res
        }
        pub fn disk() -> Dynamic {
            let mut res = Map::new();
            thread::sleep(time::Duration::from_millis(100));
            METRIC_SYS.lock().unwrap().refresh_processes();
            for (_pid, p) in METRIC_SYS.lock().unwrap().processes() {
                let key = Identifier::from(format!("{}", p.pid()));
                let mut val = Map::new();
                let du = p.disk_usage();
                val.insert("total_written".into(), Dynamic::from(du.total_written_bytes));
                val.insert("written_bytes".into(), Dynamic::from(du.written_bytes));
                val.insert("total_read".into(), Dynamic::from(du.total_read_bytes));
                val.insert("read_bytes".into(), Dynamic::from(du.read_bytes));
                res.insert(key, Dynamic::from(val.clone()));
            }
            Dynamic::from(res)
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::System functions init");
    let mut module = exported_module!(metrics_module);
    let mut dmesg_module = Module::new();
    dmesg_module.set_id("dmesg");
    dmesg_module.set_native_fn("buffer", dmesg::dmesg_buffer);
    dmesg_module.set_native_fn("buffer_dev", dmesg::dmesg_buffer_dev);
    dmesg_module.set_native_fn("buffer_klog", dmesg::dmesg_buffer_klog);
    module.set_sub_module("dmesg", dmesg_module);
    engine.register_static_module("metrics", module.into());
}
