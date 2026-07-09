# Security checklist (Ch 24)

This document maps [SPEC.md](../../SPEC.md) Chapter 24 security requirements to verifiable checks in the reference implementation.

## Automated probes

Run via `dtcs conformance run` (security section of the report) or `./scripts/security-checklist.sh`:

| Probe ID | Ch 24 area | Automated check |
|----------|------------|-----------------|
| `contract-integrity` | §5 Contract integrity | Rejects duplicate JSON parameter keys in rule `parameters` |
| `registry-trust` | §7 Registry trust | Rejects novel `dtcs:` entries when merging vendor catalogs |
| `trusted-extensions` | §6 Trusted extensions | Blocks mandatory unsupported extension namespaces |
| `diagnostics-stability` | §10 Diagnostics | Emits stable `dtcs:` codes without filesystem paths in messages |
| `no-network-surface` | §9 Sensitive information | Documents that validator/runtime perform no network I/O |

## Manual review items

These remain operator responsibilities for production deployments:

| Area | Guidance |
|------|----------|
| §5 Oversize inputs | Configure parser byte and depth limits appropriate to your environment |
| §6 Extension policy | Review vendor registry catalogs before merge; treat `experimental` entries cautiously |
| §7 Registry provenance | Load registries only from trusted sources; prefer offline cache for air-gapped use |
| §8 Supply chain | Pin crate, wheel, and npm package versions; verify release signatures and CI artifacts |
| §9 Runtime data | Do not log raw PII from runtime inputs; scope execution environments appropriately |
| §11 Operational security | Restrict who can publish contracts and registries in your organization |

## Conformance integration

Security probes are included in every `dtcs conformance run` report under the `security` array. A passing Integrated Platform run satisfies Ch 24 §12 for the reference implementation's automated subset.

## Reporting issues

Report security defects through the repository's standard issue tracker with the `security` label. Do not include sensitive contract data or credentials in public issues.
