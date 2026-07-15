# JSON Output Reference

All CLI commands support `--json` for machine-readable output. Field names use **camelCase** in JSON (matching the Canonical Object Model serialization).

## validate

```bash
dtcs validate contract.yaml --json
```

```json
{
  "valid": true,
  "diagnostics": []
}
```

When invalid, `valid` is `false` and `diagnostics` contains error entries.

## diagnostics

```bash
dtcs diagnostics contract.yaml --json
```

```json
{
  "diagnostics": []
}
```

`diagnostics` always emits the diagnostics array. It does not include a `valid` field.

## inspect

```json
{
  "id": "customer.normalize",
  "name": "Normalize Customer",
  "version": "0.2.0",
  "dtcsVersion": "1.0.0",
  "inputs": 1,
  "outputs": 1,
  "semanticActions": 1,
  "rules": 0,
  "expressions": 0,
  "functions": 0
}
```

## analyze

```bash
dtcs analyze contract.yaml --json
```

```json
{
  "validation": {
    "valid": true,
    "diagnostics": []
  },
  "analysis": {
    "valid": true,
    "diagnostics": [],
    "findings": []
  }
}
```

## compat

```json
{
  "level": "backwardCompatible",
  "aspects": [
    {
      "aspect": "interfaces",
      "status": "changed",
      "message": "input 'aux' added in target"
    }
  ],
  "diagnostics": [
    {
      "id": "dtcs:conditional-compatibility",
      "severity": "warning",
      "stage": "analysis",
      "category": "compatibility",
      "message": "input 'aux' added in target",
      "objectRef": "inputs.aux"
    }
  ]
}
```

`level` values: `identical`, `backwardCompatible`, `forwardCompatible`, `conditionallyCompatible`, `incompatible`.

## evolve

```json
{
  "sourceId": "evolution.sample",
  "targetId": "evolution.sample",
  "sameIdentity": true,
  "compatibility": "backwardCompatible",
  "changes": [
    {
      "category": "interface",
      "message": "input 'aux' added in target",
      "objectRef": "inputs.aux"
    }
  ],
  "migrationHints": [
    "Newer revision is backward compatible; downstream consumers may adopt without input changes."
  ],
  "diagnostics": []
}
```

## lineage

```json
{
  "graph": [
    {
      "output": "customer_summary",
      "inputs": ["customers"]
    },
    {
      "output": "order_enriched",
      "inputs": ["orders", "customers"]
    }
  ],
  "impact": {
    "input": "customers",
    "outputs": ["customer_summary", "order_enriched"]
  },
  "dependency": null,
  "governance": {
    "owner": "data-platform",
    "steward": "analytics"
  },
  "diagnostics": []
}
```

`impact` and `dependency` are populated when `--impact` or `--dependency` flags are used.

## plan

```bash
dtcs plan contract.yaml --json
```

On success, emits the transformation plan document directly (not wrapped in a result envelope):

```json
{
  "identity": {
    "dtcsVersion": "1.0.0",
    "id": "customer.normalize",
    "name": "Normalize Customer",
    "version": "0.2.0"
  },
  "inputs": [],
  "outputs": [],
  "nodes": [],
  "dependencies": [],
  "lineage": { "mappings": [] },
  "guarantees": {}
}
```

On failure, emits a diagnostics envelope (same shape as `diagnostics --json`):

```json
{
  "diagnostics": [
    {
      "id": "dtcs:missing-lineage",
      "severity": "error",
      "stage": "validation",
      "category": "lineage",
      "message": "output 'out' has no lineage mapping"
    }
  ]
}
```

## optimize

```bash
dtcs optimize contract.yaml --json
```

On success, emits an optimization result envelope:

