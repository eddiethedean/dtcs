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

For expressions, see [expressions.md](expressions.md).

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

The validator checks that action identifiers are well-formed, that `dtcs:` identifiers exist in the embedded standard library, and that targets satisfy registry definition constraints (for example `dtcs:lowercase` requires a non-nullable `string` field; `dtcs:capitalize` allows nullable strings).

### Available semantic actions

| Identifier | Target type | Nullable target |
|------------|-------------|-----------------|
| `dtcs:lowercase` | `string` | No |
| `dtcs:uppercase` | `string` | No |
| `dtcs:capitalize` | `string` | Yes |
| `dtcs:trim` | `string` | Yes |
| `dtcs:normalize_whitespace` | `string` | Yes |
| `dtcs:hash_sha256` | `string` | Yes |

## Functions

Declare reusable functions in the contract `functions` block. Standard library function identifiers are validated against embedded registry definitions (parameter count, argument types, return type):

```yaml
functions:
  - id: "full_name"
    function: "dtcs:concat"
    parameters:
      - name: "first"
        type: "string"
      - name: "last"
        type: "string"
    returns:
      type: "string"
      nullable: false
```

### Available functions

| Identifier | Arity | Notes |
|------------|-------|-------|
| `dtcs:lower`, `dtcs:upper` | 1 | `string` → `string` |
| `dtcs:concat` | 2+ | all `string` arguments |
| `dtcs:substr` | 2–3 | `string`, `integer` start, optional `integer` length |
| `dtcs:replace` | 3 | `string` arguments |
| `dtcs:coalesce` | 1+ | homogeneous argument types |
| `dtcs:length` | 1 | `string` or `binary` → `integer` |
| `dtcs:to_string` | 1 | primitive → `string` |
| `dtcs:to_integer`, `dtcs:to_decimal` | 1 | numeric/string coercion |

## Rules

Rules express constraints on fields:

```yaml
rules:
  - id: "customer_id_required"
    rule: "dtcs:not_null"
    target: "customer_raw.customer_id"
    phase: "postcondition"
```

Additional rule examples (not in the primary example contract):

```yaml
rules:
  - id: "email_min"
    rule: "dtcs:min_length"
    target: "customer_raw.email"
    phase: "postcondition"
    parameters:
      min: 5
```

Standard library rules and actions are discoverable via:

```bash
dtcs registry list
dtcs registry resolve dtcs:uppercase --json
dtcs registry resolve dtcs:length --json
dtcs registry resolve dtcs:range --json
```

### Available rules

| Identifier | Target type | Phases |
|------------|-------------|--------|
| `dtcs:not_null` | any | precondition, execution, postcondition |
| `dtcs:min_length`, `dtcs:max_length`, `dtcs:regex_match` | `string` | precondition, execution, postcondition |
| `dtcs:range` | `integer` | precondition, execution, postcondition |

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
| `dtcs:unknown-registry-entry` | Use a `dtcs:` identifier from `dtcs registry list`, or declare a vendor extension |
| stdlib semantics errors | Match target field type/nullability and rule `phase` to the registry definition |

See [faq.md](faq.md) and [troubleshooting.md](troubleshooting.md) for more help.

## Next steps

- Expression syntax: [expressions.md](expressions.md)
- Compare contract versions: [compatibility.md](compatibility.md)
- All CLI commands: [cli-guide.md](cli-guide.md)
- Full specification: [SPEC.md](../../SPEC.md)
