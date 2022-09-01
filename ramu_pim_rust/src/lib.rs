use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

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
                .with_default_directive(LevelFilter::INFO.into())
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
    use crate::{init_logger, rinfo, rinfo_with_target};

    #[test]
    fn test_log() {
        init_logger();
        rinfo("hello");
        rinfo_with_target("app", "hello");
    }
}
