# Changelog

## 0.2.0

Phase 0.2 — Contract Model.

- Metadata validation (identity, governance, provenance, classification, documentation)
- Extended type system: conversions, collections, extension types, expression typing
- I/O interface depth: optional inputs, streaming, pre/postconditions, I/O extensions
- Namespace validation hardening (reject `http:` / `https:` false positives)
- Python/Rust fixture parity via shared `tests/fixture_expectations.json`
- Python CLI aligned with Rust (`parse_file` for all commands)

## 0.1.x

Phase 0.1 — Foundation: parser, COM, seven-phase validation, diagnostics, CLI, Python bindings.
