//! Phase 0.8 integration tests — transformation plan optimization.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::{
    analysis, compile, equivalent, optimize, parse, plan, runtime, validate, DocumentFormat,
    RuntimeInputs, RuntimeOutputs, RuntimeValue,
};

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

fn lower_valid_plan(name: &str) -> plan::TransformationPlan {
    let contract = load_valid_contract(name);
    let analysis = analysis::check_contract(&contract, None);
    let result = plan::lower(&contract, None, Some(&analysis));
    assert!(result.is_valid(), "{name}: {:?}", result.diagnostics);
    result.plan.expect("plan")
}

#[derive(serde::Deserialize)]
struct OptimizeManifest {
    fixtures: Vec<OptimizeManifestEntry>,
}

#[derive(serde::Deserialize)]
struct OptimizeManifestEntry {
    file: String,
    optimize_valid: bool,
    equivalent: bool,
    golden: Option<String>,
    runtime_input: Option<String>,
    expected_output: Option<String>,
}

fn load_runtime_inputs(relative: &str) -> RuntimeInputs {
    let path = fixture(relative);
    serde_json::from_str(&fs::read_to_string(path).expect("read runtime input")).expect("parse")
}

fn execute_plan(plan: &plan::TransformationPlan, inputs: &RuntimeInputs) -> RuntimeOutputs {
    let compiled = compile::compile(plan);
    assert!(compiled.is_valid(), "{:?}", compiled.diagnostics);
    let execution_plan = compiled.plan.expect("execution plan");
    let result = runtime::execute(&execution_plan, inputs);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    result.outputs.expect("outputs")
}

fn load_optimize_manifest() -> OptimizeManifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/optimize_expectations.json");
    serde_json::from_str(&fs::read_to_string(path).expect("read optimize manifest")).expect("parse")
}

#[test]
fn optimize_constant_fold() {
    let original = lower_valid_plan("optimize_constant_fold.yaml");
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    assert!(!result.transforms.is_empty());
    let optimized = result.plan.expect("optimized plan");
    assert!(
        optimized.nodes.is_empty(),
        "folded constant expression should be eliminated as dead"
    );
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_rule_dedup_params() {
    let original = lower_valid_plan("optimize_rule_dedup_params.yaml");
    assert_eq!(original.nodes.len(), 3);
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    assert_eq!(
        optimized.nodes.len(),
        2,
        "only identical parameter sets should deduplicate"
    );
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_dead_after_fold() {
    let original = lower_valid_plan("optimize_dead_after_fold.yaml");
    assert!(original.nodes.iter().any(|n| n.id == "dead_mul"));
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    assert!(!optimized.nodes.iter().any(|n| n.id == "dead_mul"));
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_rejects_invalid_input_plan() {
    let mut plan = lower_valid_plan("optimize_constant_fold.yaml");
    plan.dependencies.push(plan::PlanDependency {
        from: "const_add".into(),
        to: "const_add".into(),
        reason: plan::DependencyReason::FieldRead,
    });
    let result = plan::optimize_with_registry(
        &plan,
        dtcs::registry::default_registry(),
        &plan::OptimizeOptions::default(),
    );
    assert!(!result.is_valid());
    assert!(result.plan.is_none());
    assert!(result.transforms.is_empty());
}

#[test]
fn parse_validate_and_optimize_integration() {
    let content = fs::read(fixture("optimize_constant_fold.yaml")).expect("read");
    let result = dtcs::parse_validate_and_optimize(&content, DocumentFormat::Yaml);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    assert!(optimized.nodes.is_empty());
}

#[test]
fn cli_optimize_plan_json() {
    let original = lower_valid_plan("optimize_constant_fold.yaml");
    let plan_json = serde_json::to_string(&original).expect("serialize plan");
    let temp = std::env::temp_dir().join("dtcs_optimize_plan_test.json");
    fs::write(&temp, plan_json).expect("write temp plan");

    let output = Command::new(env!("CARGO_BIN_EXE_dtcs"))
        .arg("optimize")
        .arg(&temp)
        .arg("--plan")
        .arg("--json")
        .output()
        .expect("run dtcs optimize --plan");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert!(payload.get("plan").is_some());
    let _ = fs::remove_file(temp);
}

#[test]
fn optimize_algebraic_simplify() {
    let original = lower_valid_plan("optimize_algebraic.yaml");
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    let expr = optimized
        .nodes
        .iter()
        .find(|n| n.id == "scale_identity")
        .expect("expression node");
    if let plan::PlanNodeKind::Expression(expression) = &expr.kind {
        assert_eq!(expression.expr.as_deref(), Some("in.value"));
    } else {
        panic!("expected expression node");
    }
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_action_fusion() {
    let original = lower_valid_plan("optimize_action_fusion.yaml");
    assert_eq!(original.nodes.len(), 2);
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    let actions: Vec<_> = optimized
        .nodes
        .iter()
        .filter(|n| matches!(n.kind, plan::PlanNodeKind::SemanticAction(_)))
        .collect();
    assert_eq!(actions.len(), 1);
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_function_inline() {
    let original = lower_valid_plan("optimize_function_inline.yaml");
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    assert!(!result.transforms.is_empty());
    let optimized = result.plan.expect("optimized plan");
    if let Some(expr) = optimized.nodes.iter().find(|n| n.id == "normalized") {
        if let plan::PlanNodeKind::Expression(expression) = &expr.kind {
            assert_eq!(expression.expr.as_deref(), Some("\"abc\""));
        }
    }
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_rule_dedup() {
    let original = lower_valid_plan("optimize_rule_dedup.yaml");
    assert_eq!(original.nodes.len(), 2);
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    assert_eq!(optimized.nodes.len(), 1);
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_dead_expression() {
    let original = lower_valid_plan("optimize_dead_expr.yaml");
    assert!(original.nodes.iter().any(|n| n.id == "unused_const"));
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    assert!(!optimized.nodes.iter().any(|n| n.id == "unused_const"));
    assert!(optimized.nodes.iter().any(|n| n.id == "read_value"));
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_idempotent_on_minimal_plan() {
    let original = lower_valid_plan("valid_customer.yaml");
    let result = optimize(&original);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let optimized = result.plan.expect("optimized plan");
    assert!(equivalent(&original, &optimized));
}

#[test]
fn optimize_twice_is_stable() {
    let original = lower_valid_plan("optimize_constant_fold.yaml");
    let first = optimize(&original).plan.expect("first");
    let second = optimize(&first).plan.expect("second");
    assert_eq!(
        serde_json::to_value(&first).expect("first json"),
        serde_json::to_value(&second).expect("second json")
    );
}

#[test]
fn optimize_action_fusion_preserves_lowercasing_behavior() {
    let original = lower_valid_plan("optimize_action_fusion.yaml");
    let optimized = optimize(&original).plan.expect("optimized plan");
    let inputs = load_runtime_inputs("runtime/optimize_action_fusion_input.json");
    let outputs = execute_plan(&optimized, &inputs);
    let email = outputs
        .get("out")
        .and_then(|rows| rows.first())
        .and_then(|row| row.get("email"));
    assert_eq!(
        email,
        Some(&RuntimeValue::String("upper@email.com".into())),
        "fusion must not remove required lowercase semantics"
    );
}

#[test]
fn optimize_manifest_goldens_are_change_detectors_not_semantic_oracle() {
    // Golden JSON under tests/fixtures/plans_optimized/ records plan shape only.
    // Semantic correctness is verified via runtime I/O equivalence in optimize_manifest_golden_files.
    let manifest = load_optimize_manifest();
    assert_eq!(manifest.fixtures.len(), 8);
    for entry in manifest.fixtures {
        assert!(entry.golden.is_some(), "{} missing golden", entry.file);
        assert!(
            entry.runtime_input.is_some(),
            "{} missing runtime input",
            entry.file
        );
    }
}

#[test]
fn optimize_manifest_golden_files() {
    let manifest = load_optimize_manifest();
    for entry in manifest.fixtures {
        let original = lower_valid_plan(&entry.file);
        let result = optimize(&original);
        assert_eq!(
            result.is_valid(),
            entry.optimize_valid,
            "{}: {:?}",
            entry.file,
            result.diagnostics
        );
        if !entry.optimize_valid {
            continue;
        }
        let optimized = result.plan.expect("optimized plan");
        if entry.equivalent {
            assert!(
                equivalent(&original, &optimized),
                "{} not equivalent",
                entry.file
            );
        }
        if let Some(runtime_input) = &entry.runtime_input {
            let inputs = load_runtime_inputs(runtime_input);
            let original_outputs = execute_plan(&original, &inputs);
            let optimized_outputs = execute_plan(&optimized, &inputs);
            assert_eq!(
                original_outputs, optimized_outputs,
                "{} optimize changed runtime behavior",
                entry.file
            );
            if let Some(expected_output) = &entry.expected_output {
                let expected: BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>> =
                    serde_json::from_str(
                        &fs::read_to_string(fixture(expected_output)).expect("read expected"),
                    )
                    .expect("parse expected output");
                assert_eq!(
                    optimized_outputs, expected,
                    "{} unexpected optimized runtime output",
                    entry.file
                );
            }
        }
        if let Some(golden) = entry.golden {
            let golden_path = fixture(&golden);
            let expected: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&golden_path).expect("read golden"))
                    .expect("parse golden");
            let actual = serde_json::to_value(&optimized).expect("serialize optimized");
            assert_eq!(actual, expected, "golden mismatch for {}", entry.file);
        }
    }
}

#[test]
fn all_plan_goldens_optimize_cleanly() {
    let plan_manifest_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plan_expectations.json");
    #[derive(serde::Deserialize)]
    struct PlanManifest {
        fixtures: Vec<PlanEntry>,
    }
    #[derive(serde::Deserialize)]
    struct PlanEntry {
        file: String,
        plan_valid: bool,
    }
    let manifest: PlanManifest =
        serde_json::from_str(&fs::read_to_string(plan_manifest_path).expect("read"))
            .expect("parse");
    for entry in manifest.fixtures {
        if !entry.plan_valid {
            continue;
        }
        let original = lower_valid_plan(&entry.file);
        let result = optimize(&original);
        assert!(
            result.is_valid(),
            "{}: {:?}",
            entry.file,
            result.diagnostics
        );
        let optimized = result.plan.expect("optimized");
        assert!(equivalent(&original, &optimized), "{}", entry.file);
    }
}

#[test]
fn cli_optimize_contract_json() {
    let fixture = fixture("optimize_constant_fold.yaml");
    let output = Command::new(env!("CARGO_BIN_EXE_dtcs"))
        .arg("optimize")
        .arg(&fixture)
        .arg("--json")
        .output()
        .expect("run dtcs optimize");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert!(payload.get("plan").is_some());
    let transforms = payload["transforms"].as_array().expect("transforms array");
    assert!(!transforms.is_empty());
    assert_eq!(
        payload["plan"]["nodes"].as_array().map(|n| n.len()),
        Some(0)
    );
}
