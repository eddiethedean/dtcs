# Architecture

Follow the architecture defined in `SPEC.md`.

Implementation pipeline (Phase 0.3):

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
Validator (0.1–0.2)              Analyzer (0.3)
        │                              │
        ▼                              ├─ compatibility::analyze
Diagnostics                            ├─ analyze_evolution
        │                              ├─ versioning::validate
        │                              └─ lineage::analyze
        │                              │
        │                              ▼
        │                         Analysis reports
        ▼
   (valid contracts only for analysis)
```

Analysis is **read-only** — it never mutates the Canonical Object Model.

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

For this crate through 0.3.0, implement through Diagnostics and Contract Analysis.
