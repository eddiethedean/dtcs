# Examples

Sample DTCS transformation contracts for learning and testing.

## Primary example

| File | Description |
|------|-------------|
| [customer_normalize.dtcs.yaml](customer_normalize.dtcs.yaml) | Realistic contract with metadata, semantic actions, rules, and lineage |

```bash
dtcs validate customer_normalize.dtcs.yaml
dtcs inspect customer_normalize.dtcs.yaml
```

## Analysis examples

These copies live under `examples/analysis/` for easy discovery. Equivalent fixtures also exist under `tests/fixtures/` for integration tests.

### Compatibility

| File | Description |
|------|-------------|
| [analysis/backward_old.yaml](analysis/backward_old.yaml) | Older revision (integer input/output) |
| [analysis/backward_new.yaml](analysis/backward_new.yaml) | Backward-compatible update (adds optional input) |

```bash
dtcs compat analysis/backward_old.yaml analysis/backward_new.yaml
# Expected: backwardCompatible
```

Additional classification pairs (identical, forward, conditional, incompatible) are in `tests/fixtures/compatibility/`.

### Evolution

| File | Description |
|------|-------------|
| [analysis/evolution/rev1.yaml](analysis/evolution/rev1.yaml) | First revision |
| [analysis/evolution/rev2.yaml](analysis/evolution/rev2.yaml) | Second revision (same `id`, version bump) |

```bash
dtcs evolve analysis/evolution/rev1.yaml analysis/evolution/rev2.yaml
```

### Lineage

| File | Description |
|------|-------------|
| [analysis/lineage_multi.yaml](analysis/lineage_multi.yaml) | Multi-input, multi-output contract with governance metadata |

```bash
dtcs lineage analysis/lineage_multi.yaml
dtcs lineage analysis/lineage_multi.yaml --impact customers
dtcs lineage analysis/lineage_multi.yaml --dependency order_enriched
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
