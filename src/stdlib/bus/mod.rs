extern crate log;
use rhai::{Engine, Module};
use crossbeam_deque::{Worker};
use crossbeam_channel::{Sender, Receiver};
use std::collections::btree_map::BTreeMap;

use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod queue;
pub mod pipe;
pub mod bus_update;

lazy_static! {
    static ref QUEUES: Mutex<BTreeMap<String,Worker<String>>> = {
        let e: Mutex<BTreeMap<String,Worker<String>>> = Mutex::new(BTreeMap::new());
        e
    };
}

lazy_static! {
    static ref PIPES: Mutex<BTreeMap<String,(Sender<String>, Receiver<String>)>> = {
        let e: Mutex<BTreeMap<String,(Sender<String>, Receiver<String>)>> = Mutex::new(BTreeMap::new());
        e
    };
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::BUS init");
    let mut module = Module::new();
    let mut internal_module = Module::new();
    let mut internal_queue_module = Module::new();
    let mut internal_pipe_module = Module::new();
    let mut cluster_module = Module::new();
    module.set_id("bus");
    internal_module.set_id("internal");
    internal_queue_module.set_id("queue");
    internal_pipe_module.set_id("pipe");
    cluster_module.set_id("cluster");
    // Configuring queue module
    internal_queue_module.set_native_fn("push", queue::queue_push);
    internal_queue_module.set_native_fn("pull", queue::queue_pull);
    internal_queue_module.set_native_fn("is_empty", queue::queue_is_empty);
    internal_module.set_sub_module("queue", internal_queue_module);
    // COnfiguring pipe module
    internal_pipe_module.set_native_fn("push", pipe::pipe_push);
    internal_pipe_module.set_native_fn("pull", pipe::pipe_pull);
    internal_pipe_module.set_native_fn("is_empty", pipe::pipe_is_empty);
    internal_module.set_sub_module("pipe", internal_pipe_module);
    // Configuring cluster module
    cluster_module.set_native_fn("push", bus_update::update_bus_push);

    module.set_sub_module("internal", internal_module);
    module.set_sub_module("cluster", cluster_module);


    engine.register_static_module("bus", module.into());
}
