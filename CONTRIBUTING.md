# Contributing to DTCS

Thank you for contributing to the Data Transformation Contract Standard and its reference implementation.

## Contributor quickstart

1. Clone the repo and install **Rust 1.75+** and **Python 3.9+**.
2. Build the Python extension:
   ```bash
   python -m venv .venv && source .venv/bin/activate
   pip install maturin pytest
   maturin develop --no-default-features --features python --locked
   ```
3. Run tests:
   ```bash
   cargo test --locked
   cargo clippy --all-targets -- -D warnings
   cargo fmt --all -- --check
   pytest python/tests -v
   ./scripts/check-docs.sh
   ```
4. Read [docs/implementation/architecture.md](docs/implementation/architecture.md) and [public-api.md](docs/implementation/public-api.md).
5. See [docs/implementation/testing-plan.md](docs/implementation/testing-plan.md) for fixtures and golden files.

Full user-oriented dev notes: [docs/user/getting-started.md](docs/user/getting-started.md#develop-from-source).

### CI checks

Pull requests must pass the workflow in [`.github/workflows/checks.yml`](.github/workflows/checks.yml):

- **Rust:** `cargo fmt --check`, `cargo clippy`, `cargo test --locked`, `./scripts/check-docs.sh`
- **Python:** `maturin develop` + `pytest python/tests -v` on Python 3.9–3.13

### Pull request workflow

1. Fork the repository and create a feature branch.
2. Describe whether the change is specification, implementation, or editorial.
3. Update [CHANGELOG.md](CHANGELOG.md) for user-visible behavioral changes.
4. Add or update tests for behavioral changes (see testing-plan.md).
5. Ensure `cargo test --locked` and `pytest python/tests -v` pass locally.

Please follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

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

The reference crate through **0.11.0** implements parsing, the canonical object model, validation (including metadata, types, expressions, and I/O interfaces), diagnostics, contract analysis (compatibility, evolution, versioning, and dataset-level lineage), identifier registries with extension validation, starter standard libraries with registry-driven semantics validation, static semantic and expression analysis (Ch 7–8), transformation plan lowering (Ch 13), semantics-preserving plan optimization (Ch 13 §9), engine capability matching (Ch 14), compilation to execution plans (Ch 15), reference in-memory runtime execution (Ch 16), conformance profiles and offline certification (Ch 23), and automated security checklist probes (Ch 24). See [docs/implementation/non-goals.md](docs/implementation/non-goals.md) for remaining out-of-scope work.

### Conformance maintenance (Ch 26 §9)

When changing observable behavior covered by conformance tests, update `src/conformance/manifest.json` and the mirrored `tests/conformance/manifest.json` together. Run `cargo test --test phase_0_10` and `dtcs conformance run --profile all` before release.

### Registry governance (Ch 26 §7)

Novel `dtcs:` registry entries require a specification change. Vendor catalogs merge only non-standard identifiers; builtin entries remain authoritative.

### Extension review (Ch 26 §8)

Extension namespaces declared in conformance profiles must match registry validation behavior. Document new optional capabilities in profile definitions under `src/conformance/profiles.rs`.

### Change management (Ch 26 §5)

Pull requests that affect normative behavior should cite the SPEC chapter and section. Include conformance fixture updates when exit criteria are affected.

### Code style

- Run `cargo fmt` and `cargo clippy` before submitting changes.
- Keep modules aligned with [docs/implementation/crate-layout.md](docs/implementation/crate-layout.md).
- Prefer conservative behavior when the spec is ambiguous; document open questions with a `TODO` referencing the spec section.

## Releasing

Releases are automated by [`.github/workflows/release.yml`](.github/workflows/release.yml) when a semver tag is pushed:

```bash
# Ensure Cargo.toml and pyproject.toml are both 0.11.0 and CI is green
git tag v0.11.0
git push origin v0.11.0
```

The workflow verifies the tag matches `Cargo.toml` and `pyproject.toml`, runs checks, publishes to crates.io, builds multi-platform Python wheels, publishes to PyPI via `uv publish`, builds WASM/npm packages when configured, and attaches `dtcs-conformance-declaration.json` to the GitHub release.

Required repository secrets: `CARGO_REGISTRY_TOKEN`, `PYPI_API_TOKEN`. Optional: `NPM_TOKEN` for `@eddiethedean/dtcs-wasm` and `@eddiethedean/dtcs`.

Before tagging, confirm locally:

```bash
cargo test --locked
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
cargo publish --dry-run --locked
maturin develop --no-default-features --features python --locked
cargo test --test phase_0_10 --locked
cargo run --bin dtcs -- conformance run --profile all
pytest python/tests -v
```

## Pull requests

1. Describe whether the change is specification, implementation, or editorial.
2. Link related issues or design discussions when available.
3. Include or update tests for behavioral changes.
4. Ensure `cargo test` and `pytest python/tests -v` pass.

## Questions

For ambiguous spec language, open an issue with the relevant chapter and section number rather than inventing behavior in code.
