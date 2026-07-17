# WASM API

Package: [`@eddiethedean/dtcs-wasm`](https://github.com/eddiethedean/dtcs/tree/main/bindings/wasm).

**Maturity:** experimental / optional publish (see release runbook `NPM_TOKEN`). Pin versions explicitly when consuming from npm. Package version tracks tools `0.13.x`.

## Install

```bash
npm install @eddiethedean/dtcs-wasm@0.13.0
```

For browsers or custom hosts you must load the `.wasm` bytes and call `initSync` before other APIs. The Node wrapper ([node.md](node.md)) does this for you.

## Surface (intentionally small)

| Export | Arguments | Returns / notes |
|--------|-----------|-----------------|
| `initSync({ module })` | WASM bytes (`Uint8Array` / `Buffer`) | must call before other APIs |
| `parseDocument(content, format)` | `Uint8Array`/`string`, `"yaml"`\|`"json"` | `{ contract, report }` (may be `Map`-like) |
| `validateContract(contract)` | plain COM object | `{ diagnostics }` — check severities; does not throw on invalid |
| `conformanceDeclare(profile?)` | optional profile id | capability declaration object |
| `specVersion()` | — | `"3.0.0"` |

**3.0 surface limits:** parse / validate / `conformanceDeclare` only — no reference runtime, plan export, optimize/compile/run, or offline `conformanceRun`. Use the Rust CLI or Python package for Rich Portable Analytics execution and certification.

## Map vs plain object

Hosts may deserialize objects as ES `Map`. Coerce with a recursive `mapToPlain` helper (see [node.md](node.md)) before reading fields as plain properties.

## Node bootstrap (after `npm run build` in `bindings/wasm`)

```javascript
import { readFileSync } from "node:fs";
import {
  initSync,
  parseDocument,
  validateContract,
  specVersion,
} from "./pkg/dtcs_wasm.js";

initSync({ module: readFileSync("./pkg/dtcs_wasm_bg.wasm") });
console.log(specVersion()); // "3.0.0"

const yaml = `dtcsVersion: "3.0.0"
id: "demo.minimal"
name: "Minimal"
version: "0.1.0"
metadata:
  description: "wasm smoke"
  classification: internal
  governance: { owner: "docs", steward: "docs" }
  provenance: { author: "docs", createdAt: "2026-01-01T00:00:00Z" }
inputs:
  - id: "in"
    schema:
      fields: [{ name: "v", type: "string", nullable: false }]
outputs:
  - id: "out"
    schema:
      fields: [{ name: "v", type: "string", nullable: false }]
lineage:
  mappings: [{ output: "out", inputs: ["in"] }]
`;
const parsed = parseDocument(new TextEncoder().encode(yaml), "yaml");
```

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
cargo install wasm-pack   # once
npm run build             # or: npx wasm-pack build --target web --out-dir pkg
npm test
```

See the package [README](https://github.com/eddiethedean/dtcs/blob/main/bindings/wasm/README.md).
