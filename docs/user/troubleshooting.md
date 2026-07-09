# Troubleshooting

Common problems when installing, running, or authoring DTCS contracts.

## Installation

### `pip install dtcs` tries to compile from source

Pre-built wheels are published for common platforms and Python 3.9–3.13. If pip falls back to a source build:

1. Ensure you are on a supported Python version.
2. Install Rust 1.75+ and [maturin](https://www.maturin.rs/).
3. Or pin a released version: `pip install dtcs==0.9.0`.

See [faq.md](faq.md#pip-install-dtcs-fails-to-build-what-do-i-do).

### `dtcs: command not found` after pip install

The `dtcs` CLI should be on your PATH after `pip install dtcs`. If not:

```bash
python -m dtcs version
```

Add your Python scripts directory to `PATH`, or use `python -m dtcs` in scripts and CI.

### `cargo install dtcs` is slow

`cargo install` compiles from source. This is normal on first install. Subsequent updates reuse the cargo cache.

## Path and working directory

### Example commands fail with "file not found"

Most repository examples assume you run commands from the **repository root**:

```bash
dtcs validate examples/customer_normalize.dtcs.yaml
```

If you are in the `examples/` directory, adjust paths or return to the repo root. See [examples/README.md](../../examples/README.md).

### `dtcs run --input` file not found

The `--input` path is relative to your current working directory. Clone the repository or provide an absolute path to your input JSON.

Pip-only installs do not include `examples/` or `tests/fixtures/`. Copy input JSON from the repository or author your own.

## Validation errors

Run diagnostics for structured output:

```bash
dtcs diagnostics my_contract.yaml
dtcs diagnostics my_contract.yaml --json
```

| Diagnostic | Likely cause | Fix |
|------------|--------------|-----|
| `dtcs:unsupported-version` | `dtcsVersion` not exactly `1.0.0` | Set `dtcsVersion: "1.0.0"` (patch variants like `1.0.1` are rejected) |
| `dtcs:missing-lineage` | Output without lineage mapping | Add `lineage.mappings` entry for each output |
| `dtcs:unresolved-reference` | Field path does not exist | Use `interface.field` format matching an input/output `id` |
| `dtcs:invalid-type` | Malformed type expression | Use `list<string>`, not bare `list` |
| `dtcs:unknown-registry-entry` | Unrecognized `dtcs:` identifier | Run `dtcs registry list`; use embedded stdlib identifiers |
| stdlib semantics errors | Wrong target type, nullability, or rule phase | Run `dtcs registry resolve dtcs:IDENTIFIER --json` |

See [writing-contracts.md](writing-contracts.md#validate-as-you-write) for more diagnostic fixes.

## Analysis and planning

### `dtcs analyze` exits non-zero on a valid contract

`analyze` requires a valid contract **and** no error-severity analysis diagnostics. Warnings may still appear. Run with `--json` to inspect `validation` and `analysis` sections separately.

### `dtcs plan` fails after `validate` succeeds

Planning also requires static analysis to pass. Run `dtcs analyze` first and fix expression or semantics errors.

## Python API

### `KeyError` on `runtime_execute` result

`dtcs.runtime_execute` returns an envelope, not a flat output map:

```python
result = dtcs.runtime_execute(execution_plan, inputs)
outputs = result["outputs"]  # not result["customer_clean"]
```

The CLI `dtcs run --json` emits a flat output map. See [json-output.md](json-output.md#run).

### Contract dict keys rejected

Python contract dicts must use **camelCase** keys (`dtcsVersion`, `semanticActions`), matching the Canonical Object Model. See [json-output.md](json-output.md#python-note).

### `is_valid` on wrong object

`dtcs.is_valid` checks a diagnostic report (`{"diagnostics": [...]}`). For `runtime_execute`, pass the full result or check `result["outputs"]` separately.

## CI integration

See [ci-integration.md](ci-integration.md) for validate/analyze/compat gates with JSON parsing.

## Getting help

- [FAQ](faq.md)
- [GitHub issues](https://github.com/eddiethedean/dtcs/issues) — include `dtcs version`, the command run, and `dtcs diagnostics --json` output
- [SPEC.md](../../SPEC.md) for normative behavior
