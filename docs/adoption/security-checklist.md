# Security checklist (Ch 24)

This document maps [SPEC.md](../SPEC.md) Chapter 24 security requirements to verifiable checks in the reference implementation.

> Spec is draft; tools are alpha. Automated probes cover a **subset** of Ch 24. Operators remain responsible for oversize inputs, registry trust, and PII handling. See [SECURITY.md](https://github.com/eddiethedean/dtcs/blob/main/SECURITY.md).

## Pip-first automated probes

No Rust toolchain required after `pip install dtcs`:

```bash
dtcs conformance run --profile all
```

Inspect the report's `security` array (JSON with `--json`). Probe IDs:

| Probe ID | Ch 24 area | Automated check |
|----------|------------|-----------------|
| `contract-integrity` | §5 Contract integrity | Rejects duplicate JSON parameter keys in rule / action `parameters` |
| `registry-trust` | §7 Registry trust | Rejects novel `dtcs:` entries when merging vendor catalogs |
| `trusted-extensions` | §6 Trusted extensions | Blocks mandatory unsupported extension namespaces |
| `diagnostics-stability` | §10 Diagnostics | Emits stable `dtcs:` codes without filesystem paths in messages |
| `no-network-surface` | §9 Sensitive information | Documents that validator/runtime perform no network I/O by default |

## Contributor script (optional)

From a full git checkout with Rust:

```bash
./scripts/security-checklist.sh
```

Prefer the `dtcs conformance` path for adopter CI.

## Manual review items

| Area | Guidance |
|------|----------|
| §5 Oversize inputs | Configure parser byte and depth limits for your environment |
| §6 Extension policy | Review vendor registry catalogs before merge; treat `experimental` entries cautiously |
| §7 Registry provenance | Load registries only from trusted sources; prefer offline cache for air-gapped use |
| §8 Supply chain | Pin crate, wheel, and npm versions. Release signatures/attestations are **not consistently published yet** — use index checksums and CI provenance as your policy requires |
| §9 Runtime data | Do not log raw PII from runtime inputs; scope execution environments appropriately |
| §11 Operational security | Restrict who can publish contracts and registries in your organization |

## Conformance integration

Security probes appear under the `security` array of every `dtcs conformance run` report. A passing Integrated Platform run satisfies Ch 24 §12 for the reference implementation's **automated subset** only.

## Reporting issues

See [SECURITY.md](https://github.com/eddiethedean/dtcs/blob/main/SECURITY.md). Do not include sensitive contract data or credentials in public issues.
