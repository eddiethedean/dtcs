# Project Goal

Build the reference Rust crate for the Data Transformation Contract Standard (DTCS).

`SPEC.md` is the source of truth.

## Implemented through 0.2.0

- Canonical Object Model
- YAML and JSON parsing
- Seven-phase validation pipeline
- Diagnostics
- Type model (primitives, composites, conversions, extension types, expression typing)
- Metadata validation (identity, governance, provenance, classification, documentation)
- Semantic Action, Function, and Rule identity validation
- I/O interfaces (optional inputs, streaming, pre/postconditions)
- CLI and Python bindings

## Stubs / future milestones

- Registry model (`src/model/registry.rs` — minimal struct only)
- Compatibility model (`src/compatibility/` — empty stub)
- Execution, runtime behavior, backend compilation, and optimization
