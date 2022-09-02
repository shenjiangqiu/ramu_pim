use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod controller;
pub mod ddr4;
pub(crate) mod dram;
pub mod memory;
pub(crate) mod refresh;
pub(crate) mod request;
pub(crate) mod rowpolicy;
pub(crate) mod rowtable;
pub(crate) mod scheduler;
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn init_logger();
        fn rdebug(msg: &str);
        fn rinfo(msg: &str);
        fn rerror(msg: &str);
        fn rdebug_with_target(target: &str, msg: &str);
        fn rinfo_with_target(target: &str, msg: &str);
        fn rerror_with_target(target: &str, msg: &str);

    }
}
fn init_logger() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .try_init()
        .unwrap_or_else(|e| {
            eprintln!("Failed to init tracing: {}", e);
        });
}

fn rdebug(msg: &str) {
    tracing::debug!(msg = msg);
}
fn rdebug_with_target(target: &str, msg: &str) {
    tracing::debug!(target = target, msg);
}
fn rinfo(msg: &str) {
    tracing::info!(msg = msg);
}
fn rinfo_with_target(target: &str, msg: &str) {
    tracing::info!(target = target, msg);
}

fn rerror(msg: &str) {
    tracing::error!(msg = msg);
}
fn rerror_with_target(target: &str, msg: &str) {
    tracing::error!(target = target, msg);
}

#[cfg(test)]
mod test {
    use crate::{
        config::Config,
        controller::Controller,
        ddr4::DDR4,
        dram::{Dram, DramSpec},
        init_logger,
        memory::{MemoryTrait, SimpleMemory},
        request::{ReqType, Request},
        rinfo, rinfo_with_target,
    };

    #[test]
    fn test_log() {
        init_logger();
        rinfo("hello");
        rinfo_with_target("app", "hello");
    }

    #[test]
    fn test_memory() {
        init_logger();
        let config = Config::default();
        let ddr4 = DDR4::new(&config);
        let child_size = ddr4.get_child_size();
        let num_channels = child_size[0];
        let mut controllers = vec![];
        for _i in 0..num_channels {
            let channel = Dram::new(&ddr4, crate::memory::Level::Channel, &child_size);
            let controller = Controller::new(&config, channel);
            controllers.push(controller);
        }
        let mut mem = SimpleMemory::new(&config, controllers, &ddr4);
        let req = Request::new(0, ReqType::Read);
        mem.try_send(req).unwrap();
        mem.tick();
    }
}
