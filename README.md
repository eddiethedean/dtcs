# DTCS — Data Transformation Contract Standard

[![CI](https://github.com/eddiethedean/dtcs/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/dtcs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/dtcs)](https://crates.io/crates/dtcs)
[![PyPI](https://img.shields.io/pypi/v/dtcs)](https://pypi.org/project/dtcs/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

**Vendor-neutral contracts for data transformation semantics** — describe *what* a transformation means without locking you to Spark, SQL, Polars, or any single runtime.

This repository contains:

- **[SPEC.md](SPEC.md)** — normative DTCS 1.0 draft (26 chapters)
- **Reference tools** — parse, validate, and analyze contracts in Rust and Python

| | |
|---|---|
| **Spec status** | Draft (`1.0.0-draft`) |
| **Reference implementation** | `0.7.0` — validation, analysis, registries, standard libraries, and transformation plan lowering |
| **Document `dtcsVersion`** | `1.0.0` (currently exact; patch releases are rejected) |
| **Try it now** | `pip install dtcs` or `cargo install dtcs` |

**What you can do today:** validate YAML/JSON contracts, resolve `dtcs:` identifiers through the embedded registry (including standard actions, functions, and rules), compare versions for compatibility, analyze evolution between revisions, trace dataset lineage, and lower validated contracts to transformation plans — all read-only, no execution engine required.

[Quick start](#quick-start) · [User docs](docs/user/getting-started.md) · [Examples](examples/) · [Changelog](CHANGELOG.md) · [Roadmap](ROADMAP.md)

## Install

**Requirements:** Python 3.9+ (PyPI package); Rust 1.75+ (`cargo install` or building from source).

```bash
pip install dtcs
cargo install dtcs

dtcs version
dtcs validate examples/customer_normalize.dtcs.yaml
```

Both packages install the `dtcs` CLI on `PATH`:

`validate` · `analyze` · `plan` · `inspect` · `diagnostics` · `compat` · `evolve` · `lineage` · `registry` · `version`

**Develop from source** (requires Rust + [maturin](https://www.maturin.rs/)): see [CONTRIBUTING.md](CONTRIBUTING.md).

## Quick start

```bash
# Validate a contract (exit 0 = valid)
dtcs validate examples/customer_normalize.dtcs.yaml

# Analyze a contract (static semantics; no runtime evaluation)
dtcs analyze examples/customer_normalize.dtcs.yaml

# Lower a validated contract to a transformation plan
dtcs plan examples/customer_normalize.dtcs.yaml

# Human-readable summary
dtcs inspect examples/customer_normalize.dtcs.yaml

# Compare two contract versions
dtcs compat examples/analysis/backward_old.yaml examples/analysis/backward_new.yaml

# Trace lineage impact
dtcs lineage examples/analysis/lineage_multi.yaml --impact customers

# Resolve a standard identifier
dtcs registry resolve dtcs:lowercase
```

```python
import dtcs

report = dtcs.parse_and_validate(
    open("examples/customer_normalize.dtcs.yaml", "rb").read()
)
assert dtcs.is_valid(report)

plan = dtcs.plan_lower(dtcs.parse_file("examples/customer_normalize.dtcs.yaml")["contract"])
assert dtcs.is_valid({"diagnostics": plan["diagnostics"]})
```

Read [docs/user/getting-started.md](docs/user/getting-started.md) for a full walkthrough. For normative definitions, see [SPEC.md](SPEC.md) — start with [Chapter 3 (COM)](SPEC.md#chapter-3----canonical-object-model) and [Chapter 9 (Validation)](SPEC.md#chapter-9----validation).

## Pipeline

The reference implementation through Phase 0.7:

```text
DTCS Document
        │
        ▼
Parser → Canonical Object Model
        │
        ├──────────────────────────────┐
        ▼                              ▼
Validator (0.1–0.6)              Analyzer (0.3, 0.6)
        │                              │
        │  registry::resolve           ├─ compatibility::analyze
        │  stdlib definition checks    ├─ analyze_evolution
        ▼                              ├─ versioning::validate
Diagnostics                            └─ lineage::analyze
        │                              │
        ▼                              ▼
   Plan lowering (0.7)            Analysis reports
        │
        ▼
Transformation Plan
```

Phase 0.2 adds metadata validation, extended type system checks, expression typing, and I/O interface depth. Phase 0.3 adds compatibility classification, evolution analysis, versioning validation, and dataset-level lineage analysis. Phase 0.4 adds the identifier registry, file/URI loading with offline cache, and registry-aware extension validation. Phase 0.5 embeds starter standard libraries for semantic actions, functions, and rules, and validates contract usage against structured registry definitions (target types, phases, arity, and return types). Phase 0.6 adds static semantic analysis. Phase 0.7 lowers validated contracts into transformation plans with dependency graphs and plan validation.

Execution, backend compilation, and runtime behavior remain out of scope. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

## Repository layout

| Path | Purpose |
|------|---------|
| [SPEC.md](SPEC.md) | Full DTCS 1.0 draft specification (26 chapters) |
| [docs/user/](docs/user/) | User guides — getting started, CLI, compatibility |
| [docs/adoption/](docs/adoption/) | Adoption overview for evaluators |
| [docs/implementation/](docs/implementation/) | Reference implementation design guides |
| [docs/editorial/](docs/editorial/) | Specification authoring process |
| [examples/](examples/) | Sample transformation contracts |
| [src/](src/) | Rust crate source (`dtcs`) |
| [python/](python/) | Python package source (`dtcs` on PyPI) |
| [tests/](tests/) | Integration tests and fixtures |
| [ROADMAP.md](ROADMAP.md) | Reference implementation milestones |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for editorial conventions, implementation guidelines, and the review process.

When implementation guidance conflicts with the specification, **SPEC.md wins**. See [docs/implementation/spec-usage.md](docs/implementation/spec-usage.md).

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
