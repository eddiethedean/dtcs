# Writing Contracts

A DTCS transformation contract is a YAML or JSON document describing **what** a data transformation means — inputs, outputs, semantics, and lineage — without prescribing an execution engine.

This guide uses field tables and snippets. For end-to-end samples after you clone (or download from GitHub), see [`examples/minimal.dtcs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/examples/minimal.dtcs.yaml) and the sample pipeline [`examples/customer_pipeline.dtcs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/examples/customer_pipeline.dtcs.yaml). Normative rules: [SPEC.md](../SPEC.md) Chapter 3 (COM), Chapters 5–6, and [Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative). Coverage: [spec-completeness.md](../implementation/spec-completeness.md). Versions: [versioning.md](versioning.md).

## Minimal structure

Every contract needs:

```yaml
dtcsVersion: "3.0.0"
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
      # operation defaults to dtcs:derive when omitted
      # flow defaults to derived when omitted
```

## Top-level fields

| Field | Required | Purpose |
|-------|----------|---------|
| `dtcsVersion` | Yes | Spec edition (`"3.0.0"` preferred; `"2.0.0"` / `"1.0.0"` / `"1.0.0-draft"` accepted) |
| `id` | Yes | Stable contract identifier (namespaced string) |
| `name` | Yes | Human-readable title |
| `version` | Yes | Contract revision (semver-like string) |
| `inputs` | Yes | At least one input interface |
| `outputs` | Yes | At least one output interface |
| `lineage` | Yes when outputs exist | Maps each output to its input sources |

Optional sections: `metadata`, `semanticActions`, `expressions`, `functions`, `rules`, `guarantees`, `compatibility`, `semantics`, `versioning`, vendor extensions (namespaced keys).

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
          constraints:
            pattern: "^.+@.+$"
```

Supported types include primitives (`string`, `integer`, `decimal`, `boolean`, `binary`, `date`, `time`, `datetime`, `duration`) and composites (`list<T>`, `map<K,V>`, `object<...>`, `tuple<...>`). See SPEC Chapter 4.

Optional field `constraints` (`min`, `max`, `pattern`, `enum`) contribute to type identity.

At least one input must be required (not all inputs may be optional).

## Guarantees and compatibility

Optional first-class COM fields (SPEC Chapter 2 §3 / Chapter 3 §9):

```yaml
guarantees:
  informationLoss: "none"
  statements:
    - "row count preserved"
  semantics:
    deterministic: true

compatibility:
  policy: "dtcs:default"
  backward: true
  forward: false
  notes: "Additive optional fields only"
```

## Semantic actions

Semantic actions declare transformation intent using namespaced identifiers. Field actions target `interface.field`; dataset actions target an interface id and use a `parameters` map.

```yaml
semanticActions:
  - id: "normalize_email"
    action: "dtcs:lowercase"
    target: "customer_raw.email"

  - id: "keep_email_only"
    action: "dtcs:project"
    target: "customer_raw"
    parameters:
      fields: ["customer_id", "email"]
```

The validator checks that action identifiers are well-formed, that `dtcs:` identifiers exist in the embedded standard library, and that targets/parameters satisfy registry definitions. Vendor keys on action objects (for example `acme:tag`) are preserved as nested extensions.

### Field transforms

| Identifier | Target type | Nullable target |
|------------|-------------|-----------------|
| `dtcs:lowercase` | `string` | No |
| `dtcs:uppercase` | `string` | No |
| `dtcs:capitalize` | `string` | Yes |
| `dtcs:trim` | `string` | Yes |
| `dtcs:normalize_whitespace` | `string` | Yes |
| `dtcs:hash_sha256` | `string` | Yes |

### Dataset operators

| Identifier | Category | Required parameters |
|------------|----------|---------------------|
| `dtcs:project` | projection | `fields` (array) |
| `dtcs:select` | selection | `field`; optional `equals` |
| `dtcs:filter` | filtering | `field`; optional `equals` |
| `dtcs:aggregate` | aggregation | `groupBy`, `valueField`, `op` |
| `dtcs:group` | grouping | `groupBy`, `valueField`, `op` |
| `dtcs:join` | joining | `right`, `leftKey`; optional `rightKey` |
| `dtcs:sort` | sorting | `field`; optional `descending` |
| `dtcs:union` | union | `other` |
| `dtcs:partition` | partitioning | `field` |

`dtcs:derive` is the default lineage mapping `operation` when omitted (see below). Full catalog: [SPEC Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative).

## Functions

Declare reusable functions in the contract `functions` block. Standard library function identifiers are validated against embedded registry definitions (parameter count, argument types, return type, null behavior):

```yaml
functions:
  - id: "full_name"
    function: "dtcs:concat"
    type: "string"
    nullable: false
    nullBehavior: propagate
    deterministic: true
    parameters:
      - name: "first"
        type: "string"
      - name: "last"
        type: "string"
