//! YAML ↔ JSON format equivalence (SPEC Chapter 3).

mod common;

use std::fs;
use std::path::PathBuf;

use dtcs::{parse, plan, validate, DocumentFormat};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn load_contract(name: &str, format: DocumentFormat) -> dtcs::TransformationContract {
    let content = fs::read(fixture(name)).expect("read fixture");
    parse(&content, format).into_contract().expect("contract")
}

#[test]
fn yaml_and_json_minimal_contracts_validate_equivalently() {
    let yaml = load_contract("valid_minimal.yaml", DocumentFormat::Yaml);
    let json = load_contract("valid_minimal.json", DocumentFormat::Json);

    assert_eq!(yaml.id, json.id);
    assert_eq!(yaml.inputs.len(), json.inputs.len());
    assert_eq!(yaml.outputs.len(), json.outputs.len());
    assert_eq!(
        yaml.lineage.as_ref().unwrap().mappings.len(),
        json.lineage.as_ref().unwrap().mappings.len()
    );

    let yaml_report = validate(&yaml);
    let json_report = validate(&json);
    assert!(yaml_report.is_valid());
    assert!(json_report.is_valid());
    assert_eq!(
        common::diagnostic_code_multiset(&yaml_report.diagnostics),
        common::diagnostic_code_multiset(&json_report.diagnostics)
    );
}

#[test]
fn yaml_and_json_minimal_plans_are_equivalent() {
    let yaml = load_contract("valid_minimal.yaml", DocumentFormat::Yaml);
    let json = load_contract("valid_minimal.json", DocumentFormat::Json);

    let yaml_plan = plan::lower(&yaml, None, None).plan.expect("yaml plan");
    let json_plan = plan::lower(&json, None, None).plan.expect("json plan");
    assert!(plan::equivalent(&yaml_plan, &json_plan));
}

#[test]
fn yaml_and_json_minimal_optimize_equivalently() {
    let yaml = load_contract("valid_minimal.yaml", DocumentFormat::Yaml);
    let json = load_contract("valid_minimal.json", DocumentFormat::Json);

    let yaml_plan = plan::lower(&yaml, None, None).plan.expect("yaml plan");
    let json_plan = plan::lower(&json, None, None).plan.expect("json plan");

    let yaml_opt = plan::optimize(&yaml_plan).plan.expect("yaml optimized");
    let json_opt = plan::optimize(&json_plan).plan.expect("json optimized");
    assert!(plan::equivalent(&yaml_opt, &json_opt));
}
