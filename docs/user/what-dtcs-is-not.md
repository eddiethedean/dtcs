# What DTCS is (and is not)

Short design philosophy for evaluators and new users. Normative detail: [SPEC.md](../SPEC.md). Limits: [limits.md](limits.md).

## What it is

- A **draft standard** for vendor-neutral **data transformation contracts** (YAML/JSON)
- A way to capture **semantics**: schemas, actions, expressions, rules, lineage, guarantees
- A **reference toolkit** to parse, validate, analyze, plan, and (for small fixtures) run contracts
- A **CI / governance** surface: fail builds on invalid contracts; compare revisions; declare capabilities

## What it is not

| Not this | Instead |
|----------|---------|
| A Spark / Polars / DuckDB / SQL engine | Contracts describe meaning; your engine executes |
| A production ETL runtime | `dtcs run` is an in-memory teaching interpreter |
| A storage or orchestration system | Out of scope by design |
| A finished, ratified standard | Spec `3.0.0` is **draft**; tools are **alpha** |
| A dual-compiler certified portable suite for all A.9 profiles | Advanced profiles are **Experimental**; `/2` is **Candidate** |
| A full Node/WASM analytics stack | npm bindings: parse / validate / declare only |

## Mental model

```text
Author contract  →  validate / analyze in CI  →  match engine capability
                                              →  execute in your platform
```

Use DTCS so teams agree on **what** a transform means. Implement **how** in the engine you already trust.

## Next

- [getting-started.md](getting-started.md)
- [concepts.md](concepts.md)
- [adoption/overview.md](../adoption/overview.md)
- [non-goals.md](../implementation/non-goals.md) (implementer detail)
