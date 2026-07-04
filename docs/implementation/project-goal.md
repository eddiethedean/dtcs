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

## Stubs / future milestones

- Full standard libraries for actions, functions, and rules (Phase 0.5)
- Transformation Plan lowering (`src/plan/` — skeleton only)
- Execution, runtime behavior, backend compilation, and optimization
