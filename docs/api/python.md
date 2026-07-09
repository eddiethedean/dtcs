# Python API reference

Public functions exported from `dtcs` (see [`python/dtcs/__init__.py`](../../python/dtcs/__init__.py)).

## Version

| Function | Description |
|----------|-------------|
| `SPEC_VERSION` | DTCS specification version string |
| `__version__` | Installed package version |

## Parse and validate

| Function | Description |
|----------|-------------|
| `parse(content, format="yaml")` | Parse a DTCS document |
| `parse_file(path)` | Parse from a file path |
| `validate(contract, registry_path=None)` | Validate a parsed contract |
| `validate_with_registry(contract, registry_path)` | Validate with merged vendor registry |
| `parse_and_validate(content, format="yaml")` | Parse and validate in one step |
| `validate_result(result, registry_path=None)` | Merge parse and validation diagnostics |
| `is_valid(report)` | True when no error-severity diagnostics |

## Analysis and planning

| Function | Description |
|----------|-------------|
| `analyze(contract, registry_path=None)` | Static semantic analysis |
| `plan_lower(contract, registry_path=None)` | Lower to transformation plan |
| `plan_validate(plan, registry_path=None)` | Validate a transformation plan |
| `plan_optimize(plan, registry_path=None, *, validate=True)` | Optimize a plan |
| `plan_equivalent(before, after)` | Semantic plan equivalence |
| `plan_topological_order(contract, plan)` | Topological execution order |
| `metadata_validate(contract)` | Metadata-only validation |
| `version_validate(contract)` | Version identifier validation |

## Compatibility and lineage

| Function | Description |
|----------|-------------|
| `compat_analyze(source, target, scope=None)` | Compatibility between contracts |
| `evolve_analyze(older, newer)` | Evolution analysis |
| `lineage_analyze(contract, impact=None, dependency=None)` | Dataset-level lineage |
| `inspect(contract)` | Human-readable summary |

## Registry

| Function | Description |
|----------|-------------|
| `registry_list(registry_path=None)` | List registry entries |
| `registry_resolve(id, registry_path=None)` | Resolve an identifier |
| `registry_load(path)` | Load a registry document |

## Execution pipeline

| Function | Description |
|----------|-------------|
| `capability_reference_profile()` | Embedded `dtcs:reference` profile |
| `capability_match(plan, profile=None)` | Match plan against capabilities |
| `compile_plan(plan)` | Compile to execution plan |
| `execution_validate(plan)` | Validate execution plan |
| `runtime_execute(execution_plan, inputs)` | Execute with runtime inputs |

## Conformance (Phase 0.10)

| Function | Description |
|----------|-------------|
| `conformance_declare(profile=None)` | Ch 23 §9 capability declaration JSON |
| `conformance_run(profile=None)` | Offline conformance report (`None` or `all` runs every profile) |

See [conformance.md](../user/conformance.md) for CLI equivalents and report interpretation.
