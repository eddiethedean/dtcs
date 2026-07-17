# DTCS 3.0 Rich Portable Analytics Proposal

- Status: Draft proposal for DTCS 3.0.0
- Proposed specification version: `3.0.0`
- Baseline: DTCS 2.0.0 and `dtcs` 0.12.0
- Proposed plan protocol: `dtcs.transform-plan/2`
- Owner: DTCS publisher and maintainers
- Primary consumer: ETLantic portable transformations

!!! info "Proposal—not published DTCS behavior"
    This document proposes normative DTCS 3.0 changes. It does not alter DTCS
    2.0 or make the identifiers below available in the current `dtcs` package.
    Implementations must not advertise these capabilities until the
    specification, registries, schemas, package, and conformance fixtures are
    published.

## 1. Executive summary

DTCS 2.0 establishes a capable portable relational foundation: canonical
Transformation Plans, structured expressions, joins, unions, aggregation,
ordering, windows, complex logical types, value states, capability profiles,
and conformance rules.

DTCS 3.0 should complete the semantic vocabulary needed for a rich modern
dataframe interface without adopting PySpark, SQL, or any backend as the
standard. The proposed release adds:

- complete string and regular-expression expressions
- strict conversion and parsing semantics
- statistical and analytic aggregates
- constructors, transforms, and expansion for complex values
- set, reshape, and sampling dataset actions
- IANA timezone and calendar-aware temporal semantics
- controlled random, UUID, and run-context expressions
- mature window and complex-type profiles
- explicit schema evolution and nested-field behavior
- stronger determinism, resource, capability, and conformance metadata

The release should preserve a small mandatory kernel. Advanced semantic
families remain separately claimable profiles so engines fail closed instead
of approximating behavior.

## 2. Problem statement

The DTCS 2.0 catalog cannot express several common, implementation-independent
transformations that users expect from a PySpark-like dataframe API.

### 2.1 Expression gaps

DTCS 2.0 does not publish general expression functions for:

- trimming and whitespace normalization
- split, regex match, regex extraction, and regex replacement
- string padding, repetition, reversal, translation, and case-insensitive
  comparison
- strict general casts or format-aware parsing
- standard deviation, variance, covariance, and correlation
- list, map, object, and tuple construction
- collection sizing, membership, sorting, slicing, key/value access, and
  higher-order transforms
- random values, seeded randomness, or UUID generation
- IANA timezone databases and daylight-saving transitions

Field Semantic Actions such as `dtcs:trim` cannot substitute for composable
expression functions. Rule-level `dtcs:regex_match` cannot substitute for a
value-producing regex Function.

### 2.2 Relational gaps

DTCS 2.0 does not standardize:

- `intersect` and `except` set operations
- pivot and unpivot
- explode/unnest of complex values
- sampling and deterministic splitting
- schema-aligned union with explicit recursive coercion
- nested-field assignment, rename, and drop
- generator expressions that produce zero or more rows

### 2.3 Semantic gaps

Several cross-engine differences require normative decisions:

- regex grammar and Unicode behavior
- strict cast failures, tolerant parsing, and overflow
- sample repeatability and seeded pseudorandom algorithms
- UUID version and byte/string representation
- collection index origin and negative indexes
- map duplicate-key policy and ordering
- object field order and duplicate names
- explode behavior for null, missing, invalid, and empty collections
- statistical algorithms, degrees of freedom, NaN, and empty inputs
- IANA timezone database version and ambiguous/nonexistent local times
- pivot output naming, category discovery, and boundedness
- whether operators may evaluate both branches of conditional expressions

Without standard answers, compilers can accept identical plans while producing
observably different results.

## 3. Goals

DTCS 3.0 SHALL:

1. Preserve implementation independence and data-only plans.
2. Make every new semantic family independently discoverable and testable.
3. Define value-state, type, error, ordering, and determinism behavior for
   every new entry.
