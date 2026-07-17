# Conformance certification

Offline conformance profiles and a certification suite per [SPEC.md](../SPEC.md) Chapter 23. Phase 0.10 introduced the machinery; Phase 0.11 deepens Analyzer assertions; Phase 0.12 adds portable semantic-family profiles and `portableDifferential` fixtures; Phase 0.13 adds DTCS 3.0 / A.9 Rich Portable Analytics profiles (mostly Experimental).

## Profiles

### Implementation classes

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

Analyzer profile capabilities include `compatibilityAnalysis`, `evolutionAnalysis`, and `lineageAnalysis`.

### Portable semantic-family profiles (0.12)

| Profile ID | Focus |
|------------|-------|
| `dtcs:profile/portable-relational-kernel/1` | Kernel operators |
| `dtcs:profile/portable-relational/1` | Rich relational ops |
| `dtcs:profile/portable-window/1` | Window frames |
| `dtcs:profile/portable-complex-types/1` | Complex access |

### DTCS 3.0 / A.9 profiles (0.13) — maturity for evaluators

| Maturity | Meaning for adopters | Profiles |
|----------|----------------------|----------|
| **Experimental** | Reference may implement; **do not** claim certification or production portability | A.9 families: string-advanced, conversion, statistics, complex-values, reshape, relational-extended, temporal-iana, nondeterministic |
| **Candidate** | Fixtures exist; waiting on two-compiler graduation criteria before Standard | `portable-relational-kernel/2`, `portable-relational/2`, `portable-window/2` |
| **Certified dual-path (today)** | Strongest portable evidence in this repo | Kernel relational **`/1`** surface with differential fixtures |

Kernel/relational `/2` defaults accompany plan v2. Advanced A.9 families remain Experimental. Implementer details: [portable-conformance.md](../implementation/portable-conformance.md).

## Assertion kinds

Manifest cases under `tests/conformance/manifest.json` (mirrored in `src/conformance/manifest.json`) use these assertion kinds:

| Kind | Meaning |
|------|---------|
| `parseValid` / `parseInvalid` | Document parses (or fails to) |
| `validateValid` / `validateInvalid` | Validation success / failure (+ optional `codes`) |
| `analyzeValid` | Static analysis produces no error diagnostics |
| `compatLevel` | Compatibility level vs `comparisonFixture` equals `level` |
| `evolveValid` | Evolution analysis vs `comparisonFixture` succeeds |
| `planValid` | Plan lowering succeeds |
| `optimizeEquivalent` | Optimizer preserves semantics |
| `matchSupported` | Capability match succeeds against `dtcs:reference` |
| `compileValid` | Compilation succeeds |
| `runtimeOutput` | Runtime output matches fixture (`input`, `expectedOutput`) |
| `runtimeInvalid` | Runtime fails with expected diagnostic `codes` |
| `portableDifferential` | Portable fixture: Direct vs StructuredLowering must match (0.12) |
| `securityProbe` | Automated Ch 24 probe (`probeId`) |

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
dtcs conformance run --profile analyzer
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

- `implementationId`, `implementationVersion` — reference implementation identity (`0.13.0`)
- `profiles` — profiles exercised in this run
- `results` — per-test outcomes (`id`, `profile`, `passed`, optional `message`)
- `security` — automated Ch 24 checklist probes
- `passed` — overall result

Failed tests list the profile, test id, and a diagnostic message. Security probes document residual manual review items (for example deployment-specific network policy).

## CI integration

```bash
cargo test --test phase_0_10 --locked
cargo test --test phase_0_11 --locked
dtcs conformance run --profile all
./scripts/security-checklist.sh
```

Also see [ci-integration.md](ci-integration.md).

## Bindings

| Surface | Conformance support |
|---------|---------------------|
| Rust | `dtcs::conformance_run_all()`, `dtcs::conformance_declare()` |
| Python | `conformance_declare()`, `conformance_run()` |
| WASM / Node | `conformanceDeclare()` only; full `run` requires CLI or Python |

See [security-checklist.md](../adoption/security-checklist.md) for Ch 24 requirements and [docs/api/python.md](../api/python.md) for the Python API reference.
