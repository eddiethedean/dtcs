# Architecture

Follow the architecture defined in `SPEC.md`.

Implementation pipeline (through Phase 0.11):

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
Validator (0.1–0.6, 0.11)        Analyzer (0.3, 0.6, 0.11)
        │                              │
        │  registry::resolve           ├─ compatibility::analyze
        │  full stdlib definitions     ├─ analyze_evolution
        │  extension pass (nested)     ├─ versioning::validate
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
Capability matching (0.9, 0.11)
        │
        ▼
Compilation (0.9, 0.11)
        │
        ▼
Reference runtime (0.9, 0.11)
        │
        ▼
Conformance (0.10, 0.11)
        │
        ▼
Outputs / certification report
```

Analysis is **read-only** — it never mutates the Canonical Object Model.
Registry resolution is also read-only; the embedded catalog (Appendix A) is authoritative for `dtcs:` identifiers.
Standard library entries include structured definitions used during semantic validation.
Plan lowering is **read-only** with respect to the COM — it produces a separate `TransformationPlan` IR.
Plan optimization transforms a validated plan into a semantically equivalent optimized plan.
Capability matching, compilation, and the reference runtime execute validated contracts in-memory for conformance and development use.
Phase 0.11 completes COM depth (lineage `operation`/`flow`, guarantees, compatibility declaration, nested extensions, null/missing/invalid tokens) and the full Ch 17–19 catalog.

See [spec-completeness.md](spec-completeness.md) and [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) Phase 0.11.
