# @eddiethedean/dtcs-wasm

WebAssembly bindings for the DTCS reference implementation (tools `0.13.x` / Spec `3.0.0`).

**Maturity:** experimental. Prefer `npm install @eddiethedean/dtcs-wasm@0.13.0`.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
npm run build
```

## API

Call `initSync({ module })` with the `.wasm` bytes before other exports.

- `parseDocument(content: Uint8Array | string, format: "yaml" | "json")`
- `validateContract(contract: object)`
- `conformanceDeclare(profile?: string)`
- `specVersion()` → `"3.0.0"`

Nested values may deserialize as ES `Map` — coerce to plain objects before using property access.

Full offline `conformanceRun` is not included in WASM builds due to binary size; use the Rust CLI or Python package for certification runs.

Docs: [docs/api/wasm.md](https://github.com/eddiethedean/dtcs/blob/main/docs/api/wasm.md). Prefer the Node wrapper [`@eddiethedean/dtcs`](../node/) for automatic WASM init.
