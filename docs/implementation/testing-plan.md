# Testing Plan

Tests should be written against `SPEC.md`.

See [test-verification-report.md](test-verification-report.md) for the latest verification pass, confidence rating, and known gaps.

## How to add a test

1. **Choose a fixture** — add a YAML or JSON file under [`tests/fixtures/`](https://github.com/eddiethedean/dtcs/blob/main/tests/fixtures/) (or reuse an existing one).
2. **Register expectations** — add an entry to [`tests/fixture_expectations.json`](https://github.com/eddiethedean/dtcs/blob/main/tests/fixture_expectations.json) for parse/validation outcomes, or to phase-specific manifests (`plan_expectations.json`, `optimize_expectations.json`, `compile_expectations.json`, `capability_expectations.json`).
3. **Add integration coverage** — extend the appropriate phase test file (`tests/phase_0_*.rs`) or parametrize in [`python/tests/test_dtcs.py`](https://github.com/eddiethedean/dtcs/blob/main/python/tests/test_dtcs.py).
4. **Golden files** — for plan, optimize, or compile output, add or regenerate JSON under `tests/fixtures/plans/`, `plans_optimized/`, or `execution_plans/`. Regenerate optimize goldens with `cargo run --example write_optimize_goldens`.
5. **Run locally:**
   ```bash
   cargo test --locked
   pytest python/tests -v
   ```

## Shared fixture manifest

[`tests/fixture_expectations.json`](https://github.com/eddiethedean/dtcs/blob/main/tests/fixture_expectations.json) records expected parse and validation outcomes for the fixture corpus under [`tests/fixtures/`](https://github.com/eddiethedean/dtcs/blob/main/tests/fixtures/). Rust integration tests in [`tests/mvp.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/mvp.rs), [`tests/phase_0_2.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_2.rs), [`tests/phase_0_3.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_3.rs), [`tests/phase_0_4.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_4.rs), [`tests/phase_0_6.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_6.rs), [`tests/phase_0_7.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_7.rs), [`tests/phase_0_8.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_8.rs), [`tests/phase_0_9.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_9.rs), and [`tests/manifest.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/manifest.rs) cover behavior in detail; Python tests parametrize over the same manifest in [`python/tests/test_dtcs.py`](https://github.com/eddiethedean/dtcs/blob/main/python/tests/test_dtcs.py).

## Phase 0.5 fixture groups (standard libraries)

| Concern | Example fixtures |
|---------|------------------|
| Semantic actions | `stdlib_action_normalize_whitespace_valid.yaml`, `invalid_semantic_action.yaml` |
| Functions | `stdlib_function_concat_valid.yaml`, `stdlib_function_coalesce_valid.yaml` |
| Rules | `stdlib_rule_range_valid.yaml`, `stdlib_rule_regex_match_valid.yaml` |
| Signature errors | `stdlib_function_concat_signature_invalid.yaml`, `stdlib_function_substr_param_order_invalid.yaml` |

Covered by [`tests/fixture_expectations.json`](https://github.com/eddiethedean/dtcs/blob/main/tests/fixture_expectations.json) and validation integration tests.

## Phase 0.4 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Registry loading | `tests/fixtures/registry/vendor_catalog.yaml`, `vendor_mandatory_extension.yaml` |
| Namespace safety | `invalid_http_rule.yaml`, `invalid_http_action.yaml`, `invalid_http_type.yaml` |

Integration tests: [`tests/phase_0_4.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_4.rs).

## Phase 0.6 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Expression analysis | `expression_with_type.yaml`, `expression_precedence_*.yaml`, `analysis_constant_expr.yaml` |
| Semantic analysis | `analysis_dtcs_call_valid.yaml`, `analysis_logical_ops.yaml` |
| Invalid semantics | `analysis_duplicate_action_target.yaml`, expression/semantics error fixtures |

Integration tests: [`tests/phase_0_6.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_6.rs).

## Phase 0.3 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Compatibility (5 levels) | `tests/fixtures/compatibility/` — `identical_*`, `backward_*`, `forward_*`, `conditional_*`, `incompatible_*` |
| Evolution | `tests/fixtures/compatibility/evolution/rev1.yaml`, `rev2.yaml`, `deprecated.yaml` |
| Versioning | `invalid_version.yaml`, `version_conflict.yaml` |
| Lineage analysis | `lineage_multi.yaml` |

Integration tests: [`tests/phase_0_3.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_3.rs).

## Phase 0.7 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Plan lowering | `valid_customer.yaml`, `valid_minimal.json`, `plan_explicit_ordering.yaml` |
| Dependency graph | `lineage_multi.yaml` |
| Ambiguous action order | `analysis_duplicate_action_target.yaml` (plan invalid) |
| Golden plans | `tests/fixtures/plans/*.plan.json` via `tests/plan_expectations.json` |

Integration tests: [`tests/phase_0_7.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_7.rs).

## Phase 0.8 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Constant folding | `optimize_constant_fold.yaml` |
| Algebraic simplification | `optimize_algebraic.yaml` |
| Action fusion | `optimize_action_fusion.yaml` |
| Function inlining | `optimize_function_inline.yaml` |
| Rule deduplication | `optimize_rule_dedup.yaml`, `optimize_rule_dedup_params.yaml` |
| Dead-expression elimination | `optimize_dead_expr.yaml`, `optimize_dead_after_fold.yaml` |
| Golden optimized plans | `tests/fixtures/plans_optimized/*.plan.json` via `tests/optimize_expectations.json` |

Integration tests: [`tests/phase_0_8.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_8.rs). Python tests parametrize over `tests/optimize_expectations.json`.

## Phase 0.9 fixture groups

| Concern | Example fixtures |
|---------|------------------|
| Capability matching | `valid_customer.yaml`, `plan_field_write_chain.yaml`, vendor action rejection |
| Compilation | `tests/compile_expectations.json`, `tests/fixtures/execution_plans/*.exec.json` |
| Runtime | `tests/fixtures/runtime/customer_normalize_input.json`, `plan_field_write_chain_input.json` |
| End-to-end | `examples/customer_normalize.dtcs.yaml` |

Integration tests: [`tests/phase_0_9.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_9.rs). Manifest-driven compile goldens use [`tests/compile_expectations.json`](https://github.com/eddiethedean/dtcs/blob/main/tests/compile_expectations.json); capability and runtime I/O expectations use [`tests/capability_expectations.json`](https://github.com/eddiethedean/dtcs/blob/main/tests/capability_expectations.json).

## Phase 0.10 fixture groups

| Concern | Example fixtures / artifacts |
|---------|------------------------------|
| Conformance manifest | `tests/conformance/manifest.json`, embedded `src/conformance/manifest.json` |
| Profile coverage | `valid_customer.yaml`, `missing_lineage.yaml`, `optimize_constant_fold.yaml` |
| Security vectors | `registry/evil_dtcs_injection.yaml`, `invalid_rule_duplicate_params.json` |
| Integrated Platform E2E | `examples/customer_normalize.dtcs.yaml`, runtime I/O fixtures |

Integration tests: [`tests/phase_0_10.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/phase_0_10.rs). CI runs `cargo test --test phase_0_10` and `dtcs conformance run --profile all`.

## Phase 0.11 / 0.12 fixture groups

| Concern | Example fixtures / artifacts |
|---------|------------------------------|
| Flagship pipeline | `examples/customer_pipeline.dtcs.yaml`, `tests/fixtures/runtime/customer_pipeline_input.json` |
| Portable differential | `tests/fixtures/portable/*.json` |
| Portable integration | [`tests/portable_relational.rs`](https://github.com/eddiethedean/dtcs/blob/main/tests/portable_relational.rs) |
| Capability accuracy | `validate_capability_accuracy` / `reference_portable_manifest` unit tests |

### How to add a portable differential fixture

1. Add `tests/fixtures/portable/<id>.json` with `actions`, `input`, `expected` (or `expectError`).
2. Use the **fixture token dialect** (`$missing` / `$invalid`) — not `{"$dtcs":…}` — see [portable-conformance.md](portable-conformance.md#token-dialects).
3. Register a `portableDifferential` case in **both** `src/conformance/manifest.json` and `tests/conformance/manifest.json`.
4. Run:
   ```bash
   cargo test --test portable_relational --locked
   cargo test --test phase_0_10 --locked
   cargo run --bin dtcs -- conformance run --profile all
   ```

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
