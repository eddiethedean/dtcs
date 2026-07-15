# Python API reference

Package: [`dtcs` on PyPI](https://pypi.org/project/dtcs/). Source: [`python/dtcs/__init__.py`](https://github.com/eddiethedean/dtcs/blob/main/python/dtcs/__init__.py).

CLI JSON shapes in [json-output.md](../user/json-output.md) use the same camelCase keys as Python dicts.

## Install

```bash
pip install 'dtcs==0.11.0'
```

```python
import dtcs
print(dtcs.__version__)   # package version, e.g. "0.11.0"
print(dtcs.SPEC_VERSION)  # "1.0.0-draft"
```

## Errors and validity

Most APIs return **dicts** that may include a `diagnostics` list (and sometimes other fields). They do not raise for validation failures.

| Helper | Behavior |
|--------|----------|
| `is_valid(report)` | `True` when no diagnostic has error severity. Missing `severity` is treated as an error. |

```python
report = dtcs.parse_and_validate(yaml_bytes)
if not dtcs.is_valid(report):
    for d in report["diagnostics"]:
        print(d["severity"], d["id"], d["message"])
```

## Parse and validate

| Function | Arguments | Returns |
|----------|-----------|---------|
| `parse(content, format="yaml")` | `bytes`/`str`, format `yaml`\|`json` | `{contract?, diagnostics}` |
| `parse_file(path)` | filesystem path | `{contract?, diagnostics}` |
| `validate(contract, registry_path=None)` | COM dict | validation report |
| `validate_with_registry(contract, registry_path)` | COM dict + vendor registry path | validation report |
| `parse_and_validate(content, format="yaml")` | document bytes | combined report |
| `validate_result(result, registry_path=None)` | parse result dict | merge parse + validate diagnostics |
| `is_valid(report)` | any report-like dict | `bool` |

### Example

```python
import urllib.request
import dtcs

url = "https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml"
content = urllib.request.urlopen(url).read()
report = dtcs.parse_and_validate(content)
assert dtcs.is_valid(report)
contract = dtcs.parse(content)["contract"]
assert contract["dtcsVersion"] == "1.0.0"
```

## Analysis and planning

| Function | Returns (typical) |
|----------|-------------------|
| `analyze(contract, registry_path=None)` | analysis diagnostics report |
| `plan_lower(contract, registry_path=None)` | `{plan, diagnostics}` |
| `plan_validate(plan, registry_path=None)` | report |
| `plan_optimize(plan, registry_path=None, *, validate=True)` | `{plan, diagnostics}` |
| `plan_equivalent(before, after)` | `bool` |
| `plan_topological_order(contract, plan)` | ordered node id list |
| `metadata_validate(contract)` / `version_validate(contract)` | focused reports |

## Compatibility and lineage

| Function | Notes |
|----------|-------|
| `compat_analyze(source, target, scope=None)` | classification of target vs source |
| `evolve_analyze(older, newer)` | same-identity revision analysis |
| `lineage_analyze(contract, impact=None, dependency=None)` | dataset lineage |
| `inspect(contract)` | summary dict (`inputs`, `outputs`, `semanticActions`, `rules`, …) |

## Registry

| Function | Notes |
|----------|-------|
| `registry_list(registry_path=None)` | entry list |
| `registry_resolve(id, registry_path=None)` | single entry or `None` |
| `registry_load(path)` | load document |

## Execution pipeline

| Function | Notes |
|----------|-------|
| `capability_reference_profile()` | embedded `dtcs:reference` |
| `capability_match(plan, profile=None)` | `{supported, …}` |
| `compile_plan(plan)` | `{plan, diagnostics}` execution plan |
| `execution_validate(plan)` | report |
| `runtime_execute(execution_plan, inputs)` | `{outputs, diagnostics}` |

`inputs` map interface id → list of row dicts. Cell values may be JSON `null`, or `{"$dtcs":"missing"}` / `{"$dtcs":"invalid"}`. **Do not coerce** missing/invalid to `None`.

```python
import json, dtcs

parsed = dtcs.parse_file("examples/customer_pipeline.dtcs.yaml")
contract = parsed["contract"]
plan = dtcs.plan_lower(contract)["plan"]
compiled = dtcs.compile_plan(plan)["plan"]
inputs = json.loads(open("tests/fixtures/runtime/customer_pipeline_input.json").read())
result = dtcs.runtime_execute(compiled, inputs)
assert dtcs.is_valid(result)
assert len(result["outputs"]["customer_clean"]) == 2
```

## Conformance

| Function | Notes |
|----------|-------|
| `conformance_declare(profile=None)` | Ch 23 capability declaration |
| `conformance_run(profile=None)` | offline report (`None`/`"all"` runs every profile) |

## Typing

The wheel does not yet ship `py.typed` / `.pyi` stubs. Treat return values as JSON-compatible dicts/lists.

## See also

- [Rust API](rust.md) · [WASM](wasm.md) · [Node](node.md)
- [migration-0.11.md](../user/migration-0.11.md)
