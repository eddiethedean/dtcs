# Changelog

## Migration summary

| Version | Breaking or notable changes |
|---------|----------------------------|
| **0.10.0** | Conformance profiles, `dtcs conformance` CLI, Python `conformance_*` APIs, WASM/Node bindings, `uv publish` migration. |
| **0.9.0** | New `match`, `compile`, `run` CLI and Python APIs. `runtime_execute` returns `{outputs, diagnostics}` envelope. |
| **0.8.0** | Plan optimization APIs (`plan_optimize`, `plan_equivalent`). |
| **0.7.0** | Transformation plan lowering (`plan_lower`, `plan_validate`). |
| **0.3.0** | Diagnostic and report JSON uses **camelCase** field names. |
| **0.2.0** | Extended validation (metadata, types, expressions, I/O interfaces). |

For upgrade questions, see [docs/user/faq.md](docs/user/faq.md) and [docs/user/troubleshooting.md](docs/user/troubleshooting.md).

## 0.10.0

Phase 0.10 — Conformance & Ecosystem (Ch 23–24, Ch 26).

### Features

- New `conformance` module with eight implementation class profiles, capability declaration, offline test runner, and security checklist probes.
- CLI: `dtcs conformance declare`, `dtcs conformance run --profile <name|all>`.
- Python: `conformance_declare()`, `conformance_run()`.
- WASM package `@eddiethedean/dtcs-wasm` (`parseDocument`, `validateContract`, `conformanceDeclare`).
- Node package `@eddiethedean/dtcs` (thin WASM wrapper).
- Documentation: [docs/user/conformance.md](docs/user/conformance.md), [docs/adoption/security-checklist.md](docs/adoption/security-checklist.md), [docs/api/python.md](docs/api/python.md).
- Release workflow publishes Python packages via `uv publish`; attaches `dtcs-conformance-declaration.json` to GitHub releases.

### Tests

- `tests/conformance/manifest.json`, `tests/phase_0_10.rs`, CI conformance gate.

