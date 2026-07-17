//! Portable Relational Profile integration tests (0.12–0.15).

use std::collections::BTreeMap;

use dtcs::plan::{export_portable_plan, KERNEL_PROFILE, TRANSFORM_PLAN_IDENTITY};
use dtcs::runtime::actions::{apply_dataset_action, is_dataset_action};
use dtcs::runtime::functions::call_function;
use dtcs::{
    is_known_action, is_known_function, is_known_operator, is_known_profile, parse_validate_and_plan,
    DocumentFormat, PortablePlan, Row, RuntimeValue,
};
use indexmap::IndexMap;
use serde_json::json;

macro_rules! btreemap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m: Row = BTreeMap::new();
        $(m.insert($k.to_string(), $v);)*
        m
    }};
}

#[test]
fn registry_exposes_portable_kernel_entries() {
    assert!(is_known_action("dtcs:project"));
    assert!(is_known_action("dtcs:with_fields"));
    assert!(is_known_action("dtcs:rename_fields"));
    assert!(is_known_action("dtcs:drop_fields"));
    assert!(is_known_action("dtcs:distinct"));
    assert!(is_known_action("dtcs:limit"));
    assert!(is_known_action("dtcs:window"));
    assert!(is_known_operator("dtcs:eq"));
    assert!(is_known_operator("dtcs:add"));
    assert!(is_known_function("dtcs:case_when"));
    assert!(is_known_function("dtcs:if_null"));
    assert!(is_known_function("dtcs:is_invalid"));
    assert!(is_known_function("dtcs:count_all"));
    assert!(is_known_function("dtcs:row_number"));
    assert!(is_known_profile("dtcs:profile/portable-relational-kernel/1"));
    assert!(is_known_profile("dtcs:profile/portable-relational/1"));
    assert!(is_known_profile("dtcs:profile/portable-window/1"));
    assert!(is_known_profile("dtcs:profile/portable-complex-types/1"));

    let project = dtcs::resolve_default("dtcs:project").expect("project");
    assert_eq!(project.version, "2.0.0");
    let coalesce = dtcs::resolve_default("dtcs:coalesce").expect("coalesce");
    assert_eq!(coalesce.version, "2.0.0");
}

#[test]
fn dataset_action_classification_includes_new_ops() {
    assert!(is_dataset_action("dtcs:with_fields"));
    assert!(is_dataset_action("dtcs:distinct"));
    assert!(is_dataset_action("dtcs:deduplicate"));
    assert!(is_dataset_action("dtcs:limit"));
    assert!(is_dataset_action("dtcs:window"));
}

#[test]
fn project_legacy_and_expression_forms() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![btreemap! {
            "a" => RuntimeValue::Integer(1),
            "b" => RuntimeValue::String("x".into()),
            "c" => RuntimeValue::Integer(3),
        }],
    );

    let mut params = IndexMap::new();
    params.insert("fields".into(), json!(["a", "b"]));
    apply_dataset_action("dtcs:project", "t", &params, &mut workspaces).unwrap();
    assert_eq!(workspaces["t"][0].len(), 2);
    assert!(workspaces["t"][0].contains_key("a"));
    assert!(!workspaces["t"][0].contains_key("c"));
}

#[test]
fn with_fields_and_drop_fields() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![btreemap! {
            "name" => RuntimeValue::String("Ada".into()),
        }],
    );
    let mut params = IndexMap::new();
    params.insert(
        "assignments".into(),
        json!([{ "as": "greeting", "expr": "\"hi\"" }]),
    );
    apply_dataset_action("dtcs:with_fields", "t", &params, &mut workspaces).unwrap();
    assert_eq!(
        workspaces["t"][0].get("greeting"),
        Some(&RuntimeValue::String("hi".into()))
    );

    let mut drop = IndexMap::new();
    drop.insert("fields".into(), json!(["greeting"]));
    apply_dataset_action("dtcs:drop_fields", "t", &drop, &mut workspaces).unwrap();
    assert!(!workspaces["t"][0].contains_key("greeting"));
}

#[test]
fn join_does_not_match_null_keys() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "left".into(),
        vec![
            btreemap! { "k" => RuntimeValue::Null, "v" => RuntimeValue::Integer(1) },
            btreemap! { "k" => RuntimeValue::Integer(2), "v" => RuntimeValue::Integer(2) },
        ],
    );
    workspaces.insert(
        "right".into(),
        vec![
            btreemap! { "k" => RuntimeValue::Null, "w" => RuntimeValue::Integer(9) },
            btreemap! { "k" => RuntimeValue::Integer(2), "w" => RuntimeValue::Integer(8) },
        ],
    );
    let mut params = IndexMap::new();
    params.insert("right".into(), json!("right"));
    params.insert("leftKey".into(), json!("k"));
    apply_dataset_action("dtcs:join", "left", &params, &mut workspaces).unwrap();
    assert_eq!(workspaces["left"].len(), 1);
    assert_eq!(
        workspaces["left"][0].get("v"),
        Some(&RuntimeValue::Integer(2))
    );
}

