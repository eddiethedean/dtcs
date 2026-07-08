# Architecture

Follow the architecture defined in `SPEC.md`.

Implementation pipeline (Phase 0.6):

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
Validator (0.1–0.6)              Analyzer (0.3, 0.6)
        │                              │
        │  registry::resolve           ├─ compatibility::analyze
        │  stdlib definition checks    ├─ analyze_evolution
        │  extension pass              ├─ versioning::validate
        ▼                              └─ lineage::analyze
Diagnostics                            │
        │                              ▼
        │                         Analysis reports
        ▼
   (analysis is read-only; no runtime evaluation)
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

For this crate through Phase 0.6 (`0.6.0`), implement through Diagnostics, Contract Analysis, Registries, starter Standard Libraries, and static semantic analysis.