```json
{
  "plan": { },
  "diagnostics": [],
  "transforms": [
    {
      "pass": "expression",
      "nodeId": "const_add",
      "description": "rewrote expression '1 + 2' to '3'"
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `plan` | object (optional) | Optimized transformation plan when optimization succeeded |
| `diagnostics` | array | Optimization and validation diagnostics |
| `transforms` | array | Informational log of applied rewrites |

When `--plan` is set, the input path is serialized plan JSON rather than a contract file.

## match

Success emits a capability match report:

```json
{
  "supported": true,
  "diagnostics": [],
  "gaps": []
}
```

Failure emits a diagnostics envelope (`{"diagnostics": [...]}`), same shape as `diagnostics --json`.

`gaps` entries describe unsupported plan requirements when `supported` is `false`:

```json
{
  "supported": false,
  "diagnostics": [],
  "gaps": [
    {
      "category": "semanticAction",
      "requirement": "dtcs:unknown_action",
      "message": "engine does not support semantic action"
    }
  ]
}
```

## compile

```bash
dtcs compile contract.yaml --json
```

Success emits an execution plan document (same shape as the Rust `ExecutionPlan` type). See [`tests/fixtures/execution_plans/valid_customer.exec.json`](https://github.com/eddiethedean/dtcs/blob/main/tests/fixtures/execution_plans/valid_customer.exec.json) for a full example.

```json
{
  "target": {
    "engineId": "dtcs:reference",
    "engineVersion": "0.11.0",
    "capabilityVersion": "1.0.0"
  },
  "identity": {
    "dtcsVersion": "1.0.0",
    "id": "customer.normalize",
    "name": "Normalize Customer",
    "version": "0.2.0"
  },
  "inputs": [],
  "outputs": [],
  "nodes": [],
  "steps": [],
  "guarantees": {},
  "lineage": { "mappings": [] }
}
```

Failure emits a diagnostics envelope (`{"diagnostics": [...]}`).

## run

```bash
dtcs run contract.yaml --input inputs.json --json
```

**CLI:** success emits output datasets keyed by output interface id (flat map, not wrapped):

```json
{
  "customer_clean": [
    {
      "customer_id": "1",
      "email": "alice@example.com"
    }
  ]
}
```

**Python API:** `dtcs.runtime_execute(execution_plan, inputs)` returns an envelope with both outputs and diagnostics:

```json
{
  "outputs": {
    "customer_clean": [
      {
        "customer_id": "1",
        "email": "alice@example.com"
      }
    ]
  },
  "diagnostics": []
}
```

Failure emits a diagnostics envelope (`{"diagnostics": [...]}`) for both CLI and Python.

## registry

### registry list

```bash
dtcs registry list --json
dtcs registry list --registry vendor_catalog.yaml --json
```

Success emits an array of registry entries:

```json
[
  {
    "id": "dtcs:lowercase",
    "name": "Lowercase",
    "category": "semanticAction",
    "version": "1.0.0",
    "status": "stable",
    "supported": true
  }
]
```

### registry resolve

```bash
dtcs registry resolve dtcs:lowercase --json
```

Success emits a single registry entry (same object shape as list items, may include a `definition` field). When the identifier is not found, the command exits non-zero and prints a diagnostic message (no JSON entry object).

## conformance declare

```json
{
  "implementationId": "dtcs:reference",
  "implementationVersion": "0.11.0",
  "dtcsVersion": "1.0.0-draft",
  "primaryProfile": "integrated-platform",
  "profiles": [
    {
      "id": "integrated-platform",
      "implementationClass": "integratedPlatform",
      "dtcsVersion": "1.0.0-draft",
      "implementationVersion": "0.11.0",
      "supportedRegistries": ["dtcs:builtin"],
      "supportedExtensions": ["acme"],
      "optionalCapabilities": ["planOptimization", "referenceRuntime"]
    }
  ]
}
```

## conformance run

```json
{
  "implementationId": "dtcs:reference",
  "implementationVersion": "0.11.0",
  "profiles": ["integrated-platform"],
  "results": [
    {
      "id": "parse-valid-customer",
      "profile": "integrated-platform",
      "passed": true
    }
  ],
  "security": [
    {
      "id": "registry-trust",
      "profile": "security",
      "passed": true
    }
  ],
  "passed": true
}
```

## version

```json
{
  "crateVersion": "0.11.0",
  "specVersion": "1.0.0-draft"
}
```

## Diagnostic object

Shared across validate, compat, and evolve:

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Diagnostic code (e.g. `dtcs:missing-lineage`) |
| `severity` | string | `error`, `warning`, or `information` |
| `stage` | string | Pipeline stage (`parse`, `validation`, `analysis`, …) |
| `category` | string | Category (`syntax`, `structure`, `type`, `compatibility`, …) |
| `message` | string | Human-readable description |
| `objectRef` | string (optional) | Path to the affected object |
| `remediation` | string (optional) | Suggested fix |

Only **error**-severity diagnostics cause `validate` to exit non-zero.

## Python note

Python API functions return dicts with the same camelCase keys as CLI JSON output. When constructing contract dicts for `validate()` or `compat_analyze()`, use camelCase keys (`dtcsVersion`, not `dtcs_version`).

See [diagnostics-guide.md](../implementation/diagnostics-guide.md) for the full list of diagnostic codes.

## Next steps

- CLI commands: [cli-guide.md](cli-guide.md)
- Library API: [public-api.md](../implementation/public-api.md)
- Troubleshooting: [troubleshooting.md](troubleshooting.md)
