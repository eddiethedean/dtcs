//! Phase 0.6 integration tests — semantic analysis.

mod common;

use std::fs;
use std::path::PathBuf;

use dtcs::{analysis, codes, parse, DocumentFormat};

use common::assert_exact_diagnostic_codes;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn parse_fixture(name: &str) -> dtcs::ParseResult {
    let content = fs::read(fixture(name)).expect("read fixture");
    parse(&content, DocumentFormat::Yaml)
}

fn analyze_fixture(name: &str) -> analysis::AnalysisReport {
    let contract = parse_fixture(name).into_contract().expect("contract");
    analysis::check_contract(&contract, None)
}

#[test]
fn analyzes_constant_expression() {
    let report = analyze_fixture("analysis_constant_expr.yaml");
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    let finding = report
        .findings
        .iter()
        .find(|f| f.kind == "constantExpression")
        .expect("constantExpression finding");
    assert_eq!(finding.object_ref, "expressions.const_add");
}

#[test]
fn analyzes_logical_ops_without_findings() {
    let report = analyze_fixture("analysis_logical_ops.yaml");
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert!(report.findings.is_empty());
}

#[test]
fn analyzes_dtcs_call_without_findings() {
    let report = analyze_fixture("analysis_dtcs_call_valid.yaml");
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert!(report.findings.is_empty());
}

#[test]
fn rejects_duplicate_action_targets_without_ordering() {
    let report = analyze_fixture("analysis_duplicate_action_target.yaml");
    assert!(!report.is_valid(), "{:?}", report.diagnostics);
    assert_exact_diagnostic_codes(&report.diagnostics, &[codes::INVALID_SEMANTICS]);
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.object_ref.as_deref() == Some("semantics.ordering")),
        "{:?}",
        report.diagnostics
    );
}

#[test]
fn analysis_dtcs_call_arity_fails_validation_before_analysis() {
    let contract = parse_fixture("analysis_dtcs_call_arity.yaml")
        .into_contract()
        .expect("contract");
    let validation = contract.validate();
    assert!(!validation.is_valid());
    assert_exact_diagnostic_codes(
        &validation.diagnostics,
        &[codes::AMBIGUOUS_REFERENCE, codes::INVALID_TYPE],
    );
}
