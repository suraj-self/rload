use std::sync::Arc;
use std::time::Instant;

use reqwest::Client;

use crate::config::Config;
use crate::metrics::Metrics;
use crate::worker::run_worker;

pub async fn run_load_test(config: Arc<Config>) {
    let client = Client::builder()
        .pool_max_idle_per_host(100)
        .build()
        .expect("Failed to build client");

    let metrics = Metrics::new();

    let per_worker = config.total_requests / config.concurrency as u64;

    let start_time = Instant::now();

    let mut handles = Vec::new();

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

    let duration = start_time.elapsed();

    // Load results
    let success = metrics.success.load(std::sync::atomic::Ordering::Relaxed);
    let failure = metrics.failure.load(std::sync::atomic::Ordering::Relaxed);
    let total_latency = metrics.total_latency.load(std::sync::atomic::Ordering::Relaxed);

    let total = success + failure;

    println!("\n--- Results ---");
    println!("Total Requests: {}", total);
    println!("Success: {}", success);
    println!("Failures: {}", failure);

    if success > 0 {
        println!("Avg Latency: {} ms", total_latency / success);
    }

    println!("Total Time: {:.2?}", duration);

    if duration.as_secs_f64() > 0.0 {
        let rps = total as f64 / duration.as_secs_f64();
        println!("Requests/sec: {:.2}", rps);
    }
}