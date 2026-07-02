# Getting Started

This guide gets you from zero to a validated DTCS contract in about five minutes.

## Prerequisites

- **Python 3.9+** or **Rust 1.75+**
- A terminal

## Install

```bash
pip install dtcs
# or
cargo install dtcs
```

Verify:

```bash
dtcs version
```

## Validate your first contract

The repository includes a realistic example at [`examples/customer_normalize.dtcs.yaml`](../../examples/customer_normalize.dtcs.yaml):

```bash
dtcs validate examples/customer_normalize.dtcs.yaml
echo $?   # 0 = valid
```

If validation fails, diagnostics explain what to fix:

```bash
dtcs diagnostics examples/customer_normalize.dtcs.yaml
```

Add `--json` for machine-readable output (see [json-output.md](json-output.md)).

## Inspect a contract

```bash
dtcs inspect examples/customer_normalize.dtcs.yaml
```

This prints a short summary: contract id, version, input/output counts, and semantic action count.

## Compare two contract versions

When you change a contract, check whether downstream consumers can adopt the new version:

```bash
dtcs compat examples/analysis/backward_old.yaml examples/analysis/backward_new.yaml
```

See [compatibility.md](compatibility.md) for what each classification level means.

## Analyze evolution between revisions

For two revisions of the **same** contract identity:

```bash
dtcs evolve examples/analysis/evolution/rev1.yaml examples/analysis/evolution/rev2.yaml
```

## Trace lineage

```bash
dtcs lineage examples/analysis/lineage_multi.yaml
dtcs lineage examples/analysis/lineage_multi.yaml --impact customers
dtcs lineage examples/analysis/lineage_multi.yaml --dependency order_enriched
```

## Use from Python

```python
import dtcs

with open("examples/customer_normalize.dtcs.yaml", "rb") as f:
    report = dtcs.parse_and_validate(f.read())

assert dtcs.is_valid(report)

result = dtcs.parse_file("examples/customer_normalize.dtcs.yaml")
contract = result["contract"]
summary = dtcs.inspect(contract)
print(summary)
```

Contract dicts returned by `parse` and `parse_file` use **camelCase** keys (`dtcsVersion`, `semanticActions`, etc.).

## What to read next

| Goal | Document |
|------|----------|
| Understand contract fields | [writing-contracts.md](writing-contracts.md) |
| All CLI commands and flags | [cli-guide.md](cli-guide.md) |
| Compatibility classifications | [compatibility.md](compatibility.md) |
| JSON output shapes | [json-output.md](json-output.md) |
| Common questions | [faq.md](faq.md) |
| Normative definitions | [SPEC.md](../../SPEC.md) |

## Develop from source

Contributors need Rust, Python (for tests), and maturin:

```bash
git clone https://github.com/eddiethedean/dtcs.git
cd dtcs
python -m venv .venv && source .venv/bin/activate
pip install maturin pytest
maturin develop --no-default-features --features python --locked
pytest python/tests -v
```

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for the full workflow.
