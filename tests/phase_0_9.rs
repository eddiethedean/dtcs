//! Phase 0.9 integration tests — capability matching, compilation, and runtime.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::{
    capability, codes, compile, optimize, parse, plan, runtime, validate, DocumentFormat,
    RuntimeInputs, RuntimeValue,
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
    #[serde(default)]
    mutate_unsupported_action: bool,
}

fn load_capability_manifest() -> CapabilityManifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/capability_expectations.json");
    serde_json::from_str(&fs::read_to_string(path).expect("read manifest")).expect("parse")
}

#[derive(serde::Deserialize)]
struct CompileManifest {
    fixtures: Vec<CompileManifestEntry>,
}

#[derive(serde::Deserialize)]
struct CompileManifestEntry {
    file: String,
    compile_valid: bool,
    golden: Option<String>,
}

fn load_compile_manifest() -> CompileManifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/compile_expectations.json");
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
        let mut plan = load_plan(&entry.file);
        if entry.mutate_unsupported_action {
            let action_node = plan
                .nodes
                .iter_mut()
                .find(|node| matches!(node.kind, plan::PlanNodeKind::SemanticAction(_)))
                .expect("semantic action node");
            if let plan::PlanNodeKind::SemanticAction(action) = &mut action_node.kind {
                action.action = "acme:normalize_email".into();
            }
        }
        let profile = capability::reference_profile();
        let match_report = capability::match_plan(&plan, &profile);
        assert_eq!(
            match_report.supported, entry.match_supported,
            "{}: {:?}",
            entry.file, match_report.diagnostics
        );
        if !entry.match_supported {
            assert_exact_diagnostic_codes(
                &match_report.diagnostics,
                &[codes::UNSUPPORTED_CAPABILITY],
            );
        }

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
        action.action = "acme:normalize_email".into();
    }
    let profile = capability::reference_profile();
    let match_report = capability::match_plan(&plan, &profile);
    assert!(!match_report.supported);
    assert_exact_diagnostic_codes(&match_report.diagnostics, &[codes::UNSUPPORTED_CAPABILITY]);
}

#[test]
fn compile_manifest_goldens() {
    for entry in load_compile_manifest().fixtures {
        let plan = load_plan(&entry.file);
        let compile_result = compile::compile(&plan);
        assert_eq!(
            compile_result.is_valid(),
            entry.compile_valid,
            "{}: {:?}",
            entry.file,
            compile_result.diagnostics
        );
        if entry.compile_valid {
            let golden_path = entry.golden.expect("golden path");
            let golden = fs::read_to_string(fixture(&golden_path)).expect("read golden");
            let expected: serde_json::Value = serde_json::from_str(&golden).expect("parse golden");
            let actual =
                serde_json::to_value(compile_result.plan).expect("serialize execution plan");
            assert_eq!(actual, expected, "{}", entry.file);
        }
    }
}

#[test]
fn capability_match_accepts_reference_stdlib_actions() {
    let plan = load_plan("valid_customer.yaml");
    let profile = capability::reference_profile();
    let match_report = capability::match_plan(&plan, &profile);
    assert!(match_report.supported, "{:?}", match_report.diagnostics);
}

#[test]
fn cli_run_customer_normalize() {
    let bin = env!("CARGO_BIN_EXE_dtcs");
    let output = Command::new(bin)
        .arg("run")
        .arg("examples/customer_normalize.dtcs.yaml")
        .arg("--input")
        .arg("tests/fixtures/runtime/customer_normalize_input.json")
        .arg("--json")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run dtcs run");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).expect("run json");
    let outputs = payload.as_object().expect("outputs object");
    assert!(outputs.contains_key("customer_clean"));
    let rows = outputs["customer_clean"].as_array().expect("rows");
    assert_eq!(rows[0]["email"], "alice@example.com");
}

fn load_runtime_inputs(relative: &str) -> RuntimeInputs {
    serde_json::from_str(&fs::read_to_string(fixture(relative)).expect("read runtime input"))
        .expect("parse runtime inputs")
}

fn execute_validated_contract(name: &str, inputs: &RuntimeInputs) -> runtime::ExecuteResult {
    let content = fs::read(fixture(name)).expect("read fixture");
    let contract = parse(&content, DocumentFormat::Yaml)
        .into_contract()
        .expect("contract");
    let validation = validate(&contract);
    assert!(
        validation.is_valid(),
        "{name}: {:?}",
        validation.diagnostics
    );
    let lowered = plan::lower(&contract, None, None);
    assert!(lowered.is_valid(), "{name}: {:?}", lowered.diagnostics);
    let plan = lowered.plan.expect("plan");
    let compiled = compile::compile(&plan);
    assert!(compiled.is_valid(), "{name}: {:?}", compiled.diagnostics);
    runtime::execute(compiled.plan.as_ref().expect("execution plan"), inputs)
}

