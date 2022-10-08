extern crate log;
extern crate hostname;
use clap::{Args, Parser, Subcommand};
use std::env;
use std::fmt::Debug;

use crate::stdlib::genid;

pub mod setloglevel;
pub mod sanity;
mod tsak_shell;
mod tsak_run;
mod tsak_version;
mod tsak_event;

pub fn init() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    sanity::check_sanity(cli.clone());
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
        Commands::Event(event) => {
            tsak_event::run_event(&cli, event.l, event.every, &event.script, &event.args);
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

    #[clap(long, default_value_t = String::from("insights-collector.newrelic.com"), help="Hostname for Event API")]
    pub nr_event: String,

    #[clap(long, default_value_t = String::from("metric-api.newrelic.com"), help="Hostname for Metric API")]
    pub nr_metric: String,

    #[clap(long, default_value_t = String::from("log-api.newrelic.com"), help="Hostname for Log API")]
    pub nr_log: String,

    #[clap(long, default_value_t = String::from("trace-api.newrelic.com"), help="Hostname for Trace API")]
    pub nr_trace: String,

    #[clap(long, default_value_t = String::from("api.newrelic.com"), help="Hostname for New Relic API")]
    pub nr_api: String,

    #[clap(help="NR account", long, default_value_t = String::from(env::var("NEWRELIC_ACCOUNT").unwrap_or("0".to_string())))]
    pub nr_account: String,

    #[clap(help="NR API key", long, default_value_t = String::from(env::var("NEWRELIC_API").unwrap_or("".to_string())))]
    pub nr_api_key: String,

    #[clap(help="NR Ingestion key", long, default_value_t = String::from(env::var("NEWRELIC_INSERTKEY").unwrap_or("".to_string())))]
    pub nr_insert_key: String,

    #[clap(short, long, default_value_t = String::from(genid::generate_host_id()), help="Instance name")]
    pub name: String,

    #[clap(help="Hostname for TSAK", long, default_value_t = String::from(hostname::get().unwrap().into_string().unwrap()))]
    pub hostname: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Shell(Shell),
    Run(Run),
    Version(Version),
    Event(Event),
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

#[derive(Args, Clone, Debug)]
#[clap(about="Compute and send event")]
struct Event {
    #[clap(short, long, action = clap::ArgAction::Count, help="Run even computation in loop")]
    l:      u8,

    #[clap(short, long, default_value_t=15, help="Number of seconds between event calulations")]
    every:  u32,

    #[clap(help="Path to event computation script", short, long, default_value_t = String::from("-"))]
    pub script: String,

    #[clap(last = true)]
    args: Vec<String>,
}
