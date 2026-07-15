# Expressions

DTCS expressions (SPEC Chapter 8) declare typed computation in a contract without prescribing an execution engine. The reference implementation validates expression syntax and types during `dtcs analyze`, and evaluates expressions during `dtcs run` / `runtime_execute` when they appear in an execution plan.

For normative rules, see [SPEC.md](../SPEC.md) Chapter 8 and [Appendix A.7](../SPEC.md#a7-null-semantics-tokens). For contract structure, see [writing-contracts.md](writing-contracts.md).

## Minimal example

```yaml
dtcsVersion: "1.0.0"
id: "analysis.constant.expr"
name: "Constant expression"
version: "0.1.0"

inputs:
  - id: "in"
    schema:
      fields:
        - name: "value"
          type: "integer"
          nullable: false

outputs:
  - id: "out"
    schema:
      fields:
        - name: "value"
          type: "integer"
          nullable: false

expressions:
  - id: "const_add"
    expr: "1 + 2"
    type: "integer"
    nullBehavior: propagate
    deterministic: true

lineage:
  mappings:
    - output: "out"
      inputs: ["in"]
```

Validate and analyze:

```bash
dtcs validate tests/fixtures/analysis_constant_expr.yaml
dtcs analyze tests/fixtures/analysis_constant_expr.yaml
```

## Expression block fields

| Field | Required | Purpose |
|-------|----------|---------|
| `id` | Yes | Stable identifier within the contract |
| `expr` | Yes | Expression body (see syntax below) |
| `type` | Recommended | Declared result type |
| `nullBehavior` | No | `propagate`, `reject`, `coalesce`, or `defined` |
| `deterministic` | No | Defaults to `true` |
| `nonDeterminismSource` | No | Required when `deterministic: false` |

## Syntax overview

Expressions support:

- **Literals** — integers, decimals, strings, booleans
- **Field references** — `interface.field` (for example `in.a`)
- **Arithmetic** — `+`, `-`, `*`, `/` with standard precedence
- **Comparisons** — `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical** — `&&`, `||`, `!` (and keyword forms in some fixtures)
- **Collection operators** — `in`, `contains` (membership / containment)
- **Function calls** — `dtcs:concat(a, b)` using the standard library
- **Unary** — `-x`, `!x`

### Field references

Reference input or output interface fields by qualified name:

```yaml
expressions:
  - id: "sum_then_multiply"
    expr: "in.a + in.b * 2"
    type: "integer"
```

Multiplication binds tighter than addition (`in.a + (in.b * 2)`).

Absent fields evaluate as the **missing** token (not null). Use `dtcs:is_missing` / `dtcs:is_null` to test.

### Function calls

Call standard library functions with the `dtcs:` namespace:

```yaml
expressions:
  - id: "full_greeting"
    expr: "dtcs:concat('Hello, ', in.name)"
    type: "string"
  - id: "null_check"
    expr: "dtcs:is_null(in.optional)"
    type: "boolean"
```

Discover available functions (including `abs`, `min`, `max`, `contains`, `is_null`, `is_missing`):

```bash
dtcs registry list
dtcs registry resolve dtcs:is_missing --json
```

See [writing-contracts.md](writing-contracts.md#available-functions) and [SPEC Appendix A.4](../SPEC.md#a4-functions).

## Null, missing, and invalid

| Kind | Meaning | Serialized form |
|------|---------|-----------------|
| null | Present key with null payload | JSON `null` |
| missing | Field absent / missing token | `{"$dtcs":"missing"}` |
| invalid | Explicit invalid value | `{"$dtcs":"invalid"}` (+ optional `reason`) |

Functions declare `nullBehavior`:

- `propagate` — null or missing arguments typically yield null
- `defined` — null/missing are interpreted by the function (`is_null`, `is_missing`)

Implementations and consumers **must not** coerce missing/invalid to null unless a catalog entry explicitly defines that behavior.

## Typing

Every expression should declare a `type` consistent with its body. The analyzer reports type mismatches as diagnostics during `dtcs analyze`.

Common types: `string`, `integer`, `decimal`, `boolean`, `date`, `time`, `datetime`, `duration`, `list<T>`, `map<K,V>`.

Nullable field references affect comparison and logical typing. See SPEC Chapter 4 for the full type system.

## Relationship to other contract sections

| Mechanism | Purpose |
|-----------|---------|
| **Semantic actions** | Dataset/field mutation (`dtcs:lowercase`, `dtcs:project`, …) |
| **Expressions** | Typed computation (may reference fields and call functions) |
| **Functions** | Reusable callable signatures used in expressions |
| **Rules** | Constraints (`dtcs:not_null`, `dtcs:range`, `dtcs:one_of`) with a `phase` |

## Fixture examples

| Fixture | Demonstrates |
|---------|--------------|
| `analysis_constant_expr.yaml` | Integer literal arithmetic |
| `expression_precedence_multiply.yaml` | Operator precedence |
| `expression_unary_minus.yaml` | Unary negation |
| `analysis_logical_ops.yaml` | Logical operators |
| `analysis_dtcs_call_valid.yaml` | Standard library function call |
| `expression_type_mismatch.yaml` | Type error (invalid fixture) |

```bash
dtcs analyze tests/fixtures/expression_precedence_multiply.yaml --json
dtcs diagnostics tests/fixtures/expression_type_mismatch.yaml
```

## Next steps

- Contract structure and stdlib catalog: [writing-contracts.md](writing-contracts.md)
- Static analysis: [getting-started.md](getting-started.md#analyze-semantics-and-expressions)
- JSON output for `analyze` / `run`: [json-output.md](json-output.md)
- Normative expression language: [SPEC.md](../SPEC.md) Chapter 8
