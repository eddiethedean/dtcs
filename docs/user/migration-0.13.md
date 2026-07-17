# Migrating to 0.13.0 / Spec 3.0

Upgrade playbook from **0.12.x** / Spec **2.0** tools to **0.13.0** / Spec **3.0**.

Version axes: [versioning.md](versioning.md). Historical 0.11 → 0.12 notes: [migration-0.12.md](migration-0.12.md).

## Install pins

```bash
pip install 'dtcs==0.13.0'
# or
cargo install dtcs --version 0.13.0

dtcs version
# → dtcs 0.13.0
# → spec 3.0.0
```

## Prefer `dtcsVersion` 3.0.0

| Before (0.12-era docs) | After |
|------------------------|--------|
| Prefer `"2.0.0"` | Prefer `"3.0.0"`; `"2.0.0"` / `"1.0.0"` still accepted |

```yaml
dtcsVersion: "3.0.0"
```

## Plan identity `/2` and kernel `/2` defaults

- Canonical portable plan identity is **`dtcs.transform-plan/2`** (valid v1 envelopes migrate deterministically).
- Default portable profiles are **`dtcs:profile/portable-relational-kernel/2`** and **`dtcs:profile/portable-relational/2`** (superseding `/1` for new 3.0 work).

```python
portable = dtcs.plan_export_portable(plan)
assert portable["planIdentity"] == "dtcs.transform-plan/2"
```

## New experimental profiles

A.9 Rich Portable Analytics families (string-advanced, conversion, statistics, complex-values, reshape, relational-extended, temporal-iana, nondeterministic) are **Experimental**. `portable-window/2` is **Candidate**. Do not treat Experimental claims as certified.

## Explicit `errorMode`

Plan v2 fingerprint requirements include an explicit **`errorMode`** (`fail` \| `invalid` \| `null` \| `route`). Prefer declaring it rather than relying on implicit defaults when exporting or consuming portable plans.

## Unicode 15.1 / regex grammar gate

Fingerprint pins for 3.0 portable plans include:

| Pin | Value |
|-----|--------|
| Unicode | `unicode-15.1` |
| Regex grammar | `dtcs-regex/1` |
| Timezone data | `iana-2025b` |
| Random algorithm | `xorshift64star/1` |

String-advanced / regex behavior is gated on the published grammar and Unicode pin — do not assume host-locale or backend-native regex.

## Verify

```bash
dtcs validate your_contract.yaml
dtcs conformance run --profile all
# optional (Rust CLI):
dtcs export-portable your_contract.yaml --fingerprint
```

## Next

- [getting-started.md](getting-started.md)
- [versioning.md](versioning.md)
- [conformance.md](conformance.md)
- [CHANGELOG](https://github.com/eddiethedean/dtcs/blob/main/CHANGELOG.md)
