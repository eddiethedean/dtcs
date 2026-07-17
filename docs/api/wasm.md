# WASM API

Package: [`@eddiethedean/dtcs-wasm`](https://github.com/eddiethedean/dtcs/tree/main/bindings/wasm).

**Maturity:** experimental / optional publish (see release runbook `NPM_TOKEN`). Pin versions explicitly when consuming from npm. Package version tracks tools `0.12.x`.

## Install

```bash
npm install @eddiethedean/dtcs-wasm@0.12.0
```

## Surface (intentionally small)

| Export | Arguments | Returns / notes |
|--------|-----------|-----------------|
| `initSync({ module })` | WASM bytes | must call before other APIs |
| `parseDocument(content, format)` | `Uint8Array`/`string`, `"yaml"`\|`"json"` | `{ contract, report }` (Map-like in some hosts) |
| `validateContract(contract)` | plain COM object | `{ diagnostics }` — check severities; does not throw on invalid |
| `conformanceDeclare(profile?)` | optional profile id | capability declaration object |
| `specVersion()` | — | `"2.0.0"` |

**Not included in WASM builds** (size): full `conformanceRun`, plan/optimize/compile/run, `export-portable`. Use the Rust CLI or Python package for certification and execution.

## Parity vs CLI / Python

| Capability | CLI / Python | WASM |
|------------|--------------|------|
| Parse / validate | Yes | Yes |
| Analyze / plan / run / export-portable | Yes (Rust CLI / Python lib) | No |
| `conformance declare` | Yes | Yes |
| `conformance run` | Yes | No |

## Errors

Invalid contracts produce diagnostics in the returned report. Prefer checking diagnostic `severity` rather than expecting thrown exceptions. See [error-taxonomy.md](../user/error-taxonomy.md).

## Build from source

```bash
cd bindings/wasm
rustup target add wasm32-unknown-unknown
npm run build
npm test
```

See the package [README](https://github.com/eddiethedean/dtcs/blob/main/bindings/wasm/README.md).