4. Support safe lowering to dataframe, distributed, and SQL engines.
5. Prevent backend-native fallback from masquerading as portability.
6. Keep planning bounded and possible without reading source data.
7. Provide deterministic migration from `dtcs.transform-plan/1`.

## 4. Non-goals

DTCS 3.0 SHALL NOT standardize:

- a Python, PySpark, or SQL user interface
- arbitrary SQL text or backend expression strings
- Python or language AST tracing
- user-defined executable code or serialized closures
- dataframe actions such as `collect`, `show`, `take`, or writes
- physical partition counts, cache placement, or execution hints as semantic
  results
- orchestration, storage, medallion layers, or resource acquisition
- unbounded schema discovery from runtime data during planning

## 5. Versioning strategy

The proposal uses `dtcs.transform-plan/2` because it adds expression forms,
generator actions, semantic modes, and canonicalization rules that a version 1
reader cannot safely ignore.

DTCS 3.0 SHALL continue to read valid `dtcs.transform-plan/1` documents. A
version 1 plan migrates to version 2 without semantic change. Published 2.0
identifiers retain their meanings and entry versions unless the 3.0 review
finds an irreconcilable ambiguity; such a case receives a new identifier or
entry major rather than silent reinterpretation.

## 6. Proposed expression model additions

DTCS 3.0 should retain the five 2.0 structured node kinds (`literal`,
`fieldRef`, `unary`, `binary`, `call`) and add one bounded node:

### 6.1 Lambda expression

```json
{
  "kind": "lambda",
  "parameters": ["element", "index"],
  "body": {
    "kind": "binary",
    "operator": "dtcs:multiply",
    "left": {"kind": "fieldRef", "scope": "lambda", "name": "element"},
    "right": {"kind": "literal", "value": 2}
  }
}
```

A lambda SHALL:

- be data-only and contain no executable object
- declare one or more scoped parameters
- reference only its parameters, literals, plan parameters, and explicitly
  permitted outer fields
- use registered DTCS expressions and Functions only
- obey plan depth, node, byte, and evaluation budgets
- be allowed only by Functions whose registry entries declare lambda support

Evaluation order, short-circuit behavior, variable shadowing, and capture rules
SHALL be normative.

## 7. String and regex function profile

Proposed profile: `dtcs:profile/portable-string-advanced/1`

### 7.1 String functions

| Proposed identifier | Semantics to standardize |
|---|---|
| `dtcs:trim` | general expression trim with an optional character set |
| `dtcs:ltrim`, `dtcs:rtrim` | one-sided trim |
| `dtcs:normalize_whitespace` | Unicode whitespace collapse and edge trim |
| `dtcs:split` | bounded string split with literal or regex delimiter mode |
| `dtcs:join_strings` | join a list of strings with explicit null policy |
| `dtcs:pad_left`, `dtcs:pad_right` | code-point length and fill behavior |
| `dtcs:repeat` | bounded repetition |
| `dtcs:reverse` | Unicode code-point reversal |
| `dtcs:translate` | deterministic code-point translation |
| `dtcs:position` | substring position with explicit index origin |
| `dtcs:lower_unicode`, `dtcs:upper_unicode` | versioned Unicode case mapping |
| `dtcs:casefold` | locale-independent Unicode case folding |

The existing field actions retain their identities. General expression
functions may share a lexical suffix only when registry kind disambiguation is
unambiguous in serialized plans and capability declarations.

### 7.2 Regex functions

| Proposed identifier | Result |
|---|---|
| `dtcs:regex_matches` | boolean full match |
| `dtcs:regex_contains` | boolean search |
| `dtcs:regex_extract` | string or list of capture values |
| `dtcs:regex_extract_all` | list of matches |
| `dtcs:regex_replace` | replaced string |
| `dtcs:regex_split` | list of strings |

DTCS SHALL publish a portable regex grammar rather than referring to a host
engine. The grammar SHOULD be a safe, regular-language subset and SHALL define
Unicode version, flags, capture numbering, zero-length matches, replacement
references, invalid-pattern diagnostics, and resource limits. Backtracking or
engine-specific constructs outside the profile SHALL fail capability analysis.

