# SPEC Completeness Matrix (0.12.0)

> **Maturity:** Spec is `2.0.0`; tooling is alpha. **Covered / Full** in this matrix means reference-implementation evidence against the **draft** SPEC — not that DTCS 1.0 is finalized.

Chapter-by-chapter coverage of [`SPEC.md`](../SPEC.md) by the DTCS reference implementation at **`0.12.0`** (Portable Relational Profile phase 0.12).

## Legend

| Level | Meaning |
|-------|---------|
| **Full** | Normative requirements for that chapter are implemented in the reference crate (parse → validate → analyze → plan → optimize → match → compile → run → conformance), with registry/runtime support where applicable. |
| **Partial** | Core requirements implemented; known gaps documented. |
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
| 8 | Expression Language | Full | Structured nodes §3.1; ternary `between`; `%`, `<=>` |
| 9 | Validation | Full | `validation/` seven-phase pipeline; legacy/rich action param subsets |
| 10 | Lineage | Full | `model/lineage.rs`, `lineage/`, runtime lineage |
| 11 | Compatibility | Full | `compatibility/` |
| 12 | Evolution | Full | `compatibility/` evolution analysis |
| 13 | Transformation Plan | Full | `plan/` + portable envelope `dtcs.transform-plan/1` |
| 14 | Engine Capability Model | Full | Flat profiles + portable manifests + accuracy gate |
| 15 | Compilation | Full | `compile/` |
| 16 | Runtime | Full | Dataset actions incl. frames, join policies, complex access, datetime units |
| 17 | Semantic Actions | Full | Widened v2 actions + window frames |
| 18 | Function Model | Full | Kernel + aggregate + datetime + window (`first_value`/`last_value`) |
| 19 | Rule Model | Full | `registry/builtin/rules.yaml`; `runtime/rules.rs` |
| 20 | Diagnostics | Full | Portable codes (`plan-budget-exceeded`, …) |
| 21 | Extensibility | Full | Nested extensions |
| 22 | Registries | Full | Actions/functions/operators/profiles catalogs |
| 23 | Conformance | Full *(N/A: external cert authority)* | Class + semantic-family profiles; differential fixtures; dual-path + external process doc |
| 24 | Security Considerations | Full | Portable plan budgets; executable-object rejection |
| 25 | Versioning | Full | Registry entry version pins in portable plans |
| 26 | Governance | Full | Proposal accepted; publication artifacts |

## Portable profiles

| Profile | Status |
|---------|--------|
| `dtcs:profile/portable-relational-kernel/1` | Covered |
| `dtcs:profile/portable-relational/1` | Covered |
| `dtcs:profile/portable-window/1` | Covered (rows/range frames; ranking/offset/first/last; framed aggs) |
| `dtcs:profile/portable-complex-types/1` | Covered for access subset (`field`/`index`/`element_at`); explode/unnest non-goal |

## Explicit non-goals

- Second production SQL/DataFrame engine inside this repo
- IANA timezone database / DST calendar algebra
- Nested explode/unnest / full complex DDL

See [migration-portable-relational.md](../user/migration-portable-relational.md), [portable-conformance.md](portable-conformance.md), and [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) phase 0.12.
