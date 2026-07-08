# Project Goal

Build the reference Rust crate for the Data Transformation Contract Standard (DTCS).

`SPEC.md` is the source of truth.

## Implemented through 0.4.0

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
- CLI and Python bindings

## In development (Phase 0.5, `0.5.0` pending)

- Embedded starter standard libraries under `src/registry/builtin/` (semantic actions, functions, rules)
- Registry-driven semantics validation: target types, nullability, rule phases, function arity and return types
- Fixtures for stdlib validation under `tests/fixtures/stdlib_*.yaml`

## Stubs / future milestones

- Remaining Ch 17–19 standard library catalog entries (full libraries beyond the starter subset)
- Transformation Plan lowering (`src/plan/` — skeleton only)
- Execution, runtime behavior, backend compilation, and optimization
