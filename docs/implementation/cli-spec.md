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
```

`validate` should return:

- exit code 0 when valid
- non-zero exit code when invalid
- human-readable output by default
- JSON output with `--json`

CLI behavior should reflect the diagnostics and validation model in `SPEC.md`.
