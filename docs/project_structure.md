# 📌 3. project_structure.md

## Folder Layout

```
rload/
 ├── src/
 │   ├── main.rs
 │   ├── cli.rs
 │   ├── config.rs
 │   ├── http.rs
 │   ├── engine.rs
 │   ├── worker.rs
 │   ├── metrics.rs
 │   ├── logger.rs
 │   └── report.rs
 │
 ├── docs/
 │   ├── action_plan.md
 │   ├── hld.md
 │   ├── lld.md
 │   └── observability.md
 │
 ├── examples/
 ├── Cargo.toml
 └── README.md
```

---

## Responsibilities

| Module | Responsibility |
|------|---------------|
| cli | Parse arguments |
| config | Normalize config |
| http | Build/send requests |
| engine | Control execution |
| worker | Perform load |
| metrics | Track performance |
| logger | Output logs |
| report | Final stats |

---