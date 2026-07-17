# Diagnostic catalog (adopter)

Common diagnostic codes you will see from `dtcs diagnostics`, `validate`, and related commands. Normative/implementer detail: [diagnostics-guide.md](../implementation/diagnostics-guide.md).

| Code | Typical meaning | First fix |
|------|-----------------|-----------|
| `dtcs:unsupported-version` | `dtcsVersion` not in the supported set | Set `"2.0.0"` (preferred) or `"1.0.0"` / `"1.0.0-draft"` — [versioning.md](versioning.md) |
| `dtcs:missing-lineage` | Output lacks a lineage mapping | Add `lineage.mappings` for each output |
| `dtcs:unresolved-reference` | Field path does not resolve | Use `interface.field` matching an input/output `id` |
| `dtcs:ambiguous-reference` | Path resolves in more than one way | Disambiguate interface or field names |
| `dtcs:invalid-type` | Malformed type expression | e.g. `list<string>` not bare `list` |
| `dtcs:unknown-registry-entry` | Unknown `dtcs:` / vendor id | `dtcs registry list` / `resolve`; supply `--registry` for vendors |
| `dtcs:invalid-semantic-action` | Action shape/target invalid | Supply required `parameters`; match target type/nullability |
| `dtcs:missing-inputs` | No inputs declared | Add at least one input interface |
| `dtcs:duplicate-identifier` | Duplicate ids in COM | Make `id` values unique in scope |

Run:

```bash
dtcs diagnostics contract.yaml --json
```

For a full machine list of codes and remediations, see the implementer diagnostics guide and `dtcs registry` for stdlib identifiers.

## Related

- [error-taxonomy.md](error-taxonomy.md)
- [troubleshooting.md](troubleshooting.md)
- [writing-contracts.md](writing-contracts.md#validate-as-you-write)
