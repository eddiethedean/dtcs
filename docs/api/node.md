# Node API

Package: [`@eddiethedean/dtcs`](https://github.com/eddiethedean/dtcs/tree/main/bindings/node) — thin wrapper over [`@eddiethedean/dtcs-wasm`](wasm.md).

**Maturity:** experimental / optional npm publish. Prefer pinning both packages to the same tools version.

## Install

```bash
npm install @eddiethedean/dtcs@0.13.0 @eddiethedean/dtcs-wasm@0.13.0
```

Requires **Node.js with ESM** (`"type": "module"`). The package auto-loads WASM via `initSync` on import.

If the package is not yet on npm for your version, build from the monorepo (`file:../wasm`) — see below.

## Exact exports

| Export | Kind | Notes |
|--------|------|-------|
| `parseDocument(content, format)` | function | WASM parse; `content` is string or bytes |
| `validateContract(contract)` | function | Validate COM → `{ diagnostics }` |
| `conformanceDeclare(profile?)` | function | Capability declaration |
| `specVersion()` | function | Returns `"3.0.0"` |
| `parse(content, format?)` | alias | Encodes string → `parseDocument` (default `"yaml"`) |
| `validate(contract)` | alias | → `validateContract` |
| `conformanceDeclareAll()` | alias | → `conformanceDeclare()` with no profile |
| `SPEC_VERSION` | string const | Same as `specVersion()` at load time |
| `conformanceRun` | throws | Not available — use Rust CLI / Python |

**3.0 surface limits:** parse / validate / `conformanceDeclare` only. No reference runtime, offline `conformanceRun`, or `export-portable`. Use `pip install dtcs` or the Rust `dtcs` CLI for those.

## Map vs plain object

serde-wasm may return nested **`Map`** values. Coerce before treating as plain JSON:

```javascript
function mapToPlain(value) {
  if (value instanceof Map) {
    return Object.fromEntries(
      [...value.entries()].map(([k, v]) => [k, mapToPlain(v)]),
    );
  }
  if (Array.isArray(value)) return value.map(mapToPlain);
  return value;
}
```

## Runnable example

```javascript
import {
  parse,
  validate,
  conformanceDeclareAll,
  SPEC_VERSION,
} from "@eddiethedean/dtcs";

function mapToPlain(value) {
  if (value instanceof Map) {
    return Object.fromEntries(
      [...value.entries()].map(([k, v]) => [k, mapToPlain(v)]),
    );
  }
  if (Array.isArray(value)) return value.map(mapToPlain);
  return value;
}

const yaml = `
dtcsVersion: "3.0.0"
id: "demo.minimal"
name: "Minimal Demo"
version: "0.1.0"
metadata:
  description: "node smoke"
  classification: internal
  governance: { owner: "docs", steward: "docs" }
  provenance: { author: "docs", createdAt: "2026-01-01T00:00:00Z" }
inputs:
  - id: "in"
    schema:
      fields:
        - { name: "v", type: "string", nullable: false }
outputs:
  - id: "out"
    schema:
      fields:
        - { name: "v", type: "string", nullable: false }
lineage:
  mappings:
    - { output: "out", inputs: ["in"] }
`;

const parsed = mapToPlain(parse(yaml, "yaml"));
const report = validate(parsed.contract);
const ok = !report.diagnostics.some((d) => d.severity === "error");
console.log(SPEC_VERSION, ok, conformanceDeclareAll().primaryProfile);
```
## Errors

Validation failures are diagnostics, not thrown errors for ordinary invalid contracts. See [error-taxonomy.md](../user/error-taxonomy.md).

## Develop from the monorepo

```bash
cd bindings/wasm && npm run build
cd ../node && npm install --no-save file:../wasm && npm test
```

## See also

- [wasm.md](wasm.md) · [python.md](python.md) · [rust.md](rust.md)
- Package README: [bindings/node/README.md](https://github.com/eddiethedean/dtcs/blob/main/bindings/node/README.md)
