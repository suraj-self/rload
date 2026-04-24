# 📌 5. observability.md

## Philosophy

Keep it **minimal but structured**.

---

## v0.0.1

### Console Output
- Summary only

---

### Optional File Output

JSON lines:

```
{"status":200,"latency":120}
```

---

## v0.1

### Structured Logging

Fields:
- timestamp
- status
- latency
- error

---

### Metrics

- Success rate
- Latency percentiles

---

## v0.2

### Export Options

- JSON
- CSV

---

### Integration Targets

- Prometheus (pull model)
- Grafana dashboards

---

## Storage Strategy

| Option | Use Case |
|------|---------|
| File (JSON) | Default |
| CSV | Simple analysis |
| SQLite | Advanced users |

---