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

## Validation semantics

- `DiagnosticReport::is_valid()` returns `false` only when one or more **Error**-severity diagnostics are present.
- **Warning** and **Information** diagnostics do not block validation success.

Diagnostics must not alter transformation semantics.
