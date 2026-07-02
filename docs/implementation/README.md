# DTCS Rust Implementation Starter Pack

This guide set supports building the Rust reference implementation of DTCS.

See the canonical documentation index at [`docs/README.md`](../README.md).

Treat [`SPEC.md`](../../SPEC.md) as the authoritative source of truth.

The first implementation goal is:

```text
parse -> Canonical Object Model -> validate -> diagnostics
```

Do not implement execution, optimization, or backend compilation yet.

## Related documents

- [project-goal.md](project-goal.md)
- [architecture.md](architecture.md)
- [crate-layout.md](crate-layout.md)
- [public-api.md](public-api.md)
- [implementation-phases.md](implementation-phases.md)
- [spec-usage.md](spec-usage.md)

Cursor build prompt: [`.cursor/prompts/build.md`](../../.cursor/prompts/build.md)
