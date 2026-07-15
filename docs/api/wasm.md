# WASM API

Package: [`@eddiethedean/dtcs-wasm`](https://github.com/eddiethedean/dtcs/tree/main/bindings/wasm).

## Surface (intentionally small)

| Export | Purpose |
|--------|---------|
| `parseDocument(content, format)` | Parse YAML/JSON bytes |
| `validateContract(contract)` | Validate a parsed object |
| `conformanceDeclare(profile?)` | Capability declaration JSON |
| `specVersion()` | Spec version string |

**Not included in WASM builds** (size): full `conformanceRun`, plan/optimize/compile/run pipeline. Use the Rust CLI or Python package for certification and execution.

## Parity vs CLI / Python

| Capability | CLI / Python | WASM |
|------------|--------------|------|
| Parse / validate | Yes | Yes |
| Analyze / plan / run | Yes | No |
| `conformance declare` | Yes | Yes |
| `conformance run` | Yes | No |

## Build from source

```bash
cd bindings/wasm
rustup target add wasm32-unknown-unknown
npm run build
npm test
```

See the package [README](https://github.com/eddiethedean/dtcs/blob/main/bindings/wasm/README.md).
