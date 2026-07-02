# CLI Specification

Initial binary name:

```bash
dtcs
```

Commands:

```bash
dtcs validate <path>
dtcs inspect <path>
dtcs diagnostics <path>
dtcs version
dtcs compat <source> <target> [--scope ...] [--json]
dtcs evolve <older> <newer> [--json]
dtcs lineage <contract> [--impact INPUT] [--dependency OUTPUT] [--json]
```

`validate` should return:

- exit code 0 when valid
- non-zero exit code when invalid
- human-readable output by default
- JSON output with `--json`

`compat` returns exit code 1 when the compatibility level is `Incompatible`.

`evolve` returns exit code 1 when contracts have different identities or the evolution is incompatible.

`lineage` returns exit code 0 on success; use `--impact` or `--dependency` to filter the report.

CLI behavior should reflect the diagnostics and validation model in `SPEC.md`.

The Python package (`python -m dtcs` or the `dtcs` console script) mirrors these subcommands.
