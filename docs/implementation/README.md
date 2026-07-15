# Reference implementation docs

Guides for the Rust reference crate in [`src/`](https://github.com/eddiethedean/dtcs/tree/main/src).

Treat [`SPEC.md`](../SPEC.md) as the authoritative source of truth. User-facing docs start at [Getting started](../user/getting-started.md) and [Concepts](../user/concepts.md).

## Start here

| Document | Description |
|----------|-------------|
| [architecture.md](architecture.md) | Layers, binding matrix, error model |
| [crate-layout.md](crate-layout.md) | Module map |
| [public-api.md](public-api.md) | Library surface notes (implementers) |
| [spec-completeness.md](spec-completeness.md) | Chapter coverage matrix |
| [release-runbook.md](release-runbook.md) | Maintainer release checklist |
| [non-goals.md](non-goals.md) | Explicit out-of-scope items |
| [testing-plan.md](testing-plan.md) | Fixtures and golden strategy |

## Pipeline (current tools)

```text
parse → COM → validate → analyze → plan → optimize → match → compile → run
                                                              └→ conformance
```

Consumer API pages: [Python](../api/python.md) · [Rust](../api/rust.md) · [WASM](../api/wasm.md) · [Node](../api/node.md).
