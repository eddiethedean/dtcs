# DTCS

Vendor-neutral **data transformation contracts** in YAML/JSON — the *semantics* of transforms, not a Spark, Polars, or SQL engine.

**Status:** Spec `3.0.0` (draft) · Tools `0.13.0` (alpha). Good for validation, compatibility analysis, portable relational profiles, and conformance research — **not** a production ETL runtime.

## Try it

```bash
pip install 'dtcs==0.12.0'
dtcs version
# → dtcs 0.12.0
# → spec 3.0.0
```

Then: [Getting started](user/getting-started.md) (validate in minutes) · [Adoption](adoption/overview.md) · [Versioning](user/versioning.md) · [SPEC](SPEC.md)

## Choose your path

| Audience | Start here |
|----------|------------|
| **New users** | [getting-started](user/getting-started.md) → [concepts](user/concepts.md) → [writing contracts](user/writing-contracts.md) |
| **Evaluators / architects** | [adoption/overview](adoption/overview.md) · [limits](user/limits.md) |
| **Upgrading to 0.12 / Spec 2.0** | [migration-0.12](user/migration-0.12.md) · [Portable Relational](user/migration-portable-relational.md) |
| **Contributors** | [CONTRIBUTING](https://github.com/eddiethedean/dtcs/blob/main/CONTRIBUTING.md) · [architecture](implementation/architecture.md) |
| **Spec authors** | [editorial/baseline](editorial/baseline.md) · [SPEC](SPEC.md) |

## Documentation map

### Tutorials

| Document | Description |
|----------|-------------|
| [getting-started](user/getting-started.md) | Install, validate, and run without cloning |
| [concepts](user/concepts.md) | COM → validate → plan → run |
| [writing-contracts](user/writing-contracts.md) | Contract structure and catalog |
| [cookbook](user/cookbook.md) | Short recipes |

### Guides

| Document | Description |
|----------|-------------|
| [versioning](user/versioning.md) | Crate vs Spec vs `dtcsVersion` |
| [cli-guide](user/cli-guide.md) | Commands, flags, exit codes |
| [expressions](user/expressions.md) | Expression language and null tokens |
| [compatibility](user/compatibility.md) | Compatibility classifications |
| [json-output](user/json-output.md) | JSON envelopes |
| [ci-integration](user/ci-integration.md) | Adopter CI gates |
| [conformance](user/conformance.md) | Conformance profiles |
| [error-taxonomy](user/error-taxonomy.md) | Raises vs diagnostics vs exit codes |
| [diagnostic-catalog](user/diagnostic-catalog.md) | Common diagnostic codes |
| [extensions-and-registries](user/extensions-and-registries.md) | Vendor catalogs |
| [migration-0.12](user/migration-0.12.md) | Upgrade to tools 0.12 / Spec 2.0 |
| [migration-portable-relational](user/migration-portable-relational.md) | Portable Relational compatibility |
| [migration-0.11](user/migration-0.11.md) | Historical: 0.10.x → 0.11.x |
| [limits](user/limits.md) | Runtime / deployment limits |
| [troubleshooting](user/troubleshooting.md) | Common failures |
| [faq](user/faq.md) | FAQ |
| [glossary](user/glossary.md) | Terms |

### Adoption · API · Spec

| Area | Links |
|------|-------|
| Adoption | [overview](adoption/overview.md) · [security checklist](adoption/security-checklist.md) · [SECURITY.md](https://github.com/eddiethedean/dtcs/blob/main/SECURITY.md) |
| API | [Python](api/python.md) · [Rust](api/rust.md) · [WASM](api/wasm.md) · [Node](api/node.md) |
| Spec | [SPEC](SPEC.md) · [completeness](implementation/spec-completeness.md) · [portable conformance](implementation/portable-conformance.md) · [CHANGELOG](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md) · [ROADMAP](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) |

### Contribute (reference implementers)

| Document | Description |
|----------|-------------|
| [implementation index](implementation/README.md) | Implementer docs |
| [architecture](implementation/architecture.md) | Layers and bindings |
| [release-runbook](implementation/release-runbook.md) | Maintainer releases |
| [editorial/baseline](editorial/baseline.md) | Spec authoring process |

## Preview this site locally

```bash
pip install -r docs/requirements.txt
NO_MKDOCS_2_WARNING=true mkdocs serve
```

Configuration: [`mkdocs.yml`](https://github.com/eddiethedean/dtcs/blob/main/mkdocs.yml) · [`.readthedocs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/.readthedocs.yaml).
