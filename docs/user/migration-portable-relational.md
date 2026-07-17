# Portable Relational Profile — Compatibility Table (0.12)

> **Historical tables for tools 0.12 / Spec 2.0 profile `/1`.** For current work prefer tools **0.13** / Spec **3.0**, plan identity `dtcs.transform-plan/2`, and kernel/relational `/2` (Candidate). See [migration-0.13.md](migration-0.13.md).

Upgrade guidance for tools **0.12+** implementing the Portable Relational Profile
(`dtcs:profile/portable-relational-kernel/1` through advanced profiles). Spec for these tables is **`2.0.0`**.
Document `dtcsVersion` for that era SHOULD be `"2.0.0"`; `"1.0.0"` remains accepted for compatibility.

## Policy (option B)

Existing `dtcs:` action and function **identifiers are retained**. Entry **versions**
are bumped and parameter schemas are **widened**. Legacy parameter shapes remain a
**valid subset** of the new schema. Portable plans pin `registryVersions` so semantics
are unambiguous.

Documents that use only legacy parameters keep legacy observable behavior where the
subset is compatible. Documents that opt into new parameters get the new semantics.
Behavior that cannot remain subset-compatible (noted below) applies under the new
registry entry version pin.

As of **0.12.0**, previously declared-but-ignored parameters are **enforced** in the
reference runtime: join `collisionPolicy` / `predicate`, union `duplicatePolicy`,
sort/groupBy expression keys, and ternary `between`.

## Action compatibility

| Identifier | Entry version | Legacy params (still valid) | Widened / new params | Notes |
|---|---|---|---|---|
| `dtcs:project` | `2.0.0` | `fields: string[]` | `fields` may be expression objects `{expr, as}` or mixed; ordered; no implicit retain | Legacy name-list form unchanged |
| `dtcs:filter` / `dtcs:select` | `2.0.0` | `field`, optional `equals` | `predicate` (expression); value-state policy | Predicate form: keep `true`; drop `false`/`null`/`missing`; fail on `invalid` |
| `dtcs:with_fields` | `1.0.0` (new) | — | `assignments: [{as, expr}]` | Add/replace fields |
| `dtcs:rename_fields` | `1.0.0` (new) | — | `mapping: {old: new}` | Rename only |
| `dtcs:drop_fields` | `1.0.0` (new) | — | `fields`, optional `missingPolicy` | `error` (default) or `ignore` |
| `dtcs:join` | `2.0.0` | `right`, `leftKey`, optional `rightKey` | `type`, `on`/`predicate`, `nullSafe`, `collisionPolicy` | Under v2 pin, ordinary equality does **not** match null keys |
| `dtcs:union` | `2.0.0` | `other` | `mode` (`positional`\|`byName`), `missingColumnPolicy`, `duplicatePolicy` | Legacy = positional append |
| `dtcs:aggregate` / `dtcs:group` | `2.0.0` | `groupBy`, `valueField`, `op` | `groupBy`/`aggregates` expression lists, aliases, filters | Legacy single-op form unchanged |
| `dtcs:sort` | `2.0.0` | `field`, optional `descending` | `keys: [{expr, descending, nulls}]` | Legacy single-field form unchanged |
| `dtcs:distinct` | `1.0.0` (new) | — | optional `fields` | Full-row distinct when omitted |
| `dtcs:deduplicate` | `1.0.0` (new) | — | `keys`, `retain` | Requires retain policy |
| `dtcs:limit` | `1.0.0` (new) | — | `count`, optional `offset` | Nondeterministic without prior sort |
| `dtcs:partition` | `1.0.0` | `field` | unchanged in kernel | — |

Field transforms (`lowercase`, `uppercase`, …) remain at entry version `1.0.0`.

## Function compatibility

| Identifier | Entry version | Change |
|---|---|---|
| `dtcs:coalesce` | `2.0.0` | Null-behavior token clarified to `defined` (first present non-null/non-missing value; skips invalid). Prior `propagate` token was incorrect for documented first-present semantics. |
| Kernel additions | `1.0.0` | `dtcs:case_when`, `dtcs:if_null`, `dtcs:null_if`, `dtcs:try_cast`, `dtcs:is_invalid` |
| Relational / advanced | `1.0.0` | Aggregate, string, numeric, date/time, window families per profile |

## Operator registry

New registry document `dtcs:stdlib.operators` version `1.0.0` with identifiers such as
`dtcs:eq`, `dtcs:add`, `dtcs:and`, … Capability matching accepts both short names
(`eq`) and registry ids (`dtcs:eq`).

## Plan serialization

Internal COM plan IR (`nodes` / `dependencies`) remains. The portable envelope
`dtcs.transform-plan/1` adds `profile`, `specificationVersion`, `registryVersions`,
and canonical fingerprinting. Export via the reference tooling; existing plan fixtures
remain valid for the internal IR.

## Profiles

| Profile | Required family |
|---|---|
| `dtcs:profile/portable-relational-kernel/1` | project, filter, field shaping, scalar core, operators |
| `dtcs:profile/portable-relational/1` | joins, unions, grouping, aggregation, ordering, distinct/limit |
| `dtcs:profile/portable-window/1` | windows and analytics |
| `dtcs:profile/portable-complex-types/1` | arrays, maps, structs |

## Verify

```bash
dtcs version
dtcs registry resolve dtcs:with_fields --json
dtcs registry resolve dtcs:eq --json
dtcs validate examples/minimal.dtcs.yaml
```
