# FAQ

## General

### What is DTCS?

The Data Transformation Contract Standard (DTCS) is a vendor-neutral specification for expressing the **semantics** of data transformations. A DTCS contract describes *what* a transformation means without prescribing Spark, SQL, Polars, or any specific runtime.

### What does the reference implementation do today?

Through version 0.3.0, the reference tools can:

- Parse YAML/JSON into the Canonical Object Model
- Validate contracts with structured diagnostics
- Compare contracts for compatibility (five classification levels)
- Analyze evolution between contract revisions
- Validate versioning declarations (Ch 25)
- Analyze dataset-level lineage (dependency graph, impact, governance)

They do **not** execute transformations. See [non-goals.md](../implementation/non-goals.md).

### Is DTCS production-ready?

The specification is a **draft** (`1.0.0-draft`). The reference implementation is **alpha** (PyPI classifier: Development Status :: 3 - Alpha). It is suitable for evaluation, CI validation, and contract authoring — not yet for mission-critical execution pipelines.

## Installation

### Do I need Rust to use DTCS?

No, if you install from PyPI (`pip install dtcs`). Yes, if you build from source or use `cargo install`.

### `pip install dtcs` fails to build. What do I do?

Pre-built wheels are published for common platforms. If pip tries to compile from source, ensure Rust 1.75+ and maturin are installed, or use a Python version with available wheels (3.9–3.13).

### How do I install a specific version?

```bash
pip install dtcs==0.3.0
cargo install dtcs --version 0.3.0
```

## Contracts

### YAML or JSON?

Both are supported. Use whichever fits your toolchain. The Canonical Object Model is identical after parsing.

### Why camelCase in Python dicts?

The Canonical Object Model serializes with camelCase keys (`dtcsVersion`, `semanticActions`). Python `parse()` and `parse_file()` return dicts in this format. Pass them directly to `validate()` and analysis functions.

### Why is my contract invalid?

Run `dtcs diagnostics contract.yaml` for details. The most common first-time errors:

1. Missing `lineage.mappings` for an output
2. Unresolved field references in semantic actions or rules
3. Unsupported `dtcsVersion`
4. All inputs marked optional

See [writing-contracts.md](writing-contracts.md).

### What is the difference between `version` and `dtcsVersion`?

- `dtcsVersion` — which specification version the document conforms to
- `version` — the revision of this specific contract (semver-like)

## Analysis

### When should I use `compat` vs `evolve`?

- **`compat`** — general compatibility check between any two contracts
- **`evolve`** — structured diff between two revisions, with change categories and migration hints; expects the same contract `id`

### What does "backward compatible" mean?

Consumers of the older (source) contract can adopt the newer (target) contract without breaking. Typically this means additive optional fields or interfaces with no removed required behavior.

### Does compatibility analysis mutate my contracts?

No. All analysis is read-only.

## Contributing

### Where do I start?

Read [CONTRIBUTING.md](../../CONTRIBUTING.md) and [docs/implementation/README.md](../implementation/README.md).

### SPEC.md vs implementation docs — which wins?

**SPEC.md** is authoritative. Implementation docs in `docs/implementation/` are illustrative unless explicitly normative.

## Where to get help

Open a GitHub issue with:

- The contract file (or minimal reproduction)
- Output of `dtcs diagnostics contract.yaml --json`
- Version from `dtcs version --json`
