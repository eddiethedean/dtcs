# Changelog

## Migration summary

| Version | Breaking or notable changes |
|---------|----------------------------|
| **0.13.0** | **DTCS 3.0 Rich Portable Analytics:** Spec `3.0.0`; canonical portable plan `dtcs.transform-plan/2` with v1 migration; category-aware duplicate registry identifiers; lambda Expressions; advanced string/regex, conversion, complex-value, reshape, set, sampling, temporal, nondeterministic, and window-v2 profile declarations. |
| **0.12.0** | **Portable Relational Profile (R1–R4):** operator registry; widened actions (entry v2, legacy params still valid); portable plan `dtcs.transform-plan/1`; rich joins/unions/aggs; differential conformance fixtures; window frames; datetime (fixed-offset); complex access ops; SPEC `2.0.0`. See [docs/user/migration-portable-relational.md](docs/user/migration-portable-relational.md). |
| **0.11.0** | **Lineage:** mapping `operation` defaults to `dtcs:derive`; `flow` enum (`preserved`\|`derived`\|`aggregated`\|`filtered`\|`partitioned`\|`discarded`). **Actions:** SemanticAction `parameters` map required for dataset operators (`fields`, join keys, etc.). **COM:** first-class `guarantees` and `compatibility` fields; nested extension preservation. **Null semantics:** runtime distinguishes null vs missing (`{"$dtcs":"missing"}`) vs invalid (`{"$dtcs":"invalid", reason?}`). **Stdlib:** full Ch 17–19 catalog (dataset ops + `abs`/`min`/`max`/`contains`/`is_null`/`is_missing` + `one_of`/`equals`). |
| **0.10.1** | Test suite verification (P2/P3): plan behavioral oracles, format equivalence, determinism, `RuntimeInvalid` conformance, binding smoke parity, automated security probe. |
| **0.10.0** | Conformance profiles, `dtcs conformance` CLI, Python `conformance_*` APIs, WASM/Node bindings, `uv publish` migration. |
| **0.9.0** | New `match`, `compile`, `run` CLI and Python APIs. `runtime_execute` returns `{outputs, diagnostics}` envelope. |
| **0.8.0** | Plan optimization APIs (`plan_optimize`, `plan_equivalent`). |
| **0.7.0** | Transformation plan lowering (`plan_lower`, `plan_validate`). |
| **0.3.0** | Diagnostic and report JSON uses **camelCase** field names. |
| **0.2.0** | Extended validation (metadata, types, expressions, I/O interfaces). |

For upgrade questions, see [docs/user/faq.md](docs/user/faq.md) and [docs/user/troubleshooting.md](docs/user/troubleshooting.md).

## 0.13.0

DTCS 3.0 / tools 0.13 release.

### Features

- Canonical `dtcs.transform-plan/2` output with deterministic migration of valid v1 envelopes.
- 3.0 capability declarations: protocol/profile claims, semantic environments, modes, budgets, and guarantees.
- Category-aware registry lookup so standard Semantic Actions and Functions may share an identifier such as `dtcs:trim`.
- Bounded lambda AST nodes and reference-runtime support for `transform`, `filter_values`, `exists`, and `forall`.
- Reference-runtime support for core advanced string/regex, conversion, complex-value, explode, unpivot, set-operation, and seeded sampling behavior.
- Rich Portable Analytics profile declarations and initial conformance coverage.

### Compatibility

DTCS 1.0 and 2.0 contracts remain accepted. Valid `dtcs.transform-plan/1` envelopes are accepted and migrated to v2 without changing their envelope semantics. New profile families remain experimental or candidate until their independent-conformance criteria are met.

## 0.12.0

Phase 0.12 — Portable Relational Profile (DTCS-R1 through R4) in one release ([docs/DTCS_PORTABLE_SPEC_PROPOSAL.md](docs/DTCS_PORTABLE_SPEC_PROPOSAL.md)).

**Specification:** [SPEC.md](SPEC.md) version **`2.0.0`** (draft). Document `dtcsVersion` prefers `"2.0.0"`; `"1.0.0"` remains accepted.

### Features

- Operator registry (`dtcs:eq`, `dtcs:add`, `dtcs:between`, …) and profile registry (`portable-relational-kernel/1`, `portable-relational/1`, `portable-window/1`, `portable-complex-types/1`).
- Widened dataset actions (entry version `2.0.0`) with legacy parameter subsets; new `with_fields`, `rename_fields`, `drop_fields`, `distinct`, `deduplicate`, `limit`, `window`.
- Relational honesty: join kinds + `collisionPolicy` / `predicate`; union-by-name + `duplicatePolicy`; sort/`groupBy` expressions; multi-aggregate; missing ≠ null keys.
- Kernel/relational/advanced function families; ternary `between`; access ops `field`/`index`/`element_at`; `coalesce` clarified to `defined` / first-present.
- Portable plan export (`dtcs.transform-plan/1`) with registry version pins, canonical fingerprint, structured expression lowering, and security budgets.
- Window frames (rows/range), `first_value`/`last_value`, framed aggregates; datetime units + `date_trunc`/`extract`/`at_timezone` (fixed-offset); complex-types access subset.
- Differential portable fixtures + dual-path (string/structured) gate; capability accuracy validation; semantic-family conformance profiles.
- Python/CLI: `plan_export_portable`, `plan_fingerprint`, `expression_to_structured`, `capability_portable_manifest`, `dtcs export-portable`.

