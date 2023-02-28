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
mod tsak_metric;
mod tsak_log;
mod tsak_exec;
mod tsak_eval;
mod tsak_spawn;
mod tsak_init;
mod tsak_fin;


pub fn init() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    sanity::check_sanity(cli.clone());
    tsak_init::tsak_init(cli.clone());
    match &cli.command {
        Commands::Shell(shell) => {
            log::debug!("Interactive shell requested");
            tsak_shell::run_shell(&cli, &shell.args);
        }
        Commands::Run(run) => {
            log::debug!("Scripts execution requested");
            tsak_run::run_run(&cli, &run.expression, &run.args);
        }
        Commands::Version(_) => {
            tsak_version::run_version(&cli);
        }
        Commands::Event(event) => {
            tsak_event::run_event(&cli, event.l, event.every, &event.script, &event.args);
        }
        Commands::Metric(metric) => {
            tsak_metric::run_metric(&cli, metric.l, metric.every, &metric.script, &metric.args);
        }
        Commands::Log(nlog) => {
            tsak_log::run_log(&cli, nlog.l, nlog.every, &nlog.script, &nlog.args);
        }
        Commands::Exec(exec) => {
            tsak_exec::run_exec(&cli, &exec.script, &exec.args);
        }
        Commands::Eval(eval) => {
            tsak_eval::run_eval(&cli, &eval.args);
        }
        Commands::Spawn(spwn) => {
            tsak_spawn::run_spawn(&cli, &spwn.script, &spwn.args);
        }
    }
    tsak_fin::tsak_fin();
}

#[derive(Parser, Clone)]
#[clap(name = "tsak")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "The TSAK tool", long_about = None)]
pub struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count, help="Increase verbosity")]
    pub debug: u8,

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

    #[clap(long, default_value_t = String::from("security-api.newrelic.com"), help="Hostname for New Relic Security API")]
    pub nr_sec_api: String,

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

    #[clap(long, action = clap::ArgAction::Count, help="Pre-load languages for linguistic::* functions")]
    pub lang_preload: u8,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Shell(Shell),
    Run(Run),
    Version(Version),
    Event(Event),
    Metric(Metric),
    Log(Log),
    Exec(Exec),
    Eval(Eval),
    Spawn(Spawn),
}

#[derive(Args, Clone, Debug)]
#[clap(about="Run Interactive shell")]
struct Shell {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Evaluate TSAK-script expression")]
struct Eval {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Run scripts")]
struct Run {
    #[clap(help="Expression to evaluate", short, long, default_value_t = String::from("\"Hello world!\""))]
    pub expression: String,

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
    #[clap(short, long, action = clap::ArgAction::Count, help="Run event computation in loop")]
    l:      u8,

    #[clap(short, long, default_value_t=15, help="Number of seconds between event calulations")]
    every:  u32,

    #[clap(help="Path to event computation script", short, long, default_value_t = String::from("-"))]
    pub script: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Compute and send metric")]
struct Metric {
    #[clap(short, long, action = clap::ArgAction::Count, help="Run metric computation in loop")]
    l:      u8,

    #[clap(short, long, default_value_t=15, help="Number of seconds between metric calulations")]
    every:  u32,

    #[clap(help="Path to metric computation script", short, long, default_value_t = String::from("-"))]
    pub script: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Compute and send log")]
struct Log {
    #[clap(short, long, action = clap::ArgAction::Count, help="Run log computation in loop")]
    l:      u8,

    #[clap(short, long, default_value_t=15, help="Number of seconds between log calulations")]
    every:  u32,

    #[clap(help="Path to log computation script", short, long, default_value_t = String::from("-"))]
    pub script: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Execute TSAK script")]
struct Exec {
    #[clap(help="Path to TSAK script", short, long, default_value_t = String::from("-"))]
    pub script: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Execute TSAK script and spawn some scripts as background threads")]
struct Spawn {
    #[clap(help="Path to TSAK script", short, long, default_value_t = String::from("-"))]
    pub script: String,

    #[clap(last = true)]
    args: Vec<String>,
}
