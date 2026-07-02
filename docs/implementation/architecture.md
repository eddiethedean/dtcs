# Architecture

Follow the architecture defined in `SPEC.md`.

Initial implementation pipeline:

```text
DTCS Document
        │
        ▼
Parser
        │
        ▼
Canonical Object Model
        │
        ▼
Validator
        │
        ▼
Diagnostics
```

Future pipeline:

```text
Transformation Contract
        │
        ▼
Canonical Object Model
        │
        ▼
Transformation Plan
        │
        ▼
Execution Plan
        │
        ▼
Runtime
```

For this first crate, implement only through Diagnostics.