#[test]
fn runtime_precondition_violation_reports_exact_code() {
    let inputs = load_runtime_inputs("runtime/precondition_rule_fail_input.json");
    let result = execute_validated_contract("runtime_precondition_fail.yaml", &inputs);
    assert!(!result.is_valid());
    assert_exact_diagnostic_codes(&result.diagnostics, &[codes::PRECONDITION_VIOLATION]);
}

#[test]
fn runtime_postcondition_violation_reports_exact_code() {
    let inputs = load_runtime_inputs("runtime/postcondition_fail_input.json");
    let result = execute_validated_contract("runtime_postcondition_fail.yaml", &inputs);
    assert!(!result.is_valid());
    assert_exact_diagnostic_codes(&result.diagnostics, &[codes::POSTCONDITION_VIOLATION]);
}

#[test]
fn runtime_invalid_input_reports_exact_code_for_null_field() {
    let inputs = load_runtime_inputs("runtime/invalid_runtime_input_null_field.json");
    let result = execute_validated_contract("input_precondition.yaml", &inputs);
    assert!(!result.is_valid());
    assert_exact_diagnostic_codes(&result.diagnostics, &[codes::INVALID_RUNTIME_INPUT]);
}

#[test]
fn compile_rejects_cyclic_plan_with_exact_diagnostics() {
    let mut plan = load_plan("valid_customer.yaml");
    plan.dependencies.push(plan::PlanDependency {
        from: "normalize_email".into(),
        to: "normalize_email".into(),
        reason: plan::DependencyReason::FieldRead,
    });
    let result = compile::compile(&plan);
    assert!(!result.is_valid());
    assert_exact_diagnostic_codes(&result.diagnostics, &[codes::CYCLIC_DEPENDENCY]);
}

