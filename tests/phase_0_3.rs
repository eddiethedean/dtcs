//! Phase 0.3 integration tests — compatibility, evolution, versioning, lineage analysis.
//!
//! Exact invalid diagnostic codes for shared fixtures are enforced by
//! `tests/manifest.rs` and `phase_0_3_invalid_fixture_codes_match_manifest`.

mod common;

use std::fs;
use std::path::PathBuf;

use dtcs::{
    analyze_compatibility, analyze_evolution, codes, parse, ComparisonScope, CompatibilityLevel,
    DocumentFormat,
};

use common::assert_fixture_validation_codes;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn compat_fixture(name: &str) -> PathBuf {
    fixture(&format!("compatibility/{name}"))
}

fn load_contract(path: &PathBuf) -> dtcs::TransformationContract {
    let content = fs::read(path).expect("read fixture");
    parse(&content, DocumentFormat::Yaml)
        .into_contract()
        .expect("valid contract")
}

#[test]
fn classifies_identical_contracts() {
    let a = load_contract(&compat_fixture("identical_a.yaml"));
    let b = load_contract(&compat_fixture("identical_b.yaml"));
    let report = analyze_compatibility(&a, &b, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::Identical);
}

#[test]
fn classifies_backward_compatible_contracts() {
    let old = load_contract(&compat_fixture("backward_old.yaml"));
    let new = load_contract(&compat_fixture("backward_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::BackwardCompatible);
}

#[test]
fn classifies_forward_compatible_contracts() {
    let old = load_contract(&compat_fixture("forward_old.yaml"));
    let new = load_contract(&compat_fixture("forward_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::ForwardCompatible);
}

#[test]
fn classifies_conditionally_compatible_contracts() {
    let a = load_contract(&compat_fixture("conditional_a.yaml"));
    let b = load_contract(&compat_fixture("conditional_b.yaml"));
    let report = analyze_compatibility(&a, &b, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::ConditionallyCompatible);
}

#[test]
fn classifies_incompatible_contracts() {
    let a = load_contract(&compat_fixture("incompatible_a.yaml"));
    let b = load_contract(&compat_fixture("incompatible_b.yaml"));
    let report = analyze_compatibility(&a, &b, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::Incompatible);
    assert!(!report.is_compatible());
}

#[test]
fn analyzes_evolution_between_revisions() {
    let rev1 = load_contract(&compat_fixture("evolution/rev1.yaml"));
    let rev2 = load_contract(&compat_fixture("evolution/rev2.yaml"));
    let report = analyze_evolution(&rev1, &rev2);
    assert!(report.same_identity);
    assert_eq!(report.compatibility, CompatibilityLevel::BackwardCompatible);
    assert!(!report.changes.is_empty());
}

#[test]
fn detects_deprecation_metadata() {
    let baseline = load_contract(&compat_fixture("evolution/rev1.yaml"));
    let deprecated = load_contract(&compat_fixture("evolution/deprecated.yaml"));
    let report = analyze_evolution(&baseline, &deprecated);
    assert!(report
        .changes
        .iter()
        .any(|c| c.object_ref.as_deref() == Some("metadata.deprecated")));
}

#[test]
fn rejects_invalid_version_identifier() {
    let report = load_contract(&fixture("invalid_version.yaml")).validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_VERSION));
}

#[test]
fn warns_on_version_metadata_conflict() {
    let report = load_contract(&fixture("version_conflict.yaml")).validate();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_METADATA));
}

#[test]
fn analyzes_lineage_graph_and_impact() {
    let contract = load_contract(&fixture("lineage_multi.yaml"));
    let report = dtcs::lineage::analyze_with_options(&contract, Some("customers"), None);
    assert_eq!(report.graph.len(), 2);
    let impact = report.impact.expect("impact");
    assert!(impact.outputs.contains(&"customer_summary".to_string()));
    assert!(impact.outputs.contains(&"order_enriched".to_string()));
    assert_eq!(report.governance.owner.as_deref(), Some("data-platform"));
}

#[test]
fn analyzes_lineage_dependency() {
    let contract = load_contract(&fixture("lineage_multi.yaml"));
    let report = dtcs::lineage::analyze_with_options(&contract, None, Some("order_enriched"));
    let dep = report.dependency.expect("dependency");
    assert_eq!(dep.inputs, vec!["orders", "customers"]);
}

