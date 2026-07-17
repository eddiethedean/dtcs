# Portable Conformance (R3)

This document describes how DTCS publishes and consumes **portable semantic-family** conformance fixtures, and how an external engine participates in the dual-compiler gate from SPEC Chapter 23 §5.1.

## Fixture format

Portable differential fixtures live under `tests/fixtures/portable/` and are JSON documents:

```json
{
  "id": "kernel-between-filter",
  "actions": [
    {
      "action": "dtcs:filter",
      "target": "t",
      "parameters": { "predicate": "v between 2 and 4" }
    }
  ],
  "input": { "t": [{ "v": 1 }, { "v": 3 }] },
  "expected": { "t": [{ "v": 3 }] },
  "expectError": null
}
```

- `actions` — ordered `dtcs:` semantic actions applied by the reference runtime
- `input` / `expected` — datasets keyed by interface id (JSON scalars map to runtime values; see [token dialects](#token-dialects))
- `expectError` — optional substring; when set, each evaluation mode must fail with a matching message

### Token dialects

Portable fixtures use a **fixture dialect** optimized for compact JSON:

| Kind | Fixture encoding | Runtime / CLI I/O encoding |
|------|------------------|----------------------------|
| missing | `{ "$missing": true }` | `{ "$dtcs": "missing" }` |
| invalid | `{ "$invalid": "reason" }` | `{ "$dtcs": "invalid", "reason": "…" }` (shape may vary) |
| null | JSON `null` | JSON `null` |

Adopter-facing `dtcs run --input` and Python `runtime_execute` use the **runtime I/O dialect** (`$dtcs`). See [expressions.md](../user/expressions.md#null-missing-and-invalid).

Manifest entries use assertion kind `portableDifferential` and profile ids such as:

- `dtcs:profile/portable-relational-kernel/1` (and `/2`)
- `dtcs:profile/portable-relational/1` (and `/2`)
- `dtcs:profile/portable-window/1` (and Candidate `/2`)
- `dtcs:profile/portable-complex-types/1`

**DTCS 3.0 / A.9:** Advanced Rich Portable Analytics family profiles (`portable-string-advanced`, `portable-conversion`, `portable-statistics`, `portable-complex-values`, `portable-reshape`, `portable-relational-extended`, `portable-temporal-iana`, `portable-nondeterministic`) are **Experimental**. Treat Experimental claims as reference-surface coverage, not Standard certification.

## In-repo dual path

Each portable fixture is executed twice:

1. **Direct** — expression strings evaluated as written
2. **Structured lowering** — expression strings are lowered via `to_structured_node` / `from_structured_node` / `format_expression`, then evaluated

Both paths must produce identical outputs (or identical errors). This catches string↔AST skew without shipping a second production compiler.

## External dual-compiler gate (SPEC Ch 23 §5.1)

DTCS does not embed a second SQL/DataFrame engine. Certification against another compiler:

1. Consume the same `tests/fixtures/portable/*.json` corpus (or a published snapshot)
2. Lower each fixture’s action list into the engine’s IR / SQL
3. Execute against `input` and compare to `expected` (row multiset equality after canonical JSON encoding)
4. Publish a report with engine id, version, profile id, and per-fixture pass/fail

The reference capability manifest (`reference_portable_manifest`) must not mark `REFERENCE_UNSUPPORTED_CLAIMS` as `certified` / `supported`. Use `validate_capability_accuracy` before publishing declarations.

## Running locally

```bash
cargo test --test phase_0_10
cargo test --test portable_relational
dtcs conformance run --profile dtcs:profile/portable-relational/1
dtcs conformance run --profile all
```
