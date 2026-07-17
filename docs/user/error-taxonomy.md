# Error taxonomy

How failures surface across CLI, Python, Rust, and WASM/Node.

## Three channels

| Channel | What it is | Typical consumer action |
|---------|------------|-------------------------|
| **Diagnostics** | Structured findings (`id`, `severity`, `stage`, `message`, …) | Inspect codes; fix the contract or inputs |
| **Exit codes** | CLI process status (`0` / `1`) | Gate CI on `0` |
| **Raises / Err** | Language-level exceptions or `Result::Err` | Fix caller bugs, I/O, or programmer misuse |

Validation failures for ordinary invalid contracts are **diagnostics**, not exceptions.

## Severity → CLI exit

| Diagnostic severity | `dtcs validate` exit |
|---------------------|----------------------|
| `error` | `1` |
| `warning` / `information` | `0` (still printed) |

`analyze` / `plan` / `run` also exit `1` when validation fails or when error-severity analysis/runtime diagnostics appear. Details: [cli-guide.md](cli-guide.md).

## Python

| Situation | Behavior |
|-----------|----------|
| Invalid contract | API returns a dict with `diagnostics`; use `dtcs.is_valid(report)` |
| Nested analyze | Check `result["validation"]` and `result["analysis"]` separately |
| Bad arguments / I/O | May raise (`TypeError`, `ValueError`, `OSError`, …) |
| `runtime_execute` | Envelope `{outputs, diagnostics}` — not a flat map |

## Rust

| Situation | Behavior |
|-----------|----------|
| Invalid contract | `DiagnosticReport` / plan results with diagnostics |
| CLI | Maps error diagnostics to exit code `1` (`miette` for fatal CLI errors) |
| Library | Prefer checking `is_valid()` / `has_errors()` rather than panicking |

## WASM / Node

Parse/validate return reports. Invalid contracts do not throw solely because validation failed. Prefer reading `diagnostics` severities. See [api/wasm.md](../api/wasm.md).

## Related

- [diagnostic-catalog.md](diagnostic-catalog.md)
- [json-output.md](json-output.md#diagnostic-object)
- [troubleshooting.md](troubleshooting.md)
