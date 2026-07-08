# Architecture

Follow the architecture defined in `SPEC.md`.

Implementation pipeline (Phase 0.5):

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
Validator (0.1–0.5)              Analyzer (0.3)
        │                              │
        │  registry::resolve           ├─ compatibility::analyze
        │  stdlib definition checks    ├─ analyze_evolution
        │  extension pass              ├─ versioning::validate
        ▼                              └─ lineage::analyze
Diagnostics                            │
        │                              ▼
        │                         Analysis reports
        ▼
   (valid contracts only for analysis)
```

Analysis is **read-only** — it never mutates the Canonical Object Model.
Registry resolution is also read-only; the embedded catalog is authoritative for `dtcs:` identifiers.
Standard library entries include structured definitions used during semantic validation.

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

For this crate through Phase 0.5 (in development), implement through Diagnostics, Contract Analysis, Registries, and starter Standard Libraries.
