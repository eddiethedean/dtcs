# Versioning

How crate, Spec, and document versions relate. Keep this page as the single source of truth when other docs mention versions.

## Three version axes

| Axis | Current | Meaning |
|------|---------|---------|
| **Tools / crate** | `0.12.0` (alpha) | Published package versions (`dtcs` on crates.io / PyPI, CLI `dtcs version`) |
| **Spec** | `2.0.0` (draft) | Normative document in [`SPEC.md`](../SPEC.md); also `SPEC_VERSION` / `specVersion` in tool output |
| **Document `dtcsVersion`** | Prefer `"2.0.0"` | Field inside each Transformation Contract YAML/JSON |

These move independently. Tools `0.12.0` implement Spec draft `2.0.0`. A contract’s `dtcsVersion` declares which Spec edition the document targets.

## Accepted document `dtcsVersion` values

| Value | Status |
|-------|--------|
| `"2.0.0"` | **Preferred** for new contracts |
| `"1.0.0"` | Accepted for compatibility |
| `"1.0.0-draft"` | Accepted for compatibility |

Patch or other prerelease strings (`"1.0.1"`, `"2.0.1"`, `"2.0.0-draft"`) are **rejected** with `dtcs:unsupported-version`.

## What Spec 2.0 changes for authors

Prefer `"2.0.0"` when you use Portable Relational operators (rich joins, multi-aggs, window frames, datetime units, structured expressions). Compatibility details: [migration-portable-relational.md](migration-portable-relational.md) and [migration-0.12.md](migration-0.12.md).

## Verify locally

```bash
dtcs version
# → dtcs 0.12.0
# → spec 2.0.0

dtcs version --json
# → {"crateVersion":"0.12.0","specVersion":"2.0.0"}
```

## Related

- [getting-started.md](getting-started.md)
- [troubleshooting.md](troubleshooting.md#validation-errors)
- [migration-0.12.md](migration-0.12.md)
- [CHANGELOG.md](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md)
