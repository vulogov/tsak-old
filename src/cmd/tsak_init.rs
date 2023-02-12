extern crate log;
use crate::cmd;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use bastion::Bastion;

pub fn tsak_init(_c: cmd::Cli) {
    log::trace!("cmd::tsak_init() reached");
    Bastion::init();
    Bastion::start();
}
