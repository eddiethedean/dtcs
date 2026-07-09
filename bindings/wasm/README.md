# @eddiethedean/dtcs-wasm

WebAssembly bindings for the DTCS reference implementation.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
npm run build
```

## API

- `parseDocument(content: Uint8Array, format: "yaml" | "json")`
- `validateContract(contract: object)`
- `conformanceDeclare(profile?: string)`
- `specVersion()`

Full offline `conformanceRun` is not included in WASM builds due to binary size; use the Rust CLI or Python package for certification runs.
