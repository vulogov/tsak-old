use std;
extern crate log;


pub fn tsak_fin() {
    log::trace!("cmd::tsak_fin() reached");
    std::process::exit(0);
}
