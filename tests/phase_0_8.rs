//! Phase 0.8 integration tests — transformation plan optimization.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::{analysis, equivalent, optimize, parse, plan, validate, DocumentFormat};

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
    transforms_min: Option<usize>,
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
    if let Some(expr) = optimized.nodes.iter().find(|n| n.id == "const_add") {
        if let plan::PlanNodeKind::Expression(expression) = &expr.kind {
            assert_eq!(expression.expr.as_deref(), Some("3"));
        }
    }
    assert!(equivalent(&original, &optimized));
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
        if let Some(min) = entry.transforms_min {
            assert!(
                result.transforms.len() >= min,
                "{} expected at least {min} transforms",
                entry.file
            );
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
    assert!(payload["transforms"]
        .as_array()
        .is_some_and(|t| !t.is_empty()));
}
