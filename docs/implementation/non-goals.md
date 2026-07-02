# Non-Goals for the First Rust Crate

Do not implement these yet:

- ETL execution
- Polars backend
- Spark backend
- SQL compiler
- Runtime engine
- Optimization engine
- Full Transformation Plan lowering
- WASM bindings
- Node bindings
- Conformance profiles and Ch 23 certification suites

The first crate should be a correct spec core based on [`SPEC.md`](../../SPEC.md).

## In scope through 0.2.0

- Parse YAML and JSON into the Canonical Object Model
- Seven-phase validation with structured diagnostics
- Metadata validation (Phase 0.2)
- Type system: conversions, collections, extension types, expression typing (Phase 0.2)
- I/O interfaces: optional inputs, streaming, pre/postconditions (Phase 0.2)
- Lineage completeness enforcement
- Scoped field reference resolution
- CLI (`validate`, `inspect`, `diagnostics`, `version`)
- Python bindings via maturin

Lineage is **not** waived in the MVP core — contracts with outputs must declare provenance mappings.
