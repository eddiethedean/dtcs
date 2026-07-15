# Getting Started

Install the tools and validate a contract. Prefer `pip install` when you want a fast first success; `cargo install` compiles from source and often takes longer than a few minutes.

## Prerequisites

- **Python 3.9+** (for PyPI) **or** **Rust 1.75+** (for `cargo install` / building from source)
- A terminal with `curl` (or any way to save a YAML file)

## 1. Install

```bash
pip install dtcs
# or: cargo install dtcs
```

Verify:

```bash
dtcs version
```

Expected:

```text
dtcs 0.11.0
spec 1.0.0-draft
```

## 2. Get a minimal contract (no git clone)

PyPI and crates.io installs do **not** include the repo `examples/` directory. Download the sample:

```bash
curl -fsSL https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml \
  -o contract.dtcs.yaml
```

Or create `contract.dtcs.yaml` with this content (note `dtcsVersion` must be exactly `"1.0.0"` — the Spec is still `1.0.0-draft`, but documents use the finalized document version string):

```yaml
dtcsVersion: "1.0.0"
id: "demo.minimal"
name: "Minimal Demo"
version: "0.1.0"

metadata:
  description: "Smallest valid DTCS contract for install checks"
  classification: internal
  governance:
    owner: "docs"
    steward: "docs"
  provenance:
    author: "docs"
    createdAt: "2026-01-01T00:00:00Z"

inputs:
  - id: "people"
    schema:
      fields:
        - name: "person_id"
          type: "string"
          nullable: false
        - name: "display_name"
          type: "string"
          nullable: false

outputs:
  - id: "people_normalized"
    schema:
      fields:
        - name: "normalized_id"
          type: "string"
          nullable: false
        - name: "normalized_name"
          type: "string"
          nullable: false

semanticActions:
  - id: "lower_display_name"
    action: "dtcs:lowercase"
    target: "people.display_name"

lineage:
  mappings:
    - output: "people_normalized"
      inputs: ["people"]
      operation: "dtcs:derive"
      flow: derived
```

## 3. Validate (success milestone)

```bash
dtcs validate contract.dtcs.yaml
echo $?
```

Expected:

```text
valid
```

Exit code `0` means no error-severity diagnostics. If validation fails:

```bash
dtcs diagnostics contract.dtcs.yaml
```

Add `--json` for machine-readable output (see [json-output.md](json-output.md)).

**You are done with the first success path.** Everything below is optional.

## Next steps

| Goal | Document / action |
|------|-------------------|
| Mental model (COM → validate → plan → run) | [concepts.md](concepts.md) |
| Author richer contracts | [writing-contracts.md](writing-contracts.md) |
| 0.11 flagship sample (clone or download) | [`customer_pipeline.dtcs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/examples/customer_pipeline.dtcs.yaml) |
| Run with sample rows (needs fixture file) | Clone the repo, then see [cookbook.md](cookbook.md) |
| Upgrade from 0.10.x | [migration-0.11.md](migration-0.11.md) |
| CLI flags and exit codes | [cli-guide.md](cli-guide.md) |
| CI gates | [ci-integration.md](ci-integration.md) |
| Common problems | [troubleshooting.md](troubleshooting.md) · [faq.md](faq.md) |

### Explore the full CLI (requires a local checkout)

```bash
git clone https://github.com/eddiethedean/dtcs.git
cd dtcs

dtcs validate examples/customer_pipeline.dtcs.yaml
dtcs analyze examples/customer_pipeline.dtcs.yaml
dtcs plan examples/customer_pipeline.dtcs.yaml
dtcs run examples/customer_pipeline.dtcs.yaml \
  --input tests/fixtures/runtime/customer_pipeline_input.json
dtcs compat examples/analysis/backward_old.yaml examples/analysis/backward_new.yaml
dtcs registry resolve dtcs:lowercase
```

Expected for a valid pipeline contract:

```text
valid
```

### Use from Python

```python
import dtcs
import urllib.request

url = "https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml"
content = urllib.request.urlopen(url).read()
report = dtcs.parse_and_validate(content)
assert dtcs.is_valid(report)
print(dtcs.__version__, dtcs.SPEC_VERSION)
```

Contract dicts use **camelCase** keys (`dtcsVersion`, `semanticActions`, …). See [Python API](../api/python.md).

## Develop from source

```bash
git clone https://github.com/eddiethedean/dtcs.git
cd dtcs
python -m venv .venv && source .venv/bin/activate
pip install maturin pytest
maturin develop --no-default-features --features python --locked
pytest python/tests -v
```

See [CONTRIBUTING.md](https://github.com/eddiethedean/dtcs/blob/main/CONTRIBUTING.md#contributor-quickstart).
