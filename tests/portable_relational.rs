//! Portable Relational Profile integration tests (phase 0.12).

use std::collections::BTreeMap;

use dtcs::plan::{export_portable_plan, KERNEL_PROFILE, TRANSFORM_PLAN_IDENTITY};
use dtcs::runtime::actions::{apply_dataset_action, is_dataset_action};
use dtcs::runtime::functions::call_function;
use dtcs::{
    is_known_action, is_known_function, is_known_operator, is_known_profile,
    parse_validate_and_plan, DocumentFormat, PortablePlan, Row, RuntimeValue,
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
    assert!(is_known_profile(
        "dtcs:profile/portable-relational-kernel/1"
    ));
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
        manifest
            .limits
            .get("maxPortablePlanBytes")
            .map(String::as_str),
        Some("8388608")
    );
}

#[test]
fn between_ternary_inclusive() {
    use dtcs::runtime::expr::eval_expression_on_row;
    let row = btreemap! { "x" => RuntimeValue::Integer(5) };
    assert_eq!(
        eval_expression_on_row("x between 1 and 10", &row).unwrap(),
        RuntimeValue::Boolean(true)
    );
    assert_eq!(
        eval_expression_on_row("x between 5 and 5", &row).unwrap(),
        RuntimeValue::Boolean(true)
    );
    assert_eq!(
        eval_expression_on_row("x between 6 and 10", &row).unwrap(),
        RuntimeValue::Boolean(false)
    );
    let null_row = btreemap! { "x" => RuntimeValue::Null };
    assert_eq!(
        eval_expression_on_row("x between 1 and 10", &null_row).unwrap(),
        RuntimeValue::Null
    );
}

#[test]
fn sort_by_expression_and_union_duplicate_policy() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![
            btreemap! {
                "a" => RuntimeValue::Integer(1),
                "b" => RuntimeValue::Integer(4),
            },
            btreemap! {
                "a" => RuntimeValue::Integer(3),
                "b" => RuntimeValue::Integer(1),
            },
            btreemap! {
                "a" => RuntimeValue::Integer(2),
                "b" => RuntimeValue::Integer(2),
            },
        ],
    );
    let mut params = IndexMap::new();
    params.insert(
        "keys".into(),
        json!([{ "expr": "a + b", "descending": true }]),
    );
    apply_dataset_action("dtcs:sort", "t", &params, &mut workspaces).unwrap();
    assert_eq!(workspaces["t"][0].get("a"), Some(&RuntimeValue::Integer(1)));

    workspaces.insert(
        "u".into(),
        vec![
            btreemap! { "id" => RuntimeValue::Integer(1) },
            btreemap! { "id" => RuntimeValue::Integer(1) },
            btreemap! { "id" => RuntimeValue::Integer(2) },
        ],
    );
    workspaces.insert(
        "v".into(),
        vec![btreemap! { "id" => RuntimeValue::Integer(2) }],
    );
    let mut uparams = IndexMap::new();
    uparams.insert("other".into(), json!("v"));
    uparams.insert("duplicatePolicy".into(), json!("distinct"));
    apply_dataset_action("dtcs:union", "u", &uparams, &mut workspaces).unwrap();
    assert_eq!(workspaces["u"].len(), 2);
}

