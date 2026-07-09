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

## Transformation plan (Phase 0.7)

Lowering produces the canonical semantic IR (Ch 13) from a validated contract.

```rust
use dtcs::{analysis, parse_file, plan};

let contract = parse_file("contract.dtcs.yaml")?.into_contract()?;
let analysis = analysis::check_contract(&contract, None);
let result = plan::lower(&contract, None, Some(&analysis));
assert!(result.is_valid());
let transformation_plan = result.plan.expect("plan");

let validation = plan::validate(&transformation_plan);
assert!(validation.is_valid());
```

```python
import dtcs

contract = dtcs.parse_file("contract.dtcs.yaml")["contract"]
result = dtcs.plan_lower(contract)
assert dtcs.is_valid({"diagnostics": result["diagnostics"]})
plan = result["plan"]
assert dtcs.is_valid(dtcs.plan_validate(plan))
```

One-shot convenience:

```rust
use dtcs::{parse_validate_and_plan, DocumentFormat};

let result = parse_validate_and_plan(yaml_bytes, DocumentFormat::Yaml);
```

## Plan optimization (Phase 0.8)

Optimization applies semantics-preserving rewrites to a lowered plan.

```rust
use dtcs::plan::{equivalent, optimize};

let optimized = optimize(&transformation_plan);
assert!(optimized.is_valid());
assert!(equivalent(&transformation_plan, optimized.plan.as_ref().unwrap()));
```

```python
import dtcs

optimized = dtcs.plan_optimize(plan)
assert dtcs.is_valid({"diagnostics": optimized["diagnostics"]})
assert dtcs.plan_equivalent(plan, optimized["plan"])
```

One-shot convenience:

```rust
use dtcs::{parse_validate_and_optimize, DocumentFormat};

let result = parse_validate_and_optimize(yaml_bytes, DocumentFormat::Yaml);
```

## Execution pipeline (Phase 0.9)

### Capability matching

```rust
use dtcs::{capability, parse_file, plan};

let contract = parse_file("contract.dtcs.yaml")?.into_contract()?;
let plan_result = plan::lower(&contract, None, None);
let plan = plan_result.plan.expect("plan");
let profile = capability::reference_profile();
let match_report = capability::match_plan(&plan, &profile);
assert!(match_report.is_valid());
```

### Compilation

```rust
use dtcs::compile;

let compile_result = compile::compile(&plan);
assert!(compile_result.is_valid());
let execution_plan = compile_result.plan.expect("execution plan");
```

### Runtime execution

```rust
use dtcs::runtime::{execute, RuntimeInputs, RuntimeValue};
use std::collections::BTreeMap;

let mut inputs = RuntimeInputs::new();
let mut row = BTreeMap::new();
row.insert("email".into(), RuntimeValue::String("ALICE@EXAMPLE.COM".into()));
inputs.insert("customer_raw".into(), vec![row]);

let result = execute(&execution_plan, &inputs);
assert!(result.is_valid());
```

```python
import dtcs

plan = dtcs.plan_lower(contract)["plan"]
match = dtcs.capability_match(plan)
assert dtcs.is_valid({"diagnostics": match["diagnostics"]})

compiled = dtcs.compile_plan(plan)
execution = compiled["plan"]
result = dtcs.runtime_execute(execution, inputs)
assert dtcs.is_valid(result)
assert result["outputs"]["customer_clean"][0]["email"] == "alice@example.com"
```

One-shot convenience:

```rust
use dtcs::{parse_validate_and_run, DocumentFormat, RuntimeInputs};

let result = parse_validate_and_run(yaml_bytes, DocumentFormat::Yaml, &inputs);
```

```rust
use dtcs::{parse_validate_and_compile, discover_capabilities, DocumentFormat};

let compile_result = parse_validate_and_compile(yaml_bytes, DocumentFormat::Yaml);
let profiles = discover_capabilities();
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
| `validate` / `metadata_validate` | Validate a parsed contract dict (`registry_path` optional) |
| `validate_with_registry` | Validate with an explicit vendor registry file path |
| `validate_result` | Merge parse-time and validation diagnostics |
| `parse_and_validate` | Parse and validate in one step |
| `analyze` | Static semantic and expression analysis |
| `inspect` | Human-readable contract summary |
| `is_valid` | True when a diagnostic report has no error-severity items |
| `compat_analyze` | Compare two contracts (`scope` optional); returns level, aspects, diagnostics |
| `evolve_analyze` | Evolution diff between two revisions of the same contract |
| `lineage_analyze` | Dependency graph, impact, and governance (`impact`, `dependency` optional) |
| `plan_lower` | Lower a validated contract to a transformation plan (`registry_path` optional) |
| `plan_validate` | Validate a transformation plan |
| `plan_topological_order` | Topological execution order for plan nodes |
| `plan_optimize` | Optimize a transformation plan (`validate=False` to skip validation; `registry_path` optional) |
| `plan_equivalent` | Compare two plans for semantic equivalence |
| `capability_reference_profile` | Embedded `dtcs:reference` engine capability profile |
| `capability_match` | Match a transformation plan against a capability profile (`profile` optional) |
| `compile_plan` | Compile a transformation plan to an execution plan |
| `execution_validate` | Validate an execution plan |
| `runtime_execute` | Execute an execution plan; returns `{"outputs": {...}, "diagnostics": [...]}` |
| `version_validate` | Ch 25 versioning block validation |
| `registry_list` | List registry entries (`registry_path` optional vendor catalog) |
| `registry_resolve` | Resolve an identifier to a registry entry or `None` (`registry_path` optional) |
| `registry_load` | Load a registry document from a file path |

## CLI

Both the Rust crate (`cargo install dtcs`) and the Python package (`pip install dtcs`) install a `dtcs` command on `PATH`.

**Full command reference:** [docs/user/cli-guide.md](../user/cli-guide.md) (flags, exit codes, CI examples).

**JSON output shapes:** [docs/user/json-output.md](../user/json-output.md).

The `dtcs` binary is enabled by default in the Rust crate (`cli` feature):

```bash
dtcs validate contract.yaml
dtcs validate contract.yaml --json
dtcs analyze contract.yaml --json
dtcs plan contract.yaml --json
dtcs optimize contract.yaml --json
dtcs optimize plan.json --plan --json
dtcs match contract.yaml --json
dtcs compile contract.yaml --json
dtcs run contract.yaml --input inputs.json --json
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

## Conformance (Phase 0.10)

```rust
use dtcs::{conformance_declare, conformance_run_all, ConformanceReport};

let declaration = conformance_declare();
assert_eq!(declaration.primary_profile, "integrated-platform");

let report: ConformanceReport = conformance_run_all();
assert!(report.is_valid());
```

CLI: `dtcs conformance declare`, `dtcs conformance run --profile all`.

Python: `conformance_declare()`, `conformance_run()`.

## Type system (Phase 0.2)

```rust
use dtcs::{parse_logical_type, type_compatible, TypeCompatibility};

let integer = parse_logical_type("integer").expect("primitive");
let list = parse_logical_type("list<string>").expect("composite");
assert_eq!(type_compatible(&integer, &integer), TypeCompatibility::Identical);
```

Expression and function typing runs during the Types validation phase. Expressions with bodies must declare a `type`; functions must declare a return `type`. The validator infers expression types from field references, literals, unary operators, precedence-aware binary operators, and in-contract function calls.

Use terminology from [`SPEC.md`](../../SPEC.md). When this guide conflicts with the specification, **SPEC.md wins**.
