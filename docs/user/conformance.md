# Conformance certification

Phase 0.10 adds offline conformance profiles and a certification suite per [SPEC.md](../../SPEC.md) Chapter 23.

## Profiles

The reference implementation declares eight implementation classes:

| Profile ID | Class |
|------------|-------|
| `parser` | Parser |
| `validator` | Validator |
| `analyzer` | Analyzer |
| `planner` | Planner |
| `optimizer` | Optimizer |
| `compiler` | Compiler |
| `runtime` | Runtime |
| `integrated-platform` | Integrated Platform (primary) |

## Declare capability

Emit the Ch 23 §9 implementation capability declaration:

```bash
dtcs conformance declare
dtcs conformance declare --profile integrated-platform --json
```

```python
import dtcs

declaration = dtcs.conformance_declare()
profile_only = dtcs.conformance_declare("parser")
```

## Run offline tests

```bash
dtcs conformance run --profile integrated-platform
dtcs conformance run --profile all --json
```

```python
report = dtcs.conformance_run("integrated-platform")
assert report["passed"]
```

| Exit code | Meaning |
|-----------|---------|
| `0` | All selected profile tests and security probes passed |
| `1` | One or more tests failed |

## Interpreting reports

JSON reports include:

- `implementationId`, `implementationVersion` — reference implementation identity
- `profiles` — profiles exercised in this run
- `results` — per-test outcomes (`id`, `profile`, `passed`, optional `message`)
- `security` — automated Ch 24 checklist probes
- `passed` — overall result

Failed tests list the profile, test id, and a diagnostic message. Security probes document residual manual review items (for example deployment-specific network policy).

## CI integration

```bash
cargo test --test phase_0_10 --locked
dtcs conformance run --profile all
./scripts/security-checklist.sh
```

## Bindings

| Surface | Conformance support |
|---------|---------------------|
| Rust | `dtcs::conformance_run_all()`, `dtcs::conformance_declare()` |
| Python | `conformance_declare()`, `conformance_run()` |
| WASM / Node | `conformanceDeclare()` only; full `run` requires CLI or Python |

See [security-checklist.md](../adoption/security-checklist.md) for Ch 24 requirements and [docs/api/python.md](../api/python.md) for the Python API reference.
