# Changelog

## 0.3.0

Phase 0.3 — Contract Analysis.

- Compatibility engine with five classifications (Identical, Backward/Forward Compatible, Conditionally Compatible, Incompatible)
- Evolution analysis with change categories, deprecation detection, and migration hints
- Ch 25 versioning validation (`versioning::validate`)
- Dataset-level lineage analysis (dependency graph, impact, dependency queries, governance summary)
- CLI: `dtcs compat`, `dtcs evolve`, `dtcs lineage`
- Python: `compat_analyze`, `evolve_analyze`, `lineage_analyze`, `version_validate`
- Fixtures under `tests/fixtures/compatibility/` for all classification levels

## 0.2.0

Phase 0.2 — Contract Model.

- Metadata validation (identity, governance, provenance, classification, documentation)
- Extended type system: conversions, collections, extension types, expression typing
- I/O interface depth: optional inputs, streaming, pre/postconditions, I/O extensions
- Namespace validation hardening (reject `http:` / `https:` false positives)
- Python/Rust fixture parity via shared `tests/fixture_expectations.json`
- Python CLI aligned with Rust (`parse_file` for all commands)
- `validate_result()` and `__version__` exported from the Python package

**0.2.0 follow-up fixes (no version bump):**

- Expression operator precedence (`*`, `/` before `+`, `-`; comparisons bind correctly)
- Reject duplicate identifiers across inputs and outputs
- Directional integer/decimal assignability (no lossy narrowing)
- Function namespace validation, optional-parameter ordering, and call arity checks
- Unary minus in expressions; nullable field rejection in typed expressions
- `dtcs:lowercase` rejects nullable string targets
- Python CLI catches `parse_file` I/O errors without traceback; shared fixture manifest under `tests/`
- CI Python version matrix; maturin builds with `--no-default-features --features python`
- PyPI `--skip-existing` on release re-runs; robust `pyproject.toml` version parsing in CI

**Release:** pending `v0.2.0` tag (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

## 0.1.2

Phase 0.1 — Foundation: parser, COM, seven-phase validation, diagnostics, CLI, Python bindings.

## 0.1.x

Earlier 0.1.x releases established the initial validation pipeline and distribution tooling.
