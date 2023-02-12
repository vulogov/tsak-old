use std;
extern crate log;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use bastion::Bastion;

pub fn tsak_fin() {
    log::trace!("cmd::tsak_fin() reached");
    Bastion::stop();
    std::process::exit(0);
}
