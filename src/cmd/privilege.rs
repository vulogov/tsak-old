extern crate log;
use crate::cmd;
use std;
use privdrop;
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

    if c.drop_privileges > 0 {
        log::warn!("Drop privileges has been requested");
        match privdrop::PrivDrop::default().chroot(c.drop_jail).user(c.drop_user).apply() {
            Ok(res) => {
                log::info!("Privilege drop been succesful: {:?}", res);
            }
            Err(err) => {
                log::error!("Privilege drop had failed: {}", err);
                std::process::exit(10)
            }
        }
    }
}
