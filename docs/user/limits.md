# Performance and deployment limits

Honest boundaries for the **reference tools** (`0.11.0`, alpha).

## What `dtcs run` is

- An **in-memory** interpreter of compiled execution plans
- Useful for teaching, golden tests, and small fixtures
- **Not** a distributed query engine, warehouse, or production ETL runtime

## Guidance

| Topic | Guidance |
|-------|----------|
| Dataset size | Keep reference-runtime inputs small (fixtures / samples). Do not feed production tables into `dtcs run`. |
| Determinism | Prefer declared `semantics` / catalog `deterministic` flags; the reference runtime aims for deterministic results for supported operators. |
| Isolation | Do not treat the runtime as a multi-tenant sandbox for untrusted contracts or PII at scale. |
| Network | Validation/runtime avoid network I/O unless you explicitly load remote registries. |
| Production transforms | Use DTCS contracts for **semantics and CI gates**; implement execution in your engine of choice with a capability profile. |

## Recommended production use

1. Author and review contracts in git
2. `dtcs validate` / `analyze` / `compat` in CI
3. Optionally `dtcs conformance run` for tool certification
4. Execute transforms in your own engine that claims a capability profile via `dtcs match`

See [adoption/overview.md](../adoption/overview.md) and [SECURITY.md](https://github.com/eddiethedean/dtcs/blob/main/SECURITY.md).
