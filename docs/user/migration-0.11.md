# Migrating to 0.11.0

Upgrade playbook from **0.10.x** tools. Spec remains `1.0.0-draft`; document `dtcsVersion` stays `"1.0.0"`.

## Summary

| Area | Change |
|------|--------|
| Lineage | Mapping `operation` defaults to `dtcs:derive`; optional `flow` enum |
| Dataset actions | Require a `parameters` map (`fields`, join keys, …) |
| COM | First-class `guarantees` / `compatibility`; nested extensions preserved |
| Null semantics | Runtime distinguishes null / missing / invalid tokens |
| Stdlib | Full Ch 17–19 catalog (dataset ops, extra functions/rules) |

## Before / after

### Lineage

```yaml
# 0.10.x (still valid; defaults applied in 0.11)
lineage:
  mappings:
    - output: "customer_clean"
      inputs: ["customer_raw"]

# 0.11 explicit (recommended)
lineage:
  mappings:
    - id: "raw_to_clean"
      output: "customer_clean"
      inputs: ["customer_raw"]
      operation: "dtcs:derive"
      flow: derived
```

### Dataset operators

```yaml
# Required in 0.11
semanticActions:
  - id: "keep_columns"
    action: "dtcs:project"
    target: "customer_raw"
    parameters:
      fields: ["customer_id", "email"]
```

Overlapping targets need `semantics.ordering`:

```yaml
semantics:
  ordering:
    mode: explicit
    order: ["normalize_email", "keep_columns"]
```

### Runtime JSON tokens

```json
{ "email": null }
{ "email": { "$dtcs": "missing" } }
{ "email": { "$dtcs": "invalid", "reason": "bad format" } }
```

Do **not** treat missing/invalid as JSON `null` in CI assertions.

## Verify you are on 0.11

```bash
dtcs version
# → dtcs 0.11.0

dtcs registry resolve dtcs:project --json   # should resolve
dtcs registry resolve dtcs:one_of --json
dtcs validate examples/minimal.dtcs.yaml    # after clone or curl
```

Pin installs:

```bash
pip install 'dtcs==0.11.0'
cargo install dtcs --version 0.11.0
```

## Related

- [CHANGELOG.md](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md#0110)
- [faq.md](faq.md#migration-to-0110)
- [customer_pipeline.dtcs.yaml](https://github.com/eddiethedean/dtcs/blob/main/examples/customer_pipeline.dtcs.yaml)
