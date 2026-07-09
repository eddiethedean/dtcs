# CLI Guide

Both the Rust crate (`cargo install dtcs`) and the Python package (`pip install dtcs`) install a `dtcs` command. The Python package also supports `python -m dtcs`.

## Commands

| Command | Description |
|---------|-------------|
| `validate <path>` | Parse and validate a contract |
| `analyze <path>` | Analyze transformation semantics and expressions (static; no runtime evaluation) |
| `inspect <path>` | Print a human-readable contract summary |
| `diagnostics <path>` | Print validation diagnostics |
| `compat <source> <target>` | Compare compatibility between two contracts |
| `evolve <older> <newer>` | Analyze evolution between two revisions |
| `plan <path>` | Lower a validated contract to a transformation plan |
| `optimize <path>` | Optimize a transformation plan (contract or serialized plan JSON) |
| `match <path>` | Match a transformation plan against engine capabilities |
| `compile <path>` | Compile a transformation plan to an execution plan |
| `run <path>` | Execute a contract end-to-end using the reference runtime |
| `lineage <path>` | Analyze dataset-level lineage |
| `registry list` | List identifier registry entries |
| `registry resolve <id>` | Resolve a registry identifier |
| `conformance declare` | Emit implementation capability declaration (Ch 23) |
| `conformance run` | Run offline conformance test suite |
| `version` | Print tool and specification versions |

All commands accept `--json` for structured output except where noted below.

### `--registry`

Optional `--registry <path>` merges a vendor catalog (YAML/JSON) with the embedded `dtcs:` catalog. Builtin `dtcs:` entries remain authoritative.

| Command | Rust CLI | Python CLI |
|---------|----------|------------|
| `validate` | yes | yes |
| `analyze` | yes | yes |
| `plan` | yes | yes |
| `optimize` | yes | yes |
| `match` | yes | yes |
| `compile` | yes | yes |
| `run` | yes | yes |
| `compat` | yes | yes |
| `evolve` | yes | yes |
| `lineage` | yes | yes |
| `registry list` / `resolve` | yes | yes |
| `inspect`, `diagnostics`, `version` | no | no |

The Python package also accepts an optional registry path on `validate()`, `analyze()`, `plan_lower()`, `plan_validate()`, and `validate_with_registry()` in the API.

## validate

```bash
dtcs validate contract.yaml
dtcs validate contract.yaml --registry vendor_catalog.yaml
dtcs validate contract.yaml --json
```

| Exit code | Meaning |
|-----------|---------|
| `0` | Contract is valid (no error-severity diagnostics) |
| `1` | Contract is invalid |

Optional `--registry` merges a vendor catalog for extension and stdlib validation (builtin `dtcs:` entries remain authoritative).

Warnings and information-level diagnostics do not cause a non-zero exit.

## analyze

```bash
dtcs analyze contract.yaml
dtcs analyze contract.yaml --registry vendor_catalog.yaml
dtcs analyze contract.yaml --json
```

Runs validation and static analysis (SPEC Chapters 7–8). Requires a **valid** contract.

| Exit code | Meaning |
|-----------|---------|
| `0` | Valid and no error-severity analysis diagnostics |
| `1` | Validation failed or analysis reported errors |

## plan

```bash
dtcs plan contract.yaml
dtcs plan contract.yaml --registry vendor_catalog.yaml
dtcs plan contract.yaml --json
```

Runs validation and semantic analysis, then lowers the contract to a canonical transformation plan (SPEC Chapter 13). Requires a **valid** contract with no error-severity analysis diagnostics.

| Exit code | Meaning |
|-----------|---------|
| `0` | Plan lowering succeeded |
| `1` | Validation, analysis, or lowering failed |

With `--json`, success emits the plan document; failure emits a diagnostics array (same shape as `diagnostics --json`).

## optimize

```bash
dtcs optimize contract.yaml
dtcs optimize contract.yaml --registry vendor_catalog.yaml
dtcs optimize contract.yaml --json
dtcs optimize plan.json --plan
dtcs optimize plan.json --plan --no-validate --json
```

Lowers a validated contract to a plan (unless `--plan` is set), then applies semantics-preserving optimization passes. With `--plan`, the path must be serialized plan JSON from a prior `dtcs plan --json` run.

| Flag | Effect |
|------|--------|
| `--plan` | Treat `path` as serialized plan JSON instead of a contract |
| `--registry` | Merge a vendor catalog for registry-gated function evaluation |
| `--no-validate` | Skip validation of the input and optimized plans |
| `--json` | Emit an `OptimizeResult` envelope (`plan`, `diagnostics`, `transforms`) |

| Exit code | Meaning |
|-----------|---------|
| `0` | Optimization succeeded |
| `1` | Validation, lowering, or optimization failed |

## match

```bash
dtcs match contract.yaml
dtcs match contract.yaml --profile dtcs:reference
dtcs match plan.json --plan --json
```

Lowers a validated contract to a plan (unless `--plan` is set), then matches it against an engine capability profile (SPEC Chapter 14).

