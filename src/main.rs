
use std;

pub mod cmd;
pub mod stdlib;
pub mod lang;
pub mod tsak_lib;

fn main() {
    cmd::init();
    std::process::exit(0);
}
