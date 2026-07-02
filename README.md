# DTCS — Data Transformation Contract Standard

Vendor-neutral specification and reference implementation for expressing the semantics of data transformations.

**Status:** Draft  
**Specification version:** 1.0.0-draft  
**Document `dtcsVersion`:** `1.0.0` (accepted by the reference validator for compatible 1.0.x releases)

## Overview

[SPEC.md](https://github.com/dtcs/dtcs/blob/main/SPEC.md) is the authoritative normative specification for DTCS. It defines transformation contracts, the canonical object model, validation, diagnostics, conformance, and governance — without prescribing execution engines, storage, or orchestration.

This repository contains:

| Path | Purpose |
|------|---------|
| [SPEC.md](https://github.com/dtcs/dtcs/blob/main/SPEC.md) | Full DTCS 1.0 draft specification (26 chapters) |
| [docs/](https://github.com/dtcs/dtcs/tree/main/docs) | Documentation index |
| [docs/editorial/](https://github.com/dtcs/dtcs/tree/main/docs/editorial) | Authoring standards, style guide, and review process |
| [docs/implementation/](https://github.com/dtcs/dtcs/tree/main/docs/implementation) | Rust reference implementation design and build guides |
| [src/](https://github.com/dtcs/dtcs/tree/main/src) | Rust crate source (`dtcs`) |
| [examples/](https://github.com/dtcs/dtcs/tree/main/examples) | Sample DTCS transformation contracts |
| [tests/](https://github.com/dtcs/dtcs/tree/main/tests) | Integration tests and fixtures |
| [.github/workflows/](https://github.com/dtcs/dtcs/tree/main/.github/workflows) | CI pipeline |

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

The first implementation milestone is:

```text
DTCS Document → Parser → Canonical Object Model → Validator → Diagnostics
```

Execution, backend compilation, and runtime behavior are out of scope for the initial crate. See [docs/implementation/non-goals.md](https://github.com/dtcs/dtcs/blob/main/docs/implementation/non-goals.md).

## Repository layout

```text
dtcs/
├── SPEC.md                 # Normative specification (source of truth)
├── Cargo.toml              # Rust crate manifest
├── Cargo.lock              # Pinned dependencies (binary crate)
├── README.md
├── CONTRIBUTING.md
├── LICENSE
├── .github/workflows/      # CI
├── docs/
│   ├── README.md           # Documentation index
│   ├── editorial/          # Specification authoring process
│   └── implementation/     # Reference implementation guides
├── examples/               # Example transformation contracts
├── src/                    # Rust library, CLI binary, validation
├── tests/                  # Integration tests and fixtures
└── .cursor/prompts/        # Cursor build prompt
```

## Contributing

See [CONTRIBUTING.md](https://github.com/dtcs/dtcs/blob/main/CONTRIBUTING.md) for editorial conventions, implementation guidelines, and the review process.

When implementation guidance conflicts with the specification, **SPEC.md wins**. See [docs/implementation/spec-usage.md](https://github.com/dtcs/dtcs/blob/main/docs/implementation/spec-usage.md).

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](https://github.com/dtcs/dtcs/blob/main/LICENSE).
