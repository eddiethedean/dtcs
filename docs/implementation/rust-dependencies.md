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

[features]
default = ["cli"]
cli = ["dep:clap"]
```

| Dependency | Used for |
|------------|----------|
| `serde` / `serde_json` / `serde_yaml` | Canonical Object Model serialization |
| `miette` | CLI and file I/O error reporting |
| `semver` | `dtcsVersion` compatibility checks |
| `indexmap` | Stable-order maps and extension fields |
| `clap` | `dtcs` CLI (`cli` feature, on by default) |

Use `serde` as the serialization layer.

Use `miette` for developer-friendly CLI errors.

Use custom DTCS diagnostics (`DiagnosticReport`) for spec-level validation reporting.
