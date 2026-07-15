# SPEC Completeness Matrix (0.11.0)

Chapter-by-chapter coverage of [`SPEC.md`](../SPEC.md) by the DTCS reference implementation at **`0.11.0`**.

Coverage ratings:

| Rating | Meaning |
|--------|---------|
| **Full** | Normative requirements for that chapter are implemented in the reference crate (parse → validate → analyze → plan → optimize → match → compile → run → conformance), with registry/runtime support where applicable. |
| **N/A** | Intentionally out of scope for the reference implementation (see [non-goals.md](non-goals.md) and SPEC Ch 1 §3 / Ch 23 §13). |

The starter→full standard-library catalog for **Chapters 17–19** was completed in **0.11.0** (dataset operators, additional functions/rules, and normative catalog appendix). See [Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative) in `SPEC.md`.

## Matrix

| Ch | Title | Coverage | Evidence (`src/`) |
|----|-------|----------|-------------------|
| 1 | Introduction | Full | Principles and scope reflected in crate docs; conformance intro wired via `conformance/` |
| 2 | Core Concepts | Full | Pipeline stages across `parser/`, `validation/`, `plan/`, `compile/`, `runtime/`, `conformance/`; guarantees on COM (`model/guarantees.rs`) |
| 3 | Canonical Object Model | Full | `model/` (`contract.rs`, composition, extensions); nested extension preservation |
| 4 | Type System | Full | `model/types.rs`, `validation/types.rs`, expression typing in `analysis/expr/` — deepened by 0.11 COM nullability / conversion alignment |
| 5 | Metadata | Full | `model/metadata.rs`, `metadata/` validation |
| 6 | Inputs and Outputs | Full | `model/interface.rs`, `validation/interfaces.rs` — multiplicity, optional inputs, streaming, pre/postconditions |
| 7 | Transformation Semantics | Full | `analysis/`, `model/semantics.rs`, contract `guarantees` |
| 8 | Expression Language | Full | `analysis/expr/`, `model/expression.rs`, `model/null_behavior.rs`, runtime eval in `runtime/expr.rs` — null/missing/invalid tokens |
| 9 | Validation | Full | `validation/` seven-phase pipeline; stage codes in `diagnostics/` |
| 10 | Lineage | Full | `model/lineage.rs` (`operation` default `dtcs:derive`, `flow` enum), `validation/lineage.rs`, `lineage/`, runtime lineage in `runtime/lineage.rs` — deepened by 0.11 |
| 11 | Compatibility | Full | `compatibility/`; contract-level declaration in `model/compatibility_decl.rs` |
| 12 | Evolution | Full | `compatibility/` evolution analysis — deepened by 0.11 COM fields |
| 13 | Transformation Plan | Full | `plan/` (`lowering.rs`, `optimize.rs`, graph, validate) |
| 14 | Engine Capability Model | Full | `capability/` including expanded `dtcs:reference` profile for full stdlib — deepened by 0.11 |
| 15 | Compilation | Full | `compile/` (`ExecutionPlan`, `ReferenceCompiler`) — deepened by 0.11 action parameters |
| 16 | Runtime | Full | `runtime/` (`actions.rs`, `functions.rs`, `rules.rs`, `reference.rs`) — full catalog execution; null/missing/invalid — deepened by 0.11 |
| 17 | Semantic Actions | Full | `registry/builtin/semantic_actions.yaml`; extended field + dataset operators; `model/action.rs` parameters; `runtime/actions.rs` — **starter→full in 0.11.0** |
| 18 | Function Model | Full | `registry/builtin/functions.yaml`; `model/function.rs`; `runtime/functions.rs` — **starter→full in 0.11.0** (`abs`, `min`, `max`, `contains`, `is_null`, `is_missing`) |
| 19 | Rule Model | Full | `registry/builtin/rules.yaml`; `model/rule.rs`; `runtime/rules.rs` — **starter→full in 0.11.0** (`one_of`, `equals`) |
| 20 | Diagnostics | Full | `diagnostics/` codes and stages across the pipeline |
| 21 | Extensibility | Full | `model/extension.rs`, `validation/extensions.rs`; nested extension handling |
| 22 | Registries | Full | `registry/` load/cache/builtin catalogs |
| 23 | Conformance | Full *(N/A: external cert authority)* | `conformance/` profiles and offline suite; external certification authority remains N/A per Ch 23 §13 |
| 24 | Security Considerations | Full | Automated probes in `conformance/security.rs`; checklist docs |
| 25 | Versioning | Full | `model/versioning.rs`, version validation in analysis path |
| 26 | Governance | Full | Process alignment in CONTRIBUTING / release workflows; publication artifacts |

## Intentional non-goals (N/A)

These SPEC-adjacent items are **not** claimed as reference-implementation responsibilities:

| Item | SPEC basis |
|------|------------|
| Production ETL orchestration | Ch 1 §3 |
| Polars / Spark / SQL backends | Ch 1 §3 |
| External certification authority | Ch 23 §13 |

See [non-goals.md](non-goals.md) and [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) Phase 0.11.

## Related

- Normative catalog identifiers: [SPEC Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative)
- Phase history: [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md)
- Release notes: [CHANGELOG.md](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md)
