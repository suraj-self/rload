# 📌 1. action_plan.md

## Goal
Ship a minimal, fast, and usable Rust-based load testing CLI.

---

## Phase 0 — Setup (DONE)
- [x] Initialize Rust project
- [x] Setup CLI parsing (clap)
- [x] Config abstraction

---

## Phase 1 — Core HTTP Execution (IN PROGRESS)
- [x] Add reqwest client
- [x] Send single request
- [x] Loop over requests
- [ ] Remove per-request logging
- [ ] Add error handling improvements

---

## Phase 2 — Concurrency Engine
- [ ] Introduce Tokio workers
- [ ] Fixed concurrency model
- [ ] Work distribution logic
- [ ] Graceful task completion

---

## Phase 3 — Metrics System
- [ ] Atomic counters (success/failure)
- [ ] Latency tracking
- [ ] Percentile calculation (P95)
- [ ] Throughput calculation

---

## Phase 4 — Reporting
- [ ] CLI summary output
- [ ] JSON export (`--output`)

---

## Phase 5 — Observability (Lightweight)
- [ ] JSON logs (optional)
- [ ] File output
- [ ] Structured events

---

## Phase 6 — OSS Readiness
- [ ] README polish
- [ ] Example commands
- [ ] Benchmarks
- [ ] License (MIT)

---

## Non-Goals (v0.0.1)
- Distributed load
- UI dashboards
- Complex scripting

---