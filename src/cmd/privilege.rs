extern crate log;
use crate::cmd;
use std;
use sudo;

pub fn check_privilege(c: cmd::Cli) {
    log::trace!("TSAK check_privilege() reached");

    match sudo::check() {
        sudo::RunningAs::Root => {
            log::warn!("You are running TSAK under root privilege");
        }
        sudo::RunningAs::User => {
            if c.privilege > 0 {
                log::error!("Privilege elevation requested, but not granted");
                std::process::exit(10)
            } else {
                log::debug!("You are running TSAK in a user mode");
            }
        }
        sudo::RunningAs::Suid => {
            log::info!("You are running TSAK with SUID bit set");
        }
    }


    if c.nr_account == "0" {
        log::error!("You did not specified New Relic account");
        std::process::exit(10)
    }
    log::debug!("NR Account is {}", c.nr_account);
    if c.nr_api_key.is_empty() {
        log::error!("You did not specified New Relic API key");
        std::process::exit(10)
    }
    log::debug!("NR API key is {}", c.nr_api_key);
    if c.nr_insert_key.is_empty() {
        log::error!("You did not specified New Relic INSERT key");
        std::process::exit(10)
    }
    log::debug!("NR INSERT key is {}", c.nr_insert_key);
}
