# Validation Guide

Validation should be phase-based and deterministic.

The validation phases must follow `SPEC.md`:

1. Document Validation
2. Canonical Object Model Validation
3. Structural Validation
4. Type Validation
5. Reference Validation
6. Semantic Validation
7. Extension Validation

Validation should return a `ValidationReport`, not panic.

Semantic and extension phases resolve `dtcs:` identifiers through the
embedded registry (`registry::default_registry`). Standard library entries
include JSON `definition` blocks; the semantics pass validates actions,
functions, and rules against those definitions (target type, nullability,
rule phases, arity, return types). Callers may inject a vendor catalog with
`validate_with_registry`. Unsupported mandatory extensions produce
`dtcs:unsupported-extension`.

Invalid contracts should produce diagnostics with:

- id
- severity
- category
- stage
- message
- location/object reference
- optional remediation
