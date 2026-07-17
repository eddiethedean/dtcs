# Security Policy

## Supported versions

Security fixes are accepted against the latest published reference-implementation release on the default branch (`main`). Older patch lines may not receive backports while the Spec remains `2.0.0` and tools remain alpha.

| Component | Status |
|-----------|--------|
| Spec | Draft (`2.0.0`) |
| Tools (`dtcs` CLI / crates.io / PyPI) | Alpha (`0.12.x`) |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security-sensitive reports.

1. Prefer GitHub **Security Advisories** / private vulnerability reporting for this repository when available.
2. Otherwise email the maintainers listed in the repository profile or open a draft advisory describing:
   - affected component (CLI, Python bindings, WASM, registry loader, …)
   - impact and reproduction
   - whether a public disclosure date is requested

We will acknowledge receipt and coordinate a fix and disclosure timeline.

## What this project does and does not provide

- The validator and reference runtime perform **no network I/O** when processing local contracts (except optional registry URI loading you explicitly enable).
- Release **signatures / attestations are not yet published** for every artifact. Pin versions (`pip install dtcs==0.12.0`, `cargo install dtcs --version 0.12.0`) and verify checksums from your package index / CI provenance when required by policy.
- The reference runtime is **not** a hardened multi-tenant execution service. Do not treat `dtcs run` as a production sandbox for untrusted contracts or PII at scale.

## Automated checks

From a pip install (no Rust toolchain required):

```bash
dtcs conformance run --profile all
```

The report includes a `security` array of automated Ch 24 probes. Passing those probes covers the **automated subset** only; operators still own oversize-input limits, registry trust, and PII handling. See [docs/adoption/security-checklist.md](docs/adoption/security-checklist.md).

Contributor-only script (requires a full Rust checkout):

```bash
./scripts/security-checklist.sh
```
