# Getting Started

Install the tools and validate a contract. Prefer `pip install` when you want a fast first success; `cargo install` compiles from source and often takes longer than a few minutes.

## Prerequisites

- **Python 3.9+** (for PyPI) **or** **Rust 1.75+** (for `cargo install` / building from source)
- A terminal with `curl` (or any way to save a YAML file)

### Installation notes

| Topic | Detail |
|-------|--------|
| Pin | Prefer `pip install 'dtcs==0.13.0'` or `cargo install dtcs --version 0.13.0` |
| Platforms | PyPI publishes wheels for common CPython 3.9–3.13 platforms when available; otherwise build from source with Rust |
| Dual `dtcs` on PATH | Rust and Python both install a `dtcs` CLI — run `which dtcs` / `dtcs version` if versions look wrong |
| Offline | Paste YAML from this page; wheels do not include `examples/` |
| Full matrix | [limits.md](limits.md) · [what-dtcs-is-not.md](what-dtcs-is-not.md) |
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
dtcs 0.13.0
spec 3.0.0
```

## 2. Get a minimal contract (no git clone)

PyPI and crates.io installs do **not** include the repo `examples/` directory. Download the sample:

```bash
curl -fsSL https://raw.githubusercontent.com/eddiethedean/dtcs/main/examples/minimal.dtcs.yaml \
  -o contract.dtcs.yaml
```

Or create `contract.dtcs.yaml` with this content (prefer `dtcsVersion: "3.0.0"`; prior versions remain accepted):

```yaml
dtcsVersion: "3.0.0"
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

**You are done with the first success path (validate).** Optional: run a tiny contract with sample rows without cloning (below), or continue with the table.

## 4. Run without cloning (optional)

Create `hello.dtcs.yaml`:

```yaml
dtcsVersion: "3.0.0"
id: "demo.hello"
name: "Hello Run"
version: "0.1.0"

metadata:
  description: "Minimal contract that executes with the reference runtime"
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
        - name: "name"
          type: "string"
          nullable: false

outputs:
  - id: "people_out"
    schema:
      fields:
        - name: "name"
          type: "string"
          nullable: false

semanticActions:
  - id: "lower_name"
    action: "dtcs:lowercase"
    target: "people.name"

lineage:
  mappings:
    - output: "people_out"
      inputs: ["people"]
```

Create `hello.input.json`:

```json
{
  "people": [
    { "name": "Ada" },
    { "name": "Grace" }
  ]
}
```

```bash
dtcs validate hello.dtcs.yaml
dtcs run hello.dtcs.yaml --input hello.input.json --json
```

Expected JSON includes lowercased names under `people_out` (for example `"ada"`, `"grace"`). A warning about the shared field name `name` is harmless. This is an in-memory teaching runtime, not a warehouse executor — see [limits.md](limits.md).

## Next steps

| Goal | Document / action |
|------|-------------------|
| Mental model (COM → validate → plan → run) | [concepts.md](concepts.md) |
| Versions (crate / Spec / `dtcsVersion`) | [versioning.md](versioning.md) |
| Author richer contracts | [writing-contracts.md](writing-contracts.md) |
| Sample pipeline (clone or download) | [`customer_pipeline.dtcs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/examples/customer_pipeline.dtcs.yaml) |
| Spec 3.0 sample (coalesce) | [`spec3_coalesce.dtcs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/examples/spec3_coalesce.dtcs.yaml) |
| More recipes | [cookbook.md](cookbook.md) |
| Upgrade to 0.13 / Spec 3.0 | [migration-0.13.md](migration-0.13.md) |
| Historical: 0.12 / Spec 2.0 | [migration-0.12.md](migration-0.12.md) |
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
