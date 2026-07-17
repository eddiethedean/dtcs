# DTCS Change Proposal: Rich Portable Relational Transformations

> **Historical design note.** This proposal is **Accepted** and **implemented** in tools `0.12.0`. The normative source of truth is [SPEC.md](../SPEC.md) (draft 2.0.0). Prefer [migration-portable-relational.md](user/migration-portable-relational.md) and [migration-0.12.md](user/migration-0.12.md) for living guidance. Sections below retain original problem framing for history; present-tense “missing” language is **not** current status.

Status: **Accepted** (implemented in tools `0.12.0`; SPEC draft updated)  
Proposed target: DTCS next compatible revision plus staged registries  
Related ETLantic milestones: 0.12  
Proposal owner: DTCS publisher and maintainers

**Normative landing:** [SPEC.md](../SPEC.md) Chapter 13 §12.1, Chapter 23 §5.1, Appendix A.3–A.4, A.8.  
**Compatibility table:** [docs/user/migration-portable-relational.md](user/migration-portable-relational.md).  
**Roadmap:** phase 0.12 in [ROADMAP.md](../ROADMAP.md).

## 1. Problem statement

DTCS already defines the correct architecture for implementation-independent
transformations: Transformation Plans are the authoritative semantic IR;
Semantic Actions are the only standardized dataset-modification mechanism;
Expressions and Functions compute values; Engine Capability Models support
compiler selection; and registries provide stable `dtcs:` identifiers.

The published standard library provides a strong initial kernel, including
projection, filtering, aggregation, grouping, joining, sorting, union, and
partitioning plus common string, conversion, numeric, and null functions.

Its registered parameter shapes are not yet rich enough to represent a modern
dataframe expression interface comparable to PySpark. Missing or underspecified
behavior includes:

- expression-bearing projections and predicates
- add/replace-column operations and typed aliases
- arbitrary boolean and arithmetic expression trees
- join types and arbitrary join conditions
- multi-expression aggregation
- precise ordering and null placement
- union-by-name and missing-column policy
- conditional expressions and casts
- windows and frame boundaries
- complete null, missing, invalid, NaN, timestamp, decimal, and error behavior
- granular compiler capabilities
- one canonical interoperable Transformation Plan serialization profile

Without these additions, projects must invent vendor constructs or fall back to
engine-native logic, reducing interoperability and making cross-engine
equivalence difficult to prove.

## 2. Proposed solution

Publish a **DTCS Portable Relational Profile** that standardizes a closed,
typed relational language suitable for dataframe, SQL, and distributed
compilers.

The profile SHALL:

1. Extend the Semantic Action and Function registries with precise, composable
   constructs.
2. Standardize scalar operator identities and expression node shapes.
3. Publish a canonical serialized Transformation Plan profile.
4. Define exact type, value-state, error, ordering, and determinism semantics.
5. Publish machine-readable Engine Capability and Conformance Profiles.
6. Preserve the separation among Transformation Contracts, Transformation
   Plans, optimizers, compilers, Execution Plans, and runtimes.

ETLantic's PySpark-inspired API is one authoring facade for this profile. It is
not part of DTCS and is not the normative syntax.

## 3. Scope

In scope:

- batch relational transformations and typed scalar expressions
- multiple named input and output interfaces
- deterministic and run-stable functions
- joins, grouping, aggregation, union, and explicit ordering
- windows and complex types through optional advanced profiles
- compiler capability matching and objective conformance

Out of scope:

- physical storage, writes, and pipeline orchestration
- backend sessions and resource acquisition
- dataframe actions such as collect, show, take, or count-as-execution
- arbitrary Python or language AST tracing
- raw SQL and engine-native expression objects
- UDF definitions
- medallion architecture concepts

## 4. Affected artifacts

The proposal affects DTCS Chapters 4, 7, 8, 10-15, 17-26, Appendix A, the
public `dtcs` package, and canonical schemas, registries, profiles, and
conformance fixtures.

## 5. Proposed normative profile

Draft profile identity:

```text
dtcs:profile/portable-relational/1
```

Draft canonical plan identity:

```text
dtcs.transform-plan/1
```

Final identifiers follow registry governance and MUST NOT be represented as
published until adopted.

The profile SHALL identify DTCS specification and registry versions, required
types, expressions, actions, functions, operators, value-state behavior,
optional capability groups, and conformance fixture version.

## 6. Transformation Plan serialization

Chapter 13 permits implementation-specific serialization unless a DTCS profile
defines one. This proposal defines a canonical data-only representation:

```json
{
  "profile": "dtcs:profile/portable-relational/1",
  "specificationVersion": "...",
  "registryVersions": {
    "actions": "...",
    "functions": "...",
    "operators": "...",
    "types": "..."
  },
  "transformation": "example:normalize-customers",
  "inputs": {},
  "parameters": {},
  "actions": [],
  "outputs": {},
  "rules": [],
  "lineage": [],
  "requirements": {},
  "extensions": {}
}
```

The profile SHALL define canonical key ordering, meaningful sequence ordering,
stable references, literal/depth/node limits, canonical scalar and value-state
encoding, executable-object exclusion, semantic fingerprint inputs, optional
extension preservation, and rejection of unknown mandatory constructs.

Equivalent plans SHALL produce identical semantic fingerprints under the same
profile and registry versions.

## 7. Expression model additions

Standardize serialized nodes for field and qualified-field references,
parameters, literals, operators, functions, aliases, case expressions, strict
and tolerant casts, sort expressions, aggregates, windows, and staged complex
types.

Every node SHALL declare or permit deterministic inference of logical type,
nullability, value-state behavior, determinism, lineage contributors, and
required registry/capability identifiers. Opaque nodes and host-language
`repr()` serialization are prohibited.

### Proposed operator registry

| Category | Draft identifiers |
|---|---|
| Comparison | `dtcs:eq`, `dtcs:not_eq`, `dtcs:lt`, `dtcs:lte`, `dtcs:gt`, `dtcs:gte`, `dtcs:null_safe_eq` |
| Boolean | `dtcs:and`, `dtcs:or`, `dtcs:not` |
| Arithmetic | `dtcs:add`, `dtcs:subtract`, `dtcs:multiply`, `dtcs:divide`, `dtcs:modulo`, `dtcs:negate` |
| Membership | `dtcs:in`, `dtcs:between` |
| Access | `dtcs:field`, `dtcs:index`, `dtcs:element_at` |

Each entry SHALL define cardinality, accepted types, return type, promotion,
null/missing/invalid and NaN behavior, errors, determinism, and optimizer-safe
properties.

## 8. Semantic Action registry additions

Existing identifiers SHOULD be extended compatibly only where their published
meaning permits it. Otherwise allocate a new identifier or versioned entry.

### Projection and field shaping

Extend `dtcs:project` to accept ordered expressions with explicit output field
names, inferred types/nullability, and field lineage. No unselected field is
implicitly retained.

Propose:

- `dtcs:with_fields` — add or replace ordered field assignments
- `dtcs:rename_fields` — rename without changing value/type
- `dtcs:drop_fields` — remove fields with explicit missing-field policy

### Filtering

Extend `dtcs:filter` with a typed predicate Expression. DTCS SHALL specify
treatment of true, false, null, missing, and invalid predicates. Recommended:
retain true, discard false/null/missing, and fail or explicitly route invalid;
never silently coerce invalid to false.

### Distinct, deduplication, and limit

Propose `dtcs:distinct`, `dtcs:deduplicate`, and `dtcs:limit`.
Deduplication requires keys plus a retained-row policy. Limit without explicit
ordering is nondeterministic with respect to selected rows.

### Joins

Extend `dtcs:join` or allocate a compatible successor supporting inner, left,
right, full, semi, anti, and explicit cross joins; arbitrary predicates;
equi-key shorthand; explicit null-safe equality; relation identities; collision
policy; cardinality assertions; and field-level lineage.

Ordinary equality SHALL NOT match null keys. Missing and invalid remain
distinct and require published behavior.

### Union

Extend `dtcs:union` or add `dtcs:union_by_name` with positional/name alignment,
missing-column policy, type compatibility/widening, output field order, and
duplicate policy. Silent backend field reordering is prohibited.

### Grouping and aggregation

Extend `dtcs:group` and `dtcs:aggregate` for multiple grouping and aggregate
expressions, aliases, filters, distinct inputs, result inference, empty/all-null
inputs, and null/missing/invalid grouping keys. `count(*)`, count-expression,
and distinct count are separate semantics.

### Sorting

Extend `dtcs:sort` to accept ordered expressions with direction, null placement,
missing/invalid behavior, and optional standardized collation. Outputs remain
unordered without sort; equal-key stability is not implied.

### Windows

Add an optional window model with partitions, ordering, row/range frames,
boundaries, ranking/offset/aggregate/value functions, and determinism
requirements.

## 9. Function registry expansion

Existing Appendix A functions retain their identifiers and meaning.

Proposed additions:

