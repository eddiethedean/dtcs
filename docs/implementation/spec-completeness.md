# SPEC Completeness Matrix (0.15.0)

> **Maturity:** Spec is `2.0.0`; tooling is alpha. **Covered / Full** in this matrix means reference-implementation evidence against the **draft** SPEC — not that DTCS 1.0 is finalized.

Chapter-by-chapter coverage of [`SPEC.md`](../SPEC.md) by the DTCS reference implementation at **`0.15.0`** (includes Portable Relational Profile phases 0.12–0.15).

Coverage ratings:

| Rating | Meaning |
|--------|---------|
| **Full** | Normative requirements for that chapter are implemented in the reference crate (parse → validate → analyze → plan → optimize → match → compile → run → conformance), with registry/runtime support where applicable. |
| **N/A** | Intentionally out of scope for the reference implementation (see [non-goals.md](non-goals.md) and SPEC Ch 1 §3 / Ch 23 §13). |

## Matrix

| Ch | Title | Coverage | Evidence (`src/`) |
|----|-------|----------|-------------------|
| 1 | Introduction | Full | Principles and scope; conformance intro via `conformance/` |
| 2 | Core Concepts | Full | Pipeline stages; guarantees on COM |
| 3 | Canonical Object Model | Full | `model/`; nested extension preservation; expression `body` |
| 4 | Type System | Full | `model/types.rs`, `validation/types.rs`, `analysis/expr/` |
| 5 | Metadata | Full | `model/metadata.rs`, `metadata/` |
| 6 | Inputs and Outputs | Full | `model/interface.rs`, `validation/interfaces.rs` |
| 7 | Transformation Semantics | Full | `analysis/`, `model/semantics.rs` |
| 8 | Expression Language | Full | Structured nodes §3.1 (`body` + `to_structured_node`); string `expr` sugar; operators incl. `%`, `<=>` |
| 9 | Validation | Full | `validation/` seven-phase pipeline; legacy/rich action param subsets |
| 10 | Lineage | Full | `model/lineage.rs`, `lineage/`, runtime lineage |
| 11 | Compatibility | Full | `compatibility/` |
| 12 | Evolution | Full | `compatibility/` evolution analysis |
| 13 | Transformation Plan | Full | `plan/` + portable envelope `dtcs.transform-plan/1` (`plan/portable.rs`) |
| 14 | Engine Capability Model | Full | Flat profiles + portable per-entry manifests (`capability/portable.rs`) |
| 15 | Compilation | Full | `compile/` |
| 16 | Runtime | Full | Dataset actions incl. multi-agg, window, field shaping; datetime functions |
| 17 | Semantic Actions | Full | Widened v2 actions + `with_fields`/`distinct`/`limit`/`window` |
| 18 | Function Model | Full | Kernel + aggregate + datetime + window families |
| 19 | Rule Model | Full | `registry/builtin/rules.yaml`; `runtime/rules.rs` |
| 20 | Diagnostics | Full | Portable codes (`plan-budget-exceeded`, …) |
| 21 | Extensibility | Full | Nested extensions |
| 22 | Registries | Full | Actions/functions/operators/profiles catalogs |
| 23 | Conformance | Full *(N/A: external cert authority)* | Class + semantic-family profiles; dual-compiler gate text |
| 24 | Security Considerations | Full | Portable plan budgets; executable-object rejection |
| 25 | Versioning | Full | Registry entry version pins in portable plans |
| 26 | Governance | Full | Proposal accepted; publication artifacts |

## Portable Relational notes

| Item | Status |
|------|--------|
| `dtcs:profile/portable-relational-kernel/1` | Covered |
| `dtcs:profile/portable-relational/1` | Covered |
| `dtcs:profile/portable-window/1` | Covered (reference runtime implements partition/order/row_number/rank/lag/lead) |
| `dtcs:profile/portable-complex-types/1` | Profile + type aliases; complex-value ops remain analysis-oriented |
| Dual independent compilers for Stable promotion | SPEC gate text ready; second engine remains external |

## Intentional non-goals (N/A)

| Item | SPEC basis |
|------|------------|
| Production ETL orchestration | Ch 1 §3 |
| Polars / Spark / SQL backends | Ch 1 §3 |
| External certification authority | Ch 23 §13 |

See [migration-portable-relational.md](../user/migration-portable-relational.md) and [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) phases 0.12–0.15.

## Related

- Normative catalog: [SPEC Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative) / A.8
- Proposal: [DTCS_PORTABLE_SPEC_PROPOSAL.md](../DTCS_PORTABLE_SPEC_PROPOSAL.md) (Accepted)
- Release notes: [CHANGELOG.md](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md)
