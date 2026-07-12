//! YAML ↔ JSON format equivalence (SPEC Chapter 3).

mod common;

use std::fs;
use std::path::PathBuf;

use dtcs::{parse, plan, validate, DocumentFormat};

const PAIRS: &[(&str, &str)] = &[
    ("valid_minimal.yaml", "valid_minimal.json"),
    ("valid_metadata.yaml", "valid_metadata.json"),
    ("plan_field_write_chain.yaml", "plan_field_write_chain.json"),
    ("lineage_multi.yaml", "lineage_multi.json"),
];

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
fn yaml_and_json_pairs_validate_equivalently() {
    for (yaml_name, json_name) in PAIRS {
        let yaml = load_contract(yaml_name, DocumentFormat::Yaml);
        let json = load_contract(json_name, DocumentFormat::Json);

        assert_eq!(yaml.id, json.id, "{yaml_name} vs {json_name}");
        assert_eq!(yaml.inputs.len(), json.inputs.len(), "{yaml_name}");
        assert_eq!(yaml.outputs.len(), json.outputs.len(), "{yaml_name}");

        let yaml_report = validate(&yaml);
        let json_report = validate(&json);
        assert_eq!(
            yaml_report.is_valid(),
            json_report.is_valid(),
            "{yaml_name}"
        );
        assert_eq!(
            common::diagnostic_code_multiset(&yaml_report.diagnostics),
            common::diagnostic_code_multiset(&json_report.diagnostics),
            "{yaml_name}"
        );
    }
}

#[test]
fn yaml_and_json_pairs_plans_are_equivalent() {
    for (yaml_name, json_name) in PAIRS {
        let yaml = load_contract(yaml_name, DocumentFormat::Yaml);
        let json = load_contract(json_name, DocumentFormat::Json);

        let yaml_plan = plan::lower(&yaml, None, None).plan.expect("yaml plan");
        let json_plan = plan::lower(&json, None, None).plan.expect("json plan");
        assert!(plan::equivalent(&yaml_plan, &json_plan), "{yaml_name}");
    }
}

#[test]
fn yaml_and_json_pairs_optimize_equivalently() {
    for (yaml_name, json_name) in PAIRS {
        let yaml = load_contract(yaml_name, DocumentFormat::Yaml);
        let json = load_contract(json_name, DocumentFormat::Json);

        let yaml_plan = plan::lower(&yaml, None, None).plan.expect("yaml plan");
        let json_plan = plan::lower(&json, None, None).plan.expect("json plan");

        let yaml_opt = plan::optimize(&yaml_plan).plan.expect("yaml optimized");
        let json_opt = plan::optimize(&json_plan).plan.expect("json optimized");
        assert!(plan::equivalent(&yaml_opt, &json_opt), "{yaml_name}");
    }
}