## 8. Conversion and parsing profile

Proposed profile: `dtcs:profile/portable-conversion/1`

| Proposed identifier | Purpose |
|---|---|
| `dtcs:cast` | strict conversion; invalid input produces a declared error |
| `dtcs:try_cast` | tolerant conversion with explicit invalid-result policy |
| `dtcs:parse_date` | format-aware date parsing |
| `dtcs:parse_time` | format-aware time parsing |
| `dtcs:parse_datetime` | format and timezone-aware datetime parsing |
| `dtcs:format_date` | deterministic date formatting |
| `dtcs:format_datetime` | deterministic datetime formatting |
| `dtcs:parse_decimal` | locale-independent decimal parsing |
| `dtcs:parse_boolean` | registry-defined accepted tokens |

Every conversion SHALL declare overflow, rounding, precision, invalid, null,
missing, and NaN behavior. Format strings SHALL use a DTCS grammar rather than
backend-native tokens.

## 9. Statistical aggregate profile

Proposed profile: `dtcs:profile/portable-statistics/1`

| Proposed identifier | Required modes |
|---|---|
| `dtcs:variance` | population and sample |
| `dtcs:stddev` | population and sample |
| `dtcs:covariance` | population and sample |
| `dtcs:correlation` | Pearson initially |
| `dtcs:median` | exact |
| `dtcs:quantile` | exact and explicitly named approximate mode |
| `dtcs:first`, `dtcs:last` | explicit null and ordering policy |
| `dtcs:collect_list` | explicit ordering and size limit |
| `dtcs:collect_set` | equality, ordering, and size limit |

The registry SHALL define degrees of freedom, numeric promotion, stable
algorithms, decimal handling, empty/all-null inputs, NaN/infinity behavior,
overflow, result nullability, and whether order is significant. Approximate
algorithms SHALL declare error bounds, determinism, merge behavior, and
algorithm identity.

## 10. Complex-value profile

Proposed profile: `dtcs:profile/portable-complex-values/1`. It supersedes or
graduates `dtcs:profile/portable-complex-types/1` without changing the 2.0 type
aliases.

### 10.1 Constructors and accessors

- `dtcs:list`, `dtcs:map`, `dtcs:object`, `dtcs:tuple`
- `dtcs:size`, `dtcs:list_contains`, `dtcs:list_position`
- `dtcs:list_slice`, `dtcs:list_concat`, `dtcs:list_sort`
- `dtcs:map_keys`, `dtcs:map_values`, `dtcs:map_entries`
- `dtcs:map_from_entries`, `dtcs:map_concat`
- `dtcs:object_names`, `dtcs:object_values`
- existing `dtcs:field`, `dtcs:index`, and `dtcs:element_at`

### 10.2 Higher-order functions

- `dtcs:transform`
- `dtcs:filter_values`
- `dtcs:exists`
- `dtcs:forall`
- `dtcs:reduce`
- `dtcs:zip_with`
- `dtcs:map_filter`
- `dtcs:transform_keys`
- `dtcs:transform_values`

The profile SHALL define collection order, index origin, negative indexes,
out-of-bounds access, null elements, duplicate map keys, object field order,
type unification, lambda evaluation, and resource budgets.

## 11. Generator and reshape actions

Proposed profile: `dtcs:profile/portable-reshape/1`

### 11.1 Explode and unnest

Proposed action `dtcs:explode`:

```json
{
  "action": "dtcs:explode",
  "parameters": {
    "expr": {"kind": "fieldRef", "name": "items"},
    "as": ["item", "position"],
    "outer": false,
    "includePosition": true
  }
}
```

The action SHALL define behavior for lists, maps, null, missing, invalid, and
empty values; output ordering; positional origin; collision policy; and maximum
expansion budgets.

### 11.2 Pivot and unpivot

Proposed actions:

