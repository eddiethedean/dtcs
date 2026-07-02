# Project Goal

Build the reference Rust crate for the Data Transformation Contract Standard (DTCS).

`SPEC.md` is the source of truth.

## Implemented through 0.3.0

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
- CLI and Python bindings

## Stubs / future milestones

- Registry model (`src/model/registry.rs` — minimal struct only)
- Transformation Plan lowering (`src/plan/` — skeleton only)
- Execution, runtime behavior, backend compilation, and optimization
