// extern crate log;
// use rhai::{Engine, Scope, Map};
// use tokio::task;
// use tokio::time::sleep;
// use std::time::Duration;
// use lazy_static::lazy_static;
// use std::sync::Mutex;
// use std::collections::VecDeque;
// use std::vec::Vec;
// use serde_json::{to_string};
//
// use crate::stdlib::json::json_module::{to_map};
// use crate::stdlib::nr::metric::raw::{send_metric_payload};
//
// struct Metrics {
//     inq:  VecDeque<String>,
//     c:   i64,
//     r:   bool,
// }
//
// impl Metrics {
//     fn new() -> Self {
//         Self {
//             inq: VecDeque::new(),
//             c:   0,
//             r:   false,
//         }
//     }
// }
//
// lazy_static! {
//     static ref METRICS: Mutex<Metrics> = {
//         let e: Mutex<Metrics> = Mutex::new(Metrics::new());
//         e
//     };
// }
// 
// async fn metric_sender(api: String, key: String)  {
//     log::debug!("Metric sender will communicate to: {}", &api);
//     log::debug!("Metric sender will use: {}", &key);
//
//     loop {
//         sleep(Duration::from_millis(500)).await;
//         let mut e = METRICS.lock().unwrap();
//         if e.inq.len() > 0 {
//             e.c = 0;
//             e.r = false;
//             let mut out: Vec<Map> = Vec::new();
//             while e.inq.len() > 0 {
//                 match e.inq.pop_front() {
//                     Some(v) => {
//                         e.c += 1;
//                         let m = to_map(v);
//                         out.push(m);
//                     }
//                     None => break,
//                 }
//                 if e.c > 50 {
//                     break;
//                 }
//             }
//             match to_string(&out) {
//                 Ok(payload) => {
//                     log::debug!("Sending {} metrics, {} bytes to NR", &e.c, &payload.len());
//                     send_metric_payload(&api, &key, &payload);
//                 }
//                 Err(err) => {
//                     log::error!("Error generating payload: {}", err);
//
//                 }
//             }
//             drop(out);
//         }
//         e.r = true;
//         drop(e);
//     }
// }
//
// pub fn queue_json_payload_to_metrics(p: String) -> bool {
//     let mut e = METRICS.lock().unwrap();
//     e.inq.push_back(p);
//     true
// }
//
// pub fn wait_metrics_for_complete()  {
//     log::debug!("Waiting for metrics queue to clear");
//     loop {
//         let _ = async { sleep(Duration::from_millis(500)).await; };
//         let e = METRICS.lock().unwrap();
//         if e.r && e.inq.len() == 0 {
//             log::debug!("Metric queue is empty");
//             drop(e);
//             return;
//         }
//         drop(e);
//     }
// }
//
// pub fn queue_map_payload_to_metrics(p: Map) -> bool {
//     match to_string(&p) {
//         Ok(res) => {
//             let mut e = METRICS.lock().unwrap();
//             e.inq.push_back(res);
//             true
//         },
//         Err(err) => {
//             log::error!("Error converting to JSON: {}", err);
//             false
//         }
//     }
// }
//
// pub fn init(_engine: &mut Engine, scope: &mut Scope) {
//     log::trace!("Running STDLIB::metric::pipe type init");
//     let _ = task::spawn(metric_sender(
//         scope.get_value::<String>("NR_METRIC").unwrap(),
//         scope.get_value::<String>("NR_INSERT_KEY").unwrap()
//     ));
// }
