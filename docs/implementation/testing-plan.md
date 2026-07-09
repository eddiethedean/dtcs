# Testing Plan

Tests should be written against `SPEC.md`.

## Shared fixture manifest

[`tests/fixture_expectations.json`](../../tests/fixture_expectations.json) records expected parse and validation outcomes for the fixture corpus under [`tests/fixtures/`](../../tests/fixtures/). Rust integration tests in [`tests/mvp.rs`](../../tests/mvp.rs), [`tests/phase_0_2.rs`](../../tests/phase_0_2.rs), [`tests/phase_0_3.rs`](../../tests/phase_0_3.rs), and [`tests/manifest.rs`](../../tests/manifest.rs) cover behavior in detail; Python tests parametrize over the same manifest in [`python/tests/test_dtcs.py`](../../python/tests/test_dtcs.py).

## Phase 0.3 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Compatibility (5 levels) | `tests/fixtures/compatibility/` — `identical_*`, `backward_*`, `forward_*`, `conditional_*`, `incompatible_*` |
| Evolution | `tests/fixtures/compatibility/evolution/rev1.yaml`, `rev2.yaml`, `deprecated.yaml` |
| Versioning | `invalid_version.yaml`, `version_conflict.yaml` |
| Lineage analysis | `lineage_multi.yaml` |

Integration tests: [`tests/phase_0_3.rs`](../../tests/phase_0_3.rs).

## Phase 0.7 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Plan lowering | `valid_customer.yaml`, `valid_minimal.json`, `plan_explicit_ordering.yaml` |
| Dependency graph | `lineage_multi.yaml` |
| Ambiguous action order | `analysis_duplicate_action_target.yaml` (plan invalid) |
| Golden plans | `tests/fixtures/plans/*.plan.json` via `tests/plan_expectations.json` |

Integration tests: [`tests/phase_0_7.rs`](../../tests/phase_0_7.rs).

## Phase 0.2 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Metadata | `valid_metadata.yaml`, `invalid_metadata_timestamp.yaml`, `invalid_metadata_impossible_date.yaml` |
| Types | `map_type_valid.yaml`, `nested_collection_valid.yaml`, `valid_conversion_lossy.yaml`, `invalid_type_trailing_garbage.yaml` |
| Expressions | `expression_with_type.yaml`, `expression_missing_type.yaml`, `expression_type_mismatch.yaml` |
| Interfaces | `optional_input.yaml`, `input_precondition.yaml`, `invalid_io_extension.yaml`, `streaming_unbounded.yaml` |
| Registry | `tests/fixtures/registry/`, `tests/phase_0_4.rs` |
| Namespace safety | `invalid_http_rule.yaml`, `invalid_http_action.yaml`, `invalid_http_type.yaml` |

## Required test categories

- Parse valid YAML and JSON
- Reject malformed documents
- Reject missing required fields and duplicate identifiers
- Validate logical types, metadata, semantic actions, rules, and expressions
- Preserve extensions
- Generate deterministic diagnostics
- CLI and Python binding parity

Add snapshot tests for diagnostics where useful.