#[test]
fn versioning_validate_is_public() {
    let contract = load_contract(&fixture("valid_customer.yaml"));
    let report = dtcs::versioning::validate(&contract);
    assert!(report.is_valid());
}

#[test]
fn decimal_to_integer_is_not_backward_compatible() {
    let old = load_contract(&compat_fixture("decimal_integer_old.yaml"));
    let new = load_contract(&compat_fixture("decimal_integer_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_ne!(report.level, CompatibilityLevel::BackwardCompatible);
    assert_ne!(report.level, CompatibilityLevel::Identical);
}

#[test]
fn scoped_compat_interfaces_excludes_type_diffs() {
    let old = load_contract(&compat_fixture("decimal_integer_old.yaml"));
    let new = load_contract(&compat_fixture("decimal_integer_new.yaml"));
    let scope = ComparisonScope::from_tokens(&["interfaces".into()]).expect("scope");
    let report = analyze_compatibility(&old, &new, scope);
    assert!(report.is_compatible());
}

#[test]
fn streaming_mode_change_is_incompatible() {
    let old = load_contract(&compat_fixture("streaming_old.yaml"));
    let new = load_contract(&compat_fixture("streaming_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::Incompatible);
}

#[test]
fn required_to_optional_input_is_backward_compatible() {
    let old = load_contract(&compat_fixture("required_optional_old.yaml"));
    let new = load_contract(&compat_fixture("required_optional_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::BackwardCompatible);
}

#[test]
fn lineage_warns_on_unknown_impact_input() {
    let contract = load_contract(&fixture("lineage_multi.yaml"));
    let report = dtcs::lineage::analyze_with_options(&contract, Some("missing_input"), None);
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNRESOLVED_REFERENCE));
}

#[test]
fn evolution_identical_deprecated_has_no_deprecation_change() {
    let deprecated = load_contract(&compat_fixture("evolution/deprecated.yaml"));
    let report = analyze_evolution(&deprecated, &deprecated);
    assert!(!report
        .changes
        .iter()
        .any(|c| c.object_ref.as_deref() == Some("metadata.deprecated")));
}

#[test]
fn rule_parameter_change_is_incompatible() {
    let old = load_contract(&compat_fixture("rule_params_old.yaml"));
    let new = load_contract(&compat_fixture("rule_params_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::Incompatible);
}

#[test]
fn lineage_input_reorder_with_overlap_is_incompatible() {
    let old = load_contract(&compat_fixture("lineage_reorder_overlap_old.yaml"));
    let new = load_contract(&compat_fixture("lineage_reorder_overlap_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::Incompatible);
}

#[test]
fn extension_value_change_is_conditionally_compatible() {
    let old = load_contract(&compat_fixture("conditional_a.yaml"));
    let new = load_contract(&compat_fixture("extension_value_b.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert_eq!(report.level, CompatibilityLevel::ConditionallyCompatible);
}

#[test]
fn optional_input_removal_is_not_incompatible() {
    let old = load_contract(&compat_fixture("optional_removed_old.yaml"));
    let new = load_contract(&compat_fixture("optional_removed_new.yaml"));
    let report = analyze_compatibility(&old, &new, ComparisonScope::all());
    assert!(report.is_compatible());
    assert_ne!(report.level, CompatibilityLevel::Incompatible);
}

#[test]
fn lineage_warns_when_declared_input_has_no_graph_edge() {
    let mut contract = load_contract(&fixture("lineage_multi.yaml"));
    contract.lineage = None;
    let report = dtcs::lineage::analyze_with_options(&contract, Some("customers"), None);
    assert!(report.diagnostics.iter().any(|d| {
        d.id == codes::UNRESOLVED_REFERENCE && d.message.contains("has no lineage graph edge")
    }));
}

#[test]
fn phase_0_3_invalid_fixture_codes_match_manifest() {
    const FIXTURES: &[&str] = &["invalid_version.yaml", "version_conflict.yaml"];
    for file in FIXTURES {
        let contract = load_contract(&fixture(file));
        assert_fixture_validation_codes(file, &contract.validate().diagnostics);
    }
}
