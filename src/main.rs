pub mod cmd;
pub mod stdlib;
pub mod lang;
pub mod tsak_lib;
#[cfg(feature = "tokio-runtime")]
use bastion::prelude::*;
#[cfg(feature = "tokio-runtime")]
use tokio;

#[tokio::main]
async fn main()  {
    cmd::init();
}
