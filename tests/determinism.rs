//! Determinism and purity semantics (SPEC Chapter 7).

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use dtcs::{
    analysis, compile, parse, plan, runtime, validate, DocumentFormat, RuntimeInputs, RuntimeValue,
};

use common::assert_exact_diagnostic_codes;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn run_contract(
    name: &str,
    inputs: &RuntimeInputs,
) -> BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>> {
    let content = fs::read(fixture(name)).expect("read");
    let contract = parse(&content, DocumentFormat::Yaml)
        .into_contract()
        .expect("contract");
    assert!(validate(&contract).is_valid());
    let lowered = plan::lower(&contract, None, None);
    assert!(lowered.is_valid());
    let compiled = compile::compile(lowered.plan.as_ref().expect("plan"));
    assert!(compiled.is_valid());
    let result = runtime::execute(compiled.plan.as_ref().expect("execution plan"), inputs);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
    result.outputs.expect("outputs")
}

#[test]
fn deterministic_contract_produces_identical_outputs_on_double_run() {
    let inputs: RuntimeInputs = serde_json::from_str(
        &fs::read_to_string(fixture("runtime/deterministic_double_run_input.json")).expect("read"),
    )
    .expect("parse");
    let first = run_contract("deterministic_double_run.yaml", &inputs);
    let second = run_contract("deterministic_double_run.yaml", &inputs);
    assert_eq!(
        serde_json::to_value(&first).expect("first"),
        serde_json::to_value(&second).expect("second")
    );
    let expected: BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>> = serde_json::from_str(
        &fs::read_to_string(fixture("runtime/deterministic_double_run_output.json")).expect("read"),
    )
    .expect("parse expected");
    assert_eq!(first, expected);
}

#[test]
fn impure_without_side_effects_fails_analysis() {
    let content = fs::read(fixture("impure_side_effects_invalid.yaml")).expect("read");
    let contract = parse(&content, DocumentFormat::Yaml)
        .into_contract()
        .expect("contract");
    assert!(validate(&contract).is_valid());
    let report = analysis::check_contract(&contract, None);
    assert!(!report.is_valid());
    assert_exact_diagnostic_codes(&report.diagnostics, &[dtcs::codes::INVALID_SEMANTICS]);
}
