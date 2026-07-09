//! Phase 0.7 integration tests — transformation plan lowering and validation.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::{analysis, parse, plan, validate, DocumentFormat};

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
}

fn load_plan_manifest() -> PlanManifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plan_expectations.json");
    serde_json::from_str(&fs::read_to_string(path).expect("read plan manifest")).expect("parse")
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
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::Lineage
            && e.from == "in"
            && (e.to == "trim_value" || e.to == "lower_value")
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
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::FieldRead
            && e.from == "lower_value"
            && e.to == "read_value"
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
    assert!(plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::RulePhase && e.from == "exec_a" && e.to == "post_a"
    }));
    assert!(!plan.dependencies.iter().any(|e| {
        e.reason == plan::DependencyReason::RulePhase
            && ((e.from == "pre_a" && e.to == "post_b") || (e.from == "pre_b" && e.to == "post_a"))
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
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == dtcs::codes::CYCLIC_DEPENDENCY));
}

#[test]
fn rejects_ambiguous_action_order() {
    let contract = load_valid_contract("analysis_duplicate_action_target.yaml");
    let result = plan::lower(&contract, None, None);
    assert!(!result.is_valid());
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.id == dtcs::codes::INVALID_PLAN));
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
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == dtcs::codes::CYCLIC_DEPENDENCY));
}

#[test]
fn manifest_plan_goldens() {
    for entry in load_plan_manifest().fixtures {
        let contract = load_valid_contract(&entry.file);
        let result = plan::lower(&contract, None, None);
        if entry.plan_valid {
            assert!(
                result.is_valid(),
                "{}: {:?}",
                entry.file,
                result.diagnostics
            );
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
            if let Some(codes) = entry.codes {
                for code in codes {
                    assert!(
                        result.diagnostics.iter().any(|d| d.id == code),
                        "missing code {code} for {}",
                        entry.file
                    );
                }
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