#[test]
fn join_collision_policy_and_predicate() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "left".into(),
        vec![btreemap! {
            "k" => RuntimeValue::Integer(1),
            "name" => RuntimeValue::String("L".into()),
            "score" => RuntimeValue::Integer(10),
        }],
    );
    workspaces.insert(
        "right".into(),
        vec![btreemap! {
            "k" => RuntimeValue::Integer(1),
            "name" => RuntimeValue::String("R".into()),
            "score" => RuntimeValue::Integer(20),
        }],
    );
    let mut params = IndexMap::new();
    params.insert("right".into(), json!("right"));
    params.insert("leftKey".into(), json!("k"));
    params.insert("collisionPolicy".into(), json!("suffix"));
    params.insert("predicate".into(), json!("score_left < score_right"));
    apply_dataset_action("dtcs:join", "left", &params, &mut workspaces).unwrap();
    assert_eq!(workspaces["left"].len(), 1);
    assert_eq!(
        workspaces["left"][0].get("name_left"),
        Some(&RuntimeValue::String("L".into()))
    );
    assert_eq!(
        workspaces["left"][0].get("name_right"),
        Some(&RuntimeValue::String("R".into()))
    );

    let err = {
        let mut ws = BTreeMap::new();
        ws.insert(
            "l".into(),
            vec![btreemap! {
                "k" => RuntimeValue::Integer(1),
                "x" => RuntimeValue::Integer(1),
            }],
        );
        ws.insert(
            "r".into(),
            vec![btreemap! {
                "k" => RuntimeValue::Integer(1),
                "x" => RuntimeValue::Integer(2),
            }],
        );
        let mut p = IndexMap::new();
        p.insert("right".into(), json!("r"));
        p.insert("leftKey".into(), json!("k"));
        p.insert("collisionPolicy".into(), json!("error"));
        apply_dataset_action("dtcs:join", "l", &p, &mut ws)
    };
    assert!(err.unwrap_err().contains("collision"));
}

#[test]
fn aggregate_group_by_expression_and_empty_input() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![
            btreemap! { "v" => RuntimeValue::Integer(1) },
            btreemap! { "v" => RuntimeValue::Integer(2) },
            btreemap! { "v" => RuntimeValue::Integer(11) },
        ],
    );
    let mut params = IndexMap::new();
    params.insert("groupBy".into(), json!([{ "expr": "v > 10", "as": "big" }]));
    params.insert(
        "aggregates".into(),
        json!([{ "as": "n", "op": "count_all" }]),
    );
    apply_dataset_action("dtcs:aggregate", "t", &params, &mut workspaces).unwrap();
    assert_eq!(workspaces["t"].len(), 2);

    workspaces.insert("empty".into(), vec![]);
    let mut eparams = IndexMap::new();
    eparams.insert("groupBy".into(), json!(["g"]));
    eparams.insert(
        "aggregates".into(),
        json!([{ "as": "n", "op": "count_all" }]),
    );
    apply_dataset_action("dtcs:aggregate", "empty", &eparams, &mut workspaces).unwrap();
    assert!(workspaces["empty"].is_empty());
}

#[test]
fn and_or_short_circuit_on_row_eval() {
    use dtcs::runtime::expr::eval_expression_on_row;
    let row = btreemap! {
        "a" => RuntimeValue::Boolean(false),
        "b" => RuntimeValue::Integer(1),
    };
    // Previously panicked: evaluate_binary treated And/Or as unreachable on the row path.
    assert_eq!(
        eval_expression_on_row("a && (b == 1)", &row).unwrap(),
        RuntimeValue::Boolean(false)
    );
    assert_eq!(
        eval_expression_on_row("a || (b == 1)", &row).unwrap(),
        RuntimeValue::Boolean(true)
    );
}

#[test]
fn sort_mixed_integer_decimal_and_group_key_types() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![
            btreemap! { "v" => RuntimeValue::Integer(2) },
            btreemap! { "v" => RuntimeValue::Decimal(1.0) },
            btreemap! { "v" => RuntimeValue::Integer(3) },
        ],
    );
    let mut params = IndexMap::new();
    params.insert("keys".into(), json!([{ "field": "v" }]));
    apply_dataset_action("dtcs:sort", "t", &params, &mut workspaces).unwrap();
    assert_eq!(
        workspaces["t"][0].get("v"),
        Some(&RuntimeValue::Decimal(1.0))
    );
    assert_eq!(workspaces["t"][2].get("v"), Some(&RuntimeValue::Integer(3)));

    workspaces.insert(
        "g".into(),
        vec![
            btreemap! { "n" => RuntimeValue::Integer(1) },
            btreemap! { "n" => RuntimeValue::Integer(1) },
            btreemap! { "n" => RuntimeValue::Integer(2) },
        ],
    );
    let mut gparams = IndexMap::new();
    gparams.insert("groupBy".into(), json!(["n"]));
    gparams.insert(
        "aggregates".into(),
        json!([{ "as": "c", "op": "count_all" }]),
    );
    apply_dataset_action("dtcs:aggregate", "g", &gparams, &mut workspaces).unwrap();
    let ones = workspaces["g"]
        .iter()
        .find(|r| r.get("n") == Some(&RuntimeValue::Integer(1)))
        .expect("integer group key preserved");
    assert_eq!(ones.get("c"), Some(&RuntimeValue::Integer(2)));
}