- `dtcs:pivot`
- `dtcs:unpivot`

Pivot categories MUST be declared or bounded by a planning artifact. A
compiler MUST NOT read source rows during ordinary validation to discover an
unbounded output schema. Category ordering, output naming, duplicate cells,
aggregation, null categories, and collision behavior SHALL be explicit.

## 12. Relational set and sampling actions

Proposed profile: `dtcs:profile/portable-relational-extended/1`

| Proposed action | Required modes |
|---|---|
| `dtcs:intersect` | distinct/all, positional/by-name |
| `dtcs:except` | distinct/all, positional/by-name |
| `dtcs:sample` | fraction/count, with/without replacement, seed |
| `dtcs:random_split` | weights, seed, assignment algorithm |
| `dtcs:repartition` | logical distribution requirement only |
| `dtcs:coalesce_partitions` | optional physical hint, never row semantics |

Seeded sampling SHALL specify a portable pseudorandom algorithm, seed width,
row identity requirements, and partition-independent assignment. If stable row
identity is unavailable, the plan SHALL declare the result nondeterministic or
fail a determinism requirement.

## 13. Temporal profile

Proposed profile: `dtcs:profile/portable-temporal-iana/1`

The profile adds:

- IANA timezone identifiers
- timezone database version declaration
- ambiguous and nonexistent local-time policies
- `dtcs:to_timezone`, `dtcs:from_utc`, and `dtcs:to_utc`
- month-end and leap-day arithmetic policy
- week and ISO-week extraction
- configurable week start only through an explicit semantic mode
- sub-minute units down to a declared precision
- interval construction and comparison

Plans SHALL record the required timezone-data version or compatible range.
Compilers unable to provide it SHALL reject the profile rather than substitute
the host environment silently.

## 14. Controlled nondeterminism profile

Proposed profile: `dtcs:profile/portable-nondeterministic/1`

| Proposed identifier | Required contract |
|---|---|
| `dtcs:random` | algorithm, optional seed, numeric domain |
| `dtcs:random_normal` | algorithm, seed, mean, standard deviation |
| `dtcs:uuid` | UUID version and representation |
| `dtcs:run_id` | stable within one logical run |
| `dtcs:run_timestamp` | stable within one logical run |

Each call SHALL be classified as deterministic, run-stable, seeded-stable, or
nondeterministic. The classification SHALL affect fingerprints, caching,
retries, incremental execution, and idempotency analysis. Optimizers MUST NOT
duplicate, reorder, or eliminate calls when that changes observable results.

## 15. Window-profile graduation and expansion

DTCS 3.0 should graduate `dtcs:profile/portable-window/1` only after two
independent compilers pass its complete fixture family. Proposed additions:

- `dtcs:ntile`
- `dtcs:percent_rank`
- `dtcs:cume_dist`
- `dtcs:nth_value`
- explicit `ignoreNulls`
- named windows and safe inheritance
- exclusion modes where portable across two compilers
- precise peer, tie, NaN, and range-frame semantics

Unsupported exclusion or backend-specific frame modes remain separate
capabilities rather than weakening the base window profile.

## 16. Nested schema evolution

DTCS 3.0 SHALL define paths structurally, not as ambiguous dotted strings.
Proposed actions:

- `dtcs:with_nested_fields`
- `dtcs:rename_nested_fields`
- `dtcs:drop_nested_fields`

Each action SHALL define missing-parent behavior, automatic parent creation,
list traversal, field order, collision policy, nullability, and contract
compatibility. Schema changes SHALL remain distinct from physical storage
schema mutation.

## 17. Error and value-state model

Every new registry entry SHALL specify behavior for:

- present values
- null
- missing
- invalid
- NaN and positive/negative infinity where numeric
- type mismatch
- overflow and underflow
- resource-budget exhaustion

DTCS 3.0 should standardize expression-level error modes:

- `fail`
- `invalid`
- `null`
- `route`, only when the containing action declares an invalid-output path

An implementation MUST NOT select an error mode implicitly from backend
defaults.

