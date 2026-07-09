# Reference Implementation Boundaries

Do not implement these yet:

- ETL execution
- Polars backend
- Spark backend
- SQL compiler
- Runtime engine
- WASM bindings
- Node bindings
- Conformance profiles and Ch 23 certification suites

The reference crate should remain a correct spec core based on [`SPEC.md`](../../SPEC.md).

## In scope through 0.8.0

- Parse YAML and JSON into the Canonical Object Model
- Seven-phase validation with structured diagnostics
- Metadata validation (Phase 0.2)
- Type system: conversions, collections, extension types, expression typing (Phase 0.2)
- I/O interfaces: optional inputs, streaming, pre/postconditions (Phase 0.2)
- Lineage completeness enforcement
- Scoped field reference resolution
- Compatibility analysis with five classification levels (Phase 0.3)
- Evolution analysis with change categories and migration hints (Phase 0.3)
- Ch 25 versioning validation (Phase 0.3)
- Dataset-level lineage analysis: dependency graph, impact, governance (Phase 0.3)
- Identifier registry with embedded `dtcs:` catalog, file/URI load, offline cache (Phase 0.4)
- Registry-aware extension validation (mandatory/optional) (Phase 0.4)
- Embedded starter standard libraries with registry-driven semantics validation (Phase 0.5)
- Static semantic analysis (Phase 0.6)
- Transformation plan lowering with dependency graph and plan validation (Phase 0.7)
- Plan optimization with semantics-preserving passes and equivalence checking (Phase 0.8)
- CLI (`validate`, `analyze`, `plan`, `optimize`, `inspect`, `diagnostics`, `version`, `compat`, `evolve`, `lineage`, `registry`)
- Python bindings via maturin

Lineage is **not** waived in the MVP core — contracts with outputs must declare provenance mappings.
