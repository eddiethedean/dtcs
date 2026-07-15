# Compatibility and Evolution

Phase 0.3 adds read-only analysis for comparing contracts and tracking safe evolution.

## When to use which command

| Task | Command |
|------|---------|
| Can consumers of the old contract adopt the new one? | `dtcs compat <source> <target>` |
| What changed between two revisions of the same contract? | `dtcs evolve <older> <newer>` |
| Which outputs depend on an input? | `dtcs lineage <path> --impact INPUT` |

Both `compat` and `evolve` require **valid** contracts. Run `dtcs validate` first.

## Contract-level compatibility declaration

Contracts MAY declare a COM `compatibility` object (SPEC Chapter 3 §9) separate from analysis results:

```yaml
compatibility:
  policy: "dtcs:default"
  backward: true
  forward: false
  notes: "Additive optional fields only"
```

Evolution analysis also recognizes metadata deprecation fields: `deprecated`, `replacement`, and `anticipatedRemoval` (SPEC Chapter 12 §10). Lineage diffs include `operation` and `flow` when those mapping fields change.

## Compatibility levels

The classifier assigns one of five levels (SPEC Chapter 11):

| Level | Meaning |
|-------|---------|
| **Identical** | No differences in the compared scope |
| **Backward compatible** | Target is a strict superset; existing consumers of source can adopt target |
| **Forward compatible** | Source is a strict superset of target |
| **Conditionally compatible** | Compatible only under documented conditions (e.g. optional additions with warnings) |
| **Incompatible** | Breaking change — consumers may fail |

## Example: backward compatible

```bash
dtcs compat examples/analysis/backward_old.yaml examples/analysis/backward_new.yaml
```

The `backward_new` contract adds an optional input interface (`extra`) and a new nullable output field (`note`). Existing consumers of `backward_old` can adopt the new version without changes.

## Example: evolution analysis

```bash
dtcs evolve examples/analysis/evolution/rev1.yaml examples/analysis/evolution/rev2.yaml
```

Evolution reports include:

- `sameIdentity` — whether both contracts share the same `id`
- `compatibility` — classification level
- `changes` — categorized diffs (metadata, interface, type, semantic, rule, lineage, extension)
- `migrationHints` — informative guidance (not normative contract mutation)

## Scoping comparisons

Limit which aspects are compared:

```bash
dtcs compat old.yaml new.yaml --scope interfaces,types
```

Available scopes: `interfaces`, `types`, `semantics`, `lineage`, `metadata`, `extensions`, `all`.

## Python API

```python
import dtcs

older = dtcs.parse_file("examples/analysis/backward_old.yaml")["contract"]
newer = dtcs.parse_file("examples/analysis/backward_new.yaml")["contract"]

report = dtcs.compat_analyze(older, newer)
print(report["level"])  # e.g. "backwardCompatible"

evolution = dtcs.evolve_analyze(older, newer)
print(evolution["changes"])
```

## Rust API

```rust
use dtcs::{analyze_compatibility, analyze_evolution, ComparisonScope, parse_file};

let source = parse_file("examples/analysis/backward_old.yaml")?.into_contract()?;
let target = parse_file("examples/analysis/backward_new.yaml")?.into_contract()?;

let compat = analyze_compatibility(&source, &target, ComparisonScope::all());
let evolution = analyze_evolution(&source, &target);
```

See [public-api.md](../implementation/public-api.md) for full API details.

## Fixture pairs

All five classification levels have paired fixtures under `tests/fixtures/compatibility/`:

| Pair | Expected level |
|------|----------------|
| `identical_a.yaml` / `identical_b.yaml` | Identical |
| `backward_old.yaml` / `backward_new.yaml` | Backward compatible |
| `forward_old.yaml` / `forward_new.yaml` | Forward compatible |
| `conditional_a.yaml` / `conditional_b.yaml` | Conditionally compatible |
| `incompatible_a.yaml` / `incompatible_b.yaml` | Incompatible |

User-facing copies of key pairs are in [`examples/analysis/`](https://github.com/eddiethedean/dtcs/blob/main/examples/analysis/backward_old.yaml).

## Next steps

- JSON output for compat/evolve/lineage: [json-output.md](json-output.md)
- Getting started walkthrough: [getting-started.md](getting-started.md)
- Enterprise evaluation: [adoption/overview.md](../adoption/overview.md)
