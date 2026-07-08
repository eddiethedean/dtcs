//! Phase 0.6 integration tests — semantic analysis.

use std::fs;
use std::path::PathBuf;

use dtcs::{analysis, parse, DocumentFormat};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn parse_fixture(name: &str) -> dtcs::ParseResult {
    let content = fs::read(fixture(name)).expect("read fixture");
    parse(&content, DocumentFormat::Yaml)
}

#[test]
fn analyzes_constant_expression() {
    let contract = parse_fixture("analysis_constant_expr.yaml")
        .into_contract()
        .expect("contract");
    let report = analysis::check_contract(&contract, None);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert!(report
        .findings
        .iter()
        .any(|f| f.kind == "constantExpression"));
}

#[test]
fn rejects_duplicate_action_targets_without_ordering() {
    let contract = parse_fixture("analysis_duplicate_action_target.yaml")
        .into_contract()
        .expect("contract");
    let report = analysis::check_contract(&contract, None);
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.id == dtcs::codes::INVALID_SEMANTICS),
        "{:?}",
        report.diagnostics
    );
}

