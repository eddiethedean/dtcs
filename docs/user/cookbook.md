# Cookbook

Short recipes. Prefer the no-clone paths first. Clone-relative paths assume the repository root (PyPI installs do not ship `examples/`).

## Download + validate without cloning

```bash
curl -fsSL https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml \
  -o contract.dtcs.yaml
dtcs validate contract.dtcs.yaml
# → valid
```

## Run without cloning (inline input)

See [getting-started.md](getting-started.md#4-run-without-cloning-optional) for a pasteable `hello.dtcs.yaml` + `hello.input.json` that lowercases names via `dtcs run`.

## CI: fail on invalid contracts

```bash
# Prefer an explicit file list or find; globs are shell-dependent
find contracts -name '*.dtcs.yaml' -print0 | xargs -0 -n1 dtcs validate
```

See [ci-integration.md](ci-integration.md).

## CI: compatibility gate

```bash
dtcs compat old.dtcs.yaml new.dtcs.yaml --json
```

Accept only levels you intend (often `identical` / `backwardCompatible`). See [compatibility.md](compatibility.md).

## Run the sample customer pipeline (clone)

```bash
git clone https://github.com/eddiethedean/dtcs.git
cd dtcs
dtcs run examples/customer_pipeline.dtcs.yaml \
  --input tests/fixtures/runtime/customer_pipeline_input.json --json
```

Expected: two `customer_clean` rows (`status: active` only), lowercased emails.

## Null / missing / invalid tokens

Runtime / CLI JSON uses:

```json
{ "$dtcs": "missing" }
{ "$dtcs": "invalid" }
```

Portable conformance fixtures use a shorter dialect (`$missing` / `$invalid`). See [expressions.md](expressions.md#null-missing-and-invalid) and [portable-conformance.md](../implementation/portable-conformance.md#token-dialects).

```python
# Prefer asserting token shape, not coercing to None
row = {"email": {"$dtcs": "missing"}}
assert row["email"]["$dtcs"] == "missing"
```

## Vendor registry merge

```bash
dtcs validate contract.dtcs.yaml --registry ./vendor-registry.yaml
```

See [extensions-and-registries.md](extensions-and-registries.md).

## Export a portable plan (Rust CLI)

```bash
# Rust CLI (`cargo install dtcs`). Python API: plan_export_portable / plan_fingerprint.
dtcs export-portable contract.dtcs.yaml --json
dtcs export-portable contract.dtcs.yaml --fingerprint
```

See [cli-guide.md](cli-guide.md#export-portable) and [migration-0.12.md](migration-0.12.md).

## Conformance certification

```bash
dtcs conformance run --profile all --json
```