**Release:** push tag `v0.10.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

## 0.9.0

Phase 0.9 — Execution pipeline (Ch 14–16).

### Features

- New `capability` module with `reference_profile`, `match_plan`, `validate`, and `discover`.
- Embedded `dtcs:reference` engine capability profile derived from the 0.5 stdlib catalog.
- New `compile` module with `ExecutionPlan` IR, `Compiler` trait, and `ReferenceCompiler`.
- New `runtime` module with in-memory row-oriented `ReferenceRuntime` executing all embedded `dtcs:` actions, functions, and rules.
- Capability-stage diagnostics: `dtcs:unsupported-capability`, `dtcs:invalid-capability`.
- Compilation-stage diagnostics: `dtcs:compilation-failed`, `dtcs:invalid-execution-plan`.
- Runtime-stage diagnostics: `dtcs:invalid-runtime-input`, `dtcs:precondition-violation`, `dtcs:postcondition-violation`, `dtcs:runtime-error`.
- CLI and Python: `dtcs match`, `dtcs compile`, `dtcs run`; Rust `parse_validate_and_compile` / `parse_validate_and_run`.
- Python: `capability_match`, `capability_reference_profile`, `compile_plan`, `execution_validate`, `runtime_execute`.

### Tests

- Phase 0.9 integration tests, `tests/capability_expectations.json`, and runtime fixtures under `tests/fixtures/runtime/`.

### Bug fixes (0.9.0 hardening)

- Reference compiler emits input-interface steps before `MaterializeOutput`, then output-interface steps — fixes output-targeted actions (e.g. `plan_field_write_chain.yaml`).
- Omit no-op `EvaluateExpression` execution steps until expression write targets exist in COM.
- Capability matching collects all plan identifiers (including vendor `acme:*`), failing at match time for unsupported vendor actions.
- CLI `compile --profile` wired to `compile_with_capability`.
- Builtin contract alignment: `dtcs:concat` requires ≥2 arguments; `dtcs:length` rejects null; `dtcs:regex_match` uses the `regex` crate.
- Python CLI parity: `match`, `compile`, and `run` subcommands.
- Compile golden tests (`tests/compile_expectations.json`, `tests/fixtures/execution_plans/*.exec.json`).

**Release:** push tag `v0.9.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

## 0.8.0

Phase 0.8 — Plan optimization (Ch 13 §9, Ch 8 §14, Ch 15 §9, Ch 17–19 §11).

### Features

- New `plan::optimize` / `plan::equivalent` with semantics-preserving expression, function, action, and rule passes.
- Expression constant folding, algebraic simplification, and dead-expression elimination.
- Registry-gated evaluation of deterministic `dtcs:` function calls in expressions.
- Idempotent semantic-action fusion and duplicate rule deduplication.
- Dependency graph rebuild and fail-closed `plan::validate` after optimization.
- Optimization-stage diagnostics: `dtcs:invalid-optimization`, `dtcs:optimization-skipped` (information-level; emitted when a rewrite is skipped because a type guard would not hold).
- CLI and Python: `dtcs optimize` / `plan_optimize` / `plan_equivalent`; Rust `parse_validate_and_optimize`.

### Bug fixes

- Rule deduplication includes rule `parameters` in the dedup key and equivalence fingerprints.
- Equivalence checker preserves semantic action order and distinguishes rule parameters.
- Dead-expression elimination runs after dependency graph rebuild (not before).
- Expression rewrites fail closed on type-guard mismatch; emit `dtcs:optimization-skipped`.
- Invalid input plans are rejected before optimization transforms when validation is enabled.
- Python CLI gains `optimize` with `--plan`, `--registry`, `--no-validate`, and `--json`.

### Tests

- Phase 0.8 integration tests, `tests/optimize_expectations.json`, and golden optimized plans under `tests/fixtures/plans_optimized/`.

**Release:** push tag `v0.8.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

## 0.7.0

Phase 0.7 — Transformation Plan lowering (Ch 13 §4–8, §11–12, §14).

### Features

- New `plan` module with `lower` / `validate` lowering validated COM into canonical semantic IR.
- `TransformationPlan` IR: inputs, outputs, functions, semantic step nodes, dependency graph, lineage, and contractual guarantees.
- Dependency graph construction from lineage, field references, explicit action ordering, rule phases, and interface conditions.
- Planning-stage diagnostics: `dtcs:invalid-plan`, `dtcs:incomplete-plan`, `dtcs:cyclic-dependency`, `dtcs:plan-type-mismatch`, `dtcs:unresolved-plan-reference`.
- CLI and Python: `dtcs plan` / `plan_lower` / `plan_validate` / `plan_topological_order`; Rust `parse_validate_and_plan` / `parse_validate_and_plan_with_registry`.
- Analysis findings and diagnostics attached during lowering.

### Bug fixes

- Output-targeting semantic actions now receive lineage prerequisites from all contributing inputs.
- `FieldWrite` edges between consecutive writers on the same field; last-writer resolution respects explicit ordering.
- Multi-input lineage emits edges for every input, not only the first.
- Unified acyclic dependency check in graph construction and plan validation.
- Explicit ordering validation rejects unknown, duplicate, and incomplete `semantics.ordering` lists.
- Rule-phase edges scoped to rules sharing the same target.
- `--registry` threaded through CLI pre-validation (`plan`, `compat`, `evolve`, `lineage`) and `parse_validate_and_plan_with_registry`.
- Python: `analyze` exported in `__all__`; `plan_from_py` rejects NaN; plan CLI prints topological order.

### Tests

- Phase 0.7 integration tests, `tests/plan_expectations.json`, and golden plan files under `tests/fixtures/plans/`.

**Release:** push tag `v0.7.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

## 0.6.0

Phase 0.6 — Semantic analysis of transformation semantics (Ch 7) and expressions (Ch 8).

### Features

- New `analysis` module with `check_contract` / `check_expression` for static semantic analysis (no runtime evaluation).
- Expression parser and AST (operators, precedence, logical ops, and direct `dtcs:` call syntax).
- Registry-aware expression typing for direct `dtcs:` function calls, including `sameAsArgs` and `returnNullable`.
- Contract semantics analysis: action composition conflicts, explicit ordering validation, purity and determinism checks.
- CLI and Python: `dtcs analyze` / `dtcs.analyze(...)` returning analysis diagnostics and findings.

### Tests

- New Phase 0.6 integration tests and fixtures exercising `dtcs:` expression calls, logical operators, constant expressions, and composition diagnostics.

## 0.5.0

Phase 0.5 completion — stdlib validation fixes and rule parameters.

### Bug fixes

- Positional `argTypes` validation for multi-parameter stdlib functions (for example `dtcs:substr`)
- `sameAsArgs` return-type and homogeneous parameter checks for `dtcs:coalesce`
- Semantic action targets no longer accept bare interface IDs at reference phase
- `resolve_field` emits `dtcs:unresolved-reference` for missing targets
- `is_known_*` helpers only return true for resolved registry entries

### Features

- `Rule.parameters` on the COM model with registry-driven validation (required params, types, `dtcs:range` bounds)
- `Function.nullable` return flag validated against registry `returnNullable`
- Expression call typing honors registry `returnNullable` for `dtcs:` function declarations
- `dtcs validate --registry PATH` and Python `validate_with_registry()` / `validate(..., registry_path=...)`
- CLI/Python parity: `--json` on load failures, full `evolve` text output, lineage arrow formatting

### Tests

- Fixtures for all 21 starter stdlib catalog entries (valid and invalid variants)

**Release:** push tag `v0.5.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

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
