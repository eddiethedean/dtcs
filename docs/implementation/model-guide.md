# Canonical Object Model Guide

The Canonical Object Model must be derived from [`SPEC.md`](../../SPEC.md).

Root type in [`src/model/contract.rs`](../../src/model/contract.rs):

```rust
pub struct TransformationContract {
    pub dtcs_version: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub metadata: Option<Metadata>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub semantic_actions: Vec<SemanticAction>,
    pub expressions: Vec<Expression>,
    pub functions: Vec<Function>,
    pub rules: Vec<Rule>,
    pub lineage: Option<Lineage>,
    pub versioning: Option<Versioning>,
    pub semantics: Option<TransformationSemantics>,
    pub extensions: IndexMap<String, serde_json::Value>,
}
```

Design rules:

- Prefer explicit structs over loose maps.
- Preserve namespaced extension fields via `extensions` flatten.
- Do not add execution behavior to the model.
- Keep spec terminology in Rust names where practical.
- If this sketch conflicts with `SPEC.md`, follow `SPEC.md`.

## Lineage

Every valid contract with outputs must declare `lineage.mappings` tracing each output to one or more inputs. See [`examples/customer_normalize.dtcs.yaml`](../../examples/customer_normalize.dtcs.yaml).

## Field references

When the same field name appears in multiple interface schemas, use qualified targets such as `customer_raw.email`.

## Registry

Identifier catalogs use `RegistryDocument` and `RegistryEntry` (SPEC Chapters 21–22).
Contract-level references to external catalogs use `RegistryRef`.
See [`src/registry/`](../../src/registry/) for resolution, loading, and the embedded catalog.
