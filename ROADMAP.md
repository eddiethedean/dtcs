# DTCS Roadmap

Reference-implementation milestones for the Data Transformation Contract Standard. Uses **0.X** release phases only and covers **all 26 chapters** of [`SPEC.md`](SPEC.md).

> **Maturity:** Spec is `2.0.0` (draft). Tooling is alpha. Status **Covered** below means the **reference implementation exercises that draft SPEC area** for the listed release — not that DTCS 2.0 is finalized or production-certified.

[`SPEC.md`](SPEC.md) is the source of truth. When this roadmap and the specification disagree, the specification wins.

**Citation format:** `Ch N §M` — chapter *N*, section *M* in [`SPEC.md`](SPEC.md).

**Distribution** items marked *(distribution)* support adoption but are not normative SPEC requirements.

---

## Tiers

Phases are grouped into six tiers that follow the [Ch 2 §13](SPEC.md#chapter-2-core-concepts) pipeline.

| Tier | Phases | Pipeline stage |
|------|--------|----------------|
| **I — Foundation** | [0.1](#phase-01-foundation) | Document → COM → Validator → Diagnostics |
| **II — Contract** | [0.2](#phase-02-contract-model) · [0.3](#phase-03-contract-analysis) | Complete contract representation and analysis |
| **III — Semantic** | [0.4](#phase-04-registries-extensibility) · [0.5](#phase-05-standard-libraries) · [0.6](#phase-06-semantic-analysis) | Identifiers, libraries, static semantics |
| **IV — Planning & execution** | [0.7](#phase-07-transformation-plan) · [0.8](#phase-08-plan-optimization) · [0.9](#phase-09-execution-pipeline) | Plan → optimize → compile → run |
| **V — Certification** | [0.10](#phase-010-conformance-ecosystem) | Conformance, security, governance |
| **VI — SPEC completeness** | [0.11](#phase-011-spec-completeness) | Full Ch 17–19 catalog; COM depth; completeness matrix |
| **VII — Portable Relational** | [0.12](#phase-012-portable-relational-profile) | Portable Relational Profile (DTCS-R1–R4) |

## Status overview

| Phase | Name | SPEC focus | Reference impl coverage |
|-------|------|------------|-------------------------|
| **0.1** | [Foundation](#phase-01-foundation) | Ch 1–3, 9–10, 17–20 (core) | **Covered** (`0.1.2`) |
| **0.2** | [Contract Model](#phase-02-contract-model) | Ch 4–6 | **Covered** (`0.2.0`) |
| **0.3** | [Contract Analysis](#phase-03-contract-analysis) | Ch 10 §11–12, 11–12, 25 | **Covered** (`0.3.0`) |
| **0.4** | [Registries & Extensibility](#phase-04-registries-extensibility) | Ch 21–22 | **Covered** (`0.4.0`) |
| **0.5** | [Standard Libraries](#phase-05-standard-libraries) | Ch 17–19 | **Covered** (`0.5.0` starter; full catalog in 0.11) |
| **0.6** | [Semantic Analysis](#phase-06-semantic-analysis) | Ch 7–8 | **Covered** (`0.6.0`) |
| **0.7** | [Transformation Plan](#phase-07-transformation-plan) | Ch 13 (lowering) | **Covered** (`0.7.0`) |
| **0.8** | [Plan Optimization](#phase-08-plan-optimization) | Ch 13 §9, 8 §14, 15 §9, 17–19 §11 | **Covered** (`0.8.0`) |
| **0.9** | [Execution Pipeline](#phase-09-execution-pipeline) | Ch 14–16 | **Covered** (`0.9.0`) |
| **0.10** | [Conformance & Ecosystem](#phase-010-conformance-ecosystem) | Ch 1 §10, 2 §14, 23–24, 26 | **Covered** (`0.10.1`) |
| **0.11** | [SPEC Completeness](#phase-011-spec-completeness) | Ch 4–8, 10, 12, 14–19 (deepening); Appendix A | **Covered** (`0.11.0`) |
| **0.12** | [Portable Relational Profile](#phase-012-portable-relational-profile) | Ch 8, 13 §12.1, 14, 16–18, 23 §5.1, A.3–A.4, A.8 | **Covered** (`0.12.0`) |

## SPEC chapter index

| Ch | Title | Phase | Coverage |
|----|-------|-------|----------|
| 1 | [Introduction](SPEC.md#chapter-1-introduction) | 0.1, 0.10 | Covered — principles (0.1); conformance intro (0.10) |
| 2 | [Core Concepts](SPEC.md#chapter-2-core-concepts) | 0.1, 0.10, 0.11 | Covered — vocabulary (0.1); conformance (0.10); guarantees COM (0.11) |
| 3 | [Canonical Object Model](SPEC.md#chapter-3-canonical-object-model) | 0.1, 0.11 | Covered — deepened by 0.11 (guarantees, compatibility decl, nested extensions) |
| 4 | [Type System](SPEC.md#chapter-4-type-system) | 0.1, 0.2, 0.11 | Covered — deepened by 0.11 |
| 5 | [Metadata](SPEC.md#chapter-5-metadata) | 0.2 | Covered |
| 6 | [Inputs and Outputs](SPEC.md#chapter-6-inputs-and-outputs) | 0.1, 0.2, 0.11 | Covered — deepened by 0.11 |
| 7 | [Transformation Semantics](SPEC.md#chapter-7-transformation-semantics) | 0.6, 0.11 | Covered — deepened by 0.11 guarantees |
| 8 | [Expression Language](SPEC.md#chapter-8-expression-language) | 0.6, 0.8, 0.11 | Covered — deepened by 0.11 null/missing/invalid semantics |
| 9 | [Validation](SPEC.md#chapter-9-validation) | 0.1 | Full; deepened by 0.2, 0.6, and 0.11 |
| 10 | [Lineage](SPEC.md#chapter-10-lineage) | 0.1, 0.3, 0.11 | Covered — deepened by 0.11 (`operation`, `flow`) |
| 11 | [Compatibility](SPEC.md#chapter-11-compatibility) | 0.3, 0.11 | Covered — deepened by 0.11 compatibility declaration |
| 12 | [Evolution](SPEC.md#chapter-12-evolution) | 0.3, 0.11 | Covered — deepened by 0.11 |
| 13 | [Transformation Plan](SPEC.md#chapter-13-transformation-plan) | 0.7, 0.8 | Covered |
| 14 | [Engine Capability Model](SPEC.md#chapter-14-engine-capability-model) | 0.9, 0.11 | Covered — deepened by 0.11 full-catalog profile |
| 15 | [Compilation](SPEC.md#chapter-15-compilation) | 0.8, 0.9, 0.11 | Covered — deepened by 0.11 action parameters |
| 16 | [Runtime](SPEC.md#chapter-16-runtime) | 0.9, 0.11 | Covered — deepened by 0.11 full catalog + null tokens |
| 17 | [Semantic Actions](SPEC.md#chapter-17-semantic-actions) | 0.5, 0.8, 0.11 | Covered — starter (0.5); optimization (0.8); full catalog (0.11) |
| 18 | [Function Model](SPEC.md#chapter-18-function-model) | 0.5, 0.8, 0.11 | Covered — starter (0.5); optimization (0.8); full catalog (0.11) |
| 19 | [Rule Model](SPEC.md#chapter-19-rule-model) | 0.5, 0.8, 0.11 | Covered — starter (0.5); optimization (0.8); full catalog (0.11) |
| 20 | [Diagnostics](SPEC.md#chapter-20-diagnostics) | 0.1 | Full; stage codes added per phase |
| 21 | [Extensibility](SPEC.md#chapter-21-extensibility) | 0.4, 0.11 | Covered — deepened by 0.11 nested extensions |
| 22 | [Registries](SPEC.md#chapter-22-registries) | 0.4, 0.11 | Covered — deepened by 0.11 builtin catalogs |
| 23 | [Conformance](SPEC.md#chapter-23-conformance) | 0.10 | Full *(N/A: external cert authority)* |
| 24 | [Security Considerations](SPEC.md#chapter-24-security-considerations) | 0.10 | Covered |
| 25 | [Versioning](SPEC.md#chapter-25-versioning) | 0.3 | Covered |
| 26 | [Governance](SPEC.md#chapter-26-governance) | 0.10 | Covered |

Chapter-level evidence matrix: [`docs/implementation/spec-completeness.md`](docs/implementation/spec-completeness.md). Normative identifier catalog: [SPEC Appendix A](SPEC.md#appendix-a-standard-library-catalog-normative).

## Dependencies

```text
Tier I     0.1 Foundation
              │
Tier II       ├──► 0.2 Contract Model
              │         │
              │         └──► 0.3 Contract Analysis
              │                    │
Tier III      │                    ├──► 0.4 Registries & Extensibility
              │                    │         │
              │                    │         └──► 0.5 Standard Libraries
              │                    │                   │
              │                    │                   └──► 0.6 Semantic Analysis
              │                    │                             │
Tier IV       │                    │                             └──► 0.7 Transformation Plan
              │                    │                                       │
              │                    │                                       └──► 0.8 Plan Optimization
              │                    │                                                 │
              │                    │                                                 └──► 0.9 Execution Pipeline
              │                    │                                                           │
Tier V        │                    │                                                           └──► 0.10 Conformance
              │                    │                                                                     │
Tier VI       │                    │                                                                     └──► 0.11 SPEC Completeness
```

**Parallel work:** 0.4 and 0.3 can start after 0.2 (registries do not require compatibility engine). 0.6 requires 0.5. 0.9 ships in order: capabilities → compile → runtime. 0.11 follows 0.10.

---

## Phase 0.1 — Foundation

**Target:** `0.1.x` · **Tier:** I · **Status:** Complete (`0.1.2`)

**Implements:** Parser, Validator ([Ch 23 §4](SPEC.md#chapter-23-conformance))

### Goal

Ship the [Ch 2 §13](SPEC.md#chapter-2-core-concepts) pipeline through diagnostics — no planning, compilation, or execution ([Ch 1 §3](SPEC.md#chapter-1-introduction)).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| COM | `TransformationContract` and related types; YAML/JSON parsing; extension preservation | [Ch 3](SPEC.md#chapter-3-canonical-object-model) |
| Validation | Seven-phase pipeline: document → COM → structural → types → references → semantics → extensions | [Ch 9 §4](SPEC.md#chapter-9-validation) |
| Types | Primitives, nullability, composite types (partial) | [Ch 4 §4–6](SPEC.md#chapter-4-type-system) |
| Interfaces | Input/output schemas, pre/postconditions, lineage mappings | [Ch 6 §3–9](SPEC.md#chapter-6-inputs-and-outputs), [Ch 10 §3–10](SPEC.md#chapter-10-lineage) |
| Libraries | Identity validation for actions, functions, rules (`dtcs:`) | [Ch 17 §3–4](SPEC.md#chapter-17-semantic-actions), [Ch 18 §3–4](SPEC.md#chapter-18-function-model), [Ch 19 §3–4](SPEC.md#chapter-19-rule-model) |
| Diagnostics | Severity, category, stage, codes, remediation | [Ch 20 §3–9](SPEC.md#chapter-20-diagnostics) |
| CLI | `validate`, `inspect`, `diagnostics`, `version` | [Ch 9 §14](SPEC.md#chapter-9-validation) |
| Distribution *(distribution)* | Python package, crates.io, multi-arch wheels | [Ch 23 §4](SPEC.md#chapter-23-conformance) |

### Modules

`src/model/`, `src/parser/`, `src/validation/`, `src/diagnostics/`, `src/cli/`, `src/python.rs`

### Exit criteria

- Fixture corpus passes validation; diagnostics are deterministic ([Ch 20 §9](SPEC.md#chapter-20-diagnostics))
- `examples/customer_normalize.dtcs.yaml` validates cleanly
- Published to crates.io and PyPI at `0.1.2`

---

## Phase 0.2 — Contract Model

**Target:** `0.2.x` · **Tier:** II · **Prerequisite:** 0.1 · **Status:** Complete (`0.2.0`)

**Implements:** Validator (extended) · **SPEC:** [Ch 4](SPEC.md#chapter-4-type-system), [Ch 5](SPEC.md#chapter-5-metadata), [Ch 6](SPEC.md#chapter-6-inputs-and-outputs)

### Goal

Complete contract representation: metadata, full type system, and I/O interface depth.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Metadata | Categories (identity, governance, provenance, classification); validation; semantically neutral preservation | [Ch 5 §3–12](SPEC.md#chapter-5-metadata) |
| Types | Conversion, inference, expression typing, collection typing, extension types | [Ch 4 §8–13](SPEC.md#chapter-4-type-system) |
| Interfaces | Multiplicity, optional inputs, streaming declarations, I/O extensibility | [Ch 6 §10–13](SPEC.md#chapter-6-inputs-and-outputs) |

### Modules

`src/model/metadata.rs`, `src/model/types.rs`, `src/model/interface.rs`, `src/validation/types.rs`, new `src/metadata/`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `dtcs::metadata::validate(contract)`, `dtcs::parse_logical_type`, `dtcs::type_compatible` |
| Python | `dtcs.metadata_validate`, `dtcs.validate_result`, `dtcs.parse_and_validate` |
| CLI | `dtcs validate` (metadata, types, expressions, and interface checks integrated) |

Expression typing in 0.2 performs static analysis of declared expression bodies (field references, literals, binary operators, and function calls). Full Chapter 8 grammar parsing is deferred to Phase 0.6.

### Exit criteria

- [x] [Ch 4 §14](SPEC.md#chapter-4-type-system), [Ch 5 §13](SPEC.md#chapter-5-metadata), [Ch 6 §15](SPEC.md#chapter-6-inputs-and-outputs): chapter conformance
- [x] Fixture corpus extended with metadata, streaming, and optional-input cases
- [x] Publish to crates.io and PyPI at `0.2.0` (tag `v0.2.0`)

---

## Phase 0.3 — Contract Analysis

**Target:** `0.3.x` · **Tier:** II · **Prerequisite:** 0.2 · **Status:** Complete (`0.3.0`)

**Implements:** Analyzer ([Ch 23 §4](SPEC.md#chapter-23-conformance)) · **SPEC:** [Ch 10](SPEC.md#chapter-10-lineage) §11–12, [Ch 11](SPEC.md#chapter-11-compatibility)–[12](SPEC.md#chapter-12-evolution), [Ch 25](SPEC.md#chapter-25-versioning)

### Goal

Analyze contracts for interoperability, safe evolution, versioning, and lineage impact — without mutating the COM.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Compatibility | Five classifications; scoped COM comparison; diagnostics | [Ch 11 §4–13](SPEC.md#chapter-11-compatibility) |
| Evolution | Revision detection, change categories, deprecation, migration hints | [Ch 12 §4–12](SPEC.md#chapter-12-evolution) |
| Versioning | Artifact version validation; version ≠ compatibility | [Ch 25 §3–11](SPEC.md#chapter-25-versioning) |
| Lineage analysis | Impact, dependency, governance reports; no mutation of declared lineage | [Ch 10 §11–12](SPEC.md#chapter-10-lineage) |

### Modules

`src/compatibility/`, new `src/lineage/analysis.rs`, `tests/fixtures/compatibility/`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `compatibility::analyze`, `compatibility::analyze_evolution`, `lineage::analyze` |
| CLI | `dtcs compat`, `dtcs evolve`, `dtcs lineage` |

### Exit criteria

- [x] [Ch 11 §14](SPEC.md#chapter-11-compatibility), [Ch 12 §13](SPEC.md#chapter-12-evolution), [Ch 25 §13](SPEC.md#chapter-25-versioning), [Ch 10 §13](SPEC.md#chapter-10-lineage)
- [x] Paired contract fixtures for each compatibility classification
- [x] Publish to crates.io and PyPI at `0.3.0` (tag `v0.3.0` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml))

---

## Phase 0.4 — Registries & Extensibility

**Target:** `0.4.x` · **Tier:** III · **Prerequisite:** 0.2 · **Status:** **Complete** (`0.4.0`)

**Implements:** Validator (registry pass) · **SPEC:** [Ch 21](SPEC.md#chapter-21-extensibility)–[22](SPEC.md#chapter-22-registries)

### Goal

Resolve `dtcs:` and vendor identifiers through authoritative registries; validate extension blocks.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Extensibility | Namespaces, extension identity, compatibility, processing rules | [Ch 21 §3–12](SPEC.md#chapter-21-extensibility) |
| Registries | Entry model, identifier stability, versioning, publication status | [Ch 22 §3–10](SPEC.md#chapter-22-registries) |
| Resolution | Embedded `dtcs:` catalog; file/URI loading; offline cache | [Ch 22 §8](SPEC.md#chapter-22-registries) |
| Validation | Registry-aware extension pass wired into [Ch 9 §10](SPEC.md#chapter-9-validation) | [Ch 21 §9](SPEC.md#chapter-21-extensibility) |

### Modules

`src/registry/` (new), `src/model/registry.rs`, `src/validation/extensions.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `registry::resolve`, `registry::load` |
| CLI | `dtcs registry list`, `dtcs registry resolve` |

### Exit criteria

- [x] [Ch 21 §12](SPEC.md#chapter-21-extensibility), [Ch 22 §12](SPEC.md#chapter-22-registries)
- [x] Every `dtcs:` identifier in valid contracts resolves to a registry entry
- [x] Publish to crates.io and PyPI at `0.4.0` (tag `v0.4.0` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml))

---

## Phase 0.5 — Standard Libraries

**Target:** `0.5.x` · **Tier:** III · **Prerequisite:** 0.4 · **Status:** **Complete** (`0.5.0` starter release)

**Implements:** Validator (library-aware semantics) · **SPEC:** [Ch 17](SPEC.md#chapter-17-semantic-actions)–[19](SPEC.md#chapter-19-rule-model)

### Goal

Publish the complete `dtcs:` standard libraries in the built-in registry ([Ch 2 §8–11](SPEC.md#chapter-2-core-concepts)).

### Delivered so far

| Area | Work | Status |
|------|------|--------|
| Embedded catalogs | YAML registry docs under `src/registry/builtin/` for semantic actions, functions, and rules | Done |
| Starter semantic actions | `dtcs:lowercase`, `dtcs:uppercase`, `dtcs:capitalize`, `dtcs:trim`, `dtcs:normalize_whitespace`, `dtcs:hash_sha256` | Done |
| Starter functions | `dtcs:concat`, `dtcs:coalesce`, `dtcs:length`, `dtcs:lower`, `dtcs:upper`, `dtcs:substr`, `dtcs:replace`, `dtcs:to_string`, `dtcs:to_integer`, `dtcs:to_decimal` | Done |
| Starter rules | `dtcs:not_null`, `dtcs:min_length`, `dtcs:max_length`, `dtcs:regex_match`, `dtcs:range` | Done |
| Registry-driven validation | Structured JSON `definition` blocks drive target type, nullability, rule phases, function arity, and return-type checks in `src/validation/semantics.rs` | Done |
| Fixtures | Stdlib validation fixtures under `tests/fixtures/stdlib_*.yaml` | Done |

### Remaining (completed in 0.11)

Full Ch 17–19 catalogs (dataset operators, additional functions/rules, normative Appendix A) shipped in [Phase 0.11](#phase-011-spec-completeness). Rule parameters, positional function signatures, expression return nullability, and per-entry fixtures for the starter subset shipped in `0.5.0`.

### Modules

`src/registry/builtin/` (actions, functions, rules), `src/validation/semantics.rs`

### Exit criteria

- [x] Starter catalog embedded and validated against registry definitions
- [x] Full Ch 17–19 catalogs completed in `0.11.0` (see Phase 0.11)

---

## Phase 0.6 — Semantic Analysis

**Target:** `0.6.x` · **Tier:** III · **Prerequisite:** 0.5 · **Status:** Complete (`0.6.0`)

**Implements:** Analyzer · **SPEC:** [Ch 7](SPEC.md#chapter-7-transformation-semantics)–[8](SPEC.md#chapter-8-expression-language)

### Goal

Static analysis of transformation semantics and expressions — no runtime evaluation ([Ch 8 §1](SPEC.md#chapter-8-expression-language)).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Transformation semantics | Observable behavior, composition, determinism, purity, pre/postconditions, equivalence | [Ch 7 §3–14](SPEC.md#chapter-7-transformation-semantics) |
| Expressions | AST, evaluation context, operators, null semantics, constant expressions, type checking | [Ch 8 §3–13](SPEC.md#chapter-8-expression-language) |
| Integration | Analysis pass strengthens [Ch 9 §9](SPEC.md#chapter-9-validation); results attachable to plan nodes | [Ch 7 §14](SPEC.md#chapter-7-transformation-semantics) |

### Modules

`src/analysis/` (new), `src/model/expression.rs`, `src/model/semantics.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `analysis::check_expression`, `analysis::check_contract` |
| CLI | `dtcs analyze <contract>` |

### Exit criteria

- [x] [Ch 7 §15](SPEC.md#chapter-7-transformation-semantics), [Ch 8 §15](SPEC.md#chapter-8-expression-language)
- [x] Invalid expression/semantics fixtures rejected with specific diagnostic codes

---

## Phase 0.7 — Transformation Plan

**Target:** `0.7.x` · **Tier:** IV · **Prerequisite:** 0.6 · **Status:** Complete (`0.7.0`)

**Implements:** Planner · **SPEC:** [Ch 13](SPEC.md#chapter-13-transformation-plan) (excluding §9)

### Goal

Lower validated COM into canonical semantic IR ([Ch 2 §5](SPEC.md#chapter-2-core-concepts)).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Plan IR | Complete semantic content: inputs, outputs, actions, expressions, rules, types, lineage, guarantees | [Ch 13 §4](SPEC.md#chapter-13-transformation-plan) |
| Lowering | Deterministic COM → plan; equivalent contracts → equivalent plans | [Ch 13 §5–6](SPEC.md#chapter-13-transformation-plan) |
| Dependency graph | Acyclic logical dependencies | [Ch 13 §8](SPEC.md#chapter-13-transformation-plan) |
| Plan validation | Completeness, referential integrity, type/lineage preservation | [Ch 13 §11](SPEC.md#chapter-13-transformation-plan) |
| Serialization | Round-trip semantic equivalence | [Ch 13 §12](SPEC.md#chapter-13-transformation-plan) |

### Modules

`src/plan/`: `model.rs`, `lowering.rs`, `graph.rs`, `validate.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `plan::lower`, `plan::validate`, `parse_validate_and_plan` |
| CLI | `dtcs plan <contract>` |
| Python | `plan_lower`, `plan_validate`, `plan_topological_order` |

### Exit criteria

- [x] `plan::lower` and `plan::validate` exported and documented
- [x] `dtcs plan <contract>` CLI with `--json` and `--registry`
- [x] Python `plan_lower` / `plan_validate` / `plan_topological_order`
- [x] [Ch 13 §14](SPEC.md#chapter-13-transformation-plan) (excluding optimization)
- [x] Golden files lock plan shape for fixture corpus
- [x] Planning-stage diagnostics with standardized codes
- [x] Publish to crates.io and PyPI at `0.7.0` (tag `v0.7.0` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml))

---

## Phase 0.8 — Plan Optimization

**Target:** `0.8.x` · **Tier:** IV · **Prerequisite:** 0.7 · **Status:** Complete (`0.8.0`)

**Implements:** Optimizer · **SPEC:** [Ch 13 §9](SPEC.md#chapter-13-transformation-plan), [Ch 8 §14](SPEC.md#chapter-8-expression-language), [Ch 15 §9](SPEC.md#chapter-15-compilation), [Ch 17 §11](SPEC.md#chapter-17-semantic-actions), [Ch 18 §11](SPEC.md#chapter-18-function-model), [Ch 19 §11](SPEC.md#chapter-19-rule-model)

### Goal

Produce semantically equivalent optimized plans (plan → plan only, never plan → execution plan).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Optimization boundary | Plan in, plan out; preserve behavior, types, lineage, guarantees | [Ch 13 §9](SPEC.md#chapter-13-transformation-plan) |
| Passes | Action, expression, function, and rule optimization passes | [Ch 17 §11](SPEC.md#chapter-17-semantic-actions), [Ch 8 §14](SPEC.md#chapter-8-expression-language), [Ch 18 §11](SPEC.md#chapter-18-function-model), [Ch 19 §11](SPEC.md#chapter-19-rule-model) |
| Equivalence tests | Optimized plan validates; golden comparison proves semantic equivalence | [Ch 15 §9](SPEC.md#chapter-15-compilation) |

### Modules

`src/plan/optimize.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `plan::optimize`, `plan::equivalent` |
| CLI | `dtcs optimize <path>` |
| Python | `plan_optimize`, `plan_equivalent` |

### Exit criteria

- [x] [Ch 13 §9](SPEC.md#chapter-13-transformation-plan): optimized plans pass `plan::validate` and equivalence tests
- [x] Golden files lock optimized plan shape for optimization fixtures
- [x] Optimization-stage diagnostics with standardized codes
- [x] Publish to crates.io and PyPI at `0.8.0` (tag `v0.8.0` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml))

---

## Phase 0.9 — Execution Pipeline

**Target:** `0.9.x` · **Tier:** IV · **Prerequisite:** 0.7 (0.8 recommended) · **Status:** Complete (`0.9.0`)

**Implements:** Compiler, Runtime · **SPEC:** [Ch 14](SPEC.md#chapter-14-engine-capability-model)–[16](SPEC.md#chapter-16-runtime)

### Goal

Match engine capabilities, compile plans to execution plans, and run them in a reference runtime ([Ch 2 §6–7](SPEC.md#chapter-2-core-concepts)).

Ship in three milestones within the phase:

| Milestone | Work | SPEC |
|-----------|------|------|
| **0.9.0 — Capabilities** | Engine profiles, capability matching, `dtcs:reference` profile | [Ch 14](SPEC.md#chapter-14-engine-capability-model) |
| **0.9.1 — Compilation** | `ExecutionPlan` IR, `Compiler` trait, reference backend | [Ch 15](SPEC.md#chapter-15-compilation) |
| **0.9.2 — Runtime** | `Runtime` trait, input validation, rule evaluation, built-in actions | [Ch 16](SPEC.md#chapter-16-runtime) |

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Capabilities | Declaration format, matching, profiles, discovery | [Ch 14 §3–11](SPEC.md#chapter-14-engine-capability-model) |
| Compilation | Capability-gated compile; semantic preservation; multi-target; plan validation | [Ch 15 §3–12](SPEC.md#chapter-15-compilation) |
| Runtime | Execute plans; enforce pre/postconditions; determinism; failure diagnostics | [Ch 16 §3–12](SPEC.md#chapter-16-runtime) |
| End-to-end | `customer_normalize.dtcs.yaml` runs validate → plan → compile → execute | [Ch 2 §13](SPEC.md#chapter-2-core-concepts) |

### Modules

`src/capability/` (new), `src/compile/` (new), `src/runtime/` (new)

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `capability::match_plan`, `compile::compile`, `runtime::execute` |
| Python | `capability_match`, `compile_plan`, `runtime_execute` |
| CLI | `dtcs match`, `dtcs compile`, `dtcs run` |

### Exit criteria

- [x] [Ch 14 §12](SPEC.md#chapter-14-engine-capability-model), [Ch 15 §13](SPEC.md#chapter-15-compilation), [Ch 16 §13](SPEC.md#chapter-16-runtime)
- [x] Reference backend covers all `dtcs:` entries from 0.5
- [x] Publish to crates.io and PyPI at `0.9.0` (tag `v0.9.0` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml))

---

## Phase 0.10 — Conformance & Ecosystem

**Target:** `0.10.x` · **Tier:** V · **Prerequisite:** 0.9 · **Status:** **Complete** (`0.10.1`)

**Implements:** Integrated Platform · **SPEC:** [Ch 1 §10](SPEC.md#chapter-1-introduction), [Ch 2 §14](SPEC.md#chapter-2-core-concepts), [Ch 23](SPEC.md#chapter-23-conformance)–[24](SPEC.md#chapter-24-security-considerations), [Ch 26](SPEC.md#chapter-26-governance)

### Goal

Prove objective conformance across all declared implementation classes; align project process with governance and security requirements.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Conformance profiles | One profile per class: Parser, Validator, Analyzer, Planner, Optimizer, Compiler, Runtime, Integrated Platform | [Ch 23 §4–5](SPEC.md#chapter-23-conformance) |
| Test suite | Semantic preservation, validation, analysis, planning, compilation, runtime, diagnostics | [Ch 23 §8](SPEC.md#chapter-23-conformance) |
| Capability declaration | Machine-readable JSON published with each release | [Ch 23 §9](SPEC.md#chapter-23-conformance) |
| Security checklist | Contract integrity, extension trust, registry trust, diagnostic confidentiality | [Ch 24 §3–12](SPEC.md#chapter-24-security-considerations) |
| Governance alignment | Change management, publication, registry/extension governance in CONTRIBUTING | [Ch 26 §3–13](SPEC.md#chapter-26-governance) |
| Ecosystem *(distribution)* | WASM/Node bindings; `uv publish` migration; API reference | [Ch 23 §4](SPEC.md#chapter-23-conformance) |

### Modules

`tests/conformance/`, `src/conformance/`, `.github/workflows/`

### APIs

| Surface | Entry point |
|---------|-------------|
| CLI | `dtcs conformance run --profile <name>`, `dtcs conformance declare` |

### Exit criteria

- [x] [Ch 23 §14](SPEC.md#chapter-23-conformance): passes every declared profile offline
- [x] [Ch 24 §12](SPEC.md#chapter-24-security-considerations), [Ch 26 §13](SPEC.md#chapter-26-governance)
- [x] [Ch 2 §13](SPEC.md#chapter-2-core-concepts): full pipeline verified end-to-end

- [x] Publish to crates.io and PyPI at `0.10.0` (tag `v0.10.0` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml))
- [x] Publish patch `0.10.1` — test suite verification (tag `v0.10.1` triggers release workflow)

---

## Phase 0.11 — SPEC Completeness

**Target:** `0.11.x` · **Tier:** VI · **Prerequisite:** 0.10 · **Status:** **Complete** (`0.11.0`)

**Implements:** Full standard-library catalog, deepened COM/runtime semantics · **SPEC:** [Ch 4](SPEC.md#chapter-4-type-system)–[8](SPEC.md#chapter-8-expression-language), [Ch 10](SPEC.md#chapter-10-lineage), [Ch 12](SPEC.md#chapter-12-evolution), [Ch 14](SPEC.md#chapter-14-engine-capability-model)–[19](SPEC.md#chapter-19-rule-model), [Appendix A](SPEC.md#appendix-a-standard-library-catalog-normative)

### Goal

Close remaining SPEC gaps in the reference implementation: complete the Ch 17–19 starter→full catalog, deepen COM fields (lineage, guarantees, compatibility, null semantics), and publish an objective chapter completeness matrix. Production ETL, Polars/Spark/SQL backends, and an external certification authority remain intentional non-goals.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Semantic actions | Full catalog: field transforms + project, select, filter, aggregate, group, join, sort, union, partition, derive; action `parameters` | [Ch 17 §5–10](SPEC.md#chapter-17-semantic-actions), [Appendix A §A.3](SPEC.md#appendix-a-standard-library-catalog-normative) |
| Functions | Full catalog including `abs`, `min`, `max`, `contains`, `is_null`, `is_missing`; nullBehavior tokens | [Ch 18](SPEC.md#chapter-18-function-model), [Appendix A §A.4](SPEC.md#appendix-a-standard-library-catalog-normative) |
| Rules | Full catalog including `one_of`, `equals` (`allowIndeterminate`) | [Ch 19](SPEC.md#chapter-19-rule-model), [Appendix A §A.5](SPEC.md#appendix-a-standard-library-catalog-normative) |
| Lineage COM | Mapping `operation` (default `dtcs:derive`), `flow` enum, optional `id` | [Ch 10 §5–7](SPEC.md#chapter-10-lineage), [Appendix A §A.6](SPEC.md#appendix-a-standard-library-catalog-normative) |
| Null semantics | Distinguish null / missing / invalid at runtime | [Ch 8 §9](SPEC.md#chapter-8-expression-language), [Appendix A §A.7](SPEC.md#appendix-a-standard-library-catalog-normative) |
| Guarantees & compatibility | First-class `guarantees` and `compatibility` COM fields | [Ch 2 §3](SPEC.md#chapter-2-core-concepts), [Ch 3](SPEC.md#chapter-3-canonical-object-model), [Ch 11](SPEC.md#chapter-11-compatibility)–[12](SPEC.md#chapter-12-evolution) |
| Capabilities / compile / runtime | `dtcs:reference` profile and reference backend cover the full catalog | [Ch 14](SPEC.md#chapter-14-engine-capability-model)–[16](SPEC.md#chapter-16-runtime) |
| Completeness matrix | Chapter-by-chapter Full/N/A evidence document | [`docs/implementation/spec-completeness.md`](docs/implementation/spec-completeness.md) |
| Normative catalog | SPEC Appendix A | [Appendix A](SPEC.md#appendix-a-standard-library-catalog-normative) |

### Modules

`src/registry/builtin/`, `src/model/` (`lineage.rs`, `action.rs`, `guarantees.rs`, `compatibility_decl.rs`, `null_behavior.rs`), `src/runtime/`, `src/capability/`, `src/compile/`

### Exit criteria

- [x] Full Ch 17–19 catalogs embedded, validated, and executed by the reference runtime
- [x] Lineage `operation` / `flow` defaults and null/missing/invalid tokens implemented
- [x] [SPEC Appendix A](SPEC.md#appendix-a-standard-library-catalog-normative) published (normative identifier catalog)
- [x] [`docs/implementation/spec-completeness.md`](docs/implementation/spec-completeness.md): all 26 chapters Full except intentional N/A non-goals
- [x] Package versions at `0.11.0` (crates.io / PyPI / bindings)

---

## Phase 0.12 — Portable Relational Profile

**Status:** Covered in `0.12.0`  
**Proposal:** [docs/DTCS_PORTABLE_SPEC_PROPOSAL.md](docs/DTCS_PORTABLE_SPEC_PROPOSAL.md) (DTCS-R1–R4)  
**Profiles:** `portable-relational-kernel/1`, `portable-relational/1`, `portable-window/1`, `portable-complex-types/1`

Ships the full Portable Relational Profile in one release: kernel operators and field shaping, rich relational actions, conformance fixtures/capability accuracy, and advanced window/datetime/complex-access support.

### Deliverables

**Kernel (R1)**

- [x] Operator registry (`dtcs:stdlib.operators`) and `RegistryCategory::Operator`
- [x] Widened `dtcs:project` / `dtcs:filter` (entry v2) with legacy subset acceptance
- [x] `dtcs:with_fields`, `dtcs:rename_fields`, `dtcs:drop_fields`
- [x] Kernel functions (`case_when`, `if_null`, `null_if`, `try_cast`, `is_invalid`); `coalesce` v2
- [x] Ternary `between`; access operators `dtcs:field` / `dtcs:index` / `dtcs:element_at`
- [x] Portable plan envelope `dtcs.transform-plan/1` + fingerprint + security budgets
- [x] Structured expression COM `body` + string `expr` sugar
- [x] SPEC Ch 13 §12.1, Appendix A updates, A.8 profiles

**Relational (R2)**

- [x] Rich `join` / `union` / `group` / `aggregate` / `sort` (entry v2); null keys do not match
- [x] Join `collisionPolicy` / `on`/`predicate`; sort and `groupBy` expressions; union `duplicatePolicy`
- [x] `dtcs:distinct`, `dtcs:deduplicate`, `dtcs:limit`
- [x] Aggregate family; missing ≠ null grouping keys

**Conformance (R3)**

- [x] Semantic-family conformance profiles alongside implementation-class profiles
- [x] Differential portable fixtures (`tests/fixtures/portable/`) + dual-path string/structured gate
- [x] Capability accuracy / false-claim validation (`validate_capability_accuracy`)
- [x] [`docs/implementation/portable-conformance.md`](docs/implementation/portable-conformance.md)
- [x] Portable diagnostics; executable-object rejection; plan budgets

**Advanced (R4)**

- [x] `dtcs:window` with rows/range frames; `row_number`/`rank`/`dense_rank`/`lag`/`lead`/`first_value`/`last_value`; framed aggregates
- [x] Datetime family: day/month/year/hour/minute; `date_trunc` / `extract` / `at_timezone` (fixed-offset)
- [x] Complex-types profile: `array`/`struct` aliases + executable access ops (explode/unnest non-goal)
- [x] Package versions at `0.12.0` (crates.io / PyPI / bindings)

---

## Out of scope for 0.X

Per [Ch 1 §3](SPEC.md#chapter-1-introduction):

| Item | SPEC basis |
|------|------------|
| Production ETL orchestration | [Ch 1 §3](SPEC.md#chapter-1-introduction) |
| Polars, Spark, SQL backends | [Ch 1 §3](SPEC.md#chapter-1-introduction) |
| Storage technologies | [Ch 1 §3](SPEC.md#chapter-1-introduction) |
| User interface products | [Ch 1 §3](SPEC.md#chapter-1-introduction) |
| External certification authority | [Ch 23 §13](SPEC.md#chapter-23-conformance) |

## How to use this roadmap

1. Find the chapter in the [SPEC chapter index](#spec-chapter-index).
2. Complete phases in tier order; respect [dependencies](#dependencies).
3. Each phase ships at `0.X.0`; patch releases fix issues within a phase.
4. Add fixtures and conformance tests for every deliverable row.
5. Update [`docs/implementation/`](docs/implementation/) when public APIs change.
