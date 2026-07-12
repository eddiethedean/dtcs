//! Phase 0.7 integration tests — transformation plan lowering and validation.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::{
    analysis, compile, parse, plan, runtime, validate, DocumentFormat, RuntimeInputs, RuntimeValue,
};

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

fn load_valid_contract(name: &str) -> dtcs::TransformationContract {
    let result = parse_fixture(name);
    let contract = result.into_contract().expect("contract");
    let report = validate(&contract);
    assert!(report.is_valid(), "{name}: {:?}", report.diagnostics);
    contract
}

#[derive(serde::Deserialize)]
struct PlanManifest {
    fixtures: Vec<PlanManifestEntry>,
}

#[derive(serde::Deserialize)]
struct PlanManifestEntry {
    file: String,
    plan_valid: bool,
    golden: Option<String>,
    codes: Option<Vec<String>>,
    runtime_input: Option<String>,
    expected_output: Option<String>,
    expect_lineage_mappings: Option<usize>,
    expect_precondition_rule_steps: Option<bool>,
    expect_node_id: Option<String>,
    expect_semantic_action_id: Option<String>,
}

fn load_plan_manifest() -> PlanManifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plan_expectations.json");
    serde_json::from_str(&fs::read_to_string(path).expect("read plan manifest")).expect("parse")
}

fn assert_plan_structural_invariants(
    contract: &dtcs::TransformationContract,
    lowered: &plan::TransformationPlan,
) {
    assert_eq!(lowered.identity.id, contract.id);
    if let Some(lineage) = contract.lineage.as_ref() {
        let output_ids: std::collections::HashSet<_> =
            lowered.outputs.iter().map(|o| o.id.as_str()).collect();
        for mapping in &lineage.mappings {
            assert!(
                output_ids.contains(mapping.output.as_str()),
                "lineage output '{}' missing from plan outputs",
                mapping.output
            );
        }
    }
    assert!(plan::is_acyclic(
        contract,
        &lowered.nodes,
        &lowered.dependencies
    ));
    let validation = plan::validate(lowered);
    assert!(
        validation.is_valid(),
        "plan validate failed for {}: {:?}",
        contract.id,
        validation.diagnostics
    );
}

fn execute_lowered_plan(
    contract_file: &str,
    inputs: &RuntimeInputs,
) -> BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>> {
    let contract = load_valid_contract(contract_file);
    let lowered = plan::lower(&contract, None, None);
    assert!(
        lowered.is_valid(),
        "{contract_file}: {:?}",
        lowered.diagnostics
    );
    let plan = lowered.plan.expect("plan");
    let compiled = compile::compile(&plan);
    assert!(
        compiled.is_valid(),
        "{contract_file}: {:?}",
        compiled.diagnostics
    );
    let execution_plan = compiled.plan.expect("execution plan");
    let result = runtime::execute(&execution_plan, inputs);
    assert!(
        result.is_valid(),
        "{contract_file}: {:?}",
        result.diagnostics
    );
    result.outputs.expect("outputs")
}

#[test]
fn lowers_valid_customer_contract() {
    let contract = load_valid_contract("valid_customer.yaml");
    let analysis = analysis::check_contract(&contract, None);
    let result = plan::lower(&contract, None, Some(&analysis));
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let plan = result.plan.expect("plan");
    assert_eq!(plan.identity.id, "customer.normalize");
    assert_eq!(plan.nodes.len(), 2);
    assert!(!plan.dependencies.is_empty());
}

#[test]
fn lowers_minimal_json_contract() {
    let content = fs::read(fixture("valid_minimal.json")).expect("read");
    let contract = parse(&content, DocumentFormat::Json)
        .into_contract()
        .expect("contract");
    let result = plan::lower(&contract, None, None);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    assert!(result.plan.expect("plan").nodes.is_empty());
}

#[test]
fn plan_includes_all_node_kinds() {
    let contract = load_valid_contract("valid_customer.yaml");
    let result = plan::lower(&contract, None, None);
    let plan = result.plan.expect("plan");
    assert!(plan
        .nodes
        .iter()
        .any(|n| matches!(n.kind, plan::PlanNodeKind::SemanticAction(_))));
    assert!(plan
        .nodes
        .iter()
        .any(|n| matches!(n.kind, plan::PlanNodeKind::Rule(_))));
}

