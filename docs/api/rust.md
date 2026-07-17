# Rust API (crate consumers)

Crate: [`dtcs` on crates.io](https://crates.io/crates/dtcs).

**Generated API docs (signatures):** [https://docs.rs/dtcs/0.13.0](https://docs.rs/dtcs/0.13.0)

This page is a consumer-oriented map. Implementer design notes: [public-api.md](../implementation/public-api.md). Versions: [versioning.md](../user/versioning.md).

## Add dependency

```toml
[dependencies]
dtcs = "0.13"
```

## Common entry points

| API | Purpose |
|-----|---------|
| `parse` / `parse_file` / `parse_yaml` / `parse_json` | YAML/JSON → COM |
| `validate` / `validate_with_registry` | Validation report |
| `parse_and_validate` | One-shot parse + validate |
| `check_contract` / `check_expression` | Static semantic analysis |
| `to_structured_node` / `from_structured_node` | Structured expression trees |
| `lower_plan` / `optimize_plan` / `validate_plan` / `plan_equivalent` | Transformation plans |
| `export_portable_plan` | Portable envelope (`dtcs.transform-plan/2`) |
| `parse_validate_and_plan` / `_optimize` / `_compile` / `_run` | Pipeline helpers (+ `_with_registry` variants) |
| `analyze_compatibility` / `analyze_evolution` / `analyze_lineage` | Compatibility and lineage |
| `match_plan` / `compile` / `compile_after_match` / `execute` | Capability match, compile, runtime |
| `discover_capabilities` / `validate_capabilities` | Capability discovery |
| `reference_portable_manifest` / `validate_capability_accuracy` | Portable capability manifests |
| `conformance_run` / `conformance_declare` | Ch 23 certification |
| `default_registry` / `resolve_registry` / `is_known_*` | Identifier catalog |
| `inspect_contract` / `codes` | Diagnostics helpers |
| `Dataset` / `Row` / `RuntimeValue` / `RuntimeInputs` | Runtime value types |
| `SPEC_VERSION` / `TRANSFORM_PLAN_IDENTITY` / `KERNEL_PROFILE` | Version and profile constants |

Exact signatures: **docs.rs for the installed crate version**.

## Minimal pipeline (crates.io — no repo checkout)

```rust
use dtcs::{parse, validate, DocumentFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = br#"
dtcsVersion: "3.0.0"
id: "demo.minimal"
name: "Minimal Demo"
version: "0.1.0"
metadata:
  description: "crates.io consumer demo"
  classification: internal
  governance: { owner: "docs", steward: "docs" }
  provenance: { author: "docs", createdAt: "2026-01-01T00:00:00Z" }
inputs:
  - id: "people"
    schema:
      fields:
        - { name: "display_name", type: "string", nullable: false }
outputs:
  - id: "people_out"
    schema:
      fields:
        - { name: "display_name", type: "string", nullable: false }
semanticActions:
  - { id: "lower", action: "dtcs:lowercase", target: "people.display_name" }
lineage:
  mappings:
    - { output: "people_out", inputs: ["people"] }
"#;
    let parsed = parse(yaml, DocumentFormat::Yaml)?;
    let contract = parsed.contract.expect("parse produced a contract");
    let report = validate(&contract);
    assert!(!report.has_errors());
    assert_eq!(dtcs::SPEC_VERSION, "3.0.0");
    Ok(())
}
```

Inside this repository you may use `include_str!("../examples/minimal.dtcs.yaml")` instead of an inline string.

## Portable export sketch

```rust
use dtcs::{export_portable_plan, lower_plan, KERNEL_PROFILE, TRANSFORM_PLAN_IDENTITY};

// after you have a validated `&TransformationContract`:
let lowered = lower_plan(&contract, None, None);
let plan = lowered.plan.expect("lower succeeded");
let portable = export_portable_plan(&plan, KERNEL_PROFILE)?;
assert_eq!(portable.plan_identity, TRANSFORM_PLAN_IDENTITY);
```

Confirm parameter order and profile constants on [docs.rs](https://docs.rs/dtcs/0.13.0).
## CLI

```bash
cargo install dtcs --version 0.13.0
dtcs validate contract.dtcs.yaml
dtcs export-portable contract.dtcs.yaml --fingerprint
```

## Feature flags

Default builds include the CLI binary. Python bindings use the `python` feature via maturin (see [CONTRIBUTING.md](https://github.com/eddiethedean/dtcs/blob/main/CONTRIBUTING.md)).

## Errors

Library APIs return `DiagnosticReport` / structured results rather than panicking on invalid contracts. The CLI maps error-severity diagnostics to exit code `1`. See [error-taxonomy.md](../user/error-taxonomy.md).

## See also

- [Python API](python.md) · [WASM](wasm.md) · [Node](node.md)
- [architecture.md](../implementation/architecture.md)
- [migration-0.13.md](../user/migration-0.13.md)
- [docs.rs/dtcs](https://docs.rs/dtcs/0.13.0) (signature source of truth)
