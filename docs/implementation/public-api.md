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

## Contract analysis (Phase 0.3)

```rust
use dtcs::{
    analyze_compatibility, analyze_evolution, analyze_lineage, ComparisonScope,
    CompatibilityLevel, parse_file,
};

let source = parse_file("examples/analysis/backward_old.yaml")?.into_contract()?;
let target = parse_file("examples/analysis/backward_new.yaml")?.into_contract()?;

let compat = analyze_compatibility(&source, &target, ComparisonScope::all());
assert_eq!(compat.level, CompatibilityLevel::BackwardCompatible);

let evolution = analyze_evolution(&source, &target);
let lineage = analyze_lineage(&source);
```

```python
import dtcs

older = dtcs.parse_file("examples/analysis/backward_old.yaml")["contract"]
newer = dtcs.parse_file("examples/analysis/backward_new.yaml")["contract"]

compat = dtcs.compat_analyze(older, newer)
assert compat["level"] == "backwardCompatible"

evolution = dtcs.evolve_analyze(older, newer)
lineage = dtcs.lineage_analyze(older)
```

Versioning validation (Ch 25) runs during full contract validation when a `versioning` block is present, and is also available standalone:

```rust
use dtcs::versioning;

let report = versioning::validate(&contract);
```

## Semantic analysis (Phase 0.6)

Static semantic analysis checks transformation semantics (Ch 7) and expression semantics (Ch 8) without runtime evaluation.

```rust
use dtcs::{analysis, parse_file};

let contract = parse_file("contract.dtcs.yaml")?.into_contract()?;
let report = analysis::check_contract(&contract, None);
assert!(report.is_valid());
```

```python
import dtcs

contract = dtcs.parse_file("contract.dtcs.yaml")["contract"]
result = dtcs.analyze(contract)
assert dtcs.is_valid(result["validation"])
assert dtcs.is_valid(result["analysis"])
```

## Registry (Phase 0.4–0.5)

The embedded registry includes diagnostic codes, the reserved `dtcs` namespace,
and starter standard libraries for semantic actions, functions, and rules.
Entries may carry a JSON `definition` field consumed by semantics validation.

```rust
use dtcs::{
    default_registry, load_registry, resolve_registry, validate_with_registry,
};

let registry = default_registry();
let entry = resolve_registry(registry, "dtcs:lowercase").expect("builtin action");

let vendor = load_registry("vendor_catalog.yaml")?;
let mut merged = registry.clone();
merged.merge(&vendor);
let report = validate_with_registry(&contract, &merged);
```

```python
import dtcs

entries = dtcs.registry_list()
entry = dtcs.registry_resolve("dtcs:lowercase")
catalog = dtcs.registry_load("vendor_catalog.yaml")
```

## Python API

The `dtcs` package mirrors the Rust parse/validate surface. Contract dicts must use **camelCase** keys to match the Canonical Object Model (`dtcsVersion`, `semanticActions`, etc.).

```python
import dtcs

result = dtcs.parse(yaml_text, "yaml")
contract = result["contract"]
report = dtcs.validate(contract)
assert dtcs.is_valid(report)

result = dtcs.parse_file("contract.dtcs.yaml")
merged = dtcs.validate_result(result)
report = dtcs.parse_and_validate(yaml_text, "yaml")

metadata_report = dtcs.metadata_validate(contract)
summary = dtcs.inspect(contract)
```

| Symbol | Description |
|--------|-------------|
| `dtcs.SPEC_VERSION` | DTCS specification version targeted by this build |
| `dtcs.__version__` | Installed package version (`0.0.0+dev` when running from source without metadata) |
| `parse` / `parse_file` | Parse YAML or JSON into `{"contract": ..., "report": ...}` |
| `validate` / `metadata_validate` | Validate a parsed contract dict |
| `validate_result` | Merge parse-time and validation diagnostics |
| `parse_and_validate` | Parse and validate in one step |
| `inspect` | Human-readable contract summary |
| `is_valid` | True when a diagnostic report has no error-severity items |
| `compat_analyze` | Compare two contracts; returns level, aspects, diagnostics |
| `evolve_analyze` | Evolution diff between two revisions of the same contract |
| `lineage_analyze` | Dependency graph, impact, and governance summary |
| `version_validate` | Ch 25 versioning block validation |
| `registry_list` | List registry entries (optional vendor catalog path) |
| `registry_resolve` | Resolve an identifier to a registry entry or `None` |
| `registry_load` | Load a registry document from a file path |

## CLI

Both the Rust crate (`cargo install dtcs`) and the Python package (`pip install dtcs`) install a `dtcs` command on `PATH`.

The `dtcs` binary is enabled by default in the Rust crate (`cli` feature):

```bash
dtcs validate contract.yaml
dtcs validate contract.yaml --json
dtcs inspect contract.yaml
dtcs inspect contract.yaml --json
dtcs diagnostics contract.yaml --json
dtcs version
dtcs compat source.yaml target.yaml
dtcs compat source.yaml target.yaml --json --scope interfaces,types
dtcs evolve older.yaml newer.yaml --json
dtcs lineage contract.yaml --impact INPUT_ID --json
dtcs registry list
dtcs registry resolve dtcs:lowercase --json
dtcs registry resolve acme:transform --registry vendor_catalog.yaml
```

The Python package exposes the same subcommands via `python -m dtcs` or the `dtcs` console script.

## Type system (Phase 0.2)

```rust
use dtcs::{parse_logical_type, type_compatible, TypeCompatibility};

let integer = parse_logical_type("integer").expect("primitive");
let list = parse_logical_type("list<string>").expect("composite");
assert_eq!(type_compatible(&integer, &integer), TypeCompatibility::Identical);
```

Expression and function typing runs during the Types validation phase. Expressions with bodies must declare a `type`; functions must declare a return `type`. The validator infers expression types from field references, literals, unary operators, precedence-aware binary operators, and in-contract function calls.

Use terminology from [`SPEC.md`](../../SPEC.md). When this guide conflicts with the specification, **SPEC.md wins**.