#[test]
fn dependency_graph_from_lineage() {
    let contract = load_valid_contract("lineage_multi.yaml");
    let result = plan::lower(&contract, None, None);
    let plan = result.plan.expect("plan");
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::Lineage
            && e.from == "customers"
            && e.to == "customer_summary"
    }));
}

#[test]
fn explicit_action_ordering_edges() {
    let contract = load_valid_contract("plan_explicit_ordering.yaml");
    let result = plan::lower(&contract, None, None);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let plan = result.plan.expect("plan");
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::ExplicitOrder
            && e.from == "trim_value"
            && e.to == "lower_value"
    }));
}

#[test]
fn multi_input_lineage_edges() {
    let contract = load_valid_contract("lineage_multi.yaml");
    let result = plan::lower(&contract, None, None);
    let plan = result.plan.expect("plan");
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::Lineage
            && e.from == "customers"
            && e.to == "order_enriched"
    }));
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::Lineage
            && e.from == "orders"
            && e.to == "order_enriched"
    }));
}

#[test]
fn field_write_chain_edges() {
    let contract = load_valid_contract("plan_field_write_chain.yaml");
    let result = plan::lower(&contract, None, None);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let plan = result.plan.expect("plan");
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::FieldWrite
            && e.from == "trim_value"
            && e.to == "lower_value"
    }));
}

#[test]
fn rule_phase_edges_scoped_by_target() {
    let contract = load_valid_contract("plan_rule_phase_scoped.yaml");
    let result = plan::lower(&contract, None, None);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let plan = result.plan.expect("plan");
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::RulePhase && e.from == "pre_a" && e.to == "exec_a"
    }));
}

#[test]
fn partial_cycle_rejected_by_validate_and_is_acyclic() {
    let contract = load_valid_contract("valid_customer.yaml");
    let mut result = plan::lower(&contract, None, None);
    let plan = result.plan.as_mut().expect("plan");
    plan.dependencies.push(plan::PlanDependency {
        from: "normalize_email".into(),
        to: "customer_raw".into(),
        reason: plan::DependencyReason::FieldWrite,
    });
    assert!(!plan::is_acyclic(
        &contract,
        &plan.nodes,
        &plan.dependencies
    ));
    let report = plan::validate(plan);
    assert!(!report.is_valid());
    assert_exact_diagnostic_codes(&report.diagnostics, &[dtcs::codes::CYCLIC_DEPENDENCY]);
}

#[test]
fn rejects_ambiguous_action_order() {
    let contract = load_valid_contract("analysis_duplicate_action_target.yaml");
    let result = plan::lower(&contract, None, None);
    assert!(!result.is_valid());
    assert_exact_diagnostic_codes(
        &result.diagnostics,
        &[dtcs::codes::INVALID_PLAN, dtcs::codes::INVALID_SEMANTICS],
    );
}

#[test]
fn deterministic_lowering() {
    let contract = load_valid_contract("valid_customer.yaml");
    let a = plan::lower(&contract, None, None);
    let b = plan::lower(&contract, None, None);
    let json_a = serde_json::to_string(&a.plan).expect("serialize");
    let json_b = serde_json::to_string(&b.plan).expect("serialize");
    assert_eq!(json_a, json_b);
}

#[test]
fn findings_attached_from_analysis() {
    let contract = load_valid_contract("analysis_constant_expr.yaml");
    let analysis = analysis::check_contract(&contract, None);
    let result = plan::lower(&contract, None, Some(&analysis));
    let plan = result.plan.expect("plan");
    assert!(plan.findings.iter().any(|f| f.kind == "constantExpression"));
}

#[test]
fn plan_validate_catches_cyclic_dependencies() {
    let contract = load_valid_contract("valid_customer.yaml");
    let mut result = plan::lower(&contract, None, None);
    let plan = result.plan.as_mut().expect("plan");
    plan.dependencies.push(plan::PlanDependency {
        from: "normalize_email".into(),
        to: "customer_raw".into(),
        reason: plan::DependencyReason::FieldWrite,
    });
    let report = plan::validate(plan);
    assert!(!report.is_valid());
    assert_exact_diagnostic_codes(&report.diagnostics, &[dtcs::codes::CYCLIC_DEPENDENCY]);
}

