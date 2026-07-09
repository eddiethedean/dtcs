# @eddiethedean/dtcs (Node)

Node.js bindings for DTCS, implemented as a thin wrapper over `@eddiethedean/dtcs-wasm`.

## Install

```bash
npm install @eddiethedean/dtcs @eddiethedean/dtcs-wasm
```

## Usage

```javascript
import { parse, validate, conformanceDeclareAll, SPEC_VERSION } from "@eddiethedean/dtcs";

const result = parse(yamlText, "yaml");
const report = validate(result.contract);
const declaration = conformanceDeclareAll();
```

Offline `conformanceRun` is not available in this package; use `dtcs conformance run` from the Rust CLI for certification.