#[test]
fn coalesce_and_kernel_functions() {
    let result = call_function(
        "dtcs:coalesce",
        &[
            RuntimeValue::Null,
            RuntimeValue::missing(),
            RuntimeValue::String("ok".into()),
        ],
    )
    .unwrap();
    assert_eq!(result, RuntimeValue::String("ok".into()));

    let invalid = call_function("dtcs:is_invalid", &[RuntimeValue::invalid("x")]).unwrap();
    assert_eq!(invalid, RuntimeValue::Boolean(true));

    let if_null = call_function(
        "dtcs:if_null",
        &[RuntimeValue::Null, RuntimeValue::Integer(7)],
    )
    .unwrap();
    assert_eq!(if_null, RuntimeValue::Integer(7));
}

#[test]
fn portable_plan_export_and_fingerprint() {
    let yaml = include_str!("../examples/minimal.dtcs.yaml");
    let plan_result = parse_validate_and_plan(yaml.as_bytes(), DocumentFormat::Yaml);
    assert!(
        plan_result.diagnostics.is_empty(),
        "{:?}",
        plan_result.diagnostics
    );
    let plan = plan_result.plan.expect("plan");
    let portable = export_portable_plan(&plan, KERNEL_PROFILE).expect("portable");
    assert_eq!(portable.plan_identity, TRANSFORM_PLAN_IDENTITY);
    assert_eq!(portable.profile, KERNEL_PROFILE);
    assert_eq!(portable.registry_versions.actions.as_deref(), Some("2.0.0"));
    let fp1 = portable.fingerprint().unwrap();
    let fp2 = portable.fingerprint().unwrap();
    assert_eq!(fp1, fp2);
    assert_eq!(fp1.len(), 64);
}

#[test]
fn portable_plan_rejects_executable_objects() {
    let portable = PortablePlan {
        plan_identity: TRANSFORM_PLAN_IDENTITY.into(),
        profile: KERNEL_PROFILE.into(),
        specification_version: "1.0.0".into(),
        registry_versions: Default::default(),
        transformation: "t".into(),
        inputs: Default::default(),
        parameters: Default::default(),
        actions: vec![json!({ "sqlText": "SELECT 1" })],
        outputs: Default::default(),
        rules: vec![],
        lineage: vec![],
        requirements: Default::default(),
        extensions: Default::default(),
    };
    let err = portable.validate_budgets().unwrap_err();
    assert!(err.contains("executable") || err.contains("host-language"));
}

#[test]
fn multi_aggregate_and_window_and_datetime() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![
            btreemap! {
                "g" => RuntimeValue::String("a".into()),
                "v" => RuntimeValue::Integer(1),
            },
            btreemap! {
                "g" => RuntimeValue::String("a".into()),
                "v" => RuntimeValue::Integer(3),
            },
            btreemap! {
                "g" => RuntimeValue::String("b".into()),
                "v" => RuntimeValue::Integer(5),
            },
        ],
    );
    let mut params = IndexMap::new();
    params.insert("groupBy".into(), json!(["g"]));
    params.insert(
        "aggregates".into(),
        json!([
            { "as": "total", "op": "sum", "field": "v" },
            { "as": "n", "op": "count_all" }
        ]),
    );
    apply_dataset_action("dtcs:aggregate", "t", &params, &mut workspaces).unwrap();
    assert_eq!(workspaces["t"].len(), 2);

    workspaces.insert(
        "w".into(),
        vec![
            btreemap! { "k" => RuntimeValue::Integer(1), "x" => RuntimeValue::Integer(10) },
            btreemap! { "k" => RuntimeValue::Integer(1), "x" => RuntimeValue::Integer(20) },
        ],
    );
    let mut wparams = IndexMap::new();
    wparams.insert("partitionBy".into(), json!(["k"]));
    wparams.insert("orderBy".into(), json!([{ "field": "x" }]));
    wparams.insert(
        "functions".into(),
        json!([{ "as": "rn", "function": "row_number" }]),
    );
    apply_dataset_action("dtcs:window", "w", &wparams, &mut workspaces).unwrap();
    assert_eq!(
        workspaces["w"][0].get("rn"),
        Some(&RuntimeValue::Integer(1))
    );

    assert_eq!(
        call_function("dtcs:current_date", &[]).unwrap(),
        RuntimeValue::Date("2026-01-01".into())
    );
    let added = call_function(
        "dtcs:date_add",
        &[
            RuntimeValue::Date("2026-01-01".into()),
            RuntimeValue::Integer(1),
        ],
    )
    .unwrap();
    assert_eq!(added, RuntimeValue::Date("2026-01-02".into()));
}

#[test]
fn structured_expression_round_trip_and_capability_manifest() {
    let node = dtcs::to_structured_node("1 + 2").unwrap();
    assert_eq!(node["kind"], "binary");
    let round_trip = dtcs::from_structured_node(&node).unwrap();
    let again = serde_json::to_value(&round_trip).unwrap();
    assert_eq!(again["kind"], "binary");

    let manifest = dtcs::reference_portable_manifest(KERNEL_PROFILE);
    assert_eq!(manifest.profile, KERNEL_PROFILE);
    assert!(manifest.actions.contains_key("dtcs:project"));
    assert!(manifest.operators.contains_key("dtcs:eq"));
    assert!(manifest.functions.contains_key("dtcs:coalesce"));
    assert_eq!(
        manifest.limits.get("maxPortablePlanBytes").map(String::as_str),
        Some("8388608")
    );
}