#[test]
fn predicate_only_anti_join_and_left_null_pad() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "left".into(),
        vec![
            btreemap! { "id" => RuntimeValue::Integer(1), "x" => RuntimeValue::Integer(10) },
            btreemap! { "id" => RuntimeValue::Integer(2), "x" => RuntimeValue::Integer(20) },
        ],
    );
    workspaces.insert(
        "right".into(),
        vec![btreemap! {
            "id" => RuntimeValue::Integer(1),
            "y" => RuntimeValue::Integer(99),
        }],
    );
    let mut anti = IndexMap::new();
    anti.insert("right".into(), json!("right"));
    anti.insert("type".into(), json!("anti"));
    anti.insert("leftKey".into(), json!("id"));
    apply_dataset_action("dtcs:join", "left", &anti, &mut workspaces).unwrap();
    assert_eq!(workspaces["left"].len(), 1);
    assert_eq!(
        workspaces["left"][0].get("id"),
        Some(&RuntimeValue::Integer(2))
    );

    // Predicate-only anti must honor join type (not silently become inner).
    workspaces.insert(
        "pl".into(),
        vec![
            btreemap! { "x" => RuntimeValue::Integer(1) },
            btreemap! { "x" => RuntimeValue::Integer(5) },
        ],
    );
    workspaces.insert(
        "pr".into(),
        vec![btreemap! { "y" => RuntimeValue::Integer(3) }],
    );
    let mut pred_anti = IndexMap::new();
    pred_anti.insert("right".into(), json!("pr"));
    pred_anti.insert("type".into(), json!("anti"));
    pred_anti.insert("predicate".into(), json!("x < y"));
    apply_dataset_action("dtcs:join", "pl", &pred_anti, &mut workspaces).unwrap();
    assert_eq!(workspaces["pl"].len(), 1);
    assert_eq!(
        workspaces["pl"][0].get("x"),
        Some(&RuntimeValue::Integer(5))
    );

    workspaces.insert(
        "l2".into(),
        vec![btreemap! {
            "k" => RuntimeValue::Integer(1),
            "a" => RuntimeValue::Integer(1),
        }],
    );
    workspaces.insert(
        "r2".into(),
        vec![btreemap! {
            "k" => RuntimeValue::Integer(2),
            "b" => RuntimeValue::Integer(7),
        }],
    );
    let mut left_join = IndexMap::new();
    left_join.insert("right".into(), json!("r2"));
    left_join.insert("leftKey".into(), json!("k"));
    left_join.insert("type".into(), json!("left"));
    apply_dataset_action("dtcs:join", "l2", &left_join, &mut workspaces).unwrap();
    assert_eq!(workspaces["l2"].len(), 1);
    assert_eq!(
        workspaces["l2"][0].get("b"),
        Some(&RuntimeValue::Null),
        "left join must null-pad missing right columns"
    );
}

#[test]
fn datetime_preserves_time_and_hour_diff() {
    let added = call_function(
        "dtcs:date_add",
        &[
            RuntimeValue::DateTime("2026-01-01T15:30:00Z".into()),
            RuntimeValue::Integer(1),
            RuntimeValue::String("day".into()),
        ],
    )
    .unwrap();
    assert_eq!(added, RuntimeValue::DateTime("2026-01-02T15:30:00Z".into()));

    let hours = call_function(
        "dtcs:date_diff",
        &[
            RuntimeValue::DateTime("2026-01-01T12:00:00Z".into()),
            RuntimeValue::DateTime("2026-01-01T00:00:00Z".into()),
            RuntimeValue::String("hour".into()),
        ],
    )
    .unwrap();
    assert_eq!(hours, RuntimeValue::Integer(12));
}

