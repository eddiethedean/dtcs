# Contributing to DTCS

Thank you for contributing to the Data Transformation Contract Standard and its reference implementation.

## Specification changes

Normative changes belong in [SPEC.md](SPEC.md). Follow the editorial process documented in:

- [docs/editorial/baseline.md](docs/editorial/baseline.md) — mandatory editorial conventions
- [docs/editorial/style-guide.md](docs/editorial/style-guide.md) — terminology and prose style
- [docs/editorial/authoring-guide.md](docs/editorial/authoring-guide.md) — chapter structure and normative language
- [docs/editorial/review-checklist.md](docs/editorial/review-checklist.md) — pre-publication review

### Normative language

Use RFC 2119 / RFC 8174 keywords (`MUST`, `SHALL`, `SHOULD`, `MAY`, etc.) for requirements.

### Authority

[SPEC.md](SPEC.md) is the single source of truth for semantics, terminology, and conformance. Implementation docs in [docs/implementation/](docs/implementation/) are illustrative unless explicitly normative.

## Implementation changes

The Rust reference crate lives in [src/](src/). Before implementing a module:

1. Read the relevant [SPEC.md](SPEC.md) chapter(s).
2. Consult [docs/implementation/](docs/implementation/) for architecture and API guidance.
3. Preserve spec-aligned naming and behavior.
4. Add tests that exercise spec requirements. See [docs/implementation/testing-plan.md](docs/implementation/testing-plan.md).

### Scope

The initial crate targets parsing, the canonical object model, validation, and diagnostics. Do not add execution, compilation, or runtime features without an agreed milestone. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

### Code style

- Run `cargo fmt` and `cargo clippy` before submitting changes.
- Keep modules aligned with [docs/implementation/crate-layout.md](docs/implementation/crate-layout.md).
- Prefer conservative behavior when the spec is ambiguous; document open questions with a `TODO` referencing the spec section.

## Pull requests

1. Describe whether the change is specification, implementation, or editorial.
2. Link related issues or design discussions when available.
3. Include or update tests for behavioral changes.
4. Ensure `cargo test` passes.

## Questions

For ambiguous spec language, open an issue with the relevant chapter and section number rather than inventing behavior in code.
