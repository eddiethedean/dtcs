# Writing Contracts

A DTCS transformation contract is a YAML or JSON document describing **what** a data transformation means — inputs, outputs, semantics, and lineage — without prescribing an execution engine.

This guide walks through [`examples/customer_normalize.dtcs.yaml`](../../examples/customer_normalize.dtcs.yaml). For normative rules, see [SPEC.md](../../SPEC.md) Chapter 3 (COM) and Chapters 5–6.

## Minimal structure

Every contract needs:

```yaml
dtcsVersion: "1.0.0"
id: "my.transform"
name: "My Transform"
version: "1.0.0"

inputs:
  - id: "in"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false

outputs:
  - id: "out"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false

lineage:
  mappings:
    - output: "out"
      inputs: ["in"]
```

## Top-level fields

| Field | Required | Purpose |
|-------|----------|---------|
| `dtcsVersion` | Yes | Spec version the document targets (`1.0.0` for current draft) |
| `id` | Yes | Stable contract identifier (namespaced string) |
| `name` | Yes | Human-readable title |
| `version` | Yes | Contract revision (semver-like string) |
| `inputs` | Yes | At least one input interface |
| `outputs` | Yes | At least one output interface |
| `lineage` | Yes when outputs exist | Maps each output to its input sources |

Optional sections: `metadata`, `semanticActions`, `expressions`, `functions`, `rules`, `versioning`, `extensions`.

## Inputs and outputs

Each interface has an `id` and a `schema` with typed fields:

```yaml
inputs:
  - id: "customer_raw"
    schema:
      fields:
        - name: "customer_id"
          type: "string"
          nullable: false
        - name: "email"
          type: "string"
          nullable: false
```

Supported types include primitives (`string`, `integer`, `decimal`, `boolean`, `date`, `time`, `timestamp`) and composites (`list<T>`, `map<K,V>`). See SPEC Chapter 4.

At least one input must be required (not all inputs may be optional).

## Semantic actions

Semantic actions declare transformation intent using namespaced identifiers:

```yaml
semanticActions:
  - id: "normalize_email"
    action: "dtcs:lowercase"
    target: "customer_raw.email"
```

The validator checks that action identifiers are well-formed and that field references resolve.

## Rules

Rules express constraints on fields:

```yaml
rules:
  - id: "customer_id_required"
    rule: "dtcs:not_null"
    target: "customer_raw.customer_id"
    phase: "postcondition"
```

## Lineage

Every output must appear in `lineage.mappings`:

```yaml
lineage:
  mappings:
    - output: "customer_clean"
      inputs: ["customer_raw"]
```

Missing lineage for an output produces a `dtcs:missing-lineage` error.

## Metadata

Optional but recommended for governance:

```yaml
metadata:
  description: "Normalizes customer email addresses"
  classification: internal
  governance:
    owner: "data-platform"
    steward: "customer-analytics"
  provenance:
    author: "platform-team"
    createdAt: "2026-01-01T00:00:00Z"
```

## Validate as you write

```bash
dtcs validate my_contract.yaml
dtcs diagnostics my_contract.yaml --json
```

Common first-time errors:

| Diagnostic | Fix |
|------------|-----|
| `dtcs:missing-lineage` | Add a lineage mapping for each output |
| `dtcs:unresolved-reference` | Check field paths match `interface.field` format |
| `dtcs:unsupported-version` | Set `dtcsVersion` to a supported value (`1.0.0`) |
| `dtcs:invalid-type` | Fix type syntax (e.g. `list<string>` not `list`) |

See [faq.md](faq.md) for more troubleshooting.

## Next steps

- Compare contract versions: [compatibility.md](compatibility.md)
- All CLI commands: [cli-guide.md](cli-guide.md)
- Full specification: [SPEC.md](../../SPEC.md)