#[test]
fn lineage_preserved_through_optimize_compile_and_run() {
    let contract = load_valid_contract("lineage_multi.yaml");
    let original = plan::lower(&contract, None, None).plan.expect("plan");
    let optimized = optimize(&original).plan.expect("optimized");
    assert_eq!(original.lineage, optimized.lineage);

    let compiled = compile::compile(&optimized);
    assert!(compiled.is_valid(), "{:?}", compiled.diagnostics);
    let execution_plan = compiled.plan.expect("execution plan");
    assert_eq!(execution_plan.lineage, optimized.lineage);

    let inputs: RuntimeInputs = BTreeMap::from([
        (
            "customers".into(),
            vec![BTreeMap::from([(
                "id".into(),
                RuntimeValue::String("c1".into()),
            )])],
        ),
        (
            "orders".into(),
            vec![BTreeMap::from([(
                "customer_id".into(),
                RuntimeValue::String("c1".into()),
            )])],
        ),
    ]);
    let result = runtime::execute(&execution_plan, &inputs);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    let outputs = result.outputs.expect("outputs");
    assert!(outputs.contains_key("customer_summary"));
    assert!(outputs.contains_key("order_enriched"));
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
    use dtcs::runtime::actions::apply_action;
    use dtcs::runtime::functions::call_function;
    use dtcs::runtime::rules::evaluate_rule;

    let function_cases = [
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
        (
            "dtcs:substr",
            vec![
                RuntimeValue::String("hello".into()),
                RuntimeValue::Integer(1),
                RuntimeValue::Integer(3),
            ],
            RuntimeValue::String("ell".into()),
        ),
        (
            "dtcs:replace",
            vec![
                RuntimeValue::String("foo-bar".into()),
                RuntimeValue::String("-".into()),
                RuntimeValue::String("_".into()),
            ],
            RuntimeValue::String("foo_bar".into()),
        ),
        (
            "dtcs:to_string",
            vec![RuntimeValue::Integer(42)],
            RuntimeValue::String("42".into()),
        ),
        (
            "dtcs:to_integer",
            vec![RuntimeValue::String("42".into())],
            RuntimeValue::Integer(42),
        ),
        (
            "dtcs:to_decimal",
            vec![RuntimeValue::String("3.5".into())],
            RuntimeValue::Decimal(3.5),
        ),
    ];
    for (callee, args, expected) in function_cases {
        let actual = call_function(callee, &args).expect(callee);
        assert_eq!(actual, expected, "{callee}");
    }

    let concat_err = call_function("dtcs:concat", &[RuntimeValue::String("a".into())])
        .expect_err("concat arity");
    assert!(concat_err.contains("dtcs:concat") || concat_err.contains("argument"));

    let length_err = call_function("dtcs:length", &[RuntimeValue::Null]).expect_err("length type");
    assert!(length_err.contains("dtcs:length") || length_err.contains("string"));

    let action_cases = [
        (
            "dtcs:trim",
            RuntimeValue::String("  hi  ".into()),
            RuntimeValue::String("hi".into()),
        ),
        (
            "dtcs:capitalize",
            RuntimeValue::String("hello".into()),
            RuntimeValue::String("Hello".into()),
        ),
        (
            "dtcs:hash_sha256",
            RuntimeValue::String("test".into()),
            RuntimeValue::String(
                "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".into(),
            ),
        ),
    ];
    for (action_id, input, expected) in action_cases {
        let actual = apply_action(action_id, &input).expect(action_id);
        assert_eq!(actual, expected, "{action_id}");
    }

    let not_null = dtcs::Rule {
        id: "r1".into(),
        rule: "dtcs:not_null".into(),
        target: "in.value".into(),
        phase: dtcs::RulePhase::Postcondition,
        scope: None,
        allow_indeterminate: false,
        deterministic: true,
        parameters: Default::default(),
        metadata: None,
        extensions: Default::default(),
    };
    evaluate_rule(
        &not_null,
        &RuntimeValue::String("ok".into()),
        &Default::default(),
    )
    .expect("not_null passes");
    assert!(evaluate_rule(&not_null, &RuntimeValue::Null, &Default::default()).is_err());

    let min_length = dtcs::Rule {
        id: "r2".into(),
        rule: "dtcs:min_length".into(),
        target: "in.value".into(),
        phase: dtcs::RulePhase::Postcondition,
        scope: None,
        allow_indeterminate: false,
        deterministic: true,
        parameters: indexmap::indexmap! { "min".into() => serde_json::json!(3) },
        metadata: None,
        extensions: Default::default(),
    };
    evaluate_rule(
        &min_length,
        &RuntimeValue::String("abcd".into()),
        &min_length.parameters,
    )
    .expect("min_length passes");
    assert!(evaluate_rule(
        &min_length,
        &RuntimeValue::String("ab".into()),
        &min_length.parameters
    )
    .is_err());

    let max_length = dtcs::Rule {
        id: "r3".into(),
        rule: "dtcs:max_length".into(),
        target: "in.value".into(),
        phase: dtcs::RulePhase::Postcondition,
        scope: None,
        allow_indeterminate: false,
        deterministic: true,
        parameters: indexmap::indexmap! { "max".into() => serde_json::json!(5) },
        metadata: None,
        extensions: Default::default(),
    };
    evaluate_rule(
        &max_length,
        &RuntimeValue::String("abc".into()),
        &max_length.parameters,
    )
    .expect("max_length passes");
    assert!(evaluate_rule(
        &max_length,
        &RuntimeValue::String("abcdef".into()),
        &max_length.parameters
    )
    .is_err());

    let range = dtcs::Rule {
        id: "r4".into(),
        rule: "dtcs:range".into(),
        target: "in.value".into(),
        phase: dtcs::RulePhase::Postcondition,
        scope: None,
        allow_indeterminate: false,
        deterministic: true,
        parameters: indexmap::indexmap! {
            "min".into() => serde_json::json!(1),
            "max".into() => serde_json::json!(10),
        },
        metadata: None,
        extensions: Default::default(),
    };
    evaluate_rule(&range, &RuntimeValue::Integer(5), &range.parameters).expect("range passes");
    assert!(evaluate_rule(&range, &RuntimeValue::Integer(11), &range.parameters).is_err());

    let regex_match = dtcs::Rule {
        id: "r5".into(),
        rule: "dtcs:regex_match".into(),
        target: "in.value".into(),
        phase: dtcs::RulePhase::Postcondition,
        scope: None,
        allow_indeterminate: false,
        deterministic: true,
        parameters: indexmap::indexmap! { "pattern".into() => serde_json::json!("^[a-z]+$") },
        metadata: None,
        extensions: Default::default(),
    };
    evaluate_rule(
        &regex_match,
        &RuntimeValue::String("abc".into()),
        &regex_match.parameters,
    )
    .expect("regex_match passes");
    assert!(evaluate_rule(
        &regex_match,
        &RuntimeValue::String("abc1".into()),
        &regex_match.parameters
    )
    .is_err());
}
