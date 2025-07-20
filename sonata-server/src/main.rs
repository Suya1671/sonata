#![doc = include_str!("../../README.md")]

use server::server_task;
use tokio::join;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

mod server;

#[tokio::main]
#[tracing::instrument]
async fn main() {
    let console_subscriber = tracing_subscriber::fmt::layer().pretty();
    let error_subscriber = tracing_error::ErrorLayer::default();
    let env_subscriber = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    let journald_subscriber = tracing_journald::layer().ok();

    tracing_subscriber::registry()
        .with(console_subscriber.with_filter(env_subscriber))
        .with(error_subscriber)
        .with(journald_subscriber)
        .init();

    // TODO: custom URL
    let server = tokio::spawn(server_task("localhost:8080"));

    let (server_res,) = join!(server);

    server_res
        .expect("server join failed")
        .expect("server task failed");
}
