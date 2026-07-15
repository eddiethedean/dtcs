# Maintainers release runbook

How to cut a multi-channel release. Contributors: see [CONTRIBUTING.md](https://github.com/eddiethedean/dtcs/blob/main/CONTRIBUTING.md).

## Pre-flight

1. Update versions together:
   - `Cargo.toml` / workspace versions
   - `pyproject.toml`
   - Binding manifests if versioned independently
   - Docs that hard-code `0.x.y` (getting-started expected output, migration pins)
2. Update [CHANGELOG.md](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md) (move Unreleased → version section)
3. Sync conformance manifests:
   - `src/conformance/manifest.json`
   - `tests/conformance/manifest.json`
4. Local gates:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo test --test phase_0_10 --locked
cargo test --test phase_0_11 --locked
cargo run --bin dtcs -- conformance run --profile all
pytest python/tests -v
NO_MKDOCS_2_WARNING=true ./scripts/check-docs.sh
```

5. Binding smokes (from repo):

```bash
cd bindings/wasm && npm test
cd ../node && npm test
```

## Tag and publish

```bash
git tag v0.x.y
git push origin v0.x.y
```

[`.github/workflows/release.yml`](https://github.com/eddiethedean/dtcs/blob/main/.github/workflows/release.yml) publishes crates.io / PyPI when secrets are configured. npm publish runs only when `NPM_TOKEN` is set.

## Verify

- crates.io / PyPI show `0.x.y`
- `pip install dtcs==0.x.y && dtcs version`
- docs.rs builds for the tag
- Read the Docs picks up `main` / tags as configured
- Optional: `npm view @eddiethedean/dtcs version` when npm publish is enabled

## Failure modes

| Symptom | Check |
|---------|-------|
| PyPI missing wheel | maturin/release job logs; Python version matrix |
| crates.io reject | version already published; `cargo publish --dry-run` |
| npm skipped | `NPM_TOKEN` unset (expected) |
| Conformance drift | dual manifest sync; `phase_0_10` / `phase_0_11` tests |
