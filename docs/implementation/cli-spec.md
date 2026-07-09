# CLI Specification

> **Canonical user reference:** [docs/user/cli-guide.md](../user/cli-guide.md) — full command descriptions, flags, exit codes, and CI examples. This document is a concise implementation summary.

Initial binary name:

```bash
dtcs
```

Commands:

```bash
dtcs validate <path> [--registry PATH] [--json]
dtcs analyze <path> [--registry PATH] [--json]
dtcs plan <path> [--registry PATH] [--json]
dtcs optimize <path> [--plan] [--registry PATH] [--no-validate] [--json]
dtcs match <path> [--plan] [--optimize] [--registry PATH] [--profile dtcs:reference] [--json]
dtcs compile <path> [--plan] [--optimize] [--registry PATH] [--profile dtcs:reference] [--json]
dtcs run <path> --input <json> [--optimize] [--registry PATH] [--json]
dtcs inspect <path> [--json]
dtcs diagnostics <path> [--json]
dtcs version [--json]
dtcs compat <source> <target> [--scope ...] [--registry PATH] [--json]
dtcs evolve <older> <newer> [--registry PATH] [--json]
dtcs lineage <contract> [--impact INPUT] [--dependency OUTPUT] [--registry PATH] [--json]
dtcs registry list [--registry PATH] [--json]
dtcs registry resolve <id> [--registry PATH] [--json]
```

`validate` should return:

- exit code 0 when valid
- non-zero exit code when invalid
- human-readable output by default
- JSON output with `--json`

`compat` returns exit code 1 when the compatibility level is `Incompatible`.

`evolve` returns exit code 1 when contracts have different identities or the evolution is incompatible.

`lineage` returns exit code 0 on success; use `--impact` or `--dependency` to filter the report.

`registry list` prints the embedded catalog (optionally merged with `--registry`).

`registry resolve` returns exit code 0 when the identifier is found, exit code 1 when unresolved.

`plan` returns exit code 1 when lowering or plan validation fails.

`match` returns exit code 1 when mandatory capabilities are unsupported.

`compile` returns exit code 1 when compilation or execution-plan validation fails.

`run` returns exit code 1 when compile or runtime execution fails. Runtime inputs are JSON keyed by interface id with row arrays of field objects.

CLI behavior should reflect the diagnostics and validation model in `SPEC.md`.

The Python package (`python -m dtcs` or the `dtcs` console script) mirrors these subcommands.

For JSON output shapes, see [docs/user/json-output.md](../user/json-output.md).
