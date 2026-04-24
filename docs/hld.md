# 📌 2. hld.md

## Overview

rload is a **minimal async load testing CLI** built in Rust.

---

## Architecture

```
CLI → Config → Engine → Workers → HTTP Client → Metrics → Report
```

---

## Components

### CLI
- Input parsing
- User interface

### Config
- Internal representation

### Engine
- Orchestrates execution
- Spawns workers

### Workers
- Execute requests
- Measure latency

### HTTP Client
- Connection pooling
- Request execution

### Metrics
- Aggregation layer

### Report
- Final output

---

## Concurrency Model

- Tokio async runtime
- Fixed worker pool
- Each worker handles N requests

---

## Data Flow

```
Input → Config → Engine → Workers → HTTP → Metrics → Output
```

---

## Design Principles

- Simplicity first
- Performance second
- Extensibility third

---