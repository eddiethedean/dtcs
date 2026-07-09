# Examples

Sample DTCS transformation contracts for learning and testing.

**Run all commands from the repository root** (paths below are relative to the repo root).

## Primary example

| File | Description |
|------|-------------|
| [customer_normalize.dtcs.yaml](customer_normalize.dtcs.yaml) | Realistic contract with metadata, semantic actions, rules, and lineage |

```bash
dtcs validate examples/customer_normalize.dtcs.yaml
dtcs inspect examples/customer_normalize.dtcs.yaml
dtcs analyze examples/customer_normalize.dtcs.yaml
dtcs plan examples/customer_normalize.dtcs.yaml
dtcs plan examples/customer_normalize.dtcs.yaml --json
dtcs optimize examples/customer_normalize.dtcs.yaml

# Phase 0.9 execution pipeline
dtcs match examples/customer_normalize.dtcs.yaml
dtcs compile examples/customer_normalize.dtcs.yaml
dtcs run examples/customer_normalize.dtcs.yaml \
  --input tests/fixtures/runtime/customer_normalize_input.json
```

See [cli-guide.md](../docs/user/cli-guide.md) for all flags and [getting-started.md](../docs/user/getting-started.md) for a full walkthrough.

## Analysis examples

These copies live under `examples/analysis/` for easy discovery. Equivalent fixtures also exist under `tests/fixtures/` for integration tests.

### Compatibility

| File | Description |
|------|-------------|
| [analysis/backward_old.yaml](analysis/backward_old.yaml) | Older revision (integer input/output) |
| [analysis/backward_new.yaml](analysis/backward_new.yaml) | Backward-compatible update (adds optional input interface and output field) |

```bash
dtcs compat examples/analysis/backward_old.yaml examples/analysis/backward_new.yaml
# Expected: backwardCompatible
```

Additional classification pairs (identical, forward, conditional, incompatible) are in `tests/fixtures/compatibility/`.

### Evolution

| File | Description |
|------|-------------|
| [analysis/evolution/rev1.yaml](analysis/evolution/rev1.yaml) | First revision |
| [analysis/evolution/rev2.yaml](analysis/evolution/rev2.yaml) | Second revision (same `id`, version bump) |

```bash
dtcs evolve examples/analysis/evolution/rev1.yaml examples/analysis/evolution/rev2.yaml
```

### Lineage

| File | Description |
|------|-------------|
| [analysis/lineage_multi.yaml](analysis/lineage_multi.yaml) | Multi-input, multi-output contract with governance metadata |

```bash
dtcs lineage examples/analysis/lineage_multi.yaml
dtcs lineage examples/analysis/lineage_multi.yaml --impact customers
dtcs lineage examples/analysis/lineage_multi.yaml --dependency order_enriched
```

### Semantic analysis and planning

The primary example contract exercises semantic actions, rules, and lineage suitable for analysis and plan lowering:

```bash
dtcs analyze examples/customer_normalize.dtcs.yaml
dtcs plan examples/customer_normalize.dtcs.yaml
dtcs match examples/customer_normalize.dtcs.yaml
dtcs compile examples/customer_normalize.dtcs.yaml
dtcs run examples/customer_normalize.dtcs.yaml \
  --input tests/fixtures/runtime/customer_normalize_input.json
```

Golden plan fixtures for the test corpus live under `tests/fixtures/plans/`. Compare with:

```bash
dtcs plan tests/fixtures/valid_minimal.json --json
```

## Invalid contracts (for learning diagnostics)

These live under `tests/fixtures/` and demonstrate common validation failures:

| Fixture | Error demonstrated |
|---------|-------------------|
| `missing_lineage.yaml` | Output without lineage mapping |
| `unresolved_reference.yaml` | Field reference that does not resolve |
| `invalid_type.yaml` | Malformed type expression |
| `unsupported_version.yaml` | Unsupported `dtcsVersion` |

```bash
dtcs diagnostics tests/fixtures/missing_lineage.yaml
```

## Next steps

- [Getting started](../docs/user/getting-started.md)
- [Writing contracts](../docs/user/writing-contracts.md)
- [Compatibility guide](../docs/user/compatibility.md)
- [CLI reference](../docs/user/cli-guide.md)
