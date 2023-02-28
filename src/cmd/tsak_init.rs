extern crate log;
use howlong;
use crate::cmd;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use bastion::Bastion;

use crate::stdlib::linguistic::languages_preload;

pub fn tsak_init(c: cmd::Cli) {
    log::trace!("cmd::tsak_init() reached");
    if c.lang_preload > 0 {
        let t = howlong::HighResolutionTimer::new();
        log::info!("Requesting languages pre-load for linguistic::* functions");
        languages_preload();
        log::debug!("{:?} takes to run script", t.elapsed());
    }
    Bastion::init();
    Bastion::start();
}
