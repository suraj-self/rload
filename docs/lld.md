# 📌 Low-Level Design (LLD)

### Modules

```
src/
 ├── main.rs
 ├── cli.rs
 ├── config.rs
 ├── engine.rs
 ├── worker.rs
 ├── http.rs
 ├── metrics.rs
 └── report.rs
```

---

### CLI Module (cli.rs)

Responsibilities:
- Parse arguments using `clap`

Example:
```
rload \
  --url https://api.com/login \
  --method POST \
  --concurrency 100 \
  --requests 1000000 \
  --header "Authorization: Bearer xxx" \
  --body '{"username":"a","password":"b"}'
```

---

### Config Module (config.rs)

Struct:

```rust
pub struct Config {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub concurrency: usize,
    pub total_requests: u64,
}
```

---

### Engine (engine.rs)

Responsibilities:
- Initialize runtime
- Spawn workers
- Distribute workload

Pseudo:

```rust
let requests_per_worker = total_requests / concurrency;

for _ in 0..concurrency {
    spawn(worker(requests_per_worker));
}
```

---

### Worker (worker.rs)

Responsibilities:
- Send requests in loop
- Record latency

Pseudo:

```rust
for _ in 0..requests_per_worker {
    let start = Instant::now();

    let res = client.execute(request).await;

    let latency = start.elapsed();

    metrics.record(res, latency);
}
```

---

### HTTP Layer (http.rs)

Responsibilities:
- Build request
- Reuse client

```rust
pub fn build_client() -> Client {
    Client::builder()
        .pool_max_idle_per_host(1000)
        .build()
        .unwrap()
}
```

---

### Metrics (metrics.rs)

Use atomics:

```rust
pub struct Metrics {
    pub success: AtomicU64,
    pub failure: AtomicU64,
    pub total_latency: AtomicU64,
}
```

Latency storage:
- For v0.0.1 → store sample in Vec (bounded)

---

### Reporting (report.rs)

Output:

```
Total Requests: 1000000
Success: 998000
Failures: 2000
Avg Latency: 120ms
P95 Latency: 250ms
```

---