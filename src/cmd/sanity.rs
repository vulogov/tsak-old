extern crate log;
use crate::cmd;
use std;

pub fn check_sanity(c: cmd::Cli) {
    log::trace!("TSAK check_sanity() reached");
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
