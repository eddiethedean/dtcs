# Examples

Sample DTCS contracts. **Run commands from the repository root.** PyPI/crates installs do not include this directory — download files with `curl` from GitHub raw URLs or clone the repo.

## First success (no large fixture)

| File | Description |
|------|-------------|
| [minimal.dtcs.yaml](minimal.dtcs.yaml) | Smallest install-check contract (`dtcsVersion: "1.0.0"`) |

```bash
dtcs validate examples/minimal.dtcs.yaml
# → valid
```

## Flagship (0.11 features)

| File | Description |
|------|-------------|
| [customer_pipeline.dtcs.yaml](customer_pipeline.dtcs.yaml) | Filter/project `parameters`, lineage `flow`, `one_of`, explicit ordering |
| [customer_normalize.dtcs.yaml](customer_normalize.dtcs.yaml) | Smaller classic normalize + rule example |

```bash
dtcs validate examples/customer_pipeline.dtcs.yaml
dtcs run examples/customer_pipeline.dtcs.yaml \
  --input tests/fixtures/runtime/customer_pipeline_input.json --json
```

Expected runtime: two active customers with lowercased emails.

## Analysis examples

| Path | Purpose |
|------|---------|
| [analysis/backward_old.yaml](analysis/backward_old.yaml) / [backward_new.yaml](analysis/backward_new.yaml) | Compatibility |
| [analysis/evolution/](analysis/evolution/) | Evolution revisions |
| [analysis/lineage_multi.yaml](analysis/lineage_multi.yaml) | Multi-interface lineage |

```bash
dtcs compat examples/analysis/backward_old.yaml examples/analysis/backward_new.yaml
# Expected: backwardCompatible
```

## Next

- [Getting started](../docs/user/getting-started.md)
- [Writing contracts](../docs/user/writing-contracts.md)
- [Cookbook](../docs/user/cookbook.md)
