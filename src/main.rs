mod cli;
mod config;
mod http;
mod metrics;
mod worker;
mod engine;

use clap::Parser;
use cli::CliArgs;
use config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let config = Arc::new(Config::from_cli(args));

    println!("Starting load test...\n");

    engine::run_load_test(config).await;
}