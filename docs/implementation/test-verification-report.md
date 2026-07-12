# DTCS Test Suite Verification Report

Date: 2026-07-12  
Scope: P0 + P1 (attached plan)

## Executive Summary

**Overall confidence: Moderate**

The suite now validates behavior against SPEC-derived expectations in the highest-risk areas: exact diagnostic multisets, optimizer runtime preservation, runtime failure codes, YAML↔JSON equivalence, expanded conformance coverage, and analysis/runtime negative paths. Confidence is not **High** because plan goldens (40+) and optimize goldens remain implementation-dumped change detectors, Node/WASM binding coverage is still thin, and full Ch 17–19 stdlib catalog honesty is documented but not exhaustively proven.

Verification commands executed successfully:

- `cargo fmt --all -- --check` (after auto-format)
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --locked` (207 integration + 8 unit tests)
- `pytest python/tests -v` (220 passed)
- `./scripts/check-docs.sh`

---

## Incorrect Tests

No tests were found that **asserted wrong SPEC behavior**. The primary issue was **under-specified oracles** (subset diagnostic checks, vacuous `transforms_min`, circular `equivalent()` as sole optimizer proof) that allowed incorrect implementations to pass.

**Shared hallucination surfaced:** 56/67 invalid fixtures listed only a primary diagnostic code while the validator correctly emitted cascaded codes (`dtcs:ambiguous-reference`, `dtcs:missing-lineage`, etc.). Tests and expectations agreed on the subset but both understated real behavior.

---

## Weak Tests (addressed)

| Area | Weakness | Fix |
|------|----------|-----|
| `fixture_expectations.json` + manifest | Code subset checks | Exact sorted multiset for all 68 invalid fixtures |
| `optimize_expectations.json` | `transforms_min: 1`, goldens as oracle | Runtime I/O before/after optimize; expected output for fusion |
| `phase_0_8` | `equivalent()` only | `execute_plan` on original vs optimized for all 8 fixtures |
| `phase_0_9` | `is_err()` without messages | Error substring checks; exact runtime codes |
| `phase_0_6` | 2 analysis tests | 5 tests with finding kinds / exact codes |
| `phase_0_10` / conformance | Non-empty report only | Per-case IDs + all 5 security probes |
| `mvp` CLI | `contains("valid")` | `--json` with `valid` + no error-severity diagnostics |
| Python mirror | Subset codes, weak optimize | Exact multiset + runtime I/O equivalence |

---

## Tests Rewritten

1. **`tests/manifest.rs`** — exact diagnostic multiset via `tests/common/mod.rs`
2. **`tests/optimize_expectations.json` + `phase_0_8.rs`** — runtime inputs per fixture; adversarial fusion output assertion
3. **`tests/phase_0_6.rs`** — full analysis fixture matrix
4. **`tests/phase_0_9.rs`** — runtime pre/post/invalid-input integration; lineage chain; CLI JSON run
5. **`tests/phase_0_10.rs`** — individual conformance case assertions
6. **`python/tests/test_dtcs.py`** — exact codes + optimize runtime parity
7. **`tests/mvp.rs`** — validate CLI JSON assertions
8. **`src/conformance/runner.rs`** — exact multiset for `validateInvalid`

---

## Tests Added

| Test / fixture | Why it increases confidence |
|----------------|----------------------------|
| `tests/format_equivalence.rs` | SPEC Ch 3: YAML↔JSON COM/plan/optimize parity |
| `runtime_precondition_fail.yaml` + test | Exact `dtcs:precondition-violation` |
| `runtime_postcondition_fail.yaml` + test | Exact `dtcs:postcondition-violation` |
| `invalid_metadata_policy_uri.yaml` | Audit leftover: invalid governance policy URI |
| `valid_minimal.yaml` | Paired with JSON for format equivalence |
| Optimize runtime inputs (8 JSON files) | Independent optimizer oracle via execution |
| `optimize_action_fusion_preserves_lowercasing_behavior` | Adversarial: broken fusion would leave uppercase email |
| Conformance: invalid type, policy URI, field-write runtime, security probes | Broader Ch 23–24 coverage |
| `compile_rejects_cyclic_plan_with_exact_diagnostics` | Exact `dtcs:cyclic-dependency` |
| `lineage_preserved_through_optimize_compile_and_run` | Ch 10 lineage through pipeline |
| `bindings/node/test/smoke.test.mjs` | Restores broken Node test script |

---

## Tests Removed

None. Redundant subset checks were **tightened**, not deleted, to preserve fixture coverage while strengthening oracles.

---

## Missing Coverage (highest priority remaining)

1. **Plan goldens (40 files)** — still byte-identical to implementation output; need SPEC-derived spot audits or property tests
2. **Determinism/purity declared semantics** — no fixtures with `deterministic:` / `pure:` flags + double-run checks
3. **Full Ch 17–19 stdlib** — starter set only; do not claim full catalog conformance
4. **WASM/Node depth** — smoke tests only; no plan/optimize/run binding parity
5. **Mutation testing** — no automated mutator in CI
6. **`no-network-surface` probe** — still a hardcoded pass (documented manual review)

---

## AI Failure Patterns Found

| Pattern | Evidence | Mitigation applied |
|---------|----------|-------------------|
| Shared hallucination | Diagnostic subsets matched incomplete expectations | Exact multisets from independent CLI oracle pass |
| Circular validation | `equivalent()` + dumped optimize goldens | Runtime I/O equivalence layer |
| Assertion weakness | `transforms_min: 1`, `contains("valid")`, `is_err()` | Structured JSON + exact codes |
| Copy-paste manifests | Rust/Python subset mirrors | Both updated to exact multiset |
| False confidence | Profile fan-out on 11 conformance cases | 17 cases + per-id assertions |
| Thin negative runtime | Codes existed, no integration tests | 3 runtime failure integration tests |

---

## Confidence Assessment

| Layer | Before | After |
|-------|--------|-------|
| Validation diagnostics | Low (subset) | **Moderate–High** (exact multiset) |
| Optimizer semantics | Low (circular) | **Moderate** (runtime I/O + fusion adversarial) |
| Runtime failures | Low | **Moderate** (3 exact-code integration tests) |
| Format equivalence | None | **Moderate** (minimal YAML/JSON pair) |
| Conformance | Low (smoke) | **Moderate** (17 cases, security probes wired) |
| Analysis | Low (2 tests) | **Moderate** (5 fixture-level tests) |
| Overall | Low–Moderate | **Moderate** |

The suite would now fail if: extra wrong diagnostics appear, optimize changes runtime output, pre/postconditions stop enforcing, invalid policy URIs validate, or conformance cases regress individually.

---

## Files touched (summary)

- `tests/fixture_expectations.json` — exact codes for all invalid fixtures
- `tests/common/mod.rs`, `tests/manifest.rs`, `tests/format_equivalence.rs`
- `tests/phase_0_{6,8,9,10}.rs`, `tests/mvp.rs`
- `tests/optimize_expectations.json`, `tests/conformance/manifest.json`, `src/conformance/manifest.json`, `src/conformance/runner.rs`
- New fixtures: policy URI, runtime failures, valid_minimal.yaml, optimize runtime I/O
- `python/tests/test_dtcs.py`, `bindings/node/test/smoke.test.mjs`
