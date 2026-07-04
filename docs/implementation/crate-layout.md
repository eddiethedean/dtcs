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
      interfaces.rs
      types.rs
      references.rs
      semantics.rs
      extensions.rs
    metadata/
      mod.rs
      validate.rs
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
      types.rs
      report.rs
      compare.rs
      classify.rs
      evolution.rs
    versioning/
      mod.rs
      validate.rs
    lineage/
      mod.rs
      analysis.rs
    registry/
      mod.rs
      builtin.rs
      load.rs
      cache.rs
    plan/
      mod.rs
    cli/
      mod.rs
    bin/
      dtcs.rs
  python/
    dtcs/
  tests/
    fixtures/
      compatibility/
      registry/
    fixture_expectations.json
    mvp.rs
    phase_0_2.rs
    phase_0_3.rs
    phase_0_4.rs
    manifest.rs
```

Keep modules small and aligned with `SPEC.md`.
