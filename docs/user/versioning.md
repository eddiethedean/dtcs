# Versioning

How crate, Spec, and document versions relate. Keep this page as the single source of truth when other docs mention versions.

## Three version axes

| Axis | Current | Meaning |
|------|---------|---------|
| **Tools / crate** | `0.13.0` (alpha) | Published package versions (`dtcs` on crates.io / PyPI, CLI `dtcs version`) |
| **Spec** | `3.0.0` (draft) | Normative document in [`SPEC.md`](../SPEC.md); also `SPEC_VERSION` / `specVersion` in tool output |
| **Document `dtcsVersion`** | Prefer `"3.0.0"` | Field inside each Transformation Contract YAML/JSON |

These move independently. Tools `0.13.0` implement Spec draft `3.0.0`. A contract’s `dtcsVersion` declares which Spec edition the document targets.

## Accepted document `dtcsVersion` values

| Value | Status |
|-------|--------|
| `"3.0.0"` | **Preferred** for new contracts |
| `"2.0.0"` | Accepted for compatibility |
| `"1.0.0"` | Accepted for compatibility |
| `"1.0.0-draft"` | Accepted for compatibility |

Patch or other prerelease strings (`"1.0.1"`, `"2.0.1"`, `"3.0.1"`, `"3.0.0-draft"`) are **rejected** with `dtcs:unsupported-version`.

## What Spec 3.0 changes for authors

Prefer `"3.0.0"` when you use Rich Portable Analytics profiles, plan-v2 envelopes, lambdas, advanced string/regex functions, reshape actions, or controlled nondeterminism. DTCS 2.0 contracts remain accepted. Upgrade steps: [migration-0.13.md](migration-0.13.md).

## Verify locally

```bash
dtcs version
# → dtcs 0.13.0
# → spec 3.0.0

dtcs version --json
# → {"crateVersion":"0.13.0","specVersion":"3.0.0"}
```

## Related

- [getting-started.md](getting-started.md)
- [troubleshooting.md](troubleshooting.md#validation-errors)
- [migration-0.13.md](migration-0.13.md)
- [migration-0.12.md](migration-0.12.md)
- [CHANGELOG.md](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md)
