# Rust API (crate consumers)

Crate: [`dtcs` on crates.io](https://crates.io/crates/dtcs).

**Generated API docs:** [https://docs.rs/dtcs](https://docs.rs/dtcs/0.12.0)

This page is a consumer-oriented map. Implementer design notes live in [public-api.md](../implementation/public-api.md). Versions: [versioning.md](../user/versioning.md).

## Add dependency

```toml
[dependencies]
dtcs = "0.12"
```

## Common entry points

| API | Purpose |
|-----|---------|
| `parse` / `parse_file` | YAML/JSON → COM |
| `validate` / `validate_with_registry` | Validation report |
| `check_contract` | Static semantic analysis |
| `lower_plan` / `optimize_plan` / `plan_equivalent` | Transformation plans |
| `export_portable_plan` | Portable transform-plan envelope (`dtcs.transform-plan/1`) |
| `analyze_compatibility` / `analyze_evolution` / `analyze_lineage` | Compatibility and lineage |
| `match_plan` / `compile` / `execute` | Capability match, compile, runtime |
| `reference_portable_manifest` / `validate_capability_accuracy` | Portable capability manifests |
| `conformance_run` / `conformance_declare` | Ch 23 certification |
| `resolve_registry` / `default_registry` | Identifier catalog |
| `SPEC_VERSION` | Spec version string (`"2.0.0"`) |

Exact signatures: see docs.rs for the installed crate version.

## Minimal pipeline example

```rust
use dtcs::{parse, validate, DocumentFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = include_str!("../examples/minimal.dtcs.yaml");
    let parsed = parse(yaml.as_bytes(), DocumentFormat::Yaml)?;
    let contract = parsed.contract.expect("parse produced a contract");
    let report = validate(&contract);
    assert!(!report.has_errors());
    assert_eq!(dtcs::SPEC_VERSION, "2.0.0");
    Ok(())
}
```

For plan → portable export, see `export_portable_plan` on docs.rs and the CLI below.

## CLI

```bash
cargo install dtcs --version 0.12.0
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
- [migration-0.12.md](../user/migration-0.12.md)
