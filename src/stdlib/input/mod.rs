extern crate log;
use rhai::{Engine, Map};
use rhai::plugin::*;
use fsio::{file};
use crate::tsak_lib::io::{get_file};
use crate::cmd;

pub mod command;
pub mod watch;
pub mod snmp;
pub mod prometheus;
pub mod zabbix;
pub mod ssh;
pub mod textfile;
pub mod binfile;
pub mod distributions;
pub mod spawn;
pub mod docker;
pub mod socket;


#[export_module]
pub mod input_module {
    pub fn stdin() -> String {
        get_file::get_file("-".to_string())
    }
    pub fn url(u: &str) -> String {
        get_file::get_file(u.to_string())
    }
    pub fn file(u: &str) -> String {
        match file::read_text_file(u) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error reading {}", err);
                return "".to_string();
            }
        }
    }
    pub fn snmp(addr: String, oid: String, community: String) -> Dynamic {
        snmp::snmp_get(&addr, &oid, &community)
    }
    pub fn prometheus(addr: String) -> Map {
        prometheus::prometheus_get(&addr)
    }
    pub fn zabbix(addr: String, key: String) -> String {
        zabbix::zabbix_get(addr, key)
    }
}

pub fn init(engine: &mut Engine, c: cmd::Cli) {
    log::trace!("Running STDLIB::input init");

    let mut module = exported_module!(input_module);
    module.set_native_fn("watch", watch::file_watch);
    module.set_native_fn("docker", docker::docker_stat);
    module.set_native_fn("socket", socket::get_from_socket);
    if c.sandbox == 0 {
        module.set_native_fn("expect", spawn::expect_input);
        module.set_native_fn("command", command::os_command);
        module.set_native_fn("ssh", ssh::ssh_command);
    } else {
        log::warn!("TSAK is in sandbox mode. input::expect() will be disabled");
        module.set_native_fn("expect", spawn::disabled_expect_input);
        log::warn!("TSAK is in sandbox mode. input::command() will be disabled");
        module.set_native_fn("command", command::disabled_os_command);
        log::warn!("TSAK is in sandbox mode. input::ssh() will be disabled");
        module.set_native_fn("ssh", ssh::disabled_ssh_command);
    }

    let mut textfile_module = Module::new();
    textfile_module.set_native_fn("forward", textfile::textfile_forward);
    textfile_module.set_native_fn("backward", textfile::textfile_backward);
    module.set_sub_module("textfile", textfile_module);

    let mut binfile_module = Module::new();
    binfile_module.set_native_fn("read", binfile::binfile_read);
    binfile_module.set_native_fn("zip", binfile::binfile_forward);
    module.set_sub_module("binfile", binfile_module);

    let mut dist_module = Module::new();
    dist_module.set_native_fn("normal", distributions::norm_distribution_gen);
    dist_module.set_native_fn("uniform", distributions::uniform_distribution_gen);
    dist_module.set_native_fn("binomial", distributions::binomial_distribution_gen);
    dist_module.set_native_fn("exp", distributions::exp_distribution_gen);
    dist_module.set_native_fn("lognormal", distributions::lognormal_distribution_gen);
    dist_module.set_native_fn("sawtooth", distributions::sawtooth_gen);
    dist_module.set_native_fn("periodic", distributions::periodic_gen);
    dist_module.set_native_fn("sinusoidal", distributions::sinusoidal_gen);
    dist_module.set_native_fn("square", distributions::square_gen);
    dist_module.set_native_fn("triangle", distributions::triangle_gen);
    module.set_sub_module("distribution", dist_module);

    engine.register_static_module("input", module.into());
}
