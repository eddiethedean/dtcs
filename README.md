# DTCS — Data Transformation Contract Standard

[![CI](https://github.com/eddiethedean/dtcs/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/dtcs/actions/workflows/ci.yml)
[![Documentation Status](https://readthedocs.org/projects/dtcs/badge/?version=latest)](https://dtcs.readthedocs.io/en/latest/)
[![crates.io](https://img.shields.io/crates/v/dtcs)](https://crates.io/crates/dtcs)
[![PyPI](https://img.shields.io/pypi/v/dtcs)](https://pypi.org/project/dtcs/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

DTCS is a **draft** standard and toolkit for writing **vendor-neutral data transformation contracts** in YAML/JSON — the *semantics* of transforms, not a Spark, Polars, or SQL engine.

**Status:** Spec `3.0.0` (draft) · Tools `0.13.0` (alpha). Good for validation, compatibility analysis, portable relational profiles, and conformance research — **not** a production ETL runtime.

[Documentation](https://dtcs.readthedocs.io/) · [Getting started](docs/user/getting-started.md) · [What DTCS is not](docs/user/what-dtcs-is-not.md) · [Adoption](docs/adoption/overview.md) · [Examples](examples/) · [Changelog](CHANGELOG.md) · [Security](SECURITY.md)

## Try it (no clone)

**Requirements:** Python 3.9+ for PyPI wheels, or Rust 1.75+ for `cargo install` (compile can take several minutes).

```bash
pip install dtcs
dtcs version
# → dtcs 0.13.0
# → spec 3.0.0

curl -fsSL https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml \
  -o contract.dtcs.yaml
dtcs validate contract.dtcs.yaml
# → valid
# → exit 0
```

Or paste the same file from [`examples/minimal.dtcs.yaml`](examples/minimal.dtcs.yaml). Prefer `dtcsVersion: "3.0.0"`; prior versions remain accepted for compatibility.

**Bindings:** Python (`pip install dtcs`), WASM (`@eddiethedean/dtcs-wasm`), Node (`@eddiethedean/dtcs`). See [docs/api/](docs/api/).

## What this repository contains

- **[SPEC.md](SPEC.md)** — normative DTCS 3.0 draft (27 chapters + Appendix A, including Rich Portable Analytics)
- **Reference tools** — parse, validate, analyze, plan, optimize, match, compile, and run contracts (Rust CLI + Python; limited WASM/Node surface)

| | |
|---|---|
| **Spec status** | Draft (`3.0.0`) |
| **Reference tools** | `0.13.0` (alpha) — validator through reference runtime, portable profiles, and conformance |
| **Document `dtcsVersion`** | `"3.0.0"` preferred; prior supported versions remain accepted |

Coverage tables labeled “Covered” or “Complete” mean the **reference implementation exercises that draft SPEC area**, not that DTCS 3.0 is finalized or production-certified.

## Next steps (after `valid`)

Clone for richer examples and fixtures (PyPI wheels do **not** include `examples/` or `tests/`):

```bash
git clone https://github.com/eddiethedean/dtcs.git
cd dtcs
dtcs validate examples/customer_pipeline.dtcs.yaml
dtcs run examples/customer_pipeline.dtcs.yaml \
  --input tests/fixtures/runtime/customer_pipeline_input.json
```

| Goal | Where |
|------|--------|
| Concepts in two pages | [docs/user/concepts.md](docs/user/concepts.md) |
| Versions | [docs/user/versioning.md](docs/user/versioning.md) |
| Write contracts | [docs/user/writing-contracts.md](docs/user/writing-contracts.md) |
| CLI reference | [docs/user/cli-guide.md](docs/user/cli-guide.md) |
| Upgrade to 0.13 / Spec 3.0 | [docs/user/migration-0.13.md](docs/user/migration-0.13.md) |
| Historical: 0.12 / Spec 2.0 | [docs/user/migration-0.12.md](docs/user/migration-0.12.md) |
| Evaluate for your org | [docs/adoption/overview.md](docs/adoption/overview.md) |

## Develop from source

Requires Rust + [maturin](https://www.maturin.rs/). See [CONTRIBUTING.md](CONTRIBUTING.md#contributor-quickstart).

## Repository layout

| Path | Purpose |
|------|---------|
| [SPEC.md](SPEC.md) | Full DTCS 3.0.0 draft specification |
| [docs/user/](docs/user/) | User guides |
| [docs/adoption/](docs/adoption/) | Evaluator / security materials |
| [docs/api/](docs/api/) | Python, Rust, WASM, Node API docs |
| [docs/implementation/](docs/implementation/) | Reference implementation design guides |
| [examples/](examples/) | Sample contracts |
| [src/](src/) | Rust crate (`dtcs`) |
| [python/](python/) | Python package |
| [ROADMAP.md](ROADMAP.md) | Implementation milestones |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Security reports: [SECURITY.md](SECURITY.md). When guidance conflicts with the specification, **SPEC.md wins**.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
