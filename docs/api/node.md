# Node API

Package: [`@eddiethedean/dtcs`](https://github.com/eddiethedean/dtcs/tree/main/bindings/node) — thin wrapper over [`@eddiethedean/dtcs-wasm`](wasm.md).

**Maturity:** experimental / optional publish. Prefer pinning `@eddiethedean/dtcs@0.13.0` and the matching WASM package.

## Install

```bash
npm install @eddiethedean/dtcs@0.13.0 @eddiethedean/dtcs-wasm@0.13.0
```

The Node package depends on the WASM package; install both when developing from the monorepo or when your package manager does not pull nested deps as expected.

## Surface

| Export | Notes |
|--------|-------|
| `parseDocument(content, format)` | Parse YAML/JSON |
| `validateContract(contract)` | Validate COM object → diagnostics report |
| `conformanceDeclare(profile?)` | Capability declaration |
| `conformanceDeclareAll` | Alias / helper in some builds — prefer `conformanceDeclare` in new code |
| `specVersion` / `SPEC_VERSION` | Spec string `"3.0.0"` (naming differs by export style; both may exist) |

**3.0 surface limits:** same as WASM — **parse / validate / `conformanceDeclare` only**. No full Spec 3.0 reference runtime, offline `conformanceRun`, or `export-portable` in the npm package. Use `pip install dtcs` or the Rust `dtcs` CLI for those.

## Errors

Same as WASM: validation failures are diagnostics, not thrown errors for ordinary invalid contracts. See [error-taxonomy.md](../user/error-taxonomy.md).

## Develop from the monorepo

```bash
cd bindings/wasm && npm run build
cd ../node && npm install --no-save file:../wasm && npm test
```

## See also

- [wasm.md](wasm.md) · [python.md](python.md) · [rust.md](rust.md)
- Package README: [bindings/node/README.md](https://github.com/eddiethedean/dtcs/blob/main/bindings/node/README.md)
