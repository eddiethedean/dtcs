# DTCS Roadmap

Reference-implementation milestones for the Data Transformation Contract Standard. Uses **0.X** release phases only and covers **all 26 chapters** of [`SPEC.md`](SPEC.md).

[`SPEC.md`](SPEC.md) is the source of truth. When this roadmap and the specification disagree, the specification wins.

**Citation format:** `Ch N §M` — chapter *N*, section *M* in [`SPEC.md`](SPEC.md).

**Distribution** items marked *(distribution)* support adoption but are not normative SPEC requirements.

---

## Tiers

Phases are grouped into five tiers that follow the [Ch 2 §13](SPEC.md#chapter-2----core-concepts) pipeline.

| Tier | Phases | Pipeline stage |
|------|--------|----------------|
| **I — Foundation** | [0.1](#phase-01--foundation) | Document → COM → Validator → Diagnostics |
| **II — Contract** | [0.2](#phase-02--contract-model) · [0.3](#phase-03--contract-analysis) | Complete contract representation and analysis |
| **III — Semantic** | [0.4](#phase-04--registries--extensibility) · [0.5](#phase-05--standard-libraries) · [0.6](#phase-06--semantic-analysis) | Identifiers, libraries, static semantics |
| **IV — Planning & execution** | [0.7](#phase-07--transformation-plan) · [0.8](#phase-08--plan-optimization) · [0.9](#phase-09--execution-pipeline) | Plan → optimize → compile → run |
| **V — Certification** | [0.10](#phase-010--conformance--ecosystem) | Conformance, security, governance |

## Status overview

| Phase | Name | SPEC focus | Status |
|-------|------|------------|--------|
| **0.1** | [Foundation](#phase-01--foundation) | Ch 1–3, 9–10, 17–20 (core) | **Complete** (`0.1.2`) |
| **0.2** | [Contract Model](#phase-02--contract-model) | Ch 4–6 | **Complete** (`0.2.0`) |
| **0.3** | [Contract Analysis](#phase-03--contract-analysis) | Ch 10 §11–12, 11–12, 25 | **Complete** (`0.3.0`) |
| **0.4** | [Registries & Extensibility](#phase-04--registries--extensibility) | Ch 21–22 | Planned |
| **0.5** | [Standard Libraries](#phase-05--standard-libraries) | Ch 17–19 | Planned |
| **0.6** | [Semantic Analysis](#phase-06--semantic-analysis) | Ch 7–8 | Planned |
| **0.7** | [Transformation Plan](#phase-07--transformation-plan) | Ch 13 (lowering) | Planned |
| **0.8** | [Plan Optimization](#phase-08--plan-optimization) | Ch 13 §9, 8 §14, 15 §9, 17–19 §11 | Planned |
| **0.9** | [Execution Pipeline](#phase-09--execution-pipeline) | Ch 14–16 | Planned |
| **0.10** | [Conformance & Ecosystem](#phase-010--conformance--ecosystem) | Ch 1 §10, 2 §14, 23–24, 26 | Planned |

## SPEC chapter index

| Ch | Title | Phase | Coverage |
|----|-------|-------|----------|
| 1 | [Introduction](SPEC.md#chapter-1----introduction) | 0.1, 0.10 | Principles (0.1); conformance intro (0.10) |
| 2 | [Core Concepts](SPEC.md#chapter-2----core-concepts) | 0.1, 0.10 | Vocabulary (0.1); concept conformance (0.10) |
| 3 | [Canonical Object Model](SPEC.md#chapter-3----canonical-object-model) | 0.1 | Full |
| 4 | [Type System](SPEC.md#chapter-4----type-system) | 0.1 partial, 0.2 | Primitives (0.1); conversion, inference, collections (0.2) |
| 5 | [Metadata](SPEC.md#chapter-5----metadata) | 0.2 | Full |
| 6 | [Inputs and Outputs](SPEC.md#chapter-6----inputs-and-outputs) | 0.1 partial, 0.2 | Schemas, lineage hooks (0.1); multiplicity, streaming (0.2) |
| 7 | [Transformation Semantics](SPEC.md#chapter-7----transformation-semantics) | 0.6 | Full |
| 8 | [Expression Language](SPEC.md#chapter-8----expression-language) | 0.6, 0.8 | Static analysis (0.6); optimization (0.8) |
| 9 | [Validation](SPEC.md#chapter-9----validation) | 0.1 | Full; deepened by 0.2 and 0.6 |
| 10 | [Lineage](SPEC.md#chapter-10----lineage) | 0.1 partial, 0.3 | Integrity validation (0.1); impact analysis (0.3) |
| 11 | [Compatibility](SPEC.md#chapter-11----compatibility) | 0.3 | Full |
| 12 | [Evolution](SPEC.md#chapter-12----evolution) | 0.3 | Full |
| 13 | [Transformation Plan](SPEC.md#chapter-13----transformation-plan) | 0.7, 0.8 | Lowering (0.7); optimization (0.8) |
| 14 | [Engine Capability Model](SPEC.md#chapter-14----engine-capability-model) | 0.9 | Full |
| 15 | [Compilation](SPEC.md#chapter-15----compilation) | 0.8 partial, 0.9 | Optimization boundary (0.8); compilation (0.9) |
| 16 | [Runtime](SPEC.md#chapter-16----runtime) | 0.9 | Full |
| 17 | [Semantic Actions](SPEC.md#chapter-17----semantic-actions) | 0.1 partial, 0.5, 0.8 | Identity (0.1); library (0.5); optimization (0.8) |
| 18 | [Function Model](SPEC.md#chapter-18----function-model) | 0.1 partial, 0.5, 0.8 | Identity (0.1); library (0.5); optimization (0.8) |
| 19 | [Rule Model](SPEC.md#chapter-19----rule-model) | 0.1 partial, 0.5, 0.8 | Identity (0.1); library (0.5); optimization (0.8) |
| 20 | [Diagnostics](SPEC.md#chapter-20----diagnostics) | 0.1 | Full; stage codes added per phase |
| 21 | [Extensibility](SPEC.md#chapter-21----extensibility) | 0.4 | Full |
| 22 | [Registries](SPEC.md#chapter-22----registries) | 0.4 | Full |
| 23 | [Conformance](SPEC.md#chapter-23----conformance) | 0.10 | Full |
| 24 | [Security Considerations](SPEC.md#chapter-24----security-considerations) | 0.10 | Full |
| 25 | [Versioning](SPEC.md#chapter-25----versioning) | 0.3 | Full |
| 26 | [Governance](SPEC.md#chapter-26----governance) | 0.10 | Full |

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
```

**Parallel work:** 0.4 and 0.3 can start after 0.2 (registries do not require compatibility engine). 0.6 requires 0.5. 0.9 ships in order: capabilities → compile → runtime.

---

## Phase 0.1 — Foundation

**Target:** `0.1.x` · **Tier:** I · **Status:** Complete (`0.1.2`)

**Implements:** Parser, Validator ([Ch 23 §4](SPEC.md#chapter-23----conformance))

### Goal

Ship the [Ch 2 §13](SPEC.md#chapter-2----core-concepts) pipeline through diagnostics — no planning, compilation, or execution ([Ch 1 §3](SPEC.md#chapter-1----introduction)).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| COM | `TransformationContract` and related types; YAML/JSON parsing; extension preservation | [Ch 3](SPEC.md#chapter-3----canonical-object-model) |
| Validation | Seven-phase pipeline: document → COM → structural → types → references → semantics → extensions | [Ch 9 §4](SPEC.md#chapter-9----validation) |
| Types | Primitives, nullability, composite types (partial) | [Ch 4 §4–6](SPEC.md#chapter-4----type-system) |
| Interfaces | Input/output schemas, pre/postconditions, lineage mappings | [Ch 6 §3–9](SPEC.md#chapter-6----inputs-and-outputs), [Ch 10 §3–10](SPEC.md#chapter-10----lineage) |
| Libraries | Identity validation for actions, functions, rules (`dtcs:`) | [Ch 17 §3–4](SPEC.md#chapter-17----semantic-actions), [Ch 18 §3–4](SPEC.md#chapter-18----function-model), [Ch 19 §3–4](SPEC.md#chapter-19----rule-model) |
| Diagnostics | Severity, category, stage, codes, remediation | [Ch 20 §3–9](SPEC.md#chapter-20----diagnostics) |
| CLI | `validate`, `inspect`, `diagnostics`, `version` | [Ch 9 §14](SPEC.md#chapter-9----validation) |
| Distribution *(distribution)* | Python package, crates.io, multi-arch wheels | [Ch 23 §4](SPEC.md#chapter-23----conformance) |

### Modules

`src/model/`, `src/parser/`, `src/validation/`, `src/diagnostics/`, `src/cli/`, `src/python.rs`

### Exit criteria

- Fixture corpus passes validation; diagnostics are deterministic ([Ch 20 §9](SPEC.md#chapter-20----diagnostics))
- `examples/customer_normalize.dtcs.yaml` validates cleanly
- Published to crates.io and PyPI at `0.1.2`

---

## Phase 0.2 — Contract Model

**Target:** `0.2.x` · **Tier:** II · **Prerequisite:** 0.1 · **Status:** Complete (`0.2.0`)

**Implements:** Validator (extended) · **SPEC:** [Ch 4](SPEC.md#chapter-4----type-system), [Ch 5](SPEC.md#chapter-5----metadata), [Ch 6](SPEC.md#chapter-6----inputs-and-outputs)

### Goal

Complete contract representation: metadata, full type system, and I/O interface depth.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Metadata | Categories (identity, governance, provenance, classification); validation; semantically neutral preservation | [Ch 5 §3–12](SPEC.md#chapter-5----metadata) |
| Types | Conversion, inference, expression typing, collection typing, extension types | [Ch 4 §8–13](SPEC.md#chapter-4----type-system) |
| Interfaces | Multiplicity, optional inputs, streaming declarations, I/O extensibility | [Ch 6 §10–13](SPEC.md#chapter-6----inputs-and-outputs) |

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

- [x] [Ch 4 §14](SPEC.md#chapter-4----type-system), [Ch 5 §13](SPEC.md#chapter-5----metadata), [Ch 6 §15](SPEC.md#chapter-6----inputs-and-outputs): chapter conformance
- [x] Fixture corpus extended with metadata, streaming, and optional-input cases
- [x] Publish to crates.io and PyPI at `0.2.0` (tag `v0.2.0`)

---

## Phase 0.3 — Contract Analysis

**Target:** `0.3.x` · **Tier:** II · **Prerequisite:** 0.2 · **Status:** Complete (`0.3.0`)

**Implements:** Analyzer ([Ch 23 §4](SPEC.md#chapter-23----conformance)) · **SPEC:** [Ch 10](SPEC.md#chapter-10----lineage) §11–12, [Ch 11](SPEC.md#chapter-11----compatibility)–[12](SPEC.md#chapter-12----evolution), [Ch 25](SPEC.md#chapter-25----versioning)

### Goal

Analyze contracts for interoperability, safe evolution, versioning, and lineage impact — without mutating the COM.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Compatibility | Five classifications; scoped COM comparison; diagnostics | [Ch 11 §4–13](SPEC.md#chapter-11----compatibility) |
| Evolution | Revision detection, change categories, deprecation, migration hints | [Ch 12 §4–12](SPEC.md#chapter-12----evolution) |
| Versioning | Artifact version validation; version ≠ compatibility | [Ch 25 §3–11](SPEC.md#chapter-25----versioning) |
| Lineage analysis | Impact, dependency, governance reports; no mutation of declared lineage | [Ch 10 §11–12](SPEC.md#chapter-10----lineage) |

### Modules

`src/compatibility/` (expand stub), new `src/lineage/analysis.rs`, `tests/fixtures/compatibility/`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `compatibility::analyze`, `compatibility::analyze_evolution`, `lineage::analyze` |
| CLI | `dtcs compat`, `dtcs evolve`, `dtcs lineage` |

### Exit criteria

- [x] [Ch 11 §14](SPEC.md#chapter-11----compatibility), [Ch 12 §13](SPEC.md#chapter-12----evolution), [Ch 25 §13](SPEC.md#chapter-25----versioning), [Ch 10 §13](SPEC.md#chapter-10----lineage)
- [x] Paired contract fixtures for each compatibility classification
- [ ] Publish to crates.io and PyPI at `0.3.0` (tag `v0.3.0` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml))

---

## Phase 0.4 — Registries & Extensibility

**Target:** `0.4.x` · **Tier:** III · **Prerequisite:** 0.2

**Implements:** Validator (registry pass) · **SPEC:** [Ch 21](SPEC.md#chapter-21----extensibility)–[22](SPEC.md#chapter-22----registries)

### Goal

Resolve `dtcs:` and vendor identifiers through authoritative registries; validate extension blocks.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Extensibility | Namespaces, extension identity, compatibility, processing rules | [Ch 21 §3–12](SPEC.md#chapter-21----extensibility) |
| Registries | Entry model, identifier stability, versioning, publication status | [Ch 22 §3–10](SPEC.md#chapter-22----registries) |
| Resolution | Embedded `dtcs:` catalog; file/URI loading; offline cache | [Ch 22 §8](SPEC.md#chapter-22----registries) |
| Validation | Registry-aware extension pass wired into [Ch 9 §10](SPEC.md#chapter-9----validation) | [Ch 21 §9](SPEC.md#chapter-21----extensibility) |

### Modules

`src/registry/` (new), `src/model/registry.rs`, `src/validation/extensions.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `registry::resolve`, `registry::load` |
| CLI | `dtcs registry list`, `dtcs registry resolve` |

### Exit criteria

- [Ch 21 §12](SPEC.md#chapter-21----extensibility), [Ch 22 §12](SPEC.md#chapter-22----registries)
- Every `dtcs:` identifier in fixtures resolves to a registry entry

---

## Phase 0.5 — Standard Libraries

**Target:** `0.5.x` · **Tier:** III · **Prerequisite:** 0.4

**Implements:** Validator (library-aware semantics) · **SPEC:** [Ch 17](SPEC.md#chapter-17----semantic-actions)–[19](SPEC.md#chapter-19----rule-model)

### Goal

Publish the complete `dtcs:` standard libraries in the built-in registry ([Ch 2 §8–11](SPEC.md#chapter-2----core-concepts)).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Semantic actions | Full catalog: projection, selection, transformation, aggregation, grouping, joining, sorting, union, partitioning, filtering; type and lineage semantics per action | [Ch 17 §5–10](SPEC.md#chapter-17----semantic-actions) |
| Functions | Standard library with parameters, return types, null behavior, determinism | [Ch 18 §3–10](SPEC.md#chapter-18----function-model), [Ch 2 §10](SPEC.md#chapter-2----core-concepts) |
| Rules | Standard library with evaluation phases, scope, outcomes | [Ch 19 §3–10](SPEC.md#chapter-19----rule-model), [Ch 2 §11](SPEC.md#chapter-2----core-concepts) |
| Validation | Arity, type, composition, and extension checks using registry definitions | [Ch 17 §10](SPEC.md#chapter-17----semantic-actions), [Ch 18 §10](SPEC.md#chapter-18----function-model), [Ch 19 §10](SPEC.md#chapter-19----rule-model) |

### Modules

`src/registry/builtin/` (actions, functions, rules), extend `src/validation/semantics.rs`

### Exit criteria

- [Ch 17 §13](SPEC.md#chapter-17----semantic-actions), [Ch 18 §13](SPEC.md#chapter-18----function-model), [Ch 19 §13](SPEC.md#chapter-19----rule-model)
- Each library entry has registry metadata, type rules, and at least one fixture

---

## Phase 0.6 — Semantic Analysis

**Target:** `0.6.x` · **Tier:** III · **Prerequisite:** 0.5

**Implements:** Analyzer · **SPEC:** [Ch 7](SPEC.md#chapter-7----transformation-semantics)–[8](SPEC.md#chapter-8----expression-language)

### Goal

Static analysis of transformation semantics and expressions — no runtime evaluation ([Ch 8 §1](SPEC.md#chapter-8----expression-language)).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Transformation semantics | Observable behavior, composition, determinism, purity, pre/postconditions, equivalence | [Ch 7 §3–14](SPEC.md#chapter-7----transformation-semantics) |
| Expressions | AST, evaluation context, operators, null semantics, constant expressions, type checking | [Ch 8 §3–13](SPEC.md#chapter-8----expression-language) |
| Integration | Analysis pass strengthens [Ch 9 §9](SPEC.md#chapter-9----validation); results attachable to plan nodes | [Ch 7 §14](SPEC.md#chapter-7----transformation-semantics) |

### Modules

`src/analysis/` (new), `src/model/expression.rs`, `src/model/semantics.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `analysis::check_expression`, `analysis::check_contract` |
| CLI | `dtcs analyze <contract>` |

### Exit criteria

- [Ch 7 §15](SPEC.md#chapter-7----transformation-semantics), [Ch 8 §15](SPEC.md#chapter-8----expression-language)
- Invalid expression/semantics fixtures rejected with specific diagnostic codes

---

## Phase 0.7 — Transformation Plan

**Target:** `0.7.x` · **Tier:** IV · **Prerequisite:** 0.6

**Implements:** Planner · **SPEC:** [Ch 13](SPEC.md#chapter-13----transformation-plan) (excluding §9)

### Goal

Lower validated COM into canonical semantic IR ([Ch 2 §5](SPEC.md#chapter-2----core-concepts)).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Plan IR | Complete semantic content: inputs, outputs, actions, expressions, rules, types, lineage, guarantees | [Ch 13 §4](SPEC.md#chapter-13----transformation-plan) |
| Lowering | Deterministic COM → plan; equivalent contracts → equivalent plans | [Ch 13 §5–6](SPEC.md#chapter-13----transformation-plan) |
| Dependency graph | Acyclic logical dependencies | [Ch 13 §8](SPEC.md#chapter-13----transformation-plan) |
| Plan validation | Completeness, referential integrity, type/lineage preservation | [Ch 13 §11](SPEC.md#chapter-13----transformation-plan) |
| Serialization | Round-trip semantic equivalence | [Ch 13 §12](SPEC.md#chapter-13----transformation-plan) |

### Modules

`src/plan/` (expand stub): `lowering.rs`, `graph.rs`, `validate.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `plan::lower`, `plan::validate` |
| CLI | `dtcs plan <contract>` |

### Exit criteria

- [Ch 13 §14](SPEC.md#chapter-13----transformation-plan) (excluding optimization)
- Golden files lock plan shape for fixture corpus

---

## Phase 0.8 — Plan Optimization

**Target:** `0.8.x` · **Tier:** IV · **Prerequisite:** 0.7

**Implements:** Optimizer · **SPEC:** [Ch 13 §9](SPEC.md#chapter-13----transformation-plan), [Ch 8 §14](SPEC.md#chapter-8----expression-language), [Ch 15 §9](SPEC.md#chapter-15----compilation), [Ch 17 §11](SPEC.md#chapter-17----semantic-actions), [Ch 18 §11](SPEC.md#chapter-18----function-model), [Ch 19 §11](SPEC.md#chapter-19----rule-model)

### Goal

Produce semantically equivalent optimized plans (plan → plan only, never plan → execution plan).

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Optimization boundary | Plan in, plan out; preserve behavior, types, lineage, guarantees | [Ch 13 §9](SPEC.md#chapter-13----transformation-plan) |
| Passes | Action, expression, function, and rule optimization passes | [Ch 17 §11](SPEC.md#chapter-17----semantic-actions), [Ch 8 §14](SPEC.md#chapter-8----expression-language), [Ch 18 §11](SPEC.md#chapter-18----function-model), [Ch 19 §11](SPEC.md#chapter-19----rule-model) |
| Equivalence tests | Optimized plan validates; golden comparison proves semantic equivalence | [Ch 15 §9](SPEC.md#chapter-15----compilation) |

### Modules

`src/plan/optimize.rs`

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `plan::optimize` |
| CLI | `dtcs optimize <plan>` |

### Exit criteria

- [Ch 13 §9](SPEC.md#chapter-13----transformation-plan): optimized plans pass `plan::validate` and equivalence tests

---

## Phase 0.9 — Execution Pipeline

**Target:** `0.9.x` · **Tier:** IV · **Prerequisite:** 0.7 (0.8 recommended)

**Implements:** Compiler, Runtime · **SPEC:** [Ch 14](SPEC.md#chapter-14----engine-capability-model)–[16](SPEC.md#chapter-16----runtime)

### Goal

Match engine capabilities, compile plans to execution plans, and run them in a reference runtime ([Ch 2 §6–7](SPEC.md#chapter-2----core-concepts)).

Ship in three milestones within the phase:

| Milestone | Work | SPEC |
|-----------|------|------|
| **0.9.0 — Capabilities** | Engine profiles, capability matching, `dtcs:reference` profile | [Ch 14](SPEC.md#chapter-14----engine-capability-model) |
| **0.9.1 — Compilation** | `ExecutionPlan` IR, `Compiler` trait, reference backend | [Ch 15](SPEC.md#chapter-15----compilation) |
| **0.9.2 — Runtime** | `Runtime` trait, input validation, rule evaluation, built-in actions | [Ch 16](SPEC.md#chapter-16----runtime) |

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Capabilities | Declaration format, matching, profiles, discovery | [Ch 14 §3–11](SPEC.md#chapter-14----engine-capability-model) |
| Compilation | Capability-gated compile; semantic preservation; multi-target; plan validation | [Ch 15 §3–12](SPEC.md#chapter-15----compilation) |
| Runtime | Execute plans; enforce pre/postconditions; determinism; failure diagnostics | [Ch 16 §3–12](SPEC.md#chapter-16----runtime) |
| End-to-end | `customer_normalize.dtcs.yaml` runs validate → plan → compile → execute | [Ch 2 §13](SPEC.md#chapter-2----core-concepts) |

### Modules

`src/capability/` (new), `src/compile/` (new), `src/runtime/` (new)

### APIs

| Surface | Entry point |
|---------|-------------|
| Rust | `capability::match_plan`, `compile::compile`, `runtime::execute` |
| CLI | `dtcs match`, `dtcs compile`, `dtcs run` |

### Exit criteria

- [Ch 14 §12](SPEC.md#chapter-14----engine-capability-model), [Ch 15 §13](SPEC.md#chapter-15----compilation), [Ch 16 §13](SPEC.md#chapter-16----runtime)
- Reference backend covers all `dtcs:` entries from 0.5

---

## Phase 0.10 — Conformance & Ecosystem

**Target:** `0.10.x` · **Tier:** V · **Prerequisite:** 0.9

**Implements:** Integrated Platform · **SPEC:** [Ch 1 §10](SPEC.md#chapter-1----introduction), [Ch 2 §14](SPEC.md#chapter-2----core-concepts), [Ch 23](SPEC.md#chapter-23----conformance)–[24](SPEC.md#chapter-24----security-considerations), [Ch 26](SPEC.md#chapter-26----governance)

### Goal

Prove objective conformance across all declared implementation classes; align project process with governance and security requirements.

### Deliverables

| Area | Work | SPEC |
|------|------|------|
| Conformance profiles | One profile per class: Parser, Validator, Analyzer, Planner, Optimizer, Compiler, Runtime, Integrated Platform | [Ch 23 §4–5](SPEC.md#chapter-23----conformance) |
| Test suite | Semantic preservation, validation, analysis, planning, compilation, runtime, diagnostics | [Ch 23 §8](SPEC.md#chapter-23----conformance) |
| Capability declaration | Machine-readable JSON published with each release | [Ch 23 §9](SPEC.md#chapter-23----conformance) |
| Security checklist | Contract integrity, extension trust, registry trust, diagnostic confidentiality | [Ch 24 §3–12](SPEC.md#chapter-24----security-considerations) |
| Governance alignment | Change management, publication, registry/extension governance in CONTRIBUTING | [Ch 26 §3–13](SPEC.md#chapter-26----governance) |
| Ecosystem *(distribution)* | WASM/Node bindings; `uv publish` migration; API reference | [Ch 23 §4](SPEC.md#chapter-23----conformance) |

### Modules

`tests/conformance/`, `src/conformance/`, `.github/workflows/`

### APIs

| Surface | Entry point |
|---------|-------------|
| CLI | `dtcs conformance run --profile <name>`, `dtcs conformance declare` |

### Exit criteria

- [Ch 23 §14](SPEC.md#chapter-23----conformance): passes every declared profile offline
- [Ch 24 §12](SPEC.md#chapter-24----security-considerations), [Ch 26 §13](SPEC.md#chapter-26----governance)
- [Ch 2 §13](SPEC.md#chapter-2----core-concepts): full pipeline verified end-to-end

---

## Out of scope for 0.X

Per [Ch 1 §3](SPEC.md#chapter-1----introduction):

| Item | SPEC basis |
|------|------------|
| Production ETL orchestration | [Ch 1 §3](SPEC.md#chapter-1----introduction) |
| Polars, Spark, SQL backends | [Ch 1 §3](SPEC.md#chapter-1----introduction) |
| Storage technologies | [Ch 1 §3](SPEC.md#chapter-1----introduction) |
| User interface products | [Ch 1 §3](SPEC.md#chapter-1----introduction) |
| External certification authority | [Ch 23 §13](SPEC.md#chapter-23----conformance) |

## How to use this roadmap

1. Find the chapter in the [SPEC chapter index](#spec-chapter-index).
2. Complete phases in tier order; respect [dependencies](#dependencies).
3. Each phase ships at `0.X.0`; patch releases fix issues within a phase.
4. Add fixtures and conformance tests for every deliverable row.
5. Update [`docs/implementation/`](docs/implementation/) when public APIs change.