```

### Available functions

| Identifier | Arity | Notes |
|------------|-------|-------|
| `dtcs:lower`, `dtcs:upper` | 1 | `string` → `string` |
| `dtcs:concat` | 2+ | all `string` arguments |
| `dtcs:substr` | 2–3 | `string`, `integer` start, optional `integer` length |
| `dtcs:replace` | 3 | `string` arguments |
| `dtcs:coalesce` | 1+ | first non-null |
| `dtcs:length` | 1 | `string` or `binary` → `integer` |
| `dtcs:to_string` | 1 | primitive → `string` |
| `dtcs:to_integer`, `dtcs:to_decimal` | 1 | numeric/string coercion |
| `dtcs:abs` | 1 | numeric absolute value |
| `dtcs:min`, `dtcs:max` | 2+ | variadic numeric |
| `dtcs:contains` | 2 | substring containment |
| `dtcs:is_null`, `dtcs:is_missing` | 1 | null/missing predicates (`nullBehavior: defined`) |

## Rules

Rules express constraints on fields (or other scopes):

```yaml
rules:
  - id: "customer_id_required"
    rule: "dtcs:not_null"
    target: "customer_raw.customer_id"
    phase: "postcondition"
    scope: input
    deterministic: true
```

Optional fields: `scope` (`contract`, `input`, `output`, `expression`, `semanticAction`, `plan`, `executionPlan`), `allowIndeterminate`, `parameters`, nested vendor extensions.

```yaml
rules:
  - id: "email_min"
    rule: "dtcs:min_length"
    target: "customer_raw.email"
    phase: "postcondition"
    parameters:
      min: 5
  - id: "status_one_of"
    rule: "dtcs:one_of"
    target: "customer_raw.status"
    phase: "precondition"
    parameters:
      values: ["active", "pending"]
```

### Available rules

| Identifier | Parameters | Notes |
|------------|------------|-------|
| `dtcs:not_null` | — | present and non-null |
| `dtcs:min_length`, `dtcs:max_length` | `min` / `max` | string length |
| `dtcs:regex_match` | `pattern` | regular expression |
| `dtcs:range` | optional `min`, `max` | inclusive numeric bounds |
| `dtcs:one_of` | `values` (array) | membership |
| `dtcs:equals` | `value` | equality; may permit indeterminate |

Discover catalog entries:

```bash
dtcs registry list
dtcs registry resolve dtcs:project --json
dtcs registry resolve dtcs:is_missing --json
dtcs registry resolve dtcs:one_of --json
```

## Lineage

Every output must appear in `lineage.mappings`. From 0.11.0, mappings also carry semantic operation and information-flow kind:

```yaml
lineage:
  mappings:
    - id: "map_clean"
      output: "customer_clean"
      inputs: ["customer_raw"]
      operation: "dtcs:derive"   # default when omitted
      flow: derived              # preserved | derived | aggregated | filtered | partitioned | discarded
```

Missing lineage for an output produces a `dtcs:missing-lineage` error. Information loss that affects guarantees should use an explicit `flow` such as `discarded` or `aggregated`.

## Metadata

Optional but recommended for governance:

```yaml
metadata:
  description: "Normalizes customer email addresses"
  classification: internal
  ownership:
    owner: "data-platform"
    organization: "analytics"
  lifecycle:
    state: "active"
  governance:
    owner: "data-platform"
    steward: "customer-analytics"
  provenance:
    author: "platform-team"
    createdAt: "2026-01-01T00:00:00Z"
  deprecated: false
  replacement: null
  anticipatedRemoval: null   # version or date when deprecated
```

## Null, missing, and invalid

Runtime and expression evaluation distinguish:

| Kind | Meaning | JSON |
|------|---------|------|
| null | Present key, null payload | `null` |
| missing | Explicit missing token | `{"$dtcs":"missing"}` |
| invalid | Explicit invalid token | `{"$dtcs":"invalid", "reason": "..."}` |

Do not coerce missing/invalid to null in consumers. See [expressions.md](expressions.md) and [SPEC Appendix A.7](../SPEC.md#a7-null-semantics-tokens).

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
| `dtcs:unsupported-version` | Set `dtcsVersion` to `"3.0.0"` (preferred) or `"2.0.0"` / `"1.0.0"` / `"1.0.0-draft"` |
| `dtcs:invalid-type` | Fix type syntax (e.g. `list<string>` not `list`) |
| `dtcs:unknown-registry-entry` | Use a `dtcs:` identifier from `dtcs registry list`, or declare a vendor extension |
| `dtcs:invalid-semantic-action` | Supply required `parameters` for dataset actions; match target type/nullability |
| stdlib semantics errors | Match target field type/nullability and rule `phase` to the registry definition |

See [faq.md](faq.md) and [troubleshooting.md](troubleshooting.md) for more help. Upgrading to 0.13 / Spec 3.0: [migration-0.13.md](migration-0.13.md). Historical 0.12 notes: [migration-0.12.md](migration-0.12.md).

## Next steps

- Expression syntax and null semantics: [expressions.md](expressions.md)
- Compare contract versions: [compatibility.md](compatibility.md)
- All CLI commands: [cli-guide.md](cli-guide.md)
- Conformance certification: [conformance.md](conformance.md)
- Normative catalog: [SPEC Appendix A](../SPEC.md#appendix-a-standard-library-catalog-normative)
- Full specification: [SPEC.md](../SPEC.md)
