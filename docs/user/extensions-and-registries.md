# Vendor extensions and registries

How to add non-`dtcs:` identifiers without breaking validation.

## Rules of thumb

1. **Never invent new `dtcs:` IDs** in vendor catalogs — builtin entries are authoritative.
2. Use a **vendor namespace** (for example `acme:normalize_phone`).
3. Load vendor catalogs with `--registry` / `validate_with_registry` / `registry_path=`.
4. Mark capabilities you cannot guarantee as optional; unsupported **mandatory** extensions fail validation (`dtcs:unsupported-extension`).

## Minimal vendor registry sketch

```yaml
id: "acme:registry"
version: "1.0.0"
governingSpecification: "2.0.0"
publicationStatus: "experimental"
entries:
  acme:normalize_phone:
    name: "Normalize phone"
    category: "semanticAction"
    version: "1.0.0"
    status: "experimental"
```

```bash
dtcs validate contract.dtcs.yaml --registry ./acme-registry.yaml
dtcs registry resolve acme:normalize_phone --registry ./acme-registry.yaml
```

## Contract extensions

Contracts may carry nested extension objects. Unsupported **required** extensions are rejected; optional extensions may be ignored by the reference tools.

## Engine profiles

Engines claim support via capability profiles consumed by `dtcs match`. The embedded profile is `dtcs:reference`. Spark/SQL/Polars backends are **out of scope** for this repository — publish your own profile document if you implement one.

## Governance pointers

- CONTRIBUTING.md (registry / extension review)
- SPEC Chapters 21–22 and 26
