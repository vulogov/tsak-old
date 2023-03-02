extern crate log;
use howlong;
use crate::cmd;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
use bastion::{Bastion, spawn};

use crate::stdlib::linguistic::languages_preload;
use crate::stdlib::bus::queue::queue_init;
use crate::stdlib::bus::pipe::pipes_init;
use crate::stdlib::system::system_metrics::update_sysinfo;


pub fn tsak_init(c: cmd::Cli) {
    log::trace!("cmd::tsak_init() reached");
    if c.lang_preload > 0 {
        let t = howlong::HighResolutionTimer::new();
        log::info!("Requesting languages pre-load for linguistic::* functions");
        languages_preload();
        log::debug!("{:?} takes to run script", t.elapsed());
    }
    queue_init();
    pipes_init();
    Bastion::init();
    Bastion::start();
    log::debug!("Launching TSAK default background threads");
    let _ = spawn! {
        update_sysinfo().await;
    };
}
