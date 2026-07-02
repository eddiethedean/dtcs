# Recommended Rust Dependencies

Current [`Cargo.toml`](../../Cargo.toml):

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
miette = { version = "7", features = ["fancy"] }
semver = { version = "1", features = ["serde"] }
indexmap = { version = "2", features = ["serde"] }

[dependencies.clap]
version = "4"
features = ["derive"]
optional = true

[dependencies.pyo3]
version = "0.23"
optional = true
features = ["extension-module", "abi3-py39"]

[features]
default = ["cli"]
cli = ["dep:clap"]
python = ["dep:pyo3"]
```

| Dependency | Used for |
|------------|----------|
| `serde` / `serde_json` / `serde_yaml` | Canonical Object Model serialization |
| `miette` | CLI and file I/O error reporting |
| `semver` | `dtcsVersion` compatibility checks |
| `indexmap` | Stable-order maps and extension fields |
| `clap` | `dtcs` CLI (`cli` feature, on by default) |
| `pyo3` | Python bindings (`python` feature, via maturin) |

Use `serde` as the serialization layer.

Use `miette` for developer-friendly CLI errors.

Use custom DTCS diagnostics (`DiagnosticReport`) for spec-level validation reporting.