| Family | Draft functions |
|---|---|
| Conditional/value state | `dtcs:case_when`, `dtcs:is_invalid`, `dtcs:if_null`, `dtcs:null_if`, `dtcs:try_cast` |
| Strings | `dtcs:concat_ws`, trim-left/right, starts/ends-with, split, regex extract/replace, position, padding |
| Numeric | round, floor, ceil, power, sqrt, least, greatest |
| Date/time | current date/timestamp, conversions, add/subtract/diff, components, truncation |
| Aggregates | count-all, count, count-distinct, sum, average, aggregate min/max, first/last, optional collections |
| Windows | row-number, rank, dense-rank, lag, lead, first/last value |

String definitions specify Unicode unit, indexing origin, regex profile, and
value states. Numeric definitions specify decimal formulas, rounding, overflow,
zero division, NaN, infinities, and signed zero. Date/time definitions specify
timezone, parsing, daylight-saving behavior, precision, and run stability.

Scalar and aggregate functions MUST NOT share an ambiguous identity when their
empty-input or parameter semantics differ.

## 10. Type-system clarifications

Publish canonical representations and promotion rules for booleans, integer
and float widths, decimals, strings/binary, dates, timezone-explicit timestamps,
durations, arrays, maps, structs, nullability, and analysis-only unknown types.

Clarify least-widening promotion, decimal result formulas, strict/tolerant
casts, overflow, narrowing, timezone changes, structural compatibility, and the
interaction of logical types with null, missing, and invalid.

## 11. Determinism and ordering

Every action/operator/function SHALL be deterministic, execution-context
stable, or nondeterministic. This contributes to caching, retry, idempotency,
and optimization requirements.

Outputs are unordered without sort. Limit, deduplication, first/last,
collections, and windows either declare sufficient ordering or are marked
nondeterministic.

## 12. Lineage additions

Every derived field identifies contributing input fields. Filters record row
flow; joins/unions retain source-interface identity; aggregation records group
and value contributors; information loss remains explicit; and optimized/fused
plans preserve source action/expression mappings.

Existing Appendix A lineage flow tokens remain authoritative unless reviewed
and extended.

## 13. Engine Capability Model additions

Publish machine-readable capability profiles keyed by exact registry entries:

```json
{
  "profile": "dtcs:profile/portable-relational/1",
  "implementationClass": "Compiler",
  "engine": "polars",
  "supportedPlanProfiles": ["dtcs:profile/portable-relational/1"],
  "actions": {},
  "operators": {},
  "functions": {},
  "types": {},
  "semanticModes": {},
  "limits": {}
}
```

Declarations include registry versions, signatures, semantic modes,
execution-significant lazy/eager behavior, limits, and known unsupported
optional features. False claims invalidate conformance.

## 14. Diagnostics additions

Allocate standardized diagnostics for unknown registry entries, unsupported
capabilities, field resolution, expression typing, value-state coercion,
output mismatch, ordering/determinism, casts, version incompatibility, bounded
limits, and compiler semantic-preservation failure.

Diagnostics SHOULD include expression path, registry/version, types,
capability requirement, and related contract/output locations. ETLantic may map
these into `PMXFORMxxx` while preserving the originating DTCS identity.

## 15. Security additions

The canonical profile SHALL prohibit executable serialization, arbitrary
language objects, resolved secrets, live backend handles, and raw SQL. It SHALL
bound bytes, depth, node count, literal/collection size, references, and
diagnostics.

Runtime parameter values remain outside portable plans. SQL compilers use bound
parameters and safe identifiers.

## 16. Conformance profiles and tests

Proposed staged profiles:

| Profile | Required family |
|---|---|
| `portable-relational-kernel/1` | project, filter, field shaping, scalar core |
| `portable-relational/1` | joins, unions, grouping, aggregation, ordering |
| `portable-window/1` | windows and analytics |
| `portable-complex-types/1` | arrays, maps, structs |

Fixtures cover null/missing/invalid, NaN/infinities/signed zero, numeric and
decimal edges, Unicode/regex, timezone boundaries, empty/all-null inputs,
duplicate keys/collisions, joins, ordering, aggregates/windows, canonical
serialization, hostile input, and capability/diagnostic accuracy.

At least two independent compilers SHOULD pass a semantic family before its
registry status advances from Experimental to Standard.

## 17. Compatibility impact

New identifiers and optional profiles can be additive. Changing parameters or
observable behavior of published Appendix A identifiers may be breaking.
Review SHALL decide whether the original definition permits extension, a
versioned entry suffices, or a new identifier/profile major is required.

