# Non-Goals for the First Rust Crate

Do not implement these yet:

- ETL execution
- Polars backend
- Spark backend
- SQL compiler
- Runtime engine
- Optimization engine
- Full Transformation Plan lowering
- Python bindings
- WASM bindings
- Node bindings
- Conformance profiles and Ch 23 certification suites

The first crate should be a correct spec core based on [`SPEC.md`](../../SPEC.md).

## In scope for the MVP core

- Parse YAML and JSON into the Canonical Object Model
- Seven-phase validation with structured diagnostics
- Lineage completeness enforcement
- Scoped field reference resolution
- CLI (`validate`, `inspect`, `diagnostics`, `version`)

Lineage is **not** waived in the MVP core — contracts with outputs must declare provenance mappings.
