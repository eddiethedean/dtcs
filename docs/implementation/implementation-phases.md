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

## Phase 7 — Compatibility and Plan Stubs

- Add compatibility model skeleton.
- Add Transformation Plan skeleton.
- Do not implement compilers yet.