## 18. Capability model changes

Engine Capability Models SHALL declare:

- accepted plan protocol versions
- complete and partial profile claims
- action, Function, Operator, and type entry versions
- supported semantic modes
- regex, Unicode, timezone-data, random-algorithm, and format-grammar versions
- maximum plan, expression, regex, collection, expansion, pivot, and aggregate
  state budgets
- ordering and deterministic-execution guarantees
- compile-time and runtime capability conditions

Capability analysis SHALL return a finding for the narrowest unsupported
requirement. A compiler that supports `regex_replace` without the plan's flag
or Unicode version cannot claim the complete string profile.

## 19. Canonicalization and fingerprinting

`dtcs.transform-plan/2` SHALL define canonical serialization for all added
nodes and modes. Fingerprints SHALL include every semantic input, including:

- registry entry versions
- profile versions
- lambda parameter order and scope
- regex and format grammars
- timezone-data requirements
- random algorithms and seeds
- error, ordering, collision, and missing-value policies
- declared resource limits that affect observable behavior

Display names, comments, source locations, and non-semantic explain metadata
SHALL remain outside semantic fingerprints.

## 20. Security requirements

DTCS 3.0 SHALL retain the 2.0 prohibition on executable objects and raw SQL.
It SHALL additionally require:

- regex complexity and input-length budgets
- lambda depth and evaluation budgets
- collection construction and expansion limits
- pivot category and output-column limits
- aggregate-state and collected-value limits
- random seed validation
- timezone and locale data provenance
- bounded diagnostics without echoing sensitive input values

Plans, diagnostics, conformance artifacts, and fingerprints SHALL NOT contain
resolved secrets or source rows.

## 21. Conformance program

Each proposed profile SHALL ship with machine-readable positive and negative
fixtures. Required families include:

1. canonical parse/serialize/fingerprint vectors
2. type inference and invalid-plan diagnostics
3. null, missing, invalid, NaN, infinity, and overflow matrices
4. Unicode and regex corpus
5. conversion and temporal boundary corpus
6. statistical numeric-accuracy corpus
7. complex-value and lambda corpus
8. explode, pivot, set-operation, and sampling corpus
9. deterministic/retry/cache classification corpus
10. hostile bounded-input corpus
11. cross-engine differential results

A semantic family remains Experimental until two independent compilers pass
all mandatory fixtures. Reference package self-tests do not count as two
independent compilers.

## 22. Proposed profile catalog

| Profile | Initial status |
|---|---|
| `dtcs:profile/portable-relational-kernel/2` | Standard after migration fixtures pass |
| `dtcs:profile/portable-relational/2` | Standard after migration fixtures pass |
| `dtcs:profile/portable-string-advanced/1` | Experimental |
| `dtcs:profile/portable-conversion/1` | Experimental |
| `dtcs:profile/portable-statistics/1` | Experimental |
| `dtcs:profile/portable-complex-values/1` | Experimental |
| `dtcs:profile/portable-reshape/1` | Experimental |
| `dtcs:profile/portable-relational-extended/1` | Experimental |
| `dtcs:profile/portable-temporal-iana/1` | Experimental |
| `dtcs:profile/portable-nondeterministic/1` | Experimental |
| `dtcs:profile/portable-window/2` | Candidate pending two compilers |

Profile composition SHALL be explicit. Claiming an advanced profile does not
implicitly claim unrelated families.

## 23. Package requirements

The matching `dtcs` package release SHALL provide public, documented APIs for:

- immutable/effectively immutable plan v2 models
- all registry entries and profile manifests
- canonical serialization and fingerprinting
- plan v1-to-v2 migration
- type and capability analysis
- standardized diagnostics
- conformance fixture discovery and execution
- bounded safe loading

ETLantic and plugins SHALL use these public models rather than duplicate them.

## 24. Migration considerations

