//! Phase 0.2 integration tests — metadata, types, and interfaces.

use std::fs;
use std::path::PathBuf;

use dtcs::{
    codes, metadata, parse, parse_logical_type, type_compatible, DocumentFormat, LogicalType,
    ParseResult, TypeCompatibility,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn parse_fixture(name: &str) -> ParseResult {
    let content = fs::read(fixture(name)).expect("read fixture");
    parse(&content, DocumentFormat::Yaml)
}

#[test]
fn accepts_valid_metadata() {
    let result = parse_fixture("valid_metadata.yaml");
    assert!(result.report.is_valid(), "{:?}", result.report.diagnostics);
    let report = result.contract.expect("contract").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn metadata_validate_matches_full_validate() {
    let contract = parse_fixture("invalid_metadata_timestamp.yaml")
        .into_contract()
        .expect("contract");
    let metadata_report = metadata::validate(&contract);
    let full_report = contract.validate();
    let metadata_ids: Vec<_> = metadata_report
        .diagnostics
        .iter()
        .map(|d| d.id.as_str())
        .collect();
    assert!(metadata_ids.contains(&codes::INVALID_METADATA));
    assert!(full_report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn rejects_invalid_metadata_timestamp() {
    let report = parse_fixture("invalid_metadata_timestamp.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn rejects_metadata_identity_conflict() {
    let report = parse_fixture("invalid_metadata_identity_conflict.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn accepts_streaming_declarations() {
    let report = parse_fixture("streaming_unbounded.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn rejects_invalid_streaming_mode_at_parse() {
    let result = parse_fixture("invalid_streaming_mode.yaml");
    assert!(!result.report.is_valid());
}

#[test]
fn accepts_optional_input_with_required_sibling() {
    let report = parse_fixture("optional_input.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn rejects_all_inputs_optional() {
    let report = parse_fixture("all_inputs_optional.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_INTERFACE));
}

#[test]
fn accepts_input_preconditions() {
    let report = parse_fixture("input_precondition.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn rejects_unresolved_precondition_rule() {
    let report = parse_fixture("invalid_precondition_rule.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNRESOLVED_REFERENCE));
}

#[test]
fn rejects_precondition_with_wrong_rule_phase() {
    let report = parse_fixture("invalid_precondition_phase.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_INTERFACE));
}

#[test]
fn accepts_map_type_with_two_parameters() {
    let report = parse_fixture("map_type_valid.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert!(parse_logical_type("map<string,integer>").is_ok());
}

#[test]
fn rejects_invalid_map_arity() {
    let report = parse_fixture("invalid_map_arity.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE && d.message.contains("map")));
}

#[test]
fn accepts_extension_logical_type() {
    let report = parse_fixture("extension_type_valid.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    let parsed = parse_logical_type("vendor:Money").expect("extension type");
    assert!(matches!(parsed, LogicalType::Extension(_)));
}

#[test]
fn accepts_expression_and_function_types() {
    let report = parse_fixture("expression_with_type.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn rejects_conversion_without_lossy_flag() {
    let report = parse_fixture("invalid_conversion_lossy.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_CONVERSION));
}

#[test]
fn type_compatible_integer_decimal() {
    let integer = parse_logical_type("integer").expect("integer");
    let decimal = parse_logical_type("decimal").expect("decimal");
    assert_eq!(
        type_compatible(&integer, &decimal),
        TypeCompatibility::Compatible
    );
    assert_eq!(
        type_compatible(&integer, &integer),
        TypeCompatibility::Identical
    );
}

#[test]
fn type_compatible_incompatible_primitives() {
    let string = parse_logical_type("string").expect("string");
    let integer = parse_logical_type("integer").expect("integer");
    assert_eq!(
        type_compatible(&string, &integer),
        TypeCompatibility::Incompatible
    );
}

#[test]
fn accepts_nested_collection_types() {
    let report = parse_fixture("nested_collection_valid.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert!(parse_logical_type("list<map<string,integer>>").is_ok());
    assert!(parse_logical_type("tuple<string,integer>").is_ok());
}

#[test]
fn rejects_type_trailing_garbage() {
    let report = parse_fixture("invalid_type_trailing_garbage.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
    assert!(parse_logical_type("list<string>oops").is_err());
}

#[test]
fn rejects_restricted_without_governance_contact() {
    let report = parse_fixture("invalid_metadata_restricted.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn rejects_invalid_metadata_custom_key() {
    let report = parse_fixture("invalid_metadata_custom_key.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn rejects_misplaced_io_extension_key() {
    let report = parse_fixture("invalid_io_extension.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_INTERFACE));
}

#[test]
fn rejects_unresolved_postcondition_rule() {
    let report = parse_fixture("invalid_postcondition_rule.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNRESOLVED_REFERENCE));
}

#[test]
fn rejects_postcondition_with_wrong_rule_phase() {
    let report = parse_fixture("invalid_postcondition_phase.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_INTERFACE));
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.message.contains("postcondition")));
}

#[test]
fn accepts_valid_lossy_conversion() {
    let report = parse_fixture("valid_conversion_lossy.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn rejects_impossible_metadata_date() {
    let report = parse_fixture("invalid_metadata_impossible_date.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn suggests_inputs_for_input_typo() {
    let report = parse_fixture("typo_top_level_field.yaml").validate();
    assert!(report.diagnostics.iter().any(|d| {
        d.id == codes::UNKNOWN_FIELD
            && d.remediation
                .as_ref()
                .is_some_and(|text| text.contains("inputs"))
    }));
}

#[test]
fn rejects_http_rule_identifier() {
    let report = parse_fixture("invalid_http_rule.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_RULE));
}

#[test]
fn rejects_http_action_identifier() {
    let report = parse_fixture("invalid_http_action.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_SEMANTIC_ACTION));
}

#[test]
fn rejects_http_extension_type() {
    let report = parse_fixture("invalid_http_type.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}

#[test]
fn rejects_impossible_metadata_time() {
    let report = parse_fixture("invalid_metadata_impossible_time.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn rejects_misplaced_postcondition_key() {
    let report = parse_fixture("invalid_misplaced_postcondition.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_INTERFACE));
}

#[test]
fn rejects_expression_missing_type() {
    let report = parse_fixture("expression_missing_type.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::MISSING_REQUIRED_FIELD));
}

#[test]
fn rejects_expression_type_mismatch() {
    let report = parse_fixture("expression_type_mismatch.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}

#[test]
fn rejects_function_missing_return_type() {
    let report = parse_fixture("function_missing_return_type.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::MISSING_REQUIRED_FIELD));
}

#[test]
fn rejects_expression_invalid_operator() {
    let report = parse_fixture("expression_invalid_operator.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}

#[test]
fn rejects_expression_unresolved_field() {
    let report = parse_fixture("expression_unresolved_field.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}

#[test]
fn accepts_expression_precedence_multiply() {
    let report = parse_fixture("expression_precedence_multiply.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn accepts_expression_precedence_compare() {
    let report = parse_fixture("expression_precedence_compare.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn rejects_duplicate_io_id() {
    let report = parse_fixture("duplicate_io_id.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::DUPLICATE_IDENTIFIER));
}

#[test]
fn rejects_expression_narrowing_decimal() {
    let report = parse_fixture("expression_narrowing_decimal.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}

#[test]
fn accepts_expression_unary_minus() {
    let report = parse_fixture("expression_unary_minus.yaml").validate();
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn rejects_invalid_function_namespace() {
    let report = parse_fixture("invalid_function_namespace.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_FUNCTION));
}

#[test]
fn rejects_invalid_dtcs_function() {
    let report = parse_fixture("invalid_dtcs_function.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_FUNCTION));
}

#[test]
fn rejects_function_optional_param_order() {
    let report = parse_fixture("function_optional_param_order.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_FUNCTION));
}

#[test]
fn rejects_function_call_arity() {
    let report = parse_fixture("function_call_arity.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}

#[test]
fn rejects_lowercase_nullable_target() {
    let report = parse_fixture("lowercase_nullable_target.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_SEMANTIC_ACTION));
}

#[test]
fn rejects_expression_nullable_field() {
    let report = parse_fixture("expression_nullable_field.yaml").validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_TYPE));
}
