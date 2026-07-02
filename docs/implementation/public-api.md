# Public API

The reference crate exposes a small, spec-aligned API from [`src/lib.rs`](../../src/lib.rs).

## Parse and validate

```rust
use dtcs::{parse, parse_file, parse_and_validate, validate, DocumentFormat};

// From bytes
let result = parse(yaml_bytes, DocumentFormat::Yaml);
let report = result.validate();

// One-shot
let report = parse_and_validate(yaml_bytes, DocumentFormat::Yaml);
assert!(report.is_valid());

// From path
let result = parse_file("contract.dtcs.yaml")?;
```

## `TransformationContract` helpers

```rust
use dtcs::TransformationContract;

let result = TransformationContract::from_yaml(yaml_text);
let result = TransformationContract::from_json(json_text);
let result = TransformationContract::from_file("contract.dtcs.yaml")?;

if let Ok(contract) = result.into_contract() {
    let report = contract.validate();
}
```

## Diagnostics

```rust
use dtcs::{DiagnosticReport, Severity, codes};

let report: DiagnosticReport = /* ... */;
assert!(report.is_valid()); // true when no Error-severity diagnostics
for error in report.errors() {
    assert_eq!(error.severity, Severity::Error);
}
```

## Metadata validation

Phase 0.2 adds a standalone metadata validator that is also invoked during full contract validation:

```rust
use dtcs::{metadata, parse, DocumentFormat};

let result = parse(yaml_bytes, DocumentFormat::Yaml);
let contract = result.into_contract().expect("valid parse");
let report = metadata::validate(&contract);
```

## CLI

The `dtcs` binary is enabled by default (`cli` feature):

```bash
dtcs validate contract.yaml
dtcs inspect contract.yaml
dtcs diagnostics contract.yaml --json
dtcs version
```

Use terminology from [`SPEC.md`](../../SPEC.md). When this guide conflicts with the specification, **SPEC.md wins**.
