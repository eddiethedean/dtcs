# Diagnostics Guide

Diagnostics are spec-level observations implemented in [`src/diagnostics/`](../../src/diagnostics/).

## Types

```rust
pub enum Severity {
    Information,
    Warning,
    Error,
}

pub enum DiagnosticStage {
    Parse,
    CanonicalObjectModel,
    Validation,
    Analysis,
    Planning,
    Compilation,
    Runtime,
}

pub enum DiagnosticCategory {
    Syntax,
    Structure,
    Type,
    Reference,
    Semantic,
    Compatibility,
    Capability,
    Runtime,
    Extension,
}

pub struct Diagnostic {
    pub id: String,
    pub severity: Severity,
    pub stage: DiagnosticStage,
    pub category: DiagnosticCategory,
    pub message: String,
    pub object_ref: Option<String>,
    pub remediation: Option<String>,
}
```

## Standard diagnostic identifiers

Defined in [`src/diagnostics/codes.rs`](../../src/diagnostics/codes.rs):

- `dtcs:parse-error`
- `dtcs:unsupported-version`
- `dtcs:missing-required-field`
- `dtcs:duplicate-identifier`
- `dtcs:invalid-identifier`
- `dtcs:unknown-field`
- `dtcs:missing-lineage`
- `dtcs:ambiguous-reference`
- `dtcs:invalid-type`
- `dtcs:unresolved-reference`
- `dtcs:invalid-semantic-action`
- `dtcs:invalid-rule`
- `dtcs:invalid-extension`
- `dtcs:invalid-metadata` (Phase 0.2)
- `dtcs:invalid-interface` (Phase 0.2)
- `dtcs:type-incompatible` (Phase 0.2)
- `dtcs:invalid-conversion` (Phase 0.2)
- `dtcs:incompatible-contract` (Phase 0.3)
- `dtcs:conditional-compatibility` (Phase 0.3)
- `dtcs:evolution-breaking-change` (Phase 0.3)
- `dtcs:deprecated-object` (Phase 0.3)
- `dtcs:invalid-version` (Phase 0.3)
- `dtcs:version-conflict` (Phase 0.3)
- `dtcs:unknown-registry-entry` (Phase 0.4)
- `dtcs:invalid-registry` (Phase 0.4)
- `dtcs:unsupported-extension` (Phase 0.4)
- `dtcs:invalid-function` (Phase 0.5)
- `dtcs:invalid-expression` (Phase 0.6)
- `dtcs:invalid-semantics` (Phase 0.6)
- `dtcs:non-deterministic-semantics` (Phase 0.6)
- `dtcs:null-semantics-violation` (Phase 0.6)
- `dtcs:invalid-plan` (Phase 0.7)
- `dtcs:incomplete-plan` (Phase 0.7)
- `dtcs:cyclic-dependency` (Phase 0.7)
- `dtcs:plan-type-mismatch` (Phase 0.7)
- `dtcs:unresolved-plan-reference` (Phase 0.7)

## Validation semantics

- `DiagnosticReport::is_valid()` returns `false` only when one or more **Error**-severity diagnostics are present.
- **Warning** and **Information** diagnostics do not block validation success.

Diagnostics must not alter transformation semantics.
