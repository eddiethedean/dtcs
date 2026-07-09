# Contributing to DTCS

Thank you for contributing to the Data Transformation Contract Standard and its reference implementation.

## Specification changes

Normative changes belong in [SPEC.md](SPEC.md). Follow the editorial process documented in:

- [docs/editorial/baseline.md](docs/editorial/baseline.md) — mandatory editorial conventions
- [docs/editorial/style-guide.md](docs/editorial/style-guide.md) — terminology and prose style
- [docs/editorial/authoring-guide.md](docs/editorial/authoring-guide.md) — chapter structure and normative language
- [docs/editorial/review-checklist.md](docs/editorial/review-checklist.md) — pre-publication review

### Normative language

Use RFC 2119 / RFC 8174 keywords (`MUST`, `SHALL`, `SHOULD`, `MAY`, etc.) for requirements.

### Authority

[SPEC.md](SPEC.md) is the single source of truth for semantics, terminology, and conformance. Implementation docs in [docs/implementation/](docs/implementation/) are illustrative unless explicitly normative.

## Implementation changes

The Rust reference crate lives in [src/](src/). Before implementing a module:

1. Read the relevant [SPEC.md](SPEC.md) chapter(s).
2. Consult [docs/implementation/](docs/implementation/) for architecture and API guidance.
3. Preserve spec-aligned naming and behavior.
4. Add tests that exercise spec requirements. See [docs/implementation/testing-plan.md](docs/implementation/testing-plan.md).

### Scope

The reference crate through **0.8.0** implements parsing, the canonical object model, validation (including metadata, types, expressions, and I/O interfaces), diagnostics, contract analysis (compatibility, evolution, versioning, and dataset-level lineage), identifier registries with extension validation, starter standard libraries with registry-driven semantics validation, static semantic and expression analysis (Ch 7–8), transformation plan lowering (Ch 13), and semantics-preserving plan optimization (Ch 13 §9). Do not add execution, compilation, or runtime features without an agreed milestone. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

### Code style

- Run `cargo fmt` and `cargo clippy` before submitting changes.
- Keep modules aligned with [docs/implementation/crate-layout.md](docs/implementation/crate-layout.md).
- Prefer conservative behavior when the spec is ambiguous; document open questions with a `TODO` referencing the spec section.

## Releasing

Releases are automated by [`.github/workflows/release.yml`](.github/workflows/release.yml) when a semver tag is pushed:

```bash
# Ensure Cargo.toml and pyproject.toml are both 0.8.0 and CI is green
git tag v0.8.0
git push origin v0.8.0
```

The workflow verifies the tag matches `Cargo.toml` and `pyproject.toml`, runs checks, publishes to crates.io, builds multi-platform Python wheels, and uploads to PyPI.

Required repository secrets: `CARGO_REGISTRY_TOKEN`, `PYPI_API_TOKEN`.

Before tagging, confirm locally:

```bash
cargo test --locked
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
cargo publish --dry-run --locked
maturin develop --no-default-features --features python --locked
pytest python/tests -v
```

## Pull requests

1. Describe whether the change is specification, implementation, or editorial.
2. Link related issues or design discussions when available.
3. Include or update tests for behavioral changes.
4. Ensure `cargo test` and `pytest python/tests -v` pass.

## Questions

For ambiguous spec language, open an issue with the relevant chapter and section number rather than inventing behavior in code.
