# Project Goal

Build the reference Rust crate for the Data Transformation Contract Standard (DTCS).

`SPEC.md` is the source of truth.

## Implemented through 0.11.0

- Canonical Object Model (including `guarantees`, `compatibility` declaration, nested extensions)
- YAML and JSON parsing
- Seven-phase validation pipeline
- Diagnostics
- Type model (primitives, composites, conversions, extension types, expression typing)
- Metadata validation (identity, governance, ownership, lifecycle, provenance, classification, documentation, deprecation / `anticipatedRemoval`)
- Full Semantic Action, Function, and Rule standard libraries (`dtcs:`), with action parameters
- I/O interfaces (optional inputs, streaming, pre/postconditions)
- Compatibility analysis (five classification levels)
- Evolution analysis (change categories, deprecation, migration hints)
- Ch 25 versioning validation
- Dataset-level lineage analysis (dependency graph, impact, governance; mapping `operation` / `flow`)
- Identifier registry (embedded `dtcs:` catalog, file/URI load, offline cache)
- Registry-aware extension validation (mandatory/optional processing)
- Static semantic analysis (Ch 7–8), including null/missing/invalid value distinctions
- Transformation plan lowering (`plan::lower`, `plan::validate`, dependency graph)
- Plan optimization (`plan::optimize`, `plan::equivalent`, semantics-preserving passes)
- Engine capability matching (`capability::match_plan`, full-catalog `dtcs:reference` profile)
- Compilation (`compile::compile`, `ExecutionPlan` IR, `ReferenceCompiler`)
- Reference runtime (`runtime::execute`, in-memory row-oriented execution of the full stdlib)
- Conformance profiles and offline certification suite (Phase 0.10)
- WASM and Node bindings (parse, validate, conformance declare)
- CLI and Python bindings
- SPEC completeness matrix and normative Appendix A catalog

Chapter evidence: [spec-completeness.md](spec-completeness.md).

## Out of scope / future (non-goals for 0.X)

- Production ETL orchestration
- External engine backends (Spark, Polars, SQL)
- External certification authority (Ch 23 §13)

See [non-goals.md](non-goals.md) and [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md).
