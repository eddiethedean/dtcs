# Python API reference

Package: [`dtcs` on PyPI](https://pypi.org/project/dtcs/). Source: [`python/dtcs/__init__.py`](https://github.com/eddiethedean/dtcs/blob/main/python/dtcs/__init__.py).

CLI JSON shapes in [json-output.md](../user/json-output.md) use the same camelCase keys as Python dicts.

## Install

```bash
pip install 'dtcs==0.12.0'
```

```python
import dtcs
print(dtcs.__version__)   # package version, e.g. "0.12.0"
print(dtcs.SPEC_VERSION)  # "2.0.0"
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

`analyze` returns nested reports — check each:

```python
result = dtcs.analyze(contract)
assert dtcs.is_valid(result["validation"])
assert dtcs.is_valid(result["analysis"])
```

## Parse and validate

| Function | Arguments | Returns |
|----------|-----------|---------|
| `parse(content, format="yaml")` | `bytes`/`str`, format `yaml`\|`json` | `{contract, report: {diagnostics}}` |
| `parse_file(path)` | filesystem path | `{contract, report: {diagnostics}}` |
| `validate(contract, registry_path=None)` | COM dict | `{diagnostics}` |
| `validate_with_registry(contract, registry_path)` | COM dict + vendor registry path | `{diagnostics}` |
| `parse_and_validate(content, format="yaml")` | document bytes | `{diagnostics}` |
| `validate_result(result, registry_path=None)` | parse result dict | `{diagnostics}` |
| `is_valid(report)` | any report-like dict with `diagnostics` | `bool` |

### Example

```python
import urllib.request
import dtcs

url = "https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml"
content = urllib.request.urlopen(url).read()
report = dtcs.parse_and_validate(content)
assert dtcs.is_valid(report)
contract = dtcs.parse(content)["contract"]
assert contract["dtcsVersion"] == "2.0.0"
```

## Analysis and planning

| Function | Returns (typical) |
|----------|-------------------|
| `analyze(contract, registry_path=None)` | `{validation, analysis}` (each has `diagnostics`) |
| `plan_lower(contract, registry_path=None)` | `{plan, diagnostics}` |
| `plan_validate(plan, registry_path=None)` | `{diagnostics}` |
| `plan_optimize(plan, registry_path=None, *, validate=True)` | `{plan, diagnostics}` |
| `plan_equivalent(before, after)` | `bool` |
| `plan_topological_order(contract, plan)` | ordered node id list |
| `metadata_validate(contract)` / `version_validate(contract)` | focused `{diagnostics}` reports |

## Compatibility and lineage

| Function | Notes |
|----------|-------|
| `compat_analyze(source, target, scope=None)` | classification of target vs source |
| `evolve_analyze(older, newer)` | same-identity revision analysis |
| `lineage_analyze(contract, impact=None, dependency=None)` | dataset lineage |
| `inspect(contract)` | human-readable **string** summary (not a dict) |

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
| `capability_match(plan, profile=None)` | `{supported, diagnostics, …}` |
| `compile_plan(plan)` | `{plan, diagnostics}` execution plan |
| `execution_validate(plan)` | `{diagnostics}` |
| `runtime_execute(execution_plan, inputs)` | `{outputs, diagnostics}` |

`inputs` map interface id → list of row dicts. Cell values may be JSON `null`, or `{"$dtcs":"missing"}` / `{"$dtcs":"invalid"}`. **Do not coerce** missing/invalid to `None`. Fixture dialect note: [expressions.md](../user/expressions.md#null-missing-and-invalid).

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

## Portable plans

| Function | Arguments | Returns |
|----------|-----------|---------|
| `plan_export_portable(plan, profile=...)` | lowered plan dict; default profile `dtcs:profile/portable-relational-kernel/1` | portable plan dict (`identity`, nodes, …) |
| `plan_fingerprint(portable_plan)` | portable plan dict | SHA-256 hex string |
| `expression_to_structured(source)` | expression source string | structured AST dict |
| `capability_portable_manifest(profile=...)` | portable profile id | per-entry capability manifest |

```python
import dtcs

contract = dtcs.parse_file("contract.dtcs.yaml")["contract"]
plan = dtcs.plan_lower(contract)["plan"]
portable = dtcs.plan_export_portable(plan)
fp = dtcs.plan_fingerprint(portable)
manifest = dtcs.capability_portable_manifest()
assert portable["identity"] == "dtcs.transform-plan/1"
assert isinstance(fp, str) and len(fp) == 64
```

The Rust CLI exposes the same export as `dtcs export-portable`. The Python CLI (`python -m dtcs`) does **not** yet wrap this subcommand — use the functions above.

## Conformance

| Function | Notes |
|----------|-------|
| `conformance_declare(profile=None)` | Ch 23 capability declaration |
| `conformance_run(profile=None)` | offline report (`None`/`"all"` runs every profile). Fixtures are embedded in the wheel; optional `DTCS_FIXTURES` overrides the on-disk search path. |

## Typing

The wheel does not yet ship `py.typed` / `.pyi` stubs. Treat return values as JSON-compatible dicts/lists (except `inspect`, which returns `str`). Longer-term: TypedDicts or published JSON Schema for envelopes ([json-output.md](../user/json-output.md)).

## Raises vs diagnostics

Most APIs return diagnostic dicts and do **not** raise for invalid contracts. Exceptions typically indicate programmer error (e.g. `None` where a contract dict is required) or I/O failures. See [error-taxonomy.md](../user/error-taxonomy.md).

## See also

- [Rust API](rust.md) · [WASM](wasm.md) · [Node](node.md)
- [migration-0.12.md](../user/migration-0.12.md)
- [cli-guide.md](../user/cli-guide.md#export-portable)