- DTCS 2.0 contracts and plan v1 artifacts remain readable.
- Plan v1 actions and functions retain their published meaning.
- `portable-complex-types/1` maps to the compatible subset of
  `portable-complex-values/1`; constructors and lambdas are new capabilities.
- `portable-window/1` maps to the compatible subset of
  `portable-window/2`.
- Field actions such as `dtcs:trim` remain valid; expression use requires the
  new Function registry entry.
- Native or vendor regex, temporal, statistical, and collection extensions
  migrate only when equivalence is proven.
- A migration report SHALL list changed protocol/profile identifiers and any
  capability that remains vendor-specific.

## 25. Compatibility impact

The specification major and plan protocol major acknowledge that older readers
cannot safely process lambdas, generator actions, or new canonical modes.
Within DTCS 3.x, adding optional registry entries or profiles is additive.
Changing a published entry's observable semantics requires a new entry version
or DTCS major release.

## 26. Affected artifacts

Adoption requires coordinated updates to:

- DTCS specification Chapters 4, 7, 8, 10–15, 17–26, and Appendix A
- Transformation Plan JSON Schema and canonical examples
- Semantic Action, Function, Operator, type, profile, and diagnostic registries
- Engine Capability Model schema
- conformance manifests, vectors, and hostile-input fixtures
- `dtcs` Python package and migration utilities
- compatibility and security guidance
- ETLantic portable mappings and plugin compiler suites

## 27. Delivery sequence

### 3.0-R1: protocol and semantic foundations

Plan v2, lambda scoping, error modes, canonicalization, migration, capability
schema, and security budgets.

### 3.0-R2: deterministic expression families

Advanced strings/regex, conversions, statistics, and complex values.

### 3.0-R3: relational expansion

Explode, reshape, set operations, nested schema actions, and deterministic
sampling.

### 3.0-R4: temporal and controlled nondeterminism

IANA timezones, run context, random algorithms, and UUIDs.

### 3.0-R5: conformance and graduation

Two independent compiler results, differential corpus publication, and profile
status decisions.

## 28. Acceptance criteria

The proposal is complete only when:

1. Every identifier has normative argument, type, state, error, determinism,
   ordering, and lineage semantics.
2. Plan v1 migrates deterministically to plan v2.
3. Canonical plan v2 round-trips and fingerprints consistently.
4. Every profile has machine-readable capability requirements.
5. Every profile has positive, negative, boundary, and hostile fixtures.
6. At least two independent compilers pass before a new family becomes
   Standard.
7. SQL compilation requires bound parameters and safe identifiers.
8. No plan or fixture requires executable code, source-data inspection during
   planning, or resolved secrets.
9. Unsupported backend behavior fails capability analysis without
   approximation.
10. Migration and compatibility documentation ships with the specification and
    package release.

## 29. Review questions

1. Can lambda expressions be added safely without a plan-protocol major, or is
   `dtcs.transform-plan/2` required as proposed?
2. Should general expression `dtcs:trim` share the field-action identifier or
   receive a function-specific identifier?
3. Which regex subset has two practical compiler implementations and bounded
   worst-case behavior?
4. Which Unicode versioning policy balances reproducibility and deployability?
5. Should statistical aggregates mandate one algorithm or only numeric error
   bounds?
6. What stable row identity is required for partition-independent sampling?
7. How should pivot categories be supplied without runtime schema discovery?
8. Which timezone-data compatibility range is acceptable for conformance?
9. Should exact quantiles be mandatory before an approximate profile is
   claimable?
10. Which window v2 additions have two independent implementations?
11. Are physical repartition hints appropriate in DTCS or better confined to
    Execution Plans?
12. Which 2.0 experimental profiles have sufficient evidence to graduate?

## 30. Governance and publication

This proposal SHALL follow DTCS Chapter 26. Publication requires technical
review, compatibility classification, immutable specification text, governed
registry releases, public schemas, package release, conformance artifacts, and
migration guidance. Shared ETLantic/DTCS publishing authority accelerates the
work but does not waive independent versioning or conformance requirements.
