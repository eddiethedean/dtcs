# Architecture

Follow the architecture defined in `SPEC.md`.

Implementation pipeline (Phase 0.4):

```text
DTCS Document
        │
        ▼
Parser
        │
        ▼
Canonical Object Model
        │
        ├──────────────────────────────┐
        ▼                              ▼
Validator (0.1–0.4)              Analyzer (0.3)
        │                              │
        │  registry::resolve           ├─ compatibility::analyze
        │  extension pass              ├─ analyze_evolution
        ▼                              ├─ versioning::validate
Diagnostics                            └─ lineage::analyze
        │                              │
        │                              ▼
        │                         Analysis reports
        ▼
   (valid contracts only for analysis)
```

Analysis is **read-only** — it never mutates the Canonical Object Model.
Registry resolution is also read-only; the embedded catalog is authoritative for `dtcs:` identifiers.

Future pipeline:

```text
Transformation Contract
        │
        ▼
Canonical Object Model
        │
        ▼
Transformation Plan
        │
        ▼
Execution Plan
        │
        ▼
Runtime
```

For this crate through 0.4.0, implement through Diagnostics, Contract Analysis, and Registries.