### SPEC

- Chapter 13 §12.1 portable serialization; Chapter 23 §5.1 semantic-family profiles; Appendix A.3–A.4 and A.8 updates.

### Non-goals

- Second production compiler in-repo; IANA/DST timezones; explode/unnest/map_entries.

### Migration notes

See [docs/user/migration-portable-relational.md](docs/user/migration-portable-relational.md) and [docs/user/migration-0.12.md](docs/user/migration-0.12.md). Existing contracts using legacy `field`/`equals` filters and name-list projects remain valid.

### Fixes and docs (included in this release)

- Runtime correctness: And/Or short-circuit; predicate-only / outer join padding; mixed int/decimal sort; typed group keys; inverted window frames; datetime time preservation; structured lowering on expression keys.
- Binding and CI alignment: Spec `2.0.0` / twelve profiles in WASM/Node/Python smokes; rustfmt; MkDocs portable-conformance nav.
- Adoption docs: canonical [versioning](docs/user/versioning.md), RTD home rewrite, no-clone `dtcs run` recipe, portable API/CLI docs, error taxonomy and diagnostic catalog.

**Release:** move tag `v0.12.0` to the commit that includes the above (crates.io / PyPI still on `0.11.0` until this tag publishes successfully).

## 0.11.0

Phase 0.11 — SPEC Completeness (full Ch 17–19 catalog, COM deepening, normative Appendix A).

### Features

- Full standard-library catalog: dataset Semantic Actions (`project`, `select`, `filter`, `aggregate`, `group`, `join`, `sort`, `union`, `partition`, `derive`) plus field transforms; functions `abs`, `min`, `max`, `contains`, `is_null`, `is_missing`; rules `one_of`, `equals`.
- SemanticAction `parameters` map threaded through validation, planning, compilation, and reference runtime.
- Lineage mappings: optional `id`, `operation` (default `dtcs:derive`), `flow` enum.
- Contract COM: `guarantees` and `compatibility` declaration fields; nested extension preservation.
- Runtime null/missing/invalid value tokens; function `nullBehavior` (`propagate` / `defined`).
- Expanded `dtcs:reference` capability profile covering the full catalog.
- Normative [SPEC Appendix A](SPEC.md#appendix-a-standard-library-catalog-normative); completeness matrix at [docs/implementation/spec-completeness.md](docs/implementation/spec-completeness.md).

### Fixed

- Dataset semantic actions (`dtcs:filter` / `dtcs:project`, …) accept interface targets; compile includes them in execution steps.
- Rule parameter type `list<string>` accepted for `dtcs:one_of`.
- Postcondition rules iterate the current target workspace row count after filters.
- Runtime JSON deserializes `{"$dtcs":"missing"|"invalid"}` as tokens (not ordinary maps).
- Joins are no longer rejected solely because input datasets have equal row counts; non-nullable schema checks reject missing/invalid cells.
- Field writes are ordered relative to same-interface dataset actions; `dtcs:derive` is rejected as a semantic action.
- Dataset action `parameters` are validated against the stdlib catalog; unresolvable compile targets hard-fail instead of being dropped.
- Conformance fixtures and source-scan inputs are embedded so `conformance run` works from packaged crates/wheels; `no-network-surface` fails closed when sources are unavailable.
- WASM npm pack no longer empties `pkg/` via wasm-pack’s `pkg/.gitignore`; binding smokes fail in CI instead of skipping green.

### Documentation

- Adoption pass: no-clone getting started, concepts/migration/glossary/cookbook, SECURITY.md, multi-language API pages, MkDocs nav restructure, maturity “Covered” wording.
- Hosted documentation via Read the Docs (`mkdocs.yml`, `.readthedocs.yaml`).

### Migration notes

Contracts and fixtures that omit lineage `operation` now deserialize with `dtcs:derive`. Authors of dataset operators must supply the documented `parameters`. Consumers of runtime JSON must treat `{"$dtcs":"missing"}` and `{"$dtcs":"invalid"}` as distinct from JSON `null`. See [docs/user/migration-0.11.md](docs/user/migration-0.11.md).

**Release:** push tag `v0.11.0` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

## 0.10.1

Patch release — test suite verification and confidence improvements (no breaking API changes).

### Tests & quality

- Plan golden structural invariants and behavioral oracles; exact plan-failure diagnostic multisets.
- Four YAML/JSON format-equivalence pairs (Rust + Python).
- Determinism double-run fixture; impure-side-effects analysis coverage.
- Conformance `RuntimeInvalid` assertion; runtime pre/postcondition failure cases.
- WASM/Node binding validate + diagnostic smoke tests in CI.
- Automated `no-network-surface` security probe; optional `cargo-mutants` workflow.

**Release:** push tag `v0.10.1` to publish to crates.io and PyPI (see [CONTRIBUTING.md](CONTRIBUTING.md#releasing)).

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
