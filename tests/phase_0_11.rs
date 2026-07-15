//! Phase 0.11 SPEC completeness integration tests.

use std::fs;
use std::path::PathBuf;

use dtcs::{parse, parse_and_validate, DocumentFormat, InformationFlow, NullBehavior};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn read(name: &str) -> Vec<u8> {
    fs::read(fixture(name)).unwrap_or_else(|e| panic!("read {name}: {e}"))
}

#[test]
fn lineage_defaults_operation_and_flow() {
    let report = parse_and_validate(&read("valid_customer.yaml"), DocumentFormat::Yaml);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    let parsed = parse(&read("valid_customer.yaml"), DocumentFormat::Yaml);
    let contract = parsed.contract.expect("contract");
    let mapping = &contract.lineage.as_ref().unwrap().mappings[0];
    assert_eq!(mapping.operation, "dtcs:derive");
    assert_eq!(mapping.flow, InformationFlow::Derived);
}

#[test]
fn nested_vendor_extension_on_action_is_preserved() {
    let yaml = br#"
dtcsVersion: "1.0.0"
id: "ext.action"
name: "Ext Action"
version: "0.1.0"
inputs:
  - id: "in"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false
outputs:
  - id: "out"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false
semanticActions:
  - id: "act"
    action: "dtcs:trim"
    target: "in.value"
    acme:tag: "keep"
lineage:
  mappings:
    - output: "out"
      inputs: ["in"]
      operation: "dtcs:derive"
      flow: preserved
"#;
    let parsed = parse(yaml, DocumentFormat::Yaml);
    let contract = parsed.contract.expect("contract");
    assert!(
        contract.semantic_actions[0]
            .extensions
            .contains_key("acme:tag"),
        "expected nested extension preservation"
    );
    let report = dtcs::validate(&contract);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn contract_guarantees_and_compatibility_parse() {
    let yaml = br#"
dtcsVersion: "1.0.0"
id: "g.compat"
name: "Guarantees"
version: "0.1.0"
guarantees:
  informationLoss: "none"
  statements: ["row count preserved"]
compatibility:
  policy: "dtcs:default"
  backward: true
inputs:
  - id: "in"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false
outputs:
  - id: "out"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false
lineage:
  mappings:
    - output: "out"
      inputs: ["in"]
"#;
    let parsed = parse(yaml, DocumentFormat::Yaml);
    let contract = parsed.contract.expect("contract");
    assert!(contract.guarantees.is_some());
    assert_eq!(
        contract.compatibility.as_ref().unwrap().backward,
        Some(true)
    );
}

#[test]
fn expression_null_behavior_and_determinism_fields() {
    let yaml = br#"
dtcsVersion: "1.0.0"
id: "expr.null"
name: "Expr Null"
version: "0.1.0"
inputs:
  - id: "in"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: true
outputs:
  - id: "out"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: true
expressions:
  - id: "e1"
    expr: "in.value"
    type: "string"
    nullBehavior: propagate
    deterministic: true
lineage:
  mappings:
    - output: "out"
      inputs: ["in"]
"#;
    let parsed = parse(yaml, DocumentFormat::Yaml);
    let contract = parsed.contract.expect("contract");
    assert_eq!(
        contract.expressions[0].null_behavior,
        Some(NullBehavior::Propagate)
    );
}

#[test]
fn registry_includes_dataset_actions() {
    assert!(dtcs::is_known_action("dtcs:project"));
    assert!(dtcs::is_known_action("dtcs:join"));
    assert!(dtcs::is_known_action("dtcs:filter"));
    assert!(dtcs::is_known_function("dtcs:abs"));
    assert!(dtcs::is_known_function("dtcs:is_missing"));
    assert!(dtcs::is_known_rule("dtcs:one_of"));
}

#[test]
fn capability_validate_rejects_unknown_dtcs_action() {
    let mut profile = dtcs::reference_profile();
    profile
        .categories
        .semantic_actions
        .push("dtcs:not_a_real_action".into());
    let report = dtcs::validate_capabilities(&profile);
    assert!(
        !report.is_valid(),
        "expected invalid capability for unknown action"
    );
}

#[test]
fn conformance_analyzer_assertions_pass() {
    let report = dtcs::conformance::run_for_profiles(
        Some(&["analyzer".into()]),
        &dtcs::conformance::default_fixtures_dir(),
    );
    assert!(
        report.is_valid(),
        "analyzer conformance failed: {:?}",
        report
            .results
            .iter()
            .filter(|r| !r.passed)
            .collect::<Vec<_>>()
    );
}
