# Architecture

Reference-implementation architecture for adopters and contributors. Phase numbers in git history are historical; runtime behavior is defined by `SPEC.md` and the current crate.

## Layers

```text
┌─────────────────────────────────────────────────────────────┐
│ Bindings: CLI · Python (PyO3) · WASM · Node (thin wrapper) │
├─────────────────────────────────────────────────────────────┤
│ Public library API (src/lib.rs)                              │
├──────────────┬──────────────┬──────────────┬────────────────┤
│ Parse / COM  │ Validate     │ Analyze      │ Registry       │
│ model::      │ validation:: │ analysis::   │ registry::     │
│              │ diagnostics::│ compatibility│                │
├──────────────┴──────────────┴──────────────┴────────────────┤
│ Plan · Optimize · Capability · Compile · Runtime             │
│ plan:: · optimize · capability:: · compile:: · runtime::     │
├─────────────────────────────────────────────────────────────┤
│ Conformance (profiles, offline suite, security probes)       │
└─────────────────────────────────────────────────────────────┘
```

| Layer | Responsibility | Mutates COM? |
|-------|----------------|--------------|
| Parse | YAML/JSON → COM | Creates |
| Validate | Structural / type / reference / semantic checks | No (report only) |
| Analyze | Compatibility, evolution, lineage, static semantics | No |
| Registry | Resolve `dtcs:` and vendor IDs | Read-only catalog |
| Plan | Lower COM → transformation IR + dependency graph | New IR |
| Optimize | Semantics-preserving rewrites | Plan only |
| Capability | Match plan vs engine profile | No |
| Compile | Plan → execution steps | New IR |
| Runtime | Execute steps on in-memory datasets | Workspaces only |
| Conformance | Certify profiles offline | No |

## Binding matrix

| Surface | Parse/validate | Plan/run | Conformance run | Notes |
|---------|----------------|----------|-----------------|-------|
| CLI (`dtcs`) | Yes | Yes | Yes | Full surface |
| Python | Yes | Yes | Yes | Same envelopes as CLI JSON |
| Rust crate | Yes | Yes | Yes | [docs.rs/dtcs](https://docs.rs/dtcs) |
| WASM | Yes | No | Declare only | Size-constrained |
| Node | Yes | No | Declare only | Wraps WASM |

## Error model

Diagnostics use stable `dtcs:` codes, severity, category, and pipeline stage. Library APIs generally return reports rather than throwing on validation failure. See [diagnostics-guide.md](diagnostics-guide.md).

## Non-goals

Production ETL orchestration, Spark/Polars/SQL backends, and hosted registry authorities are out of scope. See [non-goals.md](non-goals.md) and [limits.md](../user/limits.md).
