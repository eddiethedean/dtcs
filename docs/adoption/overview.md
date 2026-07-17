# Adoption Overview

This document helps architects and enterprise evaluators assess DTCS for their organization.

## What DTCS provides

DTCS standardizes **transformation semantics** as portable, vendor-neutral contracts. A DTCS contract captures:

- Input and output schemas (with optional field constraints)
- Semantic actions (field transforms and dataset operators), expressions, functions, and rules
- Lineage with explicit `operation` / information-flow `flow`
- Guarantees and compatibility declarations
- Metadata (governance, ownership, lifecycle, provenance, classification, deprecation)
- Versioning policies
- Distinctions among null, missing, and invalid values at runtime

It does **not** define execution engines, storage, orchestration, or SQL dialects.

## Reference implementation maturity

> **Maturity:** Spec `2.0.0` (draft) · tools alpha (`0.12.0`). **Covered** means the reference implementation exercises that draft area — not production certification of the standard.

| Component | Reference impl coverage |
|-----------|-------------------------|
| Specification | Draft (`2.0.0`, 26 chapters + Appendix A catalog) |
| Parser (YAML/JSON) | Covered |
| Seven-phase validation | Covered |
| Metadata validation | Covered |
| Type system (incl. expressions) | Covered |
| Compatibility analysis | Covered (five classification levels) |
| Evolution analysis | Covered |
| Versioning validation (Ch 25) | Covered |
| Lineage analysis (dataset-level) | Covered (`operation` / `flow` in COM) |
| Identifier registry & extensibility | Covered (Phase 0.4) |
| Standard libraries (Ch 17–19) | Covered (full catalog, Phase 0.11) |
| Static semantic analysis | Covered (Phase 0.6) |
| Transformation plan lowering | Covered (Phase 0.7) |
| Plan optimization | Covered (Phase 0.8) |
| Capability matching | Covered (Phase 0.9; full-catalog profile in 0.11) |
| Compilation | Covered (Phase 0.9) |
| Reference runtime | Covered (Phase 0.9; full catalog + null tokens in 0.11) |
| Conformance certification (Ch 23) | Covered (Phase 0.10) |
| SPEC completeness matrix | Covered (Phase 0.11) |

*Released reference implementation: `0.12.0`.*

See [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) for the full milestone plan and [spec-completeness.md](../implementation/spec-completeness.md) for the chapter matrix.

## What you can use today

1. **Contract authoring** — write YAML/JSON contracts with IDE/CI validation
2. **CI gates** — fail builds on invalid contracts (`dtcs validate --json`)
3. **Standard library usage** — reference the full `dtcs:` action, function, and rule catalogs; validate against embedded definitions (`dtcs registry list`)
4. **Static semantic analysis** — check transformation semantics and expressions without runtime evaluation (`dtcs analyze`)
5. **Version management** — compare contract revisions for breaking changes (`dtcs compat`, `dtcs evolve`)
6. **Impact analysis** — trace which outputs depend on an input (`dtcs lineage --impact`)
7. **Plan lowering** — produce canonical transformation plans from validated contracts (`dtcs plan`)
8. **Plan optimization** — apply semantics-preserving rewrites to lowered plans (`dtcs optimize`)
9. **Capability matching** — verify a plan against an engine profile (`dtcs match`)
10. **Compilation** — produce execution plans from transformation plans (`dtcs compile`)
11. **Reference execution** — run contracts end-to-end with sample inputs (`dtcs run`)
12. **Conformance certification** — offline profiles via `dtcs conformance declare` / `dtcs conformance run`
13. **Governance hooks** — metadata validation enforces owner/steward on restricted classifications

## What is explicitly out of scope

- Production ETL execution (Spark, Polars, SQL compilation)
- Multi-backend compilers beyond the reference profile
- External certification authority (Ch 23 §13)

See [non-goals.md](../implementation/non-goals.md).

WASM and Node bindings (`@eddiethedean/dtcs-wasm`, `@eddiethedean/dtcs`) are available for parse, validate, and conformance declare.

## Security considerations

Normative security guidance is in [SPEC.md Chapter 24](../SPEC.md#chapter-24-security-considerations). At a high level:

- Contracts are **declarative documents** — the validator parses and analyzes them; it does not execute arbitrary code from contract bodies
- Extension fields are preserved but validated for structure
- Namespace validation rejects ambiguous `http:` / `https:` identifiers
- Governance metadata can enforce owner/steward requirements on restricted classifications

The reference validator performs static analysis only. It does not connect to external systems, networks, or secrets.

For governance requirements, see SPEC Chapter 26 and the metadata validation rules in Chapter 5. Automated probes: [security-checklist.md](security-checklist.md).

## Distribution

| Channel | Package |
|---------|---------|
| Rust | [crates.io/crates/dtcs](https://crates.io/crates/dtcs) |
| Python | [pypi.org/project/dtcs](https://pypi.org/project/dtcs) |
| WASM | [`@eddiethedean/dtcs-wasm`](https://www.npmjs.com/package/@eddiethedean/dtcs-wasm) |
| Node | [`@eddiethedean/dtcs`](https://www.npmjs.com/package/@eddiethedean/dtcs) |
| Source | [github.com/eddiethedean/dtcs](https://github.com/eddiethedean/dtcs) |

Rust and Python packages install the same `dtcs` CLI.

## Evaluation checklist

- [ ] Validate existing pipeline contracts with `dtcs validate`
- [ ] Review diagnostic output for schema and lineage gaps (`operation` / `flow`)
- [ ] Author or review a dataset action with `parameters` (for example `dtcs:project`)
- [ ] Exercise null / missing / invalid tokens with `dtcs run` and inspect JSON carefully
- [ ] Compare current and proposed contract versions with `dtcs compat`
- [ ] Trace lineage for critical inputs with `dtcs lineage --impact`
- [ ] Match plans against engine capabilities with `dtcs match examples/customer_pipeline.dtcs.yaml` (from a clone)
- [ ] Compile contracts to execution plans with `dtcs compile examples/customer_pipeline.dtcs.yaml`
- [ ] Execute end-to-end with the reference runtime: `dtcs run examples/customer_pipeline.dtcs.yaml --input tests/fixtures/runtime/customer_pipeline_input.json`
- [ ] Run offline conformance certification: `dtcs conformance run --profile all` (includes Analyzer assertions)
- [ ] Review [security-checklist.md](security-checklist.md) for Ch 24 requirements
- [ ] Read SPEC Chapters 1–3 and [Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative) for design principles and the standard library catalog
- [ ] Skim [faq.md](../user/faq.md#migration-to-0110) if upgrading from 0.10.x

Conformance certification is available via `dtcs conformance declare` and `dtcs conformance run`. See [conformance.md](../user/conformance.md). External certification authority remains out of scope per Ch 23 §13.

## Getting started

Practitioners: [getting-started.md](../user/getting-started.md) · [cli-guide.md](../user/cli-guide.md) · [writing-contracts.md](../user/writing-contracts.md)

## Relationship to other standards

| Standard | Relationship |
|----------|-------------|
| JSON Schema / Avro / Protobuf | Schema formats; DTCS adds transformation semantics, lineage, and governance |
| OpenAPI | API contracts; DTCS focuses on data transformation semantics |
| dbt | Transformation execution; DTCS contracts could describe dbt model semantics portably |
| W3C PROV | Provenance; DTCS lineage covers dataset-level input→output mappings |

DTCS is complementary — it does not replace schema or API standards but adds a transformation contract layer above them.

## Questions

Open a [GitHub issue](https://github.com/eddiethedean/dtcs/issues) or see [faq.md](../user/faq.md).
