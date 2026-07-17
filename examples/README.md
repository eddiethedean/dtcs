# Examples

Sample DTCS contracts. **Run commands from the repository root.** PyPI/crates installs do not include this directory — download files with `curl` from GitHub raw URLs or clone the repo.

## First success (no large fixture)

| File | Description |
|------|-------------|
| [minimal.dtcs.yaml](minimal.dtcs.yaml) | Install-check contract (`dtcsVersion: "3.0.0"`) |

```bash
dtcs validate examples/minimal.dtcs.yaml
# → valid
```

No clone:

```bash
curl -fsSL https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml \
  -o contract.dtcs.yaml
dtcs validate contract.dtcs.yaml
```

## Spec 3.0 sample

| File | Description |
|------|-------------|
| [spec3_coalesce.dtcs.yaml](spec3_coalesce.dtcs.yaml) | Spec 3.0 with `dtcs:coalesce` expression; portable plans use `dtcs.transform-plan/2` |

```bash
dtcs validate examples/spec3_coalesce.dtcs.yaml
# → valid
```

## Classic pipeline (still valid; older `dtcsVersion`)

| File | Description |
|------|-------------|
| [customer_pipeline.dtcs.yaml](customer_pipeline.dtcs.yaml) | Filter/project `parameters`, lineage `flow`, `one_of`, explicit ordering (`dtcsVersion: "1.0.0"`) |
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

## Maintainer tooling (not a user example)

| File | Purpose |
|------|---------|
| [write_optimize_goldens.rs](write_optimize_goldens.rs) | Regenerates optimized-plan golden files (`cargo run --example write_optimize_goldens`) |

## Next

- [Getting started](../docs/user/getting-started.md)
- [Writing contracts](../docs/user/writing-contracts.md)
- [Migration 0.13 / Spec 3.0](../docs/user/migration-0.13.md)
- [Cookbook](../docs/user/cookbook.md)
