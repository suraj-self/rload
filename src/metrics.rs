use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct Metrics {
    pub success: AtomicU64,
    pub failure: AtomicU64,
    pub total_latency: AtomicU64,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            success: AtomicU64::new(0),
            failure: AtomicU64::new(0),
            total_latency: AtomicU64::new(0),
        })
    }

    pub fn record_success(&self, latency: u128) {
        self.success.fetch_add(1, Ordering::Relaxed);
        self.total_latency
            .fetch_add(latency as u64, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failure.fetch_add(1, Ordering::Relaxed);
    }
}