# Proposed Crate Layout

Recommended layout (implemented as a scaffold in this repository):

```text
dtcs/
  Cargo.toml
  SPEC.md              # repository root — authoritative spec
  docs/
    editorial/         # specification authoring process
    implementation/    # this guide set
  examples/
  tests/
  src/
    lib.rs
    model/
      mod.rs
      contract.rs
      metadata.rs
      interface.rs
      types.rs
      semantics.rs
      expression.rs
      action.rs
      function.rs
      rule.rs
      lineage.rs
      versioning.rs
      extension.rs
      registry.rs
    parser/
      mod.rs
      yaml.rs
      json.rs
    validation/
      mod.rs
      phases.rs
      field_index.rs
      lineage.rs
      structural.rs
      types.rs
      references.rs
      semantics.rs
      extensions.rs
    diagnostics/
      mod.rs
      codes.rs
      diagnostic.rs
      severity.rs
      category.rs
      stage.rs
      report.rs
    compatibility/
      mod.rs
    plan/
      mod.rs
    cli/
      mod.rs
    bin/
      dtcs.rs
  examples/
  tests/
    fixtures/
```

Keep modules small and aligned with `SPEC.md`.
