# Adoption Overview

This document helps architects and enterprise evaluators assess DTCS for their organization.

## What DTCS provides

DTCS standardizes **transformation semantics** as portable, vendor-neutral contracts. A DTCS contract captures:

- Input and output schemas
- Semantic actions, expressions, functions, and rules
- Lineage (which inputs feed which outputs)
- Metadata (governance, provenance, classification)
- Versioning and compatibility policies

It does **not** define execution engines, storage, orchestration, or SQL dialects.

## Reference implementation maturity

| Component | Status |
|-----------|--------|
| Specification | Draft (`1.0.0-draft`, 26 chapters) |
| Parser (YAML/JSON) | Complete |
| Seven-phase validation | Complete |
| Metadata validation | Complete |
| Type system (incl. expressions) | Complete |
| Compatibility analysis | Complete (five classification levels) |
| Evolution analysis | Complete |
| Versioning validation (Ch 25) | Complete |
| Lineage analysis (dataset-level) | Complete |
| Identifier registry & extensibility | Complete (Phase 0.4, `0.4.0`) |
| Static semantic analysis | Complete (Phase 0.6, `0.6.0`) |
| Transformation plan lowering | Complete (Phase 0.7, `0.7.0`) |
| Plan optimization | Complete (Phase 0.8, `0.8.0`) |
| Execution / runtime | Not started (Phase 0.9) |
| Conformance certification (Ch 23) | Not started (Phase 0.10) |

*Released reference implementation: `0.8.0`.*

See [ROADMAP.md](../../ROADMAP.md) for the full milestone plan.

## What you can use today

1. **Contract authoring** — write YAML/JSON contracts with IDE/CI validation
2. **CI gates** — fail builds on invalid contracts (`dtcs validate --json`)
3. **Standard library usage** — reference `dtcs:` actions, functions, and rules; validate against embedded definitions (`dtcs registry list`)
4. **Static semantic analysis** — check transformation semantics and expressions without runtime evaluation (`dtcs analyze`)
5. **Version management** — compare contract revisions for breaking changes (`dtcs compat`, `dtcs evolve`)
6. **Impact analysis** — trace which outputs depend on an input (`dtcs lineage --impact`)
7. **Plan lowering** — produce canonical transformation plans from validated contracts (`dtcs plan`)
8. **Plan optimization** — apply semantics-preserving rewrites to lowered plans (`dtcs optimize`)
9. **Governance hooks** — metadata validation enforces owner/steward on restricted classifications

## What is explicitly out of scope

- ETL execution (Spark, Polars, SQL compilation)
- Runtime engines
- WASM/Node bindings
- Conformance profiles and certification suites

See [non-goals.md](../implementation/non-goals.md).

## Security considerations

Normative security guidance is in [SPEC.md Chapter 24](../../SPEC.md#chapter-24----security-considerations). At a high level:

- Contracts are **declarative documents** — the validator parses and analyzes them; it does not execute arbitrary code from contract bodies
- Extension fields are preserved but validated for structure
- Namespace validation rejects ambiguous `http:` / `https:` identifiers
- Governance metadata can enforce owner/steward requirements on restricted classifications

The reference validator performs static analysis only. It does not connect to external systems, networks, or secrets.

For governance requirements, see SPEC Chapter 26 and the metadata validation rules in Chapter 5.

## Distribution

| Channel | Package |
|---------|---------|
| Rust | [crates.io/crates/dtcs](https://crates.io/crates/dtcs) |
| Python | [pypi.org/project/dtcs](https://pypi.org/project/dtcs) |
| Source | [github.com/eddiethedean/dtcs](https://github.com/eddiethedean/dtcs) |

Both Rust and Python packages install the same `dtcs` CLI.

## Evaluation checklist

- [ ] Validate existing pipeline contracts with `dtcs validate`
- [ ] Review diagnostic output for schema and lineage gaps
- [ ] Compare current and proposed contract versions with `dtcs compat`
- [ ] Trace lineage for critical inputs with `dtcs lineage --impact`
- [ ] Review [ROADMAP.md](../../ROADMAP.md) for planned execution/runtime features
- [ ] Read SPEC Chapters 1–3 for design principles and scope

## Relationship to other standards

| Standard | Relationship |
|----------|-------------|
| JSON Schema / Avro / Protobuf | Schema formats; DTCS adds transformation semantics, lineage, and governance |
| OpenAPI | API contracts; DTCS focuses on data transformation semantics |
| dbt | Transformation execution; DTCS contracts could describe dbt model semantics portably |
| W3C PROV | Provenance; DTCS lineage covers dataset-level input→output mappings |

DTCS is complementary — it does not replace schema or API standards but adds a transformation contract layer above them.

## Questions

Open a GitHub issue or see [faq.md](../user/faq.md).
