extern crate log;
extern crate hostname;
use clap::{Args, Parser, Subcommand};
use std::env;
use std::fmt::Debug;

use crate::stdlib::genid;

pub mod setloglevel;
mod tsak_shell;
mod tsak_run;
mod tsak_version;

pub fn init() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    match &cli.command {
        Commands::Shell(shell) => {
            log::debug!("Interactive shell requested");
            tsak_shell::run_shell(&cli, &shell.args);
        }
        Commands::Run(run) => {
            log::debug!("Scripts execution requested");
            tsak_run::run_run(&cli, &run.args);
        }
        Commands::Version(_version) => {
            tsak_version::run_version(&cli);
        }
    }
}

#[derive(Parser, Clone)]
#[clap(name = "tsak")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "The TSAK tool", long_about = None)]
pub struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count, help="Increase verbosity")]
    debug: u8,

    #[clap(short, long, default_value_t = String::from(genid::generate_host_id()), help="Instance name")]
    name: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Shell(Shell),
    Run(Run),
    Version(Version),
}

#[derive(Args, Clone, Debug)]
#[clap(about="Run Interactive shell")]
struct Shell {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Run scripts")]
struct Run {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Display version details")]
struct Version {

}
