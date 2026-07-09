# Implementation Build Order

> **Note:** This document describes the *build order* used to scaffold the crate. For release milestones and SPEC chapter coverage, see [ROADMAP.md](../../ROADMAP.md) (Phases 0.1–0.10).

## Rule

Every implementation phase should consult `SPEC.md` first.

## Phase 1 — Project Skeleton

- Create Rust crate.
- Add dependencies.
- Add module structure.
- Add minimal README and examples.

## Phase 2 — Canonical Object Model

- Implement serializable structs.
- Use serde.
- Preserve identifiers and references.
- Avoid execution logic.

## Phase 3 — Parsing

- Load YAML.
- Load JSON.
- Convert into Canonical Object Model.
- Return structured diagnostics.

## Phase 4 — Diagnostics

- Implement diagnostic model.
- Severity, category, stage, message, object reference.
- Implement diagnostic report.

## Phase 5 — Validation

Implement validation phases from `SPEC.md`:

1. Document validation
2. Canonical Object Model validation
3. Structural validation
4. Type validation
5. Reference validation
6. Semantic validation
7. Extension validation

## Phase 6 — CLI

Commands:

```bash
dtcs validate contract.yaml
dtcs inspect contract.yaml
dtcs diagnostics contract.yaml
dtcs version
```

## Phase 7 — Contract Analysis (Phase 0.3)

- Compatibility comparison and five-level classification
- Evolution analysis with change categories
- Ch 25 versioning validation
- Dataset-level lineage analysis

Additional CLI commands:

```bash
dtcs compat source.yaml target.yaml
dtcs evolve older.yaml newer.yaml
dtcs lineage contract.yaml
```

## Phase 8 — Registries & Extensibility (Phase 0.4)

- Identifier registry model and embedded `dtcs:` catalog
- File/URI loading with offline cache
- Registry-aware semantic and extension validation
- `validate_with_registry` for vendor catalogs

Additional CLI commands:

```bash
dtcs registry list
dtcs registry resolve <id>
```

## Phase 9 — Standard Libraries (Phase 0.5)

- Embedded YAML catalogs under `src/registry/builtin/` for semantic actions, functions, and rules
- Structured JSON `definition` blocks on registry entries
- Registry-driven semantics validation in `src/validation/semantics.rs` (target types, phases, arity, return types)
- Starter catalog: string actions, common string/numeric functions, and constraint rules

Additional CLI commands (unchanged from Phase 8):

```bash
dtcs registry list
dtcs registry resolve <id>
```

## Phase 10 — Semantic Analysis (Phase 0.6)

- Static analysis of transformation semantics (Ch 7) and expression semantics (Ch 8)
- Expression AST parsing, type checking, constant folding, null semantics
- `analysis::check_contract` integrates with validation and plan lowering

Additional CLI command:

```bash
dtcs analyze <contract>
```

## Phase 11 — Transformation Plan (Phase 0.7)

- Canonical plan IR (`src/plan/model.rs`)
- Deterministic COM → plan lowering (`src/plan/lowering.rs`)
- Acyclic dependency graph (`src/plan/graph.rs`)
- Plan validation (`src/plan/validate.rs`)
- Golden plan fixtures under `tests/fixtures/plans/`

Additional CLI command:

```bash
dtcs plan <contract>
```

Do not implement compilers or runtime execution yet.

## Phase 12 — Plan Optimization (Phase 0.8)

- Semantics-preserving plan optimization (`src/plan/optimize.rs`, `src/plan/equivalence.rs`)
- Expression constant folding and algebraic simplification
- Registry-gated deterministic function evaluation
- Action fusion and rule deduplication
- Golden optimized plan fixtures under `tests/fixtures/plans_optimized/`

Additional CLI command:

```bash
dtcs optimize <contract-or-plan>
```

## Phase 13 — Execution Pipeline (Phase 0.9)

- Engine capability model and embedded `dtcs:reference` profile
- Capability matching against transformation plans
- `ExecutionPlan` IR and reference compiler
- In-memory reference runtime for embedded `dtcs:` stdlib entries
- Golden runtime fixtures and end-to-end `customer_normalize` coverage

Additional CLI commands:

```bash
dtcs match <contract-or-plan>
dtcs compile <contract-or-plan>
dtcs run <contract> --input <json>
```
