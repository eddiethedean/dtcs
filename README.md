# DTCS — Data Transformation Contract Standard

[![CI](https://github.com/eddiethedean/dtcs/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/dtcs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/dtcs)](https://crates.io/crates/dtcs)
[![PyPI](https://img.shields.io/pypi/v/dtcs)](https://pypi.org/project/dtcs/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

**Vendor-neutral contracts for data transformation semantics** — describe *what* a transformation means without locking you to Spark, SQL, Polars, or any single runtime.

This repository contains:

- **[SPEC.md](SPEC.md)** — normative DTCS 1.0 draft (26 chapters)
- **Reference tools** — parse, validate, analyze, plan, optimize, match, compile, and run contracts in Rust and Python

| | |
|---|---|
| **Spec status** | Draft (`1.0.0-draft`) |
| **Reference implementation** | `0.11.0` — validation, analysis, registries, full Ch 17–19 standard libraries, plan lowering, optimization, capability matching, compilation, reference runtime, conformance certification, and SPEC completeness |
| **Document `dtcsVersion`** | `1.0.0` (currently exact; patch releases are rejected) |
| **Try it now** | `pip install dtcs` or `cargo install dtcs` |

**What you can do today:** validate YAML/JSON contracts, resolve `dtcs:` identifiers through the embedded full standard-library catalog (actions, functions, and rules), compare versions for compatibility, analyze evolution between revisions, trace dataset lineage (`operation` / `flow`), lower validated contracts to transformation plans, optimize plans with semantics-preserving rewrites, match plans against engine capabilities, compile to execution plans, run contracts in the reference in-memory runtime (including null/missing/invalid semantics), and certify conformance across all eight implementation profiles — including end-to-end execution of sample contracts like `customer_normalize.dtcs.yaml`.

[Documentation](docs/README.md) · [Quick start](#quick-start) · [User docs](docs/user/getting-started.md) · [Adoption](docs/adoption/overview.md) · [Examples](examples/) · [Changelog](CHANGELOG.md) · [Roadmap](ROADMAP.md)

## Install

**Requirements:** Python 3.9+ (PyPI package); Rust 1.75+ (`cargo install` or building from source).

```bash
pip install dtcs
cargo install dtcs

dtcs version
dtcs validate examples/customer_normalize.dtcs.yaml
```

Both packages install the `dtcs` CLI on `PATH`:

`validate` · `analyze` · `plan` · `optimize` · `match` · `compile` · `run` · `inspect` · `diagnostics` · `compat` · `evolve` · `lineage` · `registry` · `conformance` · `version`

**Bindings:** Python (`pip install dtcs`), WASM (`@eddiethedean/dtcs-wasm`), Node (`@eddiethedean/dtcs`).

**Develop from source** (requires Rust + [maturin](https://www.maturin.rs/)): see [CONTRIBUTING.md](CONTRIBUTING.md#contributor-quickstart).

## Quick start

```bash
# Validate a contract (exit 0 = valid)
dtcs validate examples/customer_normalize.dtcs.yaml

# Analyze a contract (static semantics; no runtime evaluation)
dtcs analyze examples/customer_normalize.dtcs.yaml

# Lower a validated contract to a transformation plan
dtcs plan examples/customer_normalize.dtcs.yaml

# Optimize a lowered plan (contract path lowers first)
dtcs optimize examples/customer_normalize.dtcs.yaml
dtcs match examples/customer_normalize.dtcs.yaml
dtcs compile examples/customer_normalize.dtcs.yaml
dtcs run examples/customer_normalize.dtcs.yaml --input tests/fixtures/runtime/customer_normalize_input.json

# Human-readable summary
dtcs inspect examples/customer_normalize.dtcs.yaml

# Compare two contract versions
dtcs compat examples/analysis/backward_old.yaml examples/analysis/backward_new.yaml

# Trace lineage impact
dtcs lineage examples/analysis/lineage_multi.yaml --impact customers

# Resolve a standard identifier
dtcs registry resolve dtcs:lowercase

# Run conformance certification
dtcs conformance run --profile integrated-platform
```

```python
import json
import dtcs

report = dtcs.parse_and_validate(
    open("examples/customer_normalize.dtcs.yaml", "rb").read()
)
assert dtcs.is_valid(report)

plan = dtcs.plan_lower(dtcs.parse_file("examples/customer_normalize.dtcs.yaml")["contract"])
optimized = dtcs.plan_optimize(plan["plan"])
assert dtcs.plan_equivalent(plan["plan"], optimized["plan"])

compiled = dtcs.compile_plan(plan["plan"])
inputs = json.loads(open("tests/fixtures/runtime/customer_normalize_input.json").read())
result = dtcs.runtime_execute(compiled["plan"], inputs)
assert dtcs.is_valid(result)
assert result["outputs"]["customer_clean"][0]["email"] == "alice@example.com"
```

Read [docs/user/getting-started.md](docs/user/getting-started.md) for a full walkthrough. For normative definitions, see [SPEC.md](SPEC.md) — start with [Chapter 3 (COM)](SPEC.md#chapter-3----canonical-object-model) and [Chapter 9 (Validation)](SPEC.md#chapter-9----validation).

## Pipeline

The reference implementation through Phase 0.11:

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
   Plan optimization (0.8)
        │
        ▼
   Capability matching (0.9)
        │
        ▼
   Compilation (0.9)
        │
        ▼
   Reference runtime (0.9)
        │
        ▼
Transformation Plan → Execution Plan → Outputs
```

Phase 0.2 adds metadata validation, extended type system checks, expression typing, and I/O interface depth. Phase 0.3 adds compatibility classification, evolution analysis, versioning validation, and dataset-level lineage analysis. Phase 0.4 adds the identifier registry, file/URI loading with offline cache, and registry-aware extension validation. Phase 0.5 embeds starter standard libraries; Phase 0.11 completes the full Ch 17–19 catalogs and SPEC Appendix A. Phase 0.6 adds static semantic analysis. Phase 0.7 lowers validated contracts into transformation plans with dependency graphs and plan validation. Phase 0.8 optimizes plans with semantics-preserving expression, function, action, and rule passes. Phase 0.9 adds engine capability matching, compilation to execution plans, and a reference in-memory runtime. Phase 0.10 adds conformance profiles and WASM/Node bindings. Phase 0.11 deepens COM lineage/guarantees/null semantics and publishes the [SPEC completeness matrix](docs/implementation/spec-completeness.md).

Production ETL orchestration and external engine backends remain out of scope. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

## Repository layout

| Path | Purpose |
|------|---------|
| [SPEC.md](SPEC.md) | Full DTCS 1.0 draft specification (26 chapters) |
| [docs/user/](docs/user/) | User guides — getting started, CLI, writing contracts, expressions, compatibility, JSON output, troubleshooting, FAQ |
| [docs/adoption/](docs/adoption/) | Adoption overview for evaluators |
| [docs/implementation/](docs/implementation/) | Reference implementation design guides |
| [docs/editorial/](docs/editorial/) | Specification authoring process |
| [examples/](examples/) | Sample transformation contracts |
| [src/](src/) | Rust crate source (`dtcs`) |
| [python/](python/) | Python package source (`dtcs` on PyPI) |
| [tests/](tests/) | Integration tests and fixtures |
| [ROADMAP.md](ROADMAP.md) | Reference implementation milestones |

## Testing

Integration tests, fixture manifests, and conformance cases are documented in [docs/implementation/testing-plan.md](docs/implementation/testing-plan.md). The embedded standard library covers the full Ch 17–19 catalog (SPEC [Appendix A](SPEC.md#appendix-a----standard-library-catalog-normative)). See [spec-completeness.md](docs/implementation/spec-completeness.md) for chapter coverage and [test-verification-report.md](docs/implementation/test-verification-report.md) for suite confidence.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for editorial conventions, implementation guidelines, and the review process. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community standards.

When implementation guidance conflicts with the specification, **SPEC.md wins**. See [docs/implementation/spec-usage.md](docs/implementation/spec-usage.md).

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
