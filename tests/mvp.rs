//! Expanded integration tests for audit fixes.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::{
    codes, parse, parse_and_validate, parse_logical_type, DocumentFormat, ParseResult, Severity,
    SPEC_VERSION,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn parse_fixture(name: &str) -> ParseResult {
    let content = fs::read(fixture(name)).expect("read fixture");
    let format = if name.ends_with(".json") {
        DocumentFormat::Json
    } else {
        DocumentFormat::Yaml
    };
    parse(&content, format)
}

fn dtcs_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dtcs"))
}

#[test]
fn spec_version_is_set() {
    assert_eq!(SPEC_VERSION, "2.0.0");
}

#[test]
fn parses_valid_yaml_example() {
    let result = parse_fixture("valid_customer.yaml");
    assert!(result.report.is_valid());
    let contract = result.contract.expect("contract");
    assert_eq!(contract.id, "customer.normalize");
}

#[test]
fn parses_valid_json_example() {
    let result = parse_fixture("valid_minimal.json");
    assert!(result.report.is_valid());
    assert_eq!(result.contract.expect("contract").id, "json.example");
}

#[test]
fn validates_customer_example() {
    let report = parse_fixture("valid_customer.yaml").validate();
    assert!(report.is_valid(), "{report:?}");
}

#[test]
fn validates_repo_example_file() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/customer_normalize.dtcs.yaml");
    let content = fs::read(&path).expect("read example");
    let report = parse_and_validate(&content, DocumentFormat::Yaml);
    assert!(report.is_valid(), "{report:?}");
}

#[test]
fn rejects_malformed_yaml() {
    let result = parse_fixture("malformed.yaml");
    assert!(result.contract.is_none());
    assert!(result
        .report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::PARSE_ERROR));
}

#[test]
fn rejects_malformed_json() {
    let result = parse_fixture("malformed.json");
    assert!(result.contract.is_none());
    assert!(result
        .report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::PARSE_ERROR));
}

#[test]
fn rejects_missing_inputs() {
    let report = parse_fixture("missing_inputs.yaml").validate();
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| { d.id == codes::MISSING_REQUIRED_FIELD && d.message.contains("input") }));
}