| Flag | Effect |
|------|--------|
| `--plan` | Treat `path` as serialized plan JSON instead of a contract |
| `--optimize` | Optimize the plan before matching |
| `--profile` | Engine profile identifier (default: `dtcs:reference`) |
| `--registry` | Merge a vendor catalog for lowering |
| `--json` | Emit a capability match report |

| Exit code | Meaning |
|-----------|---------|
| `0` | Plan is supported by the selected profile |
| `1` | Validation, lowering, or capability match failed |

## compile

```bash
dtcs compile contract.yaml
dtcs compile contract.yaml --profile dtcs:reference --json
dtcs compile plan.json --plan
```

Compiles a transformation plan to an execution plan (SPEC Chapter 15). Requires capability match success for the selected profile.

| Flag | Effect |
|------|--------|
| `--plan` | Treat `path` as serialized plan JSON instead of a contract |
| `--optimize` | Optimize the plan before compilation |
| `--profile` | Engine profile identifier (default: `dtcs:reference`) |
| `--registry` | Merge a vendor catalog for lowering |
| `--json` | Emit the execution plan document |

| Exit code | Meaning |
|-----------|---------|
| `0` | Compilation succeeded |
| `1` | Validation, lowering, match, or compilation failed |

## run

```bash
dtcs run contract.yaml --input tests/fixtures/runtime/customer_normalize_input.json
dtcs run contract.yaml --input inputs.json --json
```

Validates and lowers a contract, compiles it, and executes it with the reference in-memory runtime (SPEC Chapter 16). The `--input` file maps interface ids to row arrays.

| Flag | Effect |
|------|--------|
| `--input` | JSON file with runtime inputs keyed by interface id (required) |
| `--optimize` | Optimize the plan before compilation |
| `--registry` | Merge a vendor catalog for lowering |
| `--json` | Emit output datasets as JSON |

| Exit code | Meaning |
|-----------|---------|
| `0` | Execution succeeded |
| `1` | Validation, compilation, or execution failed |

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
dtcs compat older.yaml newer.yaml --registry vendor_catalog.yaml
dtcs compat older.yaml newer.yaml --json
```

Compares two **valid** contracts. Optional `--scope` accepts comma-separated aspects:

`interfaces` · `types` · `semantics` · `lineage` · `metadata` · `extensions` · `all`

Default scope is `all` when `--scope` is omitted.

| Exit code | Meaning |
|-----------|---------|
| `0` | Compatible (any level except Incompatible) |
| `1` | Incompatible |
| `2` | Invalid `--scope` token(s) |

## evolve

```bash
dtcs evolve rev1.yaml rev2.yaml
dtcs evolve rev1.yaml rev2.yaml --registry vendor_catalog.yaml
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
dtcs lineage contract.yaml --registry vendor_catalog.yaml
dtcs lineage contract.yaml --impact INPUT_ID
dtcs lineage contract.yaml --dependency OUTPUT_ID
dtcs lineage contract.yaml --json
```

| Flag | Effect |
|------|--------|
| `--impact INPUT` | List outputs affected by the given input (transitive) |
| `--dependency OUTPUT` | List inputs required by the given output |

Always exits `0` on success.

## registry

```bash
dtcs registry list
dtcs registry list --json
dtcs registry list --registry vendor_catalog.yaml
dtcs registry resolve dtcs:lowercase
dtcs registry resolve dtcs:lowercase --json
dtcs registry resolve acme:transform --registry vendor_catalog.yaml
```

Inspects the embedded `dtcs:` identifier catalog, including the starter standard libraries for semantic actions, functions, and rules (Phase 0.5). Optional `--registry` merges an additional YAML/JSON catalog (builtin `dtcs:` entries remain authoritative).

| Exit code | Meaning |
|-----------|---------|
| `0` | `list` succeeded, or `resolve` found the identifier |
| `1` | Registry file invalid, or `resolve` did not find the identifier |

## conformance

```bash
dtcs conformance declare
dtcs conformance declare --profile integrated-platform --json
dtcs conformance run --profile integrated-platform
dtcs conformance run --profile all --json
```

Declares conformance profiles (Ch 23 §9) and runs the offline test suite (Ch 23 §8). Security checklist probes (Ch 24) are included in the run report.

| Exit code | Meaning |
|-----------|---------|
| `0` | Declaration emitted, or all selected tests passed |
| `1` | Unknown profile, or one or more conformance tests failed |

See [conformance.md](conformance.md) for profile identifiers and report fields.

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

See [ci-integration.md](ci-integration.md) for extended CI examples and [json-output.md](json-output.md) for field definitions.

## Next steps

- JSON output shapes: [json-output.md](json-output.md)
- CI integration: [ci-integration.md](ci-integration.md)
- Troubleshooting: [troubleshooting.md](troubleshooting.md)
- Library API: [public-api.md](../implementation/public-api.md)
- Enterprise evaluation: [adoption/overview.md](../adoption/overview.md)
