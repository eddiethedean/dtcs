# CI Integration

Use DTCS in continuous integration to gate contract changes before they reach production pipelines.

## Validate on every change

Fail the build when a contract is invalid:

```bash
dtcs validate contracts/my_transform.yaml --json
```

Exit code `0` means valid; non-zero means errors. Parse JSON in scripts:

```bash
#!/usr/bin/env bash
set -euo pipefail

output=$(dtcs validate contracts/my_transform.yaml --json)
valid=$(echo "$output" | jq -r '.valid')

if [ "$valid" != "true" ]; then
  echo "$output" | jq '.diagnostics'
  exit 1
fi
```

### GitHub Actions example

```yaml
name: Validate contracts

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - run: pip install dtcs==0.9.0
      - run: dtcs validate examples/customer_normalize.dtcs.yaml --json
```

## Static analysis gate

Catch expression and semantics errors that validation alone may not surface in all configurations:

```bash
dtcs analyze contracts/my_transform.yaml --json
```

Check both `validation.valid` and `analysis.valid` in the JSON output. See [json-output.md](json-output.md#analyze).

## Compatibility gate

Block breaking contract changes between revisions:

```bash
dtcs compat contracts/rev1.yaml contracts/rev2.yaml --json
```

Reject when `level` is `incompatible`:

```bash
level=$(dtcs compat contracts/old.yaml contracts/new.yaml --json | jq -r '.level')
if [ "$level" = "incompatible" ]; then
  echo "Breaking contract change detected"
  exit 1
fi
```

See [compatibility.md](compatibility.md) for classification levels.

## Evolution reporting

Generate migration hints for reviewers (informative, not a hard gate):

```bash
dtcs evolve contracts/rev1.yaml contracts/rev2.yaml --json | jq '.migrationHints'
```

## Registry-aware validation

Merge a vendor catalog when contracts use extension identifiers:

```bash
dtcs validate contracts/my_transform.yaml --registry vendor/catalog.yaml --json
```

The Python API equivalent is `dtcs.validate_with_registry(contract, registry_path)`.

## JSON output reference

All commands support `--json`. Field names use camelCase. See [json-output.md](json-output.md) for per-command schemas.

## What CI should not do (yet)

The reference runtime is for evaluation and conformance testing — not production ETL. Do not rely on `dtcs run` in mission-critical deployment pipelines until your organization has evaluated maturity. See [adoption/overview.md](../adoption/overview.md) and [non-goals.md](../implementation/non-goals.md).

## Next steps

- [cli-guide.md](cli-guide.md) — all commands and exit codes
- [troubleshooting.md](troubleshooting.md) — common failures
- [getting-started.md](getting-started.md) — local walkthrough
