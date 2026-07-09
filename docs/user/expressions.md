# Expressions

DTCS expressions (SPEC Chapter 8) declare typed computation in a contract without prescribing an execution engine. The reference implementation validates expression syntax and types during `dtcs analyze` but does not evaluate expressions at runtime unless they are lowered through semantic actions, functions, or rules.

For normative rules, see [SPEC.md](../../SPEC.md) Chapter 8. For contract structure, see [writing-contracts.md](writing-contracts.md).

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
| `type` | Yes | Declared result type |

## Syntax overview

Expressions support:

- **Literals** — integers, decimals, strings, booleans, `null`
- **Field references** — `interface.field` (for example `in.a`)
- **Arithmetic** — `+`, `-`, `*`, `/` with standard precedence
- **Comparisons** — `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical** — `and`, `or`, `not`
- **Function calls** — `dtcs:concat(a, b)` using embedded standard library functions
- **Unary** — `-x`, `not x`

### Field references

Reference input or output interface fields by qualified name:

```yaml
expressions:
  - id: "sum_then_multiply"
    expr: "in.a + in.b * 2"
    type: "integer"
```

Multiplication binds tighter than addition (`in.a + (in.b * 2)`).

### Function calls

Call standard library functions with the `dtcs:` namespace:

```yaml
functions:
  - id: "greeting"
    function: "dtcs:concat"
    parameters:
      - name: "prefix"
        type: "string"
      - name: "name"
        type: "string"
    returns:
      type: "string"
      nullable: false

expressions:
  - id: "full_greeting"
    expr: "dtcs:concat('Hello, ', in.name)"
    type: "string"
```

Discover available functions:

```bash
dtcs registry list
dtcs registry resolve dtcs:concat --json
```

## Typing

Every expression must declare a `type` consistent with its body. The analyzer reports type mismatches as diagnostics during `dtcs analyze`.

Common types: `string`, `integer`, `decimal`, `boolean`, `date`, `time`, `timestamp`, `list<T>`, `map<K,V>`.

Nullable field references affect comparison and logical typing. See SPEC Chapter 4 for the full type system.

## Relationship to other contract sections

| Mechanism | Purpose |
|-----------|---------|
| **Semantic actions** | Declare transformation intent on a field target (`dtcs:lowercase` on `in.email`) |
| **Expressions** | Declare typed computation (may reference fields and call functions) |
| **Functions** | Declare reusable callable signatures used in expressions |
| **Rules** | Declare constraints (`dtcs:not_null`, `dtcs:range`) with a `phase` |

Expressions complement semantic actions — use actions for standard library transforms on fields, and expressions for computed values.

## Fixture examples

The test corpus includes additional expression fixtures under `tests/fixtures/`:

| Fixture | Demonstrates |
|---------|--------------|
| `analysis_constant_expr.yaml` | Integer literal arithmetic |
| `expression_precedence_multiply.yaml` | Operator precedence |
| `expression_unary_minus.yaml` | Unary negation |
| `analysis_logical_ops.yaml` | `and` / `or` / `not` |
| `analysis_dtcs_call_valid.yaml` | Standard library function call |
| `expression_type_mismatch.yaml` | Type error (invalid fixture) |

```bash
dtcs analyze tests/fixtures/expression_precedence_multiply.yaml --json
dtcs diagnostics tests/fixtures/expression_type_mismatch.yaml
```

## Next steps

- Contract structure and stdlib catalog: [writing-contracts.md](writing-contracts.md)
- Static analysis: [getting-started.md](getting-started.md#analyze-semantics-and-expressions)
- JSON output for `analyze`: [json-output.md](json-output.md)
- Normative expression grammar: [SPEC.md](../../SPEC.md) Chapter 8
