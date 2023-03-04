extern crate log;
use howlong;
use crate::cmd;

use crate::stdlib::linguistic::languages_preload;
use crate::stdlib::bus::queue::queue_init;
use crate::stdlib::bus::pipe::pipes_init;
use crate::stdlib::system::system_metrics::update_sysinfo;
use crate::cmd::tsak_queue_processors;
use crate::cmd::tsak_bus_update_processors;


pub fn tsak_init(c: cmd::Cli) {
    log::debug!("cmd::tsak_init() reached");
    if c.lang_preload > 0 {
        let t = howlong::HighResolutionTimer::new();
        log::info!("Requesting languages pre-load for linguistic::* functions");
        languages_preload();
        log::debug!("{:?} takes to run script", t.elapsed());
    }
    queue_init();
    pipes_init();

    log::debug!("Launching TSAK default background threads");
    log::debug!("cmd::tsak_init(): update sysinfo thread");
    tokio::spawn(async move {
        update_sysinfo().await;
    });
    log::debug!("cmd::tsak_init(): event_processor_main thread");
    let spawn_c = c.clone();
    tokio::spawn(async move {
        tsak_queue_processors::event_processor_main(spawn_c).await;
    });
    log::debug!("cmd::tsak_init(): metric_processor_main thread");
    let spawn_c = c.clone();
    tokio::spawn(async move {
        tsak_queue_processors::metric_processor_main(spawn_c).await;
    });
    log::debug!("cmd::tsak_init(): log_processor_main thread");
    let spawn_c = c.clone();
    tokio::spawn(async move {
        tsak_queue_processors::log_processor_main(spawn_c).await;
    });
    log::debug!("cmd::tsak_init(): vulnerability_processor_main thread");
    let spawn_c = c.clone();
    tokio::spawn(async move {
        tsak_queue_processors::vulnerability_processor_main(spawn_c).await;
    });
    log::debug!("cmd::tsak_init(): zabbix_out_processor_main thread");
    let spawn_c = c.clone();
    tokio::spawn(async move {
        tsak_queue_processors::zabbix_out_processor_main(spawn_c).await;
    });
    log::debug!("cmd::tsak_init(): bus_update_server_processor_main thread");
    let spawn_c = c.clone();
    tokio::spawn(async move {
        tsak_bus_update_processors::bus_update_server_processor_main(spawn_c).await;
    });
    log::debug!("cmd::tsak_init(): bus_update_client_processor_main thread");
    let spawn_c = c.clone();
    tokio::spawn(async move {
        tsak_bus_update_processors::bus_update_client_processor_main(spawn_c).await;
    });
    log::debug!("cmd::tsak_init() done");
}
