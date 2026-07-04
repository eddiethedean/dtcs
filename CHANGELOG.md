# Changelog

## 0.4.0

Phase 0.4 — Registries & Extensibility.

### Features

- Identifier registry model (`RegistryDocument`, `RegistryEntry`, categories, publication status, extension compatibility)
- Embedded `dtcs:` catalog for actions (`dtcs:lowercase`), rules (`dtcs:not_null`), diagnostic codes, and the reserved `dtcs` namespace
- Registry resolution APIs: `registry::resolve`, `registry::load`, `registry::default_registry`, `registry::load_merged`
- Offline URI cache for registry documents (`registry::store_uri_cache`, `registry::load_uri_cached`)
- Registry-aware validation: standard identifiers resolve through the catalog; `validate_with_registry` accepts vendor catalogs
- Extension pass enforces mandatory unsupported extensions (`dtcs:unsupported-extension`)
- CLI: `dtcs registry list`, `dtcs registry resolve [--registry PATH]`
- Python: `registry_list`, `registry_resolve`, `registry_load`

### Diagnostics

- `dtcs:unknown-registry-entry`
- `dtcs:invalid-registry`
- `dtcs:unsupported-extension`

**Release:** push tag `v0.4.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

## 0.3.0

Phase 0.3 — Contract Analysis.

### Features

- Compatibility engine with five classifications (Identical, Backward/Forward Compatible, Conditionally Compatible, Incompatible)
- Evolution analysis with change categories, deprecation detection, and migration hints
- Ch 25 versioning validation (`versioning::validate`)
- Dataset-level lineage analysis (dependency graph, impact, dependency queries, governance summary)
- CLI: `dtcs compat`, `dtcs evolve`, `dtcs lineage`
- Python: `compat_analyze`, `evolve_analyze`, `lineage_analyze`, `version_validate`
- Fixtures under `tests/fixtures/compatibility/` for all classification levels
- User documentation under `docs/user/` and adoption overview under `docs/adoption/`

### Bug fixes

Audit remediation (24 defects):

- **Compatibility:** Directional integer/decimal comparison; optional input removal classified as breaking; required→optional as additive; scope-gated input type comparison; streaming/precondition/postcondition diffs; empty `objectRef` omitted from diagnostics; direction-aware classification (backward vs forward safe).
- **Validation:** Stricter `dtcsVersion` gate (exact 1.0.0 only); required schemas on inputs/outputs; ISO-8601 timezone offset validation; duplicate empty field names; version trim consistency; whitespace-only expression bodies still validate declared types; policy URI host validation; removed `.expect()` in expression typing.
- **API:** Invalid `--scope` tokens rejected (exit code 2).
- **Lineage / evolution:** Unknown `--impact` / `--dependency` IDs emit warnings; deprecation changes reported only on false→true transitions or replacement updates.
- **Python:** CLI parity for evolve/lineage text output, compat exit codes, validation diagnostics on load failure, `is_valid()` treats missing severity as error, NaN rejection in `contract_from_py`.
- **Hardening:** 16 MiB parser size limit; synthetic parse error when `into_contract` finds no contract.

### Breaking changes

- Diagnostic JSON fields use camelCase (`objectRef` instead of `object_ref`) across validate, diagnostics, compat, and lineage output.

**Release:** push tag `v0.3.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

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

**Release:** published at `v0.2.0` (crates.io and PyPI).

## 0.1.2

Phase 0.1 — Foundation: parser, COM, seven-phase validation, diagnostics, CLI, Python bindings.

## 0.1.x

Earlier 0.1.x releases established the initial validation pipeline and distribution tooling.