#[test]
fn inverted_window_frame_is_empty() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![
            btreemap! { "x" => RuntimeValue::Integer(1) },
            btreemap! { "x" => RuntimeValue::Integer(2) },
            btreemap! { "x" => RuntimeValue::Integer(3) },
        ],
    );
    let mut params = IndexMap::new();
    params.insert("orderBy".into(), json!([{ "field": "x" }]));
    params.insert(
        "frame".into(),
        json!({
            "type": "rows",
            "start": { "following": 1 },
            "end": { "preceding": 1 }
        }),
    );
    params.insert(
        "functions".into(),
        json!([{ "as": "s", "function": "sum", "field": "x" }]),
    );
    apply_dataset_action("dtcs:window", "t", &params, &mut workspaces).unwrap();
    for row in &workspaces["t"] {
        assert_eq!(row.get("s"), Some(&RuntimeValue::Null));
    }
}

#[test]
fn window_frame_first_last_and_date_units() {
    let mut workspaces = BTreeMap::new();
    workspaces.insert(
        "t".into(),
        vec![
            btreemap! { "x" => RuntimeValue::Integer(1) },
            btreemap! { "x" => RuntimeValue::Integer(2) },
            btreemap! { "x" => RuntimeValue::Integer(3) },
        ],
    );
    let mut params = IndexMap::new();
    params.insert("orderBy".into(), json!([{ "field": "x" }]));
    params.insert(
        "frame".into(),
        json!({
            "type": "rows",
            "start": { "preceding": 1 },
            "end": "currentRow"
        }),
    );
    params.insert(
        "functions".into(),
        json!([
            { "as": "fv", "function": "first_value", "field": "x" },
            { "as": "s", "function": "sum", "field": "x" }
        ]),
    );
    apply_dataset_action("dtcs:window", "t", &params, &mut workspaces).unwrap();
    assert_eq!(
        workspaces["t"][1].get("fv"),
        Some(&RuntimeValue::Integer(1))
    );
    assert_eq!(
        workspaces["t"][1].get("s"),
        Some(&RuntimeValue::Decimal(3.0))
    );

    let month = call_function(
        "dtcs:date_add",
        &[
            RuntimeValue::Date("2026-01-31".into()),
            RuntimeValue::Integer(1),
            RuntimeValue::String("month".into()),
        ],
    )
    .unwrap();
    assert_eq!(month, RuntimeValue::Date("2026-02-28".into()));

    let trunc = call_function(
        "dtcs:date_trunc",
        &[
            RuntimeValue::DateTime("2026-03-15T12:30:00Z".into()),
            RuntimeValue::String("month".into()),
        ],
    )
    .unwrap();
    assert_eq!(trunc, RuntimeValue::DateTime("2026-03-01T00:00:00Z".into()));

    assert!(
        dtcs::validate_capability_accuracy(&dtcs::reference_portable_manifest(KERNEL_PROFILE))
            .is_ok()
    );
}

#[test]
fn field_index_element_at_access_ops() {
    let map = RuntimeValue::Map({
        let mut m = BTreeMap::new();
        m.insert("a".into(), RuntimeValue::Integer(7));
        m
    });
    assert_eq!(
        call_function(
            "dtcs:field",
            &[map.clone(), RuntimeValue::String("a".into())]
        )
        .unwrap(),
        RuntimeValue::Integer(7)
    );
    let list = RuntimeValue::List(vec![
        RuntimeValue::Integer(1),
        RuntimeValue::Integer(2),
        RuntimeValue::Integer(3),
    ]);
    assert_eq!(
        call_function("dtcs:index", &[list.clone(), RuntimeValue::Integer(1)]).unwrap(),
        RuntimeValue::Integer(2)
    );
    assert_eq!(
        call_function("dtcs:element_at", &[list, RuntimeValue::Integer(99)]).unwrap(),
        RuntimeValue::Null
    );
}
