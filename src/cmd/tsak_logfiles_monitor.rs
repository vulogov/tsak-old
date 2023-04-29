extern crate log;
use crate::cmd;
use crate::stdlib::bus::pipe::{create_pipe, pipe_push_raw};
use logwatcher::{LogWatcher, LogWatcherAction};

pub async fn logfile_monitor_main(_c: cmd::Cli, fname: String) -> () {
    log::debug!("TSAK logfile monitor: {}", &fname);
    let pipe_name = format!("logfile:{}", &fname);
    log::debug!("TSAK logfile monitor will send to pipe: {}", &pipe_name);
    match LogWatcher::register(&fname) {
        Ok(mut watcher) => {
            create_pipe(pipe_name.clone());
            watcher.watch(&mut move |line: String| {
                pipe_push_raw(pipe_name.clone(), line);
                LogWatcherAction::None
            });
        }
        Err(err) => {
            log::error!("TSAK logfile monitor {} error: {}", &fname, err);
        }
    }
}
