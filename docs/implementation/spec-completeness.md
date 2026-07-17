# SPEC Completeness Matrix (0.13.0)

> **Maturity:** Spec is `3.0.0` (draft); tooling is alpha at `0.13.0`. **Covered / Full** means reference-implementation evidence against the **draft** SPEC — not that DTCS 3.0 is finalized or production-certified.

Chapter-by-chapter coverage of [`SPEC.md`](../SPEC.md) by the DTCS reference implementation at **`0.13.0`**.

## Legend

| Level | Meaning |
|-------|---------|
| **Full** | Normative requirements for that chapter are implemented in the reference crate (parse → validate → analyze → plan → optimize → match → compile → run → conformance), with registry/runtime support where applicable. |
| **Partial** | Core requirements implemented; known gaps documented (often Experimental maturity). |
| **N/A** | Intentionally out of scope for the reference toolkit. |

## Matrix

| Ch | Title | Status | Notes |
|----|-------|--------|-------|
| 1 | Introduction | Full | Principles and scope; conformance intro via `conformance/` |
| 2 | Core Concepts | Full | Pipeline stages; guarantees on COM |
| 3 | Canonical Object Model | Full | `model/`; nested extension preservation; expression `body` |
| 4 | Type System | Full | `model/types.rs`, `validation/types.rs`, `analysis/expr/` |
| 5 | Metadata | Full | `model/metadata.rs`, `metadata/` |
| 6 | Inputs and Outputs | Full | `model/interface.rs`, `validation/interfaces.rs` |
| 7 | Transformation Semantics | Full | `analysis/`, `model/semantics.rs` |
| 8 | Expression Language | Full | Structured nodes; lambdas (Ch 27); ternary `between`; `%`, `<=>` |
| 9 | Validation | Full | `validation/` seven-phase pipeline |
| 10 | Lineage | Full | `model/lineage.rs`, `lineage/`, runtime lineage |
| 11 | Compatibility | Full | `compatibility/` |
| 12 | Evolution | Full | `compatibility/` evolution analysis |
| 13 | Transformation Plan | Full | `plan/` + portable `dtcs.transform-plan/2` (v1 migrate); required `errorMode`; fingerprint pins |
| 14 | Engine Capability Model | Full | Flat profiles + portable manifests + accuracy gate (certified vs experimental) |
| 15 | Compilation | Full | `compile/` |
| 16 | Runtime | Full | Dataset actions incl. frames, joins, reshape, nested fields, IANA, seeded sample |
| 17 | Semantic Actions | Full | Widened actions + A.9 reshape/set/nested |
| 18 | Function Model | Full | Kernel + A.9 string/conversion/statistics/complex/temporal/nondet + window/2 |
| 19 | Rule Model | Full | `registry/builtin/rules.yaml`; `runtime/rules.rs`; `dtcs-regex/1` gate |
| 20 | Diagnostics | Full | Portable codes |
| 21 | Extensibility | Full | Nested extensions |
| 22 | Registries | Full | Actions/functions/operators/profiles catalogs |
| 23 | Conformance | Full *(N/A: external cert authority)* | Class + semantic-family profiles; portable differentials; plan migrate assertion |
| 24 | Security Considerations | Full | Portable plan budgets; executable-object rejection; regex/format gates |
| 25 | Versioning | Full | Registry entry version pins in portable plans |
| 26 | Governance | Full | Proposal / publication artifacts |
| 27 | Rich Portable Analytics | Partial | Reference IDs + Spec-faithful semantics + §16 family fixtures; Advanced **Experimental**; `window/2` **Candidate**; kernel/relational `/2` **Candidate** (not Standard); no dual-compiler graduation |

## Portable profiles

| Profile | Status |
|---------|--------|
| `portable-relational-kernel/1` | Covered (certified dual-path fixtures) |
| `portable-relational/1` | Covered (certified dual-path fixtures) |
| `portable-window/1` | Covered |
| `portable-complex-types/1` | Covered (access subset) |
| `portable-relational-kernel/2` | Candidate — plan v2 + fixtures; Standard graduation pending |
| `portable-relational/2` | Candidate — plan v2 + fixtures; Standard graduation pending |
| `portable-window/2` | Candidate — ntile/percent_rank/cume_dist/nth_value + fixtures |
| A.9 Advanced families (`string-advanced`, `conversion`, `statistics`, `complex-values`, `reshape`, `relational-extended`, `temporal-iana`, `nondeterministic`) | Experimental reference surface + per-family fixtures |

## Explicit non-goals

- Second production SQL/DataFrame engine inside this repo
- Standard graduation of Experimental A.9 profiles (needs two independent compilers)
- Expanding WASM/Node beyond parse / validate / conformanceDeclare

See [migration-0.13.md](../user/migration-0.13.md), [portable-conformance.md](portable-conformance.md), and [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) phase 0.13.
