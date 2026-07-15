# Canonical Object Model Guide

The Canonical Object Model must be derived from [`SPEC.md`](../SPEC.md).

Root type in [`src/model/contract.rs`](https://github.com/eddiethedean/dtcs/blob/main/src/model/contract.rs):

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
    pub guarantees: Option<ContractGuarantees>,
    pub compatibility: Option<CompatibilityDeclaration>,
    pub versioning: Option<Versioning>,
    pub semantics: Option<TransformationSemantics>,
    pub extensions: IndexMap<String, serde_json::Value>,
}
```

Additional COM modules (0.11): `guarantees.rs`, `compatibility_decl.rs`, `null_behavior.rs`. `SemanticAction` / `Expression` / `Function` / `Rule` preserve nested vendor extensions via `extensions` flatten. Actions carry optional `parameters`.

Design rules:

- Prefer explicit structs over loose maps.
- Preserve namespaced extension fields via `extensions` flatten (contract and nested objects).
- Do not add execution behavior to the model.
- Keep spec terminology in Rust names where practical.
- If this sketch conflicts with `SPEC.md`, follow `SPEC.md`.

## Lineage

Every valid contract with outputs must declare `lineage.mappings` tracing each output to one or more inputs. Each mapping may include optional `id`, `operation` (default `dtcs:derive`), and `flow` (`preserved` | `derived` | `aggregated` | `filtered` | `partitioned` | `discarded`). See [`examples/customer_normalize.dtcs.yaml`](https://github.com/eddiethedean/dtcs/blob/main/examples/customer_normalize.dtcs.yaml) and [SPEC Appendix A.6](../SPEC.md#a6-lineage-mapping-fields).

## Field references

When the same field name appears in multiple interface schemas, use qualified targets such as `customer_raw.email`.

## Registry

Identifier catalogs use `RegistryDocument` and `RegistryEntry` (SPEC Chapters 21–22).
Contract-level references to external catalogs use `RegistryRef`.
See [`src/registry/`](https://github.com/eddiethedean/dtcs/blob/main/src/registry/) for resolution, loading, and the embedded catalog.
