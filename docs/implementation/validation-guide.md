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

Invalid contracts should produce diagnostics with:

- id
- severity
- category
- stage
- message
- location/object reference
- optional remediation
