# Migrating to 0.12.0 / Spec 2.0

> **Historical.** Current tools are **0.13.0** (Spec **3.0.0**). For new upgrades use [migration-0.13.md](migration-0.13.md).

Upgrade playbook from **0.11.x** tools to **0.12.0**, and from document `dtcsVersion` `"1.0.0"` to preferred `"2.0.0"`.

For Portable Relational semantics (joins, aggs, windows, datetime), also read [migration-portable-relational.md](migration-portable-relational.md). Version axes: [versioning.md](versioning.md).

Historical 0.10 → 0.11 notes: [migration-0.11.md](migration-0.11.md) (archived; do not pin `0.11.0` for new work).

## Install pins

```bash
pip install 'dtcs==0.12.0'
# or
cargo install dtcs --version 0.12.0

dtcs version
# → dtcs 0.12.0
# → spec 2.0.0
```

## Document `dtcsVersion`

| Before (0.11-era docs) | After |
|------------------------|--------|
| `"1.0.0"` required in many guides | Prefer `"2.0.0"`; `"1.0.0"` / `"1.0.0-draft"` still accepted |

```yaml
dtcsVersion: "2.0.0"
```

## What 0.12 adds

- **Portable Relational Profile** — richer joins (`on` / `predicate` / `collisionPolicy`), multi-aggregates, window frames, datetime units, complex access ops
- **Portable plan export** — Rust CLI `dtcs export-portable`; Python `plan_export_portable` / `plan_fingerprint`
- **Structured expressions** — `expression_to_structured`
- **Capability accuracy** — `capability_portable_manifest` + conformance `portableDifferential` fixtures

## Breaking / tighter validation

Contracts that previously validated under looser 0.11 rules may need explicit join keys, union `duplicatePolicy`, or updated coalesce usage. See the compatibility tables in [migration-portable-relational.md](migration-portable-relational.md).

## Verify

```bash
dtcs validate your_contract.yaml
dtcs conformance run --profile all
# optional (Rust CLI):
dtcs export-portable your_contract.yaml --fingerprint
```

## Next

- Prefer Spec 3.0 / tools 0.13 when adopting Rich Portable Analytics: [migration-0.13.md](migration-0.13.md)
- [getting-started.md](getting-started.md)
- [cli-guide.md](cli-guide.md#export-portable)
- [api/python.md](../api/python.md#portable-plans)
- [CHANGELOG](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md)
