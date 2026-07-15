# Cookbook

Short recipes. Run clone-relative paths from the repository root (PyPI installs do not ship `examples/`).

## CI: fail on invalid contracts

```bash
dtcs validate contracts/**/*.dtcs.yaml --json
# Exit 0 = no error diagnostics
```

See [ci-integration.md](ci-integration.md).

## CI: compatibility gate

```bash
dtcs compat old.dtcs.yaml new.dtcs.yaml --json
```

Accept only levels you intend (often `identical` / `backwardCompatible`). See [compatibility.md](compatibility.md).

## Download + validate without cloning

```bash
curl -fsSL https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml \
  -o contract.dtcs.yaml
dtcs validate contract.dtcs.yaml
# → valid
```

## Run the 0.11 flagship pipeline

```bash
git clone https://github.com/eddiethedean/dtcs.git
cd dtcs
dtcs run examples/customer_pipeline.dtcs.yaml \
  --input tests/fixtures/runtime/customer_pipeline_input.json --json
```

Expected: two `customer_clean` rows (`status: active` only), lowercased emails.

## Null / missing / invalid tokens

```python
import dtcs, json
# Prefer asserting token shape, not coercing to None
row = {"email": {"$dtcs": "missing"}}
assert row["email"]["$dtcs"] == "missing"
```

## Vendor registry merge

```bash
dtcs validate contract.dtcs.yaml --registry ./vendor-registry.yaml
```

See [extensions-and-registries.md](extensions-and-registries.md).

## Conformance certification

```bash
dtcs conformance run --profile all --json
```
