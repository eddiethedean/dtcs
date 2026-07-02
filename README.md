# DTCS — Data Transformation Contract Standard

Vendor-neutral specification and reference implementation for expressing the semantics of data transformations.

**Status:** Draft  
**Specification version:** 1.0.0-draft  
**Reference implementation:** 0.2.0 (Phase 0.2 — Contract Model)  
**Document `dtcsVersion`:** `1.0.0` (accepted by the reference validator for compatible 1.0.x releases)

See [ROADMAP.md](ROADMAP.md) for milestone status and [CHANGELOG.md](CHANGELOG.md) for release notes.

## Overview

[SPEC.md](SPEC.md) is the authoritative normative specification for DTCS. It defines transformation contracts, the canonical object model, validation, diagnostics, conformance, and governance — without prescribing execution engines, storage, or orchestration.

This repository contains:

| Path | Purpose |
|------|---------|
| [SPEC.md](SPEC.md) | Full DTCS 1.0 draft specification (26 chapters) |
| [docs/](docs/) | Documentation index |
| [docs/editorial/](docs/editorial/) | Authoring standards, style guide, and review process |
| [docs/implementation/](docs/implementation/) | Rust reference implementation design and build guides |
| [src/](src/) | Rust crate source (`dtcs`) |
| [python/](python/) | Python package source (`dtcs` on PyPI) |
| [examples/](examples/) | Sample DTCS transformation contracts |
| [tests/](tests/) | Integration tests and fixtures |
| [ROADMAP.md](ROADMAP.md) | Reference implementation milestones |
| [.github/workflows/](.github/workflows/) | CI pipeline |

## Install

Published packages (after tagging `v0.2.0`):

```bash
cargo install dtcs --version 0.2.0
pip install dtcs==0.2.0
```

The current PyPI/crates.io release is `0.1.2` until `v0.2.0` is tagged. See [CONTRIBUTING.md](CONTRIBUTING.md#releasing) for the release workflow.

## Quick start

### Read the specification

```bash
less SPEC.md
```

### Build the Rust crate

```bash
cargo build
cargo test
cargo run -- validate examples/customer_normalize.dtcs.yaml
cargo run -- inspect examples/customer_normalize.dtcs.yaml
```

### Use the Python package

```bash
pip install maturin
maturin develop --features python
python -m dtcs validate examples/customer_normalize.dtcs.yaml
python -m dtcs inspect examples/customer_normalize.dtcs.yaml
pytest python/tests -v
```

```python
import dtcs

report = dtcs.parse_and_validate(open("examples/customer_normalize.dtcs.yaml", "rb").read())
assert dtcs.is_valid(report)
```

The reference implementation through Phase 0.2 implements:

```text
DTCS Document → Parser → Canonical Object Model → Validator → Diagnostics
```

Phase 0.2 adds metadata validation, extended type system checks (conversions, collections, extension types), expression typing, and I/O interface depth (optional inputs, streaming, pre/postconditions).

Execution, backend compilation, and runtime behavior remain out of scope. See [docs/implementation/non-goals.md](docs/implementation/non-goals.md).

## Repository layout

```text
dtcs/
├── SPEC.md                 # Normative specification (source of truth)
├── Cargo.toml              # Rust crate manifest
├── pyproject.toml          # Python package manifest (maturin)
├── Cargo.lock              # Pinned dependencies (binary crate)
├── CHANGELOG.md            # Release notes
├── README.md
├── CONTRIBUTING.md
├── LICENSE
├── .github/workflows/      # CI
├── docs/
│   ├── README.md           # Documentation index
│   ├── editorial/          # Specification authoring process
│   └── implementation/     # Reference implementation guides
├── examples/               # Example transformation contracts
├── python/                 # Python package source
├── src/                    # Rust library, CLI binary, validation
├── tests/                  # Integration tests and fixtures
└── .cursor/prompts/        # Cursor build prompt
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for editorial conventions, implementation guidelines, and the review process.

When implementation guidance conflicts with the specification, **SPEC.md wins**. See [docs/implementation/spec-usage.md](docs/implementation/spec-usage.md).

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
