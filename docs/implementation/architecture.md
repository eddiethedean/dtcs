# Architecture

Follow the architecture defined in `SPEC.md`.

Implementation pipeline (Phase 0.8):

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
        ▼                         Analysis reports
Plan lowering (0.7)
        │
        ▼
Plan optimization (0.8)
        │
        ▼
Transformation Plan
```

Analysis is **read-only** — it never mutates the Canonical Object Model.
Registry resolution is also read-only; the embedded catalog is authoritative for `dtcs:` identifiers.
Standard library entries include structured definitions used during semantic validation.
Plan lowering is **read-only** with respect to the COM — it produces a separate `TransformationPlan` IR.
Plan optimization transforms a validated plan into a semantically equivalent optimized plan.

Future pipeline:

```text
Transformation Plan
        │
        ▼
Execution Plan
        │
        ▼
Runtime
```

For this crate through Phase 0.8 (`0.8.0`), implement through Diagnostics, Contract Analysis, Registries, starter Standard Libraries, static semantic analysis, transformation plan lowering, and plan optimization.
