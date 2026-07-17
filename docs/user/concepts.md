# Concepts

Two-page mental model for DTCS. Normative detail lives in [SPEC.md](../SPEC.md).

## What a contract is

A **Transformation Contract** is a YAML/JSON document that declares:

1. **Identity** — `id`, `name`, `version`, and `dtcsVersion` (must be `"1.0.0"` today)
2. **Interfaces** — named `inputs` / `outputs` with schemas
3. **Semantics** — `semanticActions`, optional `functions`, `expressions`, and `rules`
4. **Lineage** — how each output relates to inputs (`operation`, `flow`)
5. **Metadata** — ownership, governance, provenance, classification

DTCS describes *what the transformation means*. Engines decide *how* to run it. The reference runtime is an in-memory teaching/checking tool, not a warehouse executor.

## Pipeline

```text
Document → parse → Canonical Object Model (COM)
                 → validate (+ registry)
                 → analyze (semantics / compat / evolve / lineage)
                 → plan (lower to IR)
                 → optimize (optional)
                 → match (engine capabilities)
                 → compile (execution plan)
                 → run (reference runtime)
```

| Stage | Question it answers |
|-------|---------------------|
| **Validate** | Is this document a well-formed, resolvable DTCS contract? |
| **Analyze** | Are semantics / expressions coherent without executing rows? |
| **Plan** | What ordered IR nodes implement the contract? |
| **Match** | Can a given engine capability profile run this plan? |
| **Compile** | What concrete steps should the reference (or another) runtime execute? |
| **Run** | Given sample rows, what outputs and diagnostics appear? |

## Identifiers and the registry

Standard actions, functions, rules, and diagnostic codes use `dtcs:` IDs (for example `dtcs:lowercase`, `dtcs:project`, `dtcs:one_of`). The tools ship an embedded registry ([Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative)). Vendor IDs use other namespaces and may require `--registry`.

## Null, missing, and invalid

At runtime, JSON `null`, missing fields, and invalid values are distinct. Encoded forms include `{"$dtcs":"missing"}` and `{"$dtcs":"invalid"}`. Do not coerce those tokens to `null` / `None` in consumers. See [expressions.md](expressions.md).

## Compatibility vs evolution

- **`compat`** — can consumers of contract A adopt contract B? (classification levels)
- **`evolve`** — how did two revisions of the *same* contract identity change?

## Maturity

Spec `2.0.0` · tools `0.12.0` (alpha). Coverage tables mean the **reference implementation exercises** draft chapters — not that DTCS 1.0 is finalized or production-certified.

## Next

- [getting-started.md](getting-started.md) — install and first `valid`
- [writing-contracts.md](writing-contracts.md) — field reference and catalog
- [cookbook.md](cookbook.md) — short recipes
