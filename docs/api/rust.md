# Rust API (crate consumers)

Crate: [`dtcs` on crates.io](https://crates.io/crates/dtcs).

**Generated API docs:** [https://docs.rs/dtcs](https://docs.rs/dtcs)

This page is a consumer-oriented map. Implementer design notes live in [public-api.md](../implementation/public-api.md).

## Add dependency

```toml
[dependencies]
dtcs = "0.11"
```

## Common entry points

| API | Purpose |
|-----|---------|
| `parse` / `parse_file` | YAML/JSON → COM |
| `validate` / `validate_with_registry` | Validation report |
| `check_contract` | Static semantic analysis |
| `lower_plan` / `optimize_plan` / `plan_equivalent` | Transformation plans |
| `analyze_compatibility` / `analyze_evolution` / `analyze_lineage` | Compatibility and lineage |
| `match_plan` / `compile` / `execute` | Capability match, compile, runtime |
| `conformance_run` / `conformance_declare` | Ch 23 certification |
| `resolve_registry` / `default_registry` | Identifier catalog |
| `SPEC_VERSION` | Spec version string |

Exact signatures: see docs.rs for the installed crate version.

## CLI

```bash
cargo install dtcs --version 0.12.0
dtcs validate contract.dtcs.yaml
```

## Feature flags

Default builds include the CLI binary. Python bindings use the `python` feature via maturin (see [CONTRIBUTING.md](https://github.com/eddiethedean/dtcs/blob/main/CONTRIBUTING.md)).

## See also

- [Python API](python.md) · [WASM](wasm.md) · [Node](node.md)
- [architecture.md](../implementation/architecture.md)
