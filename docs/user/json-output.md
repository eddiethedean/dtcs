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

Same shape as `validate` — always includes the diagnostics array.

## inspect

```json
{
  "id": "customer.normalize",
  "name": "Normalize Customer",
  "version": "0.2.0",
  "dtcsVersion": "1.0.0",
  "inputCount": 1,
  "outputCount": 1,
  "semanticActionCount": 1
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

## version

```json
{
  "crateVersion": "0.3.0",
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
