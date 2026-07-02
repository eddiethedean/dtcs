# Cursor Build Prompt

Build the first Rust implementation of DTCS.

The repository contains a single `SPEC.md` at the repository root with the full DTCS 1.0 draft specification. Implementation guides live in `docs/implementation/`.

Treat `SPEC.md` as the authoritative source of truth for terminology, architecture, object model, validation, diagnostics, conformance, registries, versioning, and semantics.

Implement a Rust crate that focuses on:

1. Canonical Object Model
2. YAML / JSON parsing
3. Diagnostics
4. Validation phases
5. CLI

Do not implement execution, backend compilation, optimization, or runtime behavior yet.

Follow the initial architecture:

```text
DTCS Document -> Parser -> Canonical Object Model -> Validator -> Diagnostics
```

Use Rust best practices, `serde`, `thiserror`, `miette`, `semver`, `indexmap`, and `clap`.

Create tests and examples as you build.

When this pack conflicts with `SPEC.md`, follow `SPEC.md`.