#[test]
fn rejects_duplicate_identifiers() {
    let report = parse_fixture("duplicate_input_id.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::DUPLICATE_IDENTIFIER));
}

#[test]
fn rejects_invalid_logical_type() {
    let report = parse_fixture("invalid_type.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}

#[test]
fn rejects_bare_composite_type() {
    let report = parse_fixture("bare_composite_type.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| { d.id == codes::INVALID_TYPE && d.message.contains("type parameters") }));
}

#[test]
fn accepts_parameterized_composite_type() {
    assert!(parse_logical_type("list<string>").is_ok());
}

#[test]
fn rejects_unresolved_reference() {
    let report = parse_fixture("unresolved_reference.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNRESOLVED_REFERENCE));
}

#[test]
fn rejects_invalid_semantic_action() {
    let report = parse_fixture("invalid_semantic_action.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_SEMANTIC_ACTION));
}

#[test]
fn rejects_invalid_rule() {
    let report = parse_fixture("invalid_rule.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_RULE));
}

#[test]
fn rejects_missing_lineage() {
    let report = parse_fixture("missing_lineage.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::MISSING_LINEAGE));
}

#[test]
fn rejects_unknown_lineage_input() {
    let report = parse_fixture("unknown_lineage_input.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNRESOLVED_REFERENCE));
}

#[test]
fn rejects_orphan_lineage_output() {
    let report = parse_fixture("orphan_output_in_lineage.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNRESOLVED_REFERENCE || d.id == codes::MISSING_LINEAGE));
}

#[test]
fn rejects_typo_top_level_field() {
    let report = parse_fixture("typo_top_level_field.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNKNOWN_FIELD));
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_EXTENSION && d.object_ref == Some("input".into())));
}

#[test]
fn rejects_duplicate_schema_fields() {
    let report = parse_fixture("duplicate_schema_field.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::DUPLICATE_IDENTIFIER));
}

#[test]
fn rejects_semantic_type_mismatch() {
    let report = parse_fixture("semantic_type_mismatch.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_SEMANTIC_ACTION));
}

#[test]
fn rejects_unsupported_version() {
    let report = parse_fixture("unsupported_version.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNSUPPORTED_VERSION));
}

#[test]
fn rejects_invalid_identifier() {
    let report = parse_fixture("invalid_identifier.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_IDENTIFIER));
}

#[test]
fn diagnostics_are_deterministic() {
    let content = fs::read(fixture("invalid_type.yaml")).expect("read fixture");
    let first = parse_and_validate(&content, DocumentFormat::Yaml);
    let second = parse_and_validate(&content, DocumentFormat::Yaml);
    assert_eq!(first.diagnostics, second.diagnostics);
}

#[test]
fn preserves_extension_fields() {
    let yaml = br#"
dtcsVersion: "1.0.0"
id: "ext.example"
name: "Extension Example"
version: "0.1.0"
acme:featureFlag: true
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
    let contract = parse(yaml, DocumentFormat::Yaml)
        .contract
        .expect("contract");
    assert!(contract.extensions.contains_key("acme:featureFlag"));
    let report = dtcs::validate(&contract);
    assert!(report.is_valid());
}

#[test]
fn report_errors_filter() {
    let report = parse_fixture("invalid_type.yaml").validate();
    assert!(!report.errors().is_empty());
    assert!(report
        .errors()
        .iter()
        .all(|d| d.severity == Severity::Error));
}

#[test]
fn into_contract_requires_valid_parse() {
    let result = parse_fixture("malformed.yaml");
    assert!(result.into_contract().is_err());
}

#[test]
fn cli_validate_succeeds_on_example() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/customer_normalize.dtcs.yaml");
    let output = dtcs_bin()
        .args(["validate", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).expect("validate json");
    assert_eq!(payload.get("valid").and_then(|v| v.as_bool()), Some(true));
    let diagnostics = payload
        .get("diagnostics")
        .and_then(|v| v.as_array())
        .expect("diagnostics array");
    assert!(
        diagnostics
            .iter()
            .all(|entry| entry.get("severity").and_then(|v| v.as_str()) != Some("error")),
        "unexpected error diagnostics: {diagnostics:?}"
    );
}

#[test]
fn cli_validate_succeeds_on_phase_0_2_fixture() {
    let path = fixture("valid_metadata.yaml");
    let output = dtcs_bin()
        .args(["validate", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).expect("validate json");
    assert_eq!(payload.get("valid").and_then(|v| v.as_bool()), Some(true));
}

#[test]
fn cli_validate_fails_on_invalid_contract() {
    let path = fixture("missing_lineage.yaml");
    let output = dtcs_bin()
        .arg("validate")
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(!output.status.success());
}

#[test]
fn cli_inspect_fails_on_invalid_contract() {
    let path = fixture("unresolved_reference.yaml");
    let output = dtcs_bin()
        .arg("inspect")
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(!output.status.success());
}

#[test]
fn cli_inspect_succeeds_on_valid_contract() {
    let path = fixture("valid_customer.yaml");
    let output = dtcs_bin()
        .arg("inspect")
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("customer.normalize"));
}

#[test]
fn cli_diagnostics_json_output() {
    let path = fixture("missing_lineage.yaml");
    let output = dtcs_bin()
        .args(["diagnostics", "--json"])
        .arg(&path)
        .output()
        .expect("run cli");
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"diagnostics\""));
    assert!(stdout.contains(codes::MISSING_LINEAGE));
}

#[test]
fn cli_version_reports_spec() {
    let output = dtcs_bin().arg("version").output().expect("run cli");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(SPEC_VERSION));
}

#[test]
fn rejects_oversized_document() {
    use dtcs::parser::MAX_DOCUMENT_BYTES;
    let oversized = vec![b' '; MAX_DOCUMENT_BYTES + 1];
    let result = parse(&oversized, DocumentFormat::Yaml);
    assert!(result
        .report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::PARSE_ERROR));
}

#[test]
fn into_contract_without_contract_emits_parse_error() {
    let result = dtcs::ParseResult {
        contract: None,
        report: dtcs::DiagnosticReport::default(),
    };
    let err = result.into_contract().expect_err("should fail");
    assert!(err.diagnostics.iter().any(|d| d.id == codes::PARSE_ERROR));
}

#[test]
fn cli_compat_rejects_invalid_scope() {
    let old = fixture("compatibility/backward_old.yaml");
    let new = fixture("compatibility/backward_new.yaml");
    let output = dtcs_bin()
        .args(["compat", "--scope", "not-a-scope"])
        .arg(&old)
        .arg(&new)
        .output()
        .expect("run cli");
    assert_eq!(output.status.code(), Some(2));
}
