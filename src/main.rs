mod cli;
mod config;
mod http;
mod metrics;
mod worker;

use clap::Parser;
use cli::CliArgs;
use config::Config;
use http::build_client;
use metrics::Metrics;
use worker::run_worker;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let config = Arc::new(Config::from_cli(args));

    let client = build_client();
    let metrics = Metrics::new();

    let per_worker = config.total_requests / config.concurrency as u64;

    let mut handles = Vec::new();

    println!("Starting load test...\n");

    for _ in 0..config.concurrency {
        let client = client.clone();
        let config = config.clone();
        let metrics = metrics.clone();

        let handle = tokio::spawn(async move {
            run_worker(client, config, metrics, per_worker).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let success = metrics.success.load(std::sync::atomic::Ordering::Relaxed);
    let failure = metrics.failure.load(std::sync::atomic::Ordering::Relaxed);
    let total_latency = metrics.total_latency.load(std::sync::atomic::Ordering::Relaxed);

    let total = success + failure;

    println!("--- Results ---");
    println!("Total Requests: {}", total);
    println!("Success: {}", success);
    println!("Failures: {}", failure);

    if success > 0 {
        println!("Avg Latency: {} ms", total_latency / success);
    }
}