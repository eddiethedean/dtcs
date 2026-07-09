# Project Goal

Build the reference Rust crate for the Data Transformation Contract Standard (DTCS).

`SPEC.md` is the source of truth.

## Implemented through 0.9.0

- Canonical Object Model
- YAML and JSON parsing
- Seven-phase validation pipeline
- Diagnostics
- Type model (primitives, composites, conversions, extension types, expression typing)
- Metadata validation (identity, governance, provenance, classification, documentation)
- Semantic Action, Function, and Rule identity validation
- I/O interfaces (optional inputs, streaming, pre/postconditions)
- Compatibility analysis (five classification levels)
- Evolution analysis (change categories, deprecation, migration hints)
- Ch 25 versioning validation
- Dataset-level lineage analysis (dependency graph, impact, governance)
- Identifier registry (embedded `dtcs:` catalog, file/URI load, offline cache)
- Registry-aware extension validation (mandatory/optional processing)
- Static semantic analysis (Ch 7–8)
- Transformation plan lowering (`plan::lower`, `plan::validate`, dependency graph)
- Plan optimization (`plan::optimize`, `plan::equivalent`, semantics-preserving passes)
- Engine capability matching (`capability::match_plan`, `dtcs:reference` profile)
- Compilation (`compile::compile`, `ExecutionPlan` IR, `ReferenceCompiler`)
- Reference runtime (`runtime::execute`, in-memory row-oriented execution)
- CLI and Python bindings

- Embedded starter standard libraries under `src/registry/builtin/` (semantic actions, functions, rules)
- Registry-driven semantics validation: target types, nullability, rule phases, function arity and return types

## Stubs / future milestones

- Remaining Ch 17–19 standard library catalog entries (full libraries beyond the starter subset)
- Plan optimization extensions (additional rewrite passes beyond the starter set)
- External engine backends (Spark, Polars, SQL)
- Conformance profiles and certification suites (Phase 0.10)
