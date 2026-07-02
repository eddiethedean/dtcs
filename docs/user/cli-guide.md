# CLI Guide

Both the Rust crate (`cargo install dtcs`) and the Python package (`pip install dtcs`) install a `dtcs` command. The Python package also supports `python -m dtcs`.

## Commands

| Command | Description |
|---------|-------------|
| `validate <path>` | Parse and validate a contract |
| `inspect <path>` | Print a human-readable contract summary |
| `diagnostics <path>` | Print validation diagnostics |
| `compat <source> <target>` | Compare compatibility between two contracts |
| `evolve <older> <newer>` | Analyze evolution between two revisions |
| `lineage <path>` | Analyze dataset-level lineage |
| `version` | Print tool and specification versions |

All commands accept `--json` for structured output except where noted below.

## validate

```bash
dtcs validate contract.yaml
dtcs validate contract.yaml --json
```

| Exit code | Meaning |
|-----------|---------|
| `0` | Contract is valid (no error-severity diagnostics) |
| `1` | Contract is invalid |

Warnings and information-level diagnostics do not cause a non-zero exit.

## inspect

```bash
dtcs inspect contract.yaml
dtcs inspect contract.yaml --json
```

Requires a **valid** contract. Exits non-zero if parsing or validation fails.

## diagnostics

```bash
dtcs diagnostics contract.yaml
dtcs diagnostics contract.yaml --json
```

Same validation as `validate`, but always prints diagnostics (even when valid).

## compat

```bash
dtcs compat older.yaml newer.yaml
dtcs compat older.yaml newer.yaml --scope interfaces,types
dtcs compat older.yaml newer.yaml --json
```

Compares two **valid** contracts. Optional `--scope` accepts comma-separated aspects:

`interfaces` · `types` · `semantics` · `lineage` · `metadata` · `extensions` · `all`

Default scope is `all` when `--scope` is omitted.

| Exit code | Meaning |
|-----------|---------|
| `0` | Compatible (any level except Incompatible) |
| `1` | Incompatible |

## evolve

```bash
dtcs evolve rev1.yaml rev2.yaml
dtcs evolve rev1.yaml rev2.yaml --json
```

Compares two revisions. Intended for contracts with the same `id`.

| Exit code | Meaning |
|-----------|---------|
| `0` | Same identity and not incompatible |
| `1` | Different identity or incompatible evolution |

## lineage

```bash
dtcs lineage contract.yaml
dtcs lineage contract.yaml --impact INPUT_ID
dtcs lineage contract.yaml --dependency OUTPUT_ID
dtcs lineage contract.yaml --json
```

| Flag | Effect |
|------|--------|
| `--impact INPUT` | List outputs affected by the given input (transitive) |
| `--dependency OUTPUT` | List inputs required by the given output |

Always exits `0` on success.

## version

```bash
dtcs version
dtcs version --json
```

Prints crate/package version and targeted specification version.

## CI integration

Example GitHub Actions step:

```yaml
- name: Validate DTCS contract
  run: |
    pip install dtcs
    dtcs validate path/to/contract.yaml --json
```

Use exit codes for pass/fail gates. Parse `--json` output for structured diagnostics in PR comments.

See [json-output.md](json-output.md) for field definitions.
