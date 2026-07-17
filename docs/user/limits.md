# Performance and deployment limits

Honest boundaries for the **reference tools** (`0.13.0`, alpha).

## What `dtcs run` is

- An **in-memory** interpreter of compiled execution plans
- Useful for teaching, golden tests, and small fixtures
- **Not** a distributed query engine, warehouse, or production ETL runtime

## Platforms and install channels

| Channel | Typical platforms | Notes |
|---------|-------------------|--------|
| PyPI `dtcs` | CPython **3.9–3.13**; manylinux / macOS / Windows wheels when published for the release | Prefer `pip install 'dtcs==0.13.0'` |
| crates.io `dtcs` | Any host with **Rust 1.75+** | `cargo install` compiles from source (often several minutes) |
| npm WASM / Node | Node ESM; browsers after `initSync` | Experimental; parse/validate/declare only — [wasm.md](../api/wasm.md) |

Exact wheel filenames vary by release — check [PyPI files](https://pypi.org/project/dtcs/#files) if install fails on an unusual platform.

### Dual CLI on `PATH`

Both the Rust binary and the Python package expose a `dtcs` command. If both are installed, `which dtcs` / `dtcs version` tells you which one runs. Prefer one channel per environment, or put the intended install first on `PATH`.

### Offline / air-gapped

Wheels and crates do **not** include `examples/`. Paste YAML from the docs or copy files before going offline. Validation and `conformance run` do not require network when fixtures are embedded (Python wheel / Rust binary).

## Guidance

| Topic | Guidance |
|-------|----------|
| Dataset size | Keep reference-runtime inputs small (fixtures / samples — typically tens to low thousands of rows). Do not feed production tables into `dtcs run`. |
| Memory | Entire inputs and outputs are held in process memory; expect O(rows × columns) growth. |
| Determinism | Prefer declared `semantics` / catalog `deterministic` flags; the reference runtime aims for deterministic results for supported operators. |
| Regex / string-advanced | Gated grammar + Unicode pin — pathological patterns can still be expensive; keep fixture patterns simple. |
| Isolation | Do not treat the runtime as a multi-tenant sandbox for untrusted contracts or PII at scale. |
| Network | Validation/runtime avoid network I/O unless you explicitly load remote registries. |
| Production transforms | Use DTCS contracts for **semantics and CI gates**; implement execution in your engine of choice with a capability profile. |

## Recommended production use

1. Author and review contracts in git
2. `dtcs validate` / `analyze` / `compat` in CI
3. Optionally `dtcs conformance run` for tool certification
4. Execute transforms in your own engine that claims a capability profile via `dtcs match`

See [adoption/overview.md](../adoption/overview.md), [what-dtcs-is-not.md](what-dtcs-is-not.md), and [SECURITY.md](https://github.com/eddiethedean/dtcs/blob/main/SECURITY.md).
