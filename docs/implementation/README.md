# DTCS Rust Implementation Starter Pack

This guide set supports building the Rust reference implementation of DTCS.

See the canonical documentation index at [`docs/README.md`](../README.md).

Treat [`SPEC.md`](../../SPEC.md) as the authoritative source of truth.

The implementation pipeline through Phase 0.9:

```text
parse -> Canonical Object Model -> validate -> diagnostics
                                              -> analyze (compat, evolution, lineage, versioning)
                                              -> analyze (semantics, expressions)
                                              -> plan (lowering, graph, validation)
                                              -> optimize (semantics-preserving rewrites)
                                              -> capability match -> compile -> runtime execute
```

Do not implement multi-backend compilers or production ETL orchestration without an agreed milestone.

## Related documents

- [project-goal.md](project-goal.md)
- [spec-completeness.md](spec-completeness.md)
- [architecture.md](architecture.md)
- [crate-layout.md](crate-layout.md)
- [public-api.md](public-api.md)
- [implementation-phases.md](implementation-phases.md)
- [spec-usage.md](spec-usage.md)
- [testing-plan.md](testing-plan.md)
