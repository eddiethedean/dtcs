# FAQ

## General

### What is DTCS?

The Data Transformation Contract Standard (DTCS) is a vendor-neutral specification for expressing the **semantics** of data transformations. A DTCS contract describes *what* a transformation means without prescribing Spark, SQL, Polars, or any specific runtime.

### What does the reference implementation do today?

Through version **0.12.0**, the reference tools can:

- Parse YAML/JSON into the Canonical Object Model (including `guarantees`, `compatibility`, nested extensions)
- Validate contracts with structured diagnostics
- Resolve the full Ch 17–19 `dtcs:` catalog (field transforms, dataset operators, functions, rules) through the embedded registry
- Load and merge vendor registry catalogs
- Compare contracts for compatibility (five classification levels)
- Analyze evolution between contract revisions (including deprecation / anticipated removal)
- Validate versioning declarations (Ch 25)
- Analyze dataset-level lineage with `operation` and `flow` (dependency graph, impact, governance)
- Run static semantic and expression analysis (Ch 7–8), including null/missing/invalid distinction
- Lower, optimize, match, compile, and execute contracts end-to-end with the reference runtime
- Export portable plans (`dtcs.transform-plan/1`) and run Portable Relational Profile semantics (joins, window frames, datetime, complex access)
- Certify conformance across implementation-class and portable semantic-family profiles (Ch 23)

The reference runtime is suitable for evaluation and conformance testing — not production ETL. See [non-goals.md](../implementation/non-goals.md).

### Is DTCS production-ready?

The specification is a **draft** (`2.0.0`). The reference implementation is **alpha** (PyPI classifier: Development Status :: 3 - Alpha). It is suitable for evaluation, CI validation, and contract authoring — not yet for mission-critical execution pipelines.

## Installation

### Do I need Rust to use DTCS?

No, if you install from PyPI (`pip install dtcs`). Yes, if you build from source or use `cargo install`.

### `pip install dtcs` fails to build. What do I do?

Pre-built wheels are published for common platforms. If pip tries to compile from source, ensure Rust 1.75+ and maturin are installed, or use a Python version with available wheels (3.9–3.13).

### How do I install a specific version?

```bash
pip install dtcs==0.12.0
cargo install dtcs --version 0.12.0
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
5. Dataset actions missing required `parameters` (for example `fields` on `dtcs:project`)

See [writing-contracts.md](writing-contracts.md).

### What is the difference between `version` and `dtcsVersion`?

- `dtcsVersion` — which specification version the document conforms to (prefer `"2.0.0"`; see [versioning.md](versioning.md))
- `version` — the revision of this specific contract (semver-like)

Supported `dtcsVersion` values: `"2.0.0"`, `"1.0.0"`, `"1.0.0-draft"`.

### What are null vs missing vs invalid values?

At runtime and in expression evaluation these are distinct (SPEC Chapter 8 §9 / Appendix A.7):

- **null** — JSON `null` (present key, null payload)
- **missing** — `{"$dtcs":"missing"}`
- **invalid** — `{"$dtcs":"invalid"}` (optional `reason`)

Do not treat missing/invalid as null in tooling. Use `dtcs:is_null` / `dtcs:is_missing` when you need predicates. See [expressions.md](expressions.md).

## Analysis

### When should I use `compat` vs `evolve`?

- **`compat`** — general compatibility check between any two contracts
- **`evolve`** — structured diff between two revisions, with change categories and migration hints; expects the same contract `id`

### What does "backward compatible" mean?

Consumers of the older (source) contract can adopt the newer (target) contract without breaking. Typically this means additive optional fields or interfaces with no removed required behavior.

### Does compatibility analysis mutate my contracts?

No. All analysis is read-only.

Contracts may also declare a COM-level `compatibility` policy (`policy`, `forward`, `backward`, `notes`) separate from analysis results. See [compatibility.md](compatibility.md).

## Spec 2.0 and tools 0.12

### What is Spec 2.0?

DTCS **2.0.0** (draft) is the current normative Spec. It supersedes `1.0.0-draft` and includes the Portable Relational Profile. Prefer document `dtcsVersion: "2.0.0"`. See [versioning.md](versioning.md) and [migration-0.12.md](migration-0.12.md).

### How do I upgrade to 0.12.0?

Dedicated guide: **[migration-0.12.md](migration-0.12.md)**. Portable Relational compatibility tables: [migration-portable-relational.md](migration-portable-relational.md).

```bash
pip install 'dtcs==0.12.0'
# or: cargo install dtcs --version 0.12.0
```

## Migration to 0.11.0 (historical)

Archived guide with before/after YAML for **0.10.x → 0.11.x**: **[migration-0.11.md](migration-0.11.md)**. Do not pin `0.11.0` for new work.

Notable changes from that era: lineage `operation`/`flow` defaults, dataset action `parameters`, null/missing/invalid runtime tokens, and the full Ch 17–19 catalog.

### How do I upgrade between versions?

Prefer [migration-0.12.md](migration-0.12.md) and the [CHANGELOG](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md). Pin with `pip install dtcs==0.12.0` or `cargo install dtcs --version 0.12.0`.

## Contributing

### Where do I start?

Read [CONTRIBUTING.md](https://github.com/eddiethedean/dtcs/blob/main/CONTRIBUTING.md) and [docs/implementation/README.md](../implementation/README.md).

### SPEC.md vs implementation docs — which wins?

**SPEC.md** is authoritative. Implementation docs in `docs/implementation/` are illustrative unless explicitly normative.

## Where to get help

Open a [GitHub issue](https://github.com/eddiethedean/dtcs/issues) with:

- The contract file (or minimal reproduction)
- Output of `dtcs diagnostics contract.yaml --json`
- Version from `dtcs version --json`

See also [troubleshooting.md](troubleshooting.md).

## Next steps

- Installation problems: [troubleshooting.md](troubleshooting.md)
- CI integration: [ci-integration.md](ci-integration.md)
- Conformance: [conformance.md](conformance.md)
- Enterprise evaluation: [adoption/overview.md](../adoption/overview.md)
