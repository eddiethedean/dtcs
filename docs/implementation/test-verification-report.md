# DTCS Test Suite Verification Report

Date: 2026-07-12  
Scope: P0 + P1 + P2 + P3 (finish plan)

## Executive Summary

**Overall confidence: High**

The suite now validates behavior against SPEC-derived expectations across diagnostics, optimizer runtime preservation, plan structural/behavioral oracles, runtime failure codes, YAML↔JSON equivalence (4 pairs), determinism double-run, expanded conformance coverage (20 cases + `RuntimeInvalid`), capability compile negatives, binding validate/diagnostic parity, and an automated `no-network-surface` security probe. Mutation testing runs in a non-blocking CI job.

Verification commands executed successfully:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --locked`
- `pytest python/tests -v` (232 passed)
- `bindings/wasm` and `bindings/node` smoke tests (`npm test`)
- `./scripts/check-docs.sh`

---

## P0+P1 (landed in `768b09d`)

Exact diagnostic multisets, optimizer runtime I/O parity, runtime failure integration tests, initial YAML↔JSON pair, conformance expansion, and Python mirror updates. See git history for full file list.

---

## P2 Additions

| Area | Change |
|------|--------|
| Plan goldens | Exact plan-failure codes (3 fixtures); structural invariants for all 40 valid goldens; behavioral oracles (runtime I/O, lineage counts, rule steps, node ids) on 10+ fixtures |
| Format equivalence | 4 YAML/JSON pairs through validate/plan/optimize (`format_equivalence.rs` + Python parametrized tests) |
| Determinism | `deterministic_double_run.yaml` double-run byte-equal outputs; `impure_side_effects_invalid.yaml` analysis finding |
| Conformance | `RuntimeInvalid` assertion kind; `runtime-precondition-fail`, `runtime-postcondition-fail`, `runtime-deterministic-double-run`, `optimize-equivalent-action-fusion` |
| Capability | Manifest-driven unsupported-action negative (`mutate_unsupported_action`) |
| Phase dedup | Consolidated manifest exact-code tests in `phase_0_2`, `phase_0_3`, `phase_0_4` |

Plan goldens remain byte-identical change detectors; structural invariants and behavioral oracles are the independent semantic layer (same pattern as optimize goldens in `phase_0_8`).

---

## P3 Additions

| Area | Change |
|------|--------|
| Bindings | WASM + Node smoke: `validateContract`, exact `missing_lineage` codes, 8 conformance profiles; wired in CI |
| Security | `no-network-surface` scans `src/runtime`, `src/validation`, `src/parser`, `src/conformance` for forbidden network imports |
| Mutation | `.github/workflows/mutation.yml` — scoped `cargo-mutants` on validation/optimize/runtime (non-blocking) |

---

## Confidence Assessment

| Layer | Rating |
|-------|--------|
| Validation diagnostics | **High** (exact multiset, manifest-driven) |
| Optimizer semantics | **High** (runtime I/O on all 8 fixtures + adversarial fusion) |
| Plan semantics | **High** (invariants + 10+ behavioral oracles; goldens as change detectors) |
| Runtime failures | **High** (integration + conformance `RuntimeInvalid`) |
| Format equivalence | **High** (4 pairs, Rust + Python) |
| Conformance | **High** (20 cases, per-id assertions, security probes automated) |
| Analysis / determinism | **Moderate–High** (impure analysis; double-run fixture) |
| Bindings | **Moderate** (validate + diagnostic smoke; not full plan/optimize/run parity) |
| Stdlib catalog | **Moderate** (starter catalog only — see README) |
| **Overall** | **High** |

The suite would now fail if: diagnostic multisets drift, optimize changes runtime output, plan invariants break, pre/postconditions stop enforcing, format pairs diverge, conformance cases regress, bindings lose validate/diagnostic parity, or network client code appears in core paths.

---

## Remaining gaps (documented, not blocking High)

1. **Full Ch 17–19 stdlib** — starter catalog only; ROADMAP/README state this explicitly
2. **Binding depth** — smoke parity only; no plan/optimize/run through WASM/Node
3. **Mutation gate** — job is informational until mutant kill rate stabilizes
4. **Phase-specific subset checks** — some remain where they assert extra behavior (message substrings, registry merge logic) beyond manifest codes

---

## Key files

- `tests/plan_expectations.json`, `tests/phase_0_7.rs` — plan oracles
- `tests/format_equivalence.rs`, `python/tests/test_dtcs.py` — format pairs
- `tests/determinism.rs` — double-run + impure analysis
- `tests/conformance/manifest.json`, `src/conformance/model.rs`, `src/conformance/runner.rs` — `RuntimeInvalid`
- `tests/capability_expectations.json`, `tests/phase_0_9.rs` — capability negative
- `src/conformance/security.rs` — automated network probe
- `bindings/wasm/test/smoke.test.mjs`, `bindings/node/test/smoke.test.mjs`
- `.github/workflows/mutation.yml`, `.github/workflows/checks.yml`

See also [testing-plan.md](testing-plan.md) for how to add tests.
