# Node API

Package: [`@eddiethedean/dtcs`](https://github.com/eddiethedean/dtcs/tree/main/bindings/node) — thin wrapper over [`@eddiethedean/dtcs-wasm`](wasm.md).

## Surface

Exports mirror the WASM module with Node-friendly names (see package README). Expect:

- parse / validate helpers
- `conformanceDeclare`
- `specVersion`

There is **no** full offline `conformanceRun` or reference runtime in the npm package. Use `pip install dtcs` or the `dtcs` CLI for those.

## Install (from published package)

```bash
npm install @eddiethedean/dtcs
```

When developing from the monorepo:

```bash
cd bindings/wasm && npm run build
cd ../node && npm install && npm test
```

## See also

- [wasm.md](wasm.md) · [python.md](python.md) · [rust.md](rust.md)
- Package README: [bindings/node/README.md](https://github.com/eddiethedean/dtcs/blob/main/bindings/node/README.md)
