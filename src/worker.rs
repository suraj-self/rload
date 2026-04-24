use std::sync::Arc;
use std::time::Instant;

use reqwest::Client;

use crate::config::Config;
use crate::http::build_request;
use crate::metrics::Metrics;

pub async fn run_worker(
    client: Client,
    config: Arc<Config>,
    metrics: Arc<Metrics>,
    requests: u64,
) {
    for _ in 0..requests {
        let start = Instant::now();

        let req = build_request(&client, &config);

        match req.send().await {
            Ok(resp) => {
                let latency = start.elapsed().as_millis();
                if resp.status().is_success() {
                    metrics.record_success(latency);
                } else {
                    metrics.record_failure();
                }
            }
            Err(_) => {
                metrics.record_failure();
            }
        }
    }
}