//! Phase 0.9 integration tests — capability matching, compilation, and runtime.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::{
    capability, compile, parse, plan, runtime, validate, DocumentFormat, RuntimeInputs,
    RuntimeValue,
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

fn load_plan(name: &str) -> plan::TransformationPlan {
    let contract = load_valid_contract(name);
    let result = plan::lower(&contract, None, None);
    assert!(result.is_valid(), "{name}: {:?}", result.diagnostics);
    result.plan.expect("plan")
}

#[derive(serde::Deserialize)]
struct CapabilityManifest {
    fixtures: Vec<CapabilityManifestEntry>,
}

#[derive(serde::Deserialize)]
struct CapabilityManifestEntry {
    file: String,
    match_supported: bool,
    compile_valid: bool,
    runtime_fixture: Option<String>,
    expected_output: Option<String>,
}

fn load_capability_manifest() -> CapabilityManifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/capability_expectations.json");
    serde_json::from_str(&fs::read_to_string(path).expect("read manifest")).expect("parse")
}

#[test]
fn reference_profile_lists_stdlib_entries() {
    let profile = capability::reference_profile();
    assert_eq!(profile.engine_id, capability::REFERENCE_ENGINE_ID);
    assert!(profile
        .categories
        .semantic_actions
        .iter()
        .any(|id| id == "dtcs:lowercase"));
    assert!(profile
        .categories
        .functions
        .iter()
        .any(|id| id == "dtcs:lower"));
    assert!(profile
        .categories
        .rules
        .iter()
        .any(|id| id == "dtcs:not_null"));
}

#[test]
fn capability_manifest_fixtures() {
    for entry in load_capability_manifest().fixtures {
        let plan = load_plan(&entry.file);
        let profile = capability::reference_profile();
        let match_report = capability::match_plan(&plan, &profile);
        assert_eq!(
            match_report.supported, entry.match_supported,
            "{}: {:?}",
            entry.file, match_report.diagnostics
        );

        let compile_result = compile::compile(&plan);
        assert_eq!(
            compile_result.is_valid(),
            entry.compile_valid,
            "{}: {:?}",
            entry.file,
            compile_result.diagnostics
        );

        if let (Some(input_fixture), Some(expected_fixture)) =
            (&entry.runtime_fixture, &entry.expected_output)
        {
            let execution_plan = compile_result.plan.expect("execution plan");
            let inputs: RuntimeInputs = serde_json::from_str(
                &fs::read_to_string(fixture(input_fixture)).expect("read input"),
            )
            .expect("parse runtime inputs");
            let execute_result = runtime::execute(&execution_plan, &inputs);
            assert!(
                execute_result.is_valid(),
                "{}: {:?}",
                entry.file,
                execute_result.diagnostics
            );
            let outputs = execute_result.outputs.expect("outputs");
            let expected: BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>> =
                serde_json::from_str(
                    &fs::read_to_string(fixture(expected_fixture)).expect("read expected"),
                )
                .expect("parse expected outputs");
            assert_eq!(outputs, expected);
        }
    }
}

#[test]
fn customer_normalize_end_to_end() {
    let content = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/customer_normalize.dtcs.yaml"),
    )
    .expect("read customer_normalize");
    let inputs: RuntimeInputs = serde_json::from_str(
        &fs::read_to_string(fixture("runtime/customer_normalize_input.json")).expect("read input"),
    )
    .expect("parse inputs");
    let result = dtcs::parse_validate_and_run(&content, DocumentFormat::Yaml, &inputs);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let outputs = result.outputs.expect("outputs");
    let email = outputs
        .get("customer_clean")
        .and_then(|rows| rows.first())
        .and_then(|row| row.get("email"));
    assert_eq!(
        email,
        Some(&RuntimeValue::String("alice@example.com".into()))
    );
}

#[test]
fn compile_rejects_unsupported_vendor_action_plan() {
    let mut plan = load_plan("valid_customer.yaml");
    let action_node = plan
        .nodes
        .iter_mut()
        .find(|node| matches!(node.kind, plan::PlanNodeKind::SemanticAction(_)))
        .expect("semantic action node");
    if let plan::PlanNodeKind::SemanticAction(action) = &mut action_node.kind {
        action.action = "dtcs:nonexistent_action".into();
    }
    let profile = capability::reference_profile();
    let match_report = capability::match_plan(&plan, &profile);
    assert!(!match_report.supported);
}

#[test]
fn cli_run_customer_normalize() {
    let bin = env!("CARGO_BIN_EXE_dtcs");
    let output = Command::new(bin)
        .arg("run")
        .arg("examples/customer_normalize.dtcs.yaml")
        .arg("--input")
        .arg("tests/fixtures/runtime/customer_normalize_input.json")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run dtcs run");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("customer_clean"));
}

#[test]
fn cli_match_and_compile_customer() {
    let bin = env!("CARGO_BIN_EXE_dtcs");
    for subcommand in ["match", "compile"] {
        let output = Command::new(bin)
            .arg(subcommand)
            .arg("examples/customer_normalize.dtcs.yaml")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("run dtcs");
        assert!(
            output.status.success(),
            "{subcommand} stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn runtime_builtin_functions_and_rules() {
    use dtcs::runtime::functions::call_function;
    use dtcs::runtime::rules::evaluate_rule;

    let cases = [
        (
            "dtcs:lower",
            vec![RuntimeValue::String("ABC".into())],
            RuntimeValue::String("abc".into()),
        ),
        (
            "dtcs:upper",
            vec![RuntimeValue::String("abc".into())],
            RuntimeValue::String("ABC".into()),
        ),
        (
            "dtcs:concat",
            vec![
                RuntimeValue::String("a".into()),
                RuntimeValue::String("b".into()),
            ],
            RuntimeValue::String("ab".into()),
        ),
        (
            "dtcs:length",
            vec![RuntimeValue::String("abc".into())],
            RuntimeValue::Integer(3),
        ),
        (
            "dtcs:coalesce",
            vec![RuntimeValue::Null, RuntimeValue::String("x".into())],
            RuntimeValue::String("x".into()),
        ),
    ];
    for (callee, args, expected) in cases {
        let actual = call_function(callee, &args).expect(callee);
        assert_eq!(actual, expected, "{callee}");
    }

    let rule = dtcs::Rule {
        id: "r1".into(),
        rule: "dtcs:not_null".into(),
        target: "in.value".into(),
        phase: dtcs::RulePhase::Postcondition,
        parameters: Default::default(),
        metadata: None,
    };
    evaluate_rule(
        &rule,
        &RuntimeValue::String("ok".into()),
        &Default::default(),
    )
    .expect("not_null passes");
    assert!(evaluate_rule(&rule, &RuntimeValue::Null, &Default::default()).is_err());
}