#[test]
fn plan_goldens_are_change_detectors_not_semantic_oracle() {
    let manifest = load_plan_manifest();
    assert!(manifest.fixtures.iter().any(|entry| entry.golden.is_some()));
}

#[test]
fn manifest_plan_goldens() {
    for entry in load_plan_manifest().fixtures {
        let contract = if entry.plan_valid {
            load_valid_contract(&entry.file)
        } else {
            parse_fixture(&entry.file)
                .into_contract()
                .expect("contract")
        };
        let result = plan::lower(&contract, None, None);
        if entry.plan_valid {
            assert!(
                result.is_valid(),
                "{}: {:?}",
                entry.file,
                result.diagnostics
            );
            let lowered = result.plan.as_ref().expect("plan");
            assert_plan_structural_invariants(&contract, lowered);

            if let Some(count) = entry.expect_lineage_mappings {
                assert_eq!(contract.lineage.as_ref().unwrap().mappings.len(), count);
            }
            if entry.expect_precondition_rule_steps == Some(true) {
                assert!(lowered.nodes.iter().any(|node| {
                    matches!(
                        &node.kind,
                        plan::PlanNodeKind::Rule(rule) if rule.phase == dtcs::RulePhase::Precondition
                    )
                }));
            }
            if let Some(node_id) = &entry.expect_node_id {
                assert!(lowered.nodes.iter().any(|node| node.id == *node_id));
            }
            if let Some(action_id) = &entry.expect_semantic_action_id {
                assert!(lowered.nodes.iter().any(|node| node.id == *action_id));
            }
            if let (Some(runtime_input), Some(expected_output)) =
                (&entry.runtime_input, &entry.expected_output)
            {
                let inputs: RuntimeInputs = serde_json::from_str(
                    &fs::read_to_string(fixture(runtime_input)).expect("read runtime input"),
                )
                .expect("parse runtime input");
                let actual = execute_lowered_plan(&entry.file, &inputs);
                let expected: BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>> =
                    serde_json::from_str(
                        &fs::read_to_string(fixture(expected_output)).expect("read expected"),
                    )
                    .expect("parse expected output");
                assert_eq!(actual, expected, "runtime oracle for {}", entry.file);
            }

            let golden_path = entry.golden.expect("golden path");
            let golden = fs::read_to_string(fixture(&golden_path)).expect("read golden");
            let expected: serde_json::Value = serde_json::from_str(&golden).expect("parse golden");
            let actual = serde_json::to_value(result.plan).expect("serialize plan");
            assert_eq!(actual, expected, "{}", entry.file);

            let round_trip: plan::TransformationPlan =
                serde_json::from_str(&golden).expect("round trip");
            let validation = plan::validate(&round_trip);
            assert!(
                validation.is_valid(),
                "round-trip validation for {}: {:?}",
                entry.file,
                validation.diagnostics
            );
        } else {
            assert!(
                !result.is_valid(),
                "expected plan failure for {}",
                entry.file
            );
            if let Some(codes) = &entry.codes {
                let expected: Vec<&str> = codes.iter().map(String::as_str).collect();
                assert_exact_diagnostic_codes(&result.diagnostics, &expected);
            }
        }
    }
}

#[test]
fn cli_plan_json_output() {
    let bin = env!("CARGO_BIN_EXE_dtcs");
    let output = Command::new(bin)
        .args(["plan", "tests/fixtures/valid_customer.yaml", "--json"])
        .output()
        .expect("run cli");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let plan: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert_eq!(plan["identity"]["id"], "customer.normalize");
}

#[test]
fn parse_validate_and_plan_convenience() {
    let content = fs::read(fixture("valid_customer.yaml")).expect("read");
    let result = dtcs::parse_validate_and_plan(&content, DocumentFormat::Yaml);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    assert!(result.plan.is_some());
}
