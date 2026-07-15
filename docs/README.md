# Documentation

Hosted site: [dtcs.readthedocs.io](https://dtcs.readthedocs.io/).

Preview locally:

```bash
pip install -r docs/requirements.txt
NO_MKDOCS_2_WARNING=true mkdocs serve
```

Configuration: [`mkdocs.yml`](https://github.com/eddiethedean/dtcs/blob/main/mkdocs.yml) · [`.readthedocs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/.readthedocs.yaml).

## Choose your path

| Audience | Start here |
|----------|------------|
| **New users** | [user/getting-started.md](user/getting-started.md) → [user/concepts.md](user/concepts.md) |
| **Evaluators / architects** | [adoption/overview.md](adoption/overview.md) |
| **Contributors** | [CONTRIBUTING.md](https://github.com/eddiethedean/dtcs/blob/main/CONTRIBUTING.md) · [implementation/architecture.md](implementation/architecture.md) |
| **Spec authors** | [editorial/baseline.md](editorial/baseline.md) · [SPEC.md](SPEC.md) |

## Tutorials

| Document | Description |
|----------|-------------|
| [getting-started.md](user/getting-started.md) | Install and validate without cloning |
| [concepts.md](user/concepts.md) | COM → validate → plan → run |
| [writing-contracts.md](user/writing-contracts.md) | Contract structure and catalog |
| [cookbook.md](user/cookbook.md) | Short recipes |

## Guides

| Document | Description |
|----------|-------------|
| [cli-guide.md](user/cli-guide.md) | Commands, flags, exit codes |
| [expressions.md](user/expressions.md) | Expression language and null tokens |
| [compatibility.md](user/compatibility.md) | Compatibility classifications |
| [json-output.md](user/json-output.md) | JSON envelopes |
| [ci-integration.md](user/ci-integration.md) | Adopter CI gates |
| [conformance.md](user/conformance.md) | Conformance profiles |
| [extensions-and-registries.md](user/extensions-and-registries.md) | Vendor catalogs |
| [migration-0.11.md](user/migration-0.11.md) | Upgrade from 0.10.x |
| [limits.md](user/limits.md) | Runtime / deployment limits |
| [troubleshooting.md](user/troubleshooting.md) | Common failures |
| [faq.md](user/faq.md) | FAQ |
| [glossary.md](user/glossary.md) | Terms |

## Adoption

| Document | Description |
|----------|-------------|
| [overview.md](adoption/overview.md) | Evaluation checklist |
| [security-checklist.md](adoption/security-checklist.md) | Ch 24 probes |
| [SECURITY.md](https://github.com/eddiethedean/dtcs/blob/main/SECURITY.md) | Vulnerability reporting |

## API reference

| Document | Description |
|----------|-------------|
| [api/python.md](api/python.md) | Python package |
| [api/rust.md](api/rust.md) | Rust crate + docs.rs |
| [api/wasm.md](api/wasm.md) | WASM bindings |
| [api/node.md](api/node.md) | Node bindings |

## Specification

| Document | Description |
|----------|-------------|
| [SPEC.md](SPEC.md) | Normative draft |
| [Appendix A](SPEC.md#appendix-a-standard-library-catalog-normative) | Stdlib catalog |
| [ROADMAP.md](https://github.com/eddiethedean/dtcs/blob/main/ROADMAP.md) | Milestones |
| [spec-completeness.md](implementation/spec-completeness.md) | Coverage matrix |
| [CHANGELOG.md](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md) | Release notes |

## Contribute

| Document | Description |
|----------|-------------|
| [implementation/README.md](implementation/README.md) | Implementer index |
| [architecture.md](implementation/architecture.md) | Layers and bindings |
| [release-runbook.md](implementation/release-runbook.md) | Maintainer releases |
| [editorial/baseline.md](editorial/baseline.md) | Spec authoring process |
