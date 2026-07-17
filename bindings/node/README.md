# @eddiethedean/dtcs (Node)

Node.js bindings for DTCS, implemented as a thin wrapper over `@eddiethedean/dtcs-wasm`.

**Maturity:** experimental. Pin both packages to the same tools version.

## Install

```bash
npm install @eddiethedean/dtcs@0.13.0 @eddiethedean/dtcs-wasm@0.13.0
```

Requires Node ESM. Import initializes WASM automatically.

## Usage

```javascript
import {
  parse,
  validate,
  conformanceDeclareAll,
  SPEC_VERSION,
} from "@eddiethedean/dtcs";

const yaml = `
dtcsVersion: "3.0.0"
id: "demo.minimal"
name: "Minimal Demo"
version: "0.1.0"
metadata:
  description: "node demo"
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

const result = parse(yaml, "yaml");
// serde may return Map — coerce nested Maps to plain objects if needed
const report = validate(result.contract);
console.log(SPEC_VERSION, report.diagnostics);
const declaration = conformanceDeclareAll();
```

### Exports

`parseDocument`, `validateContract`, `conformanceDeclare`, `specVersion`, plus aliases `parse`, `validate`, `conformanceDeclareAll`, `SPEC_VERSION`. `conformanceRun` throws (use the Rust CLI / Python package).

Offline `conformanceRun` is not available in this package; use `dtcs conformance run` from the Rust CLI for certification.

Full docs: [docs/api/node.md](https://github.com/eddiethedean/dtcs/blob/main/docs/api/node.md).