Current project/filter/aggregate/group/join/sort/union parameters are narrower
than this proposal. Implementations MUST NOT reinterpret existing documents.

The meaning of existing `dtcs:coalesce` requires clarification before an
ETLantic facade assumes SQL-like first-non-null behavior because Appendix A
currently declares its null behavior token as `propagate`.

## 18. Migration considerations

- Contracts without portable plans remain valid.
- Existing Appendix A forms retain published meaning.
- Legacy forms upgrade only when equivalence is provable.
- Vendor-to-standard mappings require reviewed migration tables.
- Compilers declare old/new profiles independently.
- Plans record specification, registry, profile, and extension versions.
- Breaking profiles require deterministic migration and semantic diffs.

## 19. Reference package requirements

Proposed public package responsibilities:

```python
from dtcs.plan import TransformationPlan
from dtcs.expressions import Expression
from dtcs.actions import SemanticAction
from dtcs.capabilities import EngineCapabilityModel
from dtcs.registries import RegistrySet
from dtcs.validation import validate_transformation_plan
```

Exact imports require package review. Models are immutable/effectively
immutable, safely parsed, deterministic, registry-aware, fingerprintable,
capability-analyzable, diagnostic-producing, and backend/ETLantic independent.

## 20. ETLantic mapping

| ETLantic facade | DTCS construct |
|---|---|
| `frame.select(...)` | rich `dtcs:project` |
| `frame.filter(...)` | rich `dtcs:filter` |
| `frame.withColumn(...)` | proposed `dtcs:with_fields` |
| `frame.drop(...)` | proposed `dtcs:drop_fields` |
| `frame.join(...)` | rich `dtcs:join` |
| `frame.groupBy(...).agg(...)` | `dtcs:group` + `dtcs:aggregate` |
| `frame.unionByName(...)` | proposed union-by-name form |
| `frame.orderBy(...)` | rich `dtcs:sort` |
| `F.lower(...)` | `dtcs:lower` |
| `F.concat(...)` | `dtcs:concat` |
| `F.when(...).otherwise(...)` | proposed `dtcs:case_when` |
| Column operators | registered DTCS operators |

ETLantic rejects or labels experimental any facade call without a sufficient
published DTCS mapping.

## 21. Delivery sequence

### DTCS-R1: kernel profile

Canonical serialization, expression/operator registries, rich
project/filter/field-shaping, value-state/type semantics, and kernel
conformance. Unblocks ETLantic 0.11 and Polars 0.12.

### DTCS-R2: relational profile

Joins, unions, grouping, aggregation, deduplication, ordering, aggregate
functions, and lineage. Unblocks ETLantic/PySpark parity in 0.13.

### DTCS-R3: conformance publication

Machine-readable conformance manifests, differential fixtures, reference
vectors, and diagnostic entries. Unblocks ETLantic 0.14.

### DTCS-R4: advanced profile

SQL lowering requirements, windows, date/time, and selected complex types.
Unblocks ETLantic 0.15+.

## 22. Acceptance criteria

1. Every standard facade operation maps to registered DTCS semantics.
2. Existing identifiers have explicit compatibility decisions.
3. Canonical plans round-trip deterministically.
4. Value states, numeric/time behavior, and ordering are unambiguous.
5. Capabilities select objective conformance fixtures.
6. At least Polars and PySpark pass the kernel profile.
7. Standard diagnostics identify invalid/unsupported plans.
8. Security budgets and executable-object rejection are tested.
9. Incompatible changes have migration guidance.
10. Specification, registry, package, profile, and conformance versions and
    publication states remain independent and explicit.

## 23. Open review questions

1. Version existing action identities or allocate richer successors?
2. What is the canonical expression grammar?
3. Are operators a registry or Function category?
4. What are filter semantics for invalid predicates?
5. Does missing form a grouping key distinct from null?
6. How do invalid values flow through projection and aggregation?
7. What decimal formulas and rounding mode are normative?
8. Which regex/Unicode profile is portable?
9. What timestamp model is mandatory in the kernel?
10. Is `dtcs:coalesce`'s `propagate` token intentional?
11. Which optimizations require proof metadata?
12. What is the minimum “portable relational” compiler profile?

## 24. Governance and publication

Following DTCS Chapter 26, publish the problem, solution, compatibility,
migration, and affected artifacts; conduct technical review; preserve
identifier stability; progress Draft → Candidate → Stable; and publish
immutable, independently versioned specification, registry, profile, package,
and conformance artifacts.

The shared ETLantic/DTCS publisher can coordinate releases and reference
implementations, while DTCS governance remains the authority for adoption.

