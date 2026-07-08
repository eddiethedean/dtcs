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

## Phase 10 — Plan Stubs

- Add Transformation Plan skeleton (`src/plan/`).
- Do not implement compilers yet.
