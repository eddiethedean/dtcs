//! Semantic action execution.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::hash::{Hash, Hasher};

use indexmap::IndexMap;
use serde_json::Value;

use crate::runtime::expr::eval_expression_on_row;
use crate::runtime::model::{lookup_field, Dataset, FieldLookup, Row, RuntimeValue};

/// Apply a `dtcs:` semantic action to a field value.
pub fn apply_action(action_id: &str, value: &RuntimeValue) -> Result<RuntimeValue, String> {
    match action_id {
        "dtcs:lowercase" => match value {
            RuntimeValue::Null => Err("dtcs:lowercase does not accept null".into()),
            RuntimeValue::Invalid(_) => Err("dtcs:lowercase does not accept invalid".into()),
            RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_lowercase())),
            other => Err(format!("dtcs:lowercase requires string, got {other:?}")),
        },
        "dtcs:uppercase" => match value {
            RuntimeValue::Null => Err("dtcs:uppercase does not accept null".into()),
            RuntimeValue::Invalid(_) => Err("dtcs:uppercase does not accept invalid".into()),
            RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_uppercase())),
            other => Err(format!("dtcs:uppercase requires string, got {other:?}")),
        },
        "dtcs:capitalize" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::Invalid(_) => Ok(value.clone()),
            RuntimeValue::String(s) => {
                let mut chars = s.chars();
                let out = match chars.next() {
                    Some(first) => {
                        first.to_uppercase().collect::<String>()
                            + chars.as_str().to_lowercase().as_str()
                    }
                    None => String::new(),
                };
                Ok(RuntimeValue::String(out))
            }
            other => Err(format!("dtcs:capitalize requires string, got {other:?}")),
        },
        "dtcs:trim" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::Invalid(_) => Ok(value.clone()),
            RuntimeValue::String(s) => Ok(RuntimeValue::String(s.trim().to_string())),
            other => Err(format!("dtcs:trim requires string, got {other:?}")),
        },
        "dtcs:normalize_whitespace" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::Invalid(_) => Ok(value.clone()),
            RuntimeValue::String(s) => {
                let normalized = s.split_whitespace().collect::<Vec<_>>().join(" ");
                Ok(RuntimeValue::String(normalized))
            }
            other => Err(format!(
                "dtcs:normalize_whitespace requires string, got {other:?}"
            )),
        },
        "dtcs:hash_sha256" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::Invalid(_) => Ok(value.clone()),
            RuntimeValue::String(s) => {
                use sha2::{Digest, Sha256};
                let digest = Sha256::digest(s.as_bytes());
                Ok(RuntimeValue::String(hex::encode(digest)))
            }
            other => Err(format!("dtcs:hash_sha256 requires string, got {other:?}")),
        },
        other => Err(format!("unsupported field semantic action '{other}'")),
    }
}

/// Apply an action to a qualified field across all rows in a workspace.
pub fn apply_action_to_rows(
    action_id: &str,
    rows: &mut [Row],
    field_name: &str,
) -> Result<(), String> {
    for row in rows.iter_mut() {
        let updated = match lookup_field(row, field_name) {
            FieldLookup::Missing => {
                if matches!(action_id, "dtcs:lowercase" | "dtcs:uppercase") {
                    return Err(format!(
                        "{action_id} does not accept missing field '{field_name}'"
                    ));
                }
                apply_action(action_id, &RuntimeValue::Null)?
            }
            FieldLookup::Present(value) => apply_action(action_id, &value)?,
        };
        row.insert(field_name.to_string(), updated);
    }
    Ok(())
}

/// Returns true when the action mutates datasets rather than a single field.
#[must_use]
pub fn is_dataset_action(action_id: &str) -> bool {
    matches!(
        action_id,
        "dtcs:project"
            | "dtcs:select"
            | "dtcs:filter"
            | "dtcs:with_fields"
            | "dtcs:rename_fields"
            | "dtcs:drop_fields"
            | "dtcs:aggregate"
            | "dtcs:group"
            | "dtcs:join"
            | "dtcs:sort"
            | "dtcs:union"
            | "dtcs:distinct"
            | "dtcs:deduplicate"
            | "dtcs:limit"
            | "dtcs:partition"
            | "dtcs:window"
    )
}

/// Apply a dataset-level semantic action to workspaces.
pub fn apply_dataset_action(
    action_id: &str,
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    match action_id {
        "dtcs:project" => apply_project(target, parameters, workspaces),
        "dtcs:select" | "dtcs:filter" => apply_filter(target, parameters, workspaces),
        "dtcs:with_fields" => apply_with_fields(target, parameters, workspaces),
        "dtcs:rename_fields" => apply_rename_fields(target, parameters, workspaces),
        "dtcs:drop_fields" => apply_drop_fields(target, parameters, workspaces),
        "dtcs:sort" => apply_sort(target, parameters, workspaces),
        "dtcs:union" => apply_union(target, parameters, workspaces),
        "dtcs:join" => apply_join(target, parameters, workspaces),
        "dtcs:aggregate" | "dtcs:group" => apply_aggregate(target, parameters, workspaces),
        "dtcs:distinct" => apply_distinct(target, parameters, workspaces),
        "dtcs:deduplicate" => apply_deduplicate(target, parameters, workspaces),
        "dtcs:limit" => apply_limit(target, parameters, workspaces),
        "dtcs:partition" => {
            let field = param_string(parameters, "field")?;
            let dataset = workspaces
                .get(target)
                .cloned()
                .ok_or_else(|| format!("unknown interface '{target}'"))?;
            let mut out = Vec::new();
            for mut row in dataset {
                let key = row.get(&field).cloned().unwrap_or(RuntimeValue::Null);
                row.insert("_partition".into(), key);
                out.push(row);
            }
            workspaces.insert(target.to_string(), out);
            Ok(())
        }
        "dtcs:window" => apply_window(target, parameters, workspaces),
        other => Err(format!("unsupported dataset semantic action '{other}'")),
    }
}

fn apply_project(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let fields = parameters
        .get("fields")
        .ok_or_else(|| "missing parameter 'fields'".to_string())?;
    let dataset = workspaces
        .get(target)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    let mut out = Vec::new();
    for row in dataset {
        let mut next = BTreeMap::new();
        match fields {
            Value::Array(items) => {
                for item in items {
                    match item {
                        Value::String(name) => {
                            if let Some(v) = row.get(name) {
                                next.insert(name.clone(), v.clone());
                            }
                        }
                        Value::Object(obj) => {
                            let as_name = obj
                                .get("as")
                                .and_then(Value::as_str)
                                .ok_or_else(|| "project expression requires 'as'".to_string())?;
                            let expr = obj
                                .get("expr")
                                .and_then(Value::as_str)
                                .ok_or_else(|| "project expression requires 'expr'".to_string())?;
                            let value = eval_expression_on_row(expr, &row)?;
                            next.insert(as_name.to_string(), value);
                        }
                        _ => {
                            return Err(
                                "parameter 'fields' must be names or {expr, as} objects".into()
                            )
                        }
                    }
                }
            }
            _ => return Err("parameter 'fields' must be an array".into()),
        }
        out.push(next);
    }
    workspaces.insert(target.to_string(), out);
    Ok(())
}

fn apply_filter(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let dataset = workspaces
        .get_mut(target)
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    if let Some(predicate) = parameters.get("predicate").and_then(Value::as_str) {
        let mut kept = Vec::new();
        for row in dataset.iter() {
            let value = eval_expression_on_row(predicate, row)?;
            match value {
                RuntimeValue::Boolean(true) => kept.push(row.clone()),
                RuntimeValue::Boolean(false)
                | RuntimeValue::Null
                | RuntimeValue::Missing(_) => {}
                RuntimeValue::Invalid(inv) => {
                    return Err(format!(
                        "filter predicate produced invalid{}",
                        inv.reason
                            .as_ref()
                            .map(|r| format!(": {r}"))
                            .unwrap_or_default()
                    ));
                }
                other => {
                    return Err(format!(
                        "filter predicate must be boolean, got {other:?}"
                    ));
                }
            }
        }
        *dataset = kept;
        return Ok(());
    }
    let field = param_string(parameters, "field")?;
    let equals = parameters.get("equals");
    dataset.retain(|row| match lookup_field(row, &field) {
        FieldLookup::Missing => false,
        FieldLookup::Present(value) => match equals {
            None => !value.is_null() && !value.is_invalid(),
            Some(expected) => values_equal(&value, expected),
        },
    });
    Ok(())
}

fn apply_with_fields(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let assignments = parameters
        .get("assignments")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing array parameter 'assignments'".to_string())?;
    let dataset = workspaces
        .get_mut(target)
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    for row in dataset.iter_mut() {
        for item in assignments {
            let obj = item
                .as_object()
                .ok_or_else(|| "assignments entries must be objects".to_string())?;
            let as_name = obj
                .get("as")
                .and_then(Value::as_str)
                .ok_or_else(|| "assignment requires 'as'".to_string())?;
            let expr = obj
                .get("expr")
                .and_then(Value::as_str)
                .ok_or_else(|| "assignment requires 'expr'".to_string())?;
            let value = eval_expression_on_row(expr, row)?;
            row.insert(as_name.to_string(), value);
        }
    }
    Ok(())
}

fn apply_rename_fields(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let mapping = parameters
        .get("mapping")
        .and_then(Value::as_object)
        .ok_or_else(|| "missing object parameter 'mapping'".to_string())?;
    let dataset = workspaces
        .get_mut(target)
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    for row in dataset.iter_mut() {
        for (old, new_val) in mapping {
            let new_name = new_val
                .as_str()
                .ok_or_else(|| "rename mapping values must be strings".to_string())?;
            if let Some(value) = row.remove(old) {
                row.insert(new_name.to_string(), value);
            }
        }
    }
    Ok(())
}

fn apply_drop_fields(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let fields = param_string_list(parameters, "fields")?;
    let missing_policy = parameters
        .get("missingPolicy")
        .and_then(Value::as_str)
        .unwrap_or("error");
    let dataset = workspaces
        .get_mut(target)
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    if missing_policy == "error" {
        for field in &fields {
            if dataset.iter().all(|row| !row.contains_key(field)) && !dataset.is_empty() {
                return Err(format!("drop_fields: missing field '{field}'"));
            }
        }
    }
    for row in dataset.iter_mut() {
        for field in &fields {
            row.remove(field);
        }
    }
    Ok(())
}

fn apply_sort(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let dataset = workspaces
        .get_mut(target)
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    if let Some(keys) = parameters.get("keys").and_then(Value::as_array) {
        let key_specs: Vec<(String, bool, Nulls)> = keys
            .iter()
            .map(|item| {
                let obj = item
                    .as_object()
                    .ok_or_else(|| "sort keys entries must be objects".to_string())?;
                let field = obj
                    .get("expr")
                    .or_else(|| obj.get("field"))
                    .and_then(Value::as_str)
                    .ok_or_else(|| "sort key requires expr or field".to_string())?
                    .to_string();
                let descending = obj
                    .get("descending")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let nulls = match obj.get("nulls").and_then(Value::as_str) {
                    Some("first") => Nulls::First,
                    Some("last") | None => Nulls::Last,
                    Some(other) => return Err(format!("unknown nulls placement '{other}'")),
                };
                Ok((field, descending, nulls))
            })
            .collect::<Result<Vec<_>, String>>()?;
        dataset.sort_by(|a, b| {
            for (field, descending, nulls) in &key_specs {
                let av = a.get(field).cloned().unwrap_or(RuntimeValue::Null);
                let bv = b.get(field).cloned().unwrap_or(RuntimeValue::Null);
                let ord = compare_values_with_nulls(&av, &bv, *nulls);
                if ord != std::cmp::Ordering::Equal {
                    return if *descending { ord.reverse() } else { ord };
                }
            }
            std::cmp::Ordering::Equal
        });
        return Ok(());
    }
    let field = param_string(parameters, "field")?;
    let descending = parameters
        .get("descending")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    dataset.sort_by(|a, b| {
        let av = a.get(&field).cloned().unwrap_or(RuntimeValue::Null);
        let bv = b.get(&field).cloned().unwrap_or(RuntimeValue::Null);
        let ord = compare_values(&av, &bv);
        if descending {
            ord.reverse()
        } else {
            ord
        }
    });
    Ok(())
}

fn apply_union(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let other = param_string(parameters, "other")?;
    let mode = parameters
        .get("mode")
        .and_then(Value::as_str)
        .unwrap_or("positional");
    let missing_policy = parameters
        .get("missingColumnPolicy")
        .and_then(Value::as_str)
        .unwrap_or("nullFill");
    let other_ds = workspaces
        .get(&other)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{other}'"))?;
    let dataset = workspaces
        .get_mut(target)
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    if mode == "byName" {
        let mut columns: BTreeSet<String> = BTreeSet::new();
        for row in dataset.iter().chain(other_ds.iter()) {
            columns.extend(row.keys().cloned());
        }
        let align = |rows: Dataset| -> Result<Dataset, String> {
            rows.into_iter()
                .map(|row| {
                    let mut aligned = BTreeMap::new();
                    for col in &columns {
                        match row.get(col) {
                            Some(v) => {
                                aligned.insert(col.clone(), v.clone());
                            }
                            None if missing_policy == "error" => {
                                return Err(format!("union_by_name missing column '{col}'"));
                            }
                            None => {
                                aligned.insert(col.clone(), RuntimeValue::Null);
                            }
                        }
                    }
                    Ok(aligned)
                })
                .collect()
        };
        let mut left = align(std::mem::take(dataset))?;
        let right = align(other_ds)?;
        left.extend(right);
        *dataset = left;
    } else {
        dataset.extend(other_ds);
    }
    Ok(())
}

fn apply_join(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let right = param_string(parameters, "right")?;
    let join_type = parameters
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("inner");
    let left_key = parameters
        .get("leftKey")
        .and_then(Value::as_str)
        .map(str::to_string);
    let right_key = parameters
        .get("rightKey")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| left_key.clone());
    let null_safe = parameters
        .get("nullSafe")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let right_ds = workspaces
        .get(&right)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{right}'"))?;
    let left_ds = workspaces
        .get(target)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{target}'"))?;

    if join_type == "cross" {
        let mut out = Vec::new();
        for left_row in &left_ds {
            for right_row in &right_ds {
                out.push(merge_rows(left_row, right_row));
            }
        }
        workspaces.insert(target.to_string(), out);
        return Ok(());
    }

    let left_key = left_key.ok_or_else(|| "join requires leftKey or type=cross".to_string())?;
    let right_key = right_key.unwrap_or_else(|| left_key.clone());
    let mut out = Vec::new();
    let mut matched_right = HashSet::new();

    for left_row in &left_ds {
        let lk = left_row
            .get(&left_key)
            .cloned()
            .unwrap_or(RuntimeValue::Null);
        let mut matched = false;
        for (ri, right_row) in right_ds.iter().enumerate() {
            let rk = right_row
                .get(&right_key)
                .cloned()
                .unwrap_or(RuntimeValue::Null);
            if keys_equal(&lk, &rk, null_safe) {
                matched = true;
                matched_right.insert(ri);
                if matches!(join_type, "inner" | "left" | "full" | "right") {
                    out.push(merge_rows(left_row, right_row));
                } else if join_type == "semi" {
                    out.push(left_row.clone());
                    break;
                }
            }
        }
        if !matched {
            match join_type {
                "left" | "full" => out.push(left_row.clone()),
                "anti" => out.push(left_row.clone()),
                _ => {}
            }
        }
    }
    if matches!(join_type, "right" | "full") {
        for (ri, right_row) in right_ds.iter().enumerate() {
            if !matched_right.contains(&ri) {
                out.push(right_row.clone());
            }
        }
    }
    workspaces.insert(target.to_string(), out);
    Ok(())
}

fn apply_aggregate(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    if parameters.get("aggregates").is_some() {
        return apply_multi_aggregate(target, parameters, workspaces);
    }
    let group_by = param_string(parameters, "groupBy")?;
    let value_field = param_string(parameters, "valueField")?;
    let op = parameters
        .get("op")
        .and_then(Value::as_str)
        .unwrap_or("count");
    let dataset = workspaces
        .get(target)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    let mut groups: BTreeMap<GroupKey, Vec<RuntimeValue>> = BTreeMap::new();
    for row in dataset {
        let key = group_key_from_value(row.get(&group_by));
        let value = row.get(&value_field).cloned().unwrap_or(RuntimeValue::Null);
        groups.entry(key).or_default().push(value);
    }
    let mut out = Vec::new();
    for (key, values) in groups {
        let mut row = BTreeMap::new();
        row.insert(group_by.clone(), group_key_to_value(key));
        row.insert(value_field.clone(), reduce_values(&values, op)?);
        out.push(row);
    }
    workspaces.insert(target.to_string(), out);
    Ok(())
}

fn apply_multi_aggregate(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let aggregates = parameters
        .get("aggregates")
        .and_then(Value::as_array)
        .ok_or_else(|| "aggregates must be an array".to_string())?;
    let group_by_specs = match parameters.get("groupBy") {
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| {
                item.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| "groupBy array entries must be field names".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?,
        Some(Value::String(s)) => vec![s.clone()],
        None => Vec::new(),
        Some(_) => return Err("groupBy must be a string or array of strings".into()),
    };
    let filter_expr = parameters.get("filter").and_then(Value::as_str);
    let dataset = workspaces
        .get(target)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{target}'"))?;

    let mut groups: BTreeMap<Vec<GroupKey>, Vec<Row>> = BTreeMap::new();
    for row in dataset {
        if let Some(pred) = filter_expr {
            match eval_expression_on_row(pred, &row)? {
                RuntimeValue::Boolean(true) => {}
                RuntimeValue::Boolean(false)
                | RuntimeValue::Null
                | RuntimeValue::Missing(_) => continue,
                RuntimeValue::Invalid(inv) => {
                    return Err(format!(
                        "aggregate filter produced invalid{}",
                        inv.reason
                            .as_ref()
                            .map(|r| format!(": {r}"))
                            .unwrap_or_default()
                    ));
                }
                other => {
                    return Err(format!("aggregate filter must be boolean, got {other:?}"));
                }
            }
        }
        let keys: Vec<GroupKey> = group_by_specs
            .iter()
            .map(|field| group_key_from_value(row.get(field)))
            .collect();
        groups.entry(keys).or_default().push(row);
    }

    let mut out = Vec::new();
    for (keys, rows) in groups {
        let mut out_row = BTreeMap::new();
        for (field, key) in group_by_specs.iter().zip(keys.into_iter()) {
            out_row.insert(field.clone(), group_key_to_value(key));
        }
        for agg in aggregates {
            let obj = agg
                .as_object()
                .ok_or_else(|| "aggregate entries must be objects".to_string())?;
            let as_name = obj
                .get("as")
                .and_then(Value::as_str)
                .ok_or_else(|| "aggregate requires 'as'".to_string())?;
            let op = obj
                .get("op")
                .or_else(|| obj.get("function"))
                .and_then(Value::as_str)
                .unwrap_or("count");
            let op = op.strip_prefix("dtcs:").unwrap_or(op);
            let values: Vec<RuntimeValue> = if matches!(op, "count_all" | "count(*)") {
                rows.iter().map(|_| RuntimeValue::Integer(1)).collect()
            } else if let Some(expr) = obj.get("expr").and_then(Value::as_str) {
                rows.iter()
                    .map(|row| eval_expression_on_row(expr, row))
                    .collect::<Result<Vec<_>, _>>()?
            } else if let Some(field) = obj.get("field").and_then(Value::as_str) {
                rows.iter()
                    .map(|row| row.get(field).cloned().unwrap_or(RuntimeValue::Null))
                    .collect()
            } else if matches!(op, "count") {
                rows.iter().map(|_| RuntimeValue::Integer(1)).collect()
            } else {
                return Err(format!(
                    "aggregate '{as_name}' requires expr or field (op={op})"
                ));
            };
            let distinct = obj
                .get("distinct")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let values = if distinct || op == "count_distinct" {
                let mut seen = HashSet::new();
                values
                    .into_iter()
                    .filter(|v| seen.insert(format!("{v:?}")))
                    .collect()
            } else {
                values
            };
            let reduced = match op {
                "count_all" | "count(*)" => RuntimeValue::Integer(rows.len() as i64),
                "count_distinct" => RuntimeValue::Integer(values.len() as i64),
                other => reduce_values(&values, other)?,
            };
            out_row.insert(as_name.to_string(), reduced);
        }
        out.push(out_row);
    }
    workspaces.insert(target.to_string(), out);
    Ok(())
}

fn group_key_from_value(value: Option<&RuntimeValue>) -> GroupKey {
    match value {
        Some(RuntimeValue::Null) => GroupKey::Null,
        Some(RuntimeValue::Missing(_)) => GroupKey::Missing,
        Some(RuntimeValue::Invalid(_)) => GroupKey::Invalid,
        Some(RuntimeValue::String(s)) => GroupKey::Present(s.clone()),
        Some(RuntimeValue::Integer(i)) => GroupKey::Present(i.to_string()),
        Some(other) => GroupKey::Present(format!("{other:?}")),
        None => GroupKey::Missing,
    }
}

fn group_key_to_value(key: GroupKey) -> RuntimeValue {
    match key {
        GroupKey::Null => RuntimeValue::Null,
        GroupKey::Missing => RuntimeValue::missing(),
        GroupKey::Invalid => RuntimeValue::invalid("grouping key"),
        GroupKey::Present(s) => RuntimeValue::String(s),
    }
}

fn reduce_values(values: &[RuntimeValue], op: &str) -> Result<RuntimeValue, String> {
    let op = op.strip_prefix("dtcs:").unwrap_or(op);
    Ok(match op {
        "sum" => RuntimeValue::Decimal(values.iter().filter_map(RuntimeValue::as_decimal).sum()),
        "min" => values
            .iter()
            .filter_map(RuntimeValue::as_decimal)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(RuntimeValue::Decimal)
            .unwrap_or(RuntimeValue::Null),
        "max" => values
            .iter()
            .filter_map(RuntimeValue::as_decimal)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(RuntimeValue::Decimal)
            .unwrap_or(RuntimeValue::Null),
        "avg" | "average" => {
            let nums: Vec<f64> = values.iter().filter_map(RuntimeValue::as_decimal).collect();
            if nums.is_empty() {
                RuntimeValue::Null
            } else {
                RuntimeValue::Decimal(nums.iter().sum::<f64>() / nums.len() as f64)
            }
        }
        "count" => RuntimeValue::Integer(
            values
                .iter()
                .filter(|v| !v.is_null() && !v.is_missing() && !v.is_invalid())
                .count() as i64,
        ),
        "first" => values.first().cloned().unwrap_or(RuntimeValue::Null),
        "last" => values.last().cloned().unwrap_or(RuntimeValue::Null),
        other => {
            return Err(format!("unsupported aggregate op '{other}'"));
        }
    })
}

fn apply_window(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let functions = parameters
        .get("functions")
        .and_then(Value::as_array)
        .ok_or_else(|| "window requires functions array".to_string())?;
    let partition_by: Vec<String> = parameters
        .get("partitionBy")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let order_by: Vec<(String, bool)> = parameters
        .get("orderBy")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    let obj = item.as_object()?;
                    let field = obj
                        .get("expr")
                        .or_else(|| obj.get("field"))
                        .and_then(Value::as_str)?
                        .to_string();
                    let descending = obj
                        .get("descending")
                        .and_then(Value::as_bool)
                        .unwrap_or(false);
                    Some((field, descending))
                })
                .collect()
        })
        .unwrap_or_default();

    let dataset = workspaces
        .get(target)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{target}'"))?;

    let mut partitions: BTreeMap<Vec<String>, Vec<usize>> = BTreeMap::new();
    for (idx, row) in dataset.iter().enumerate() {
        let key: Vec<String> = partition_by
            .iter()
            .map(|f| format!("{:?}", row.get(f).unwrap_or(&RuntimeValue::Null)))
            .collect();
        partitions.entry(key).or_default().push(idx);
    }

    let mut out = dataset.clone();
    for indices in partitions.values() {
        let mut ordered = indices.clone();
        ordered.sort_by(|&a, &b| {
            for (field, descending) in &order_by {
                let av = dataset[a]
                    .get(field)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null);
                let bv = dataset[b]
                    .get(field)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null);
                let ord = compare_values(&av, &bv);
                if ord != std::cmp::Ordering::Equal {
                    return if *descending { ord.reverse() } else { ord };
                }
            }
            a.cmp(&b)
        });

        let mut prev_rank_key: Option<String> = None;
        let mut dense_rank = 0i64;
        let mut rank = 0i64;
        for (pos, &idx) in ordered.iter().enumerate() {
            let rank_key = order_by
                .iter()
                .map(|(f, _)| format!("{:?}", dataset[idx].get(f).unwrap_or(&RuntimeValue::Null)))
                .collect::<Vec<_>>()
                .join("|");
            if prev_rank_key.as_ref() != Some(&rank_key) {
                dense_rank += 1;
                rank = (pos as i64) + 1;
                prev_rank_key = Some(rank_key);
            }
            for func in functions {
                let obj = func
                    .as_object()
                    .ok_or_else(|| "window functions entries must be objects".to_string())?;
                let as_name = obj
                    .get("as")
                    .and_then(Value::as_str)
                    .ok_or_else(|| "window function requires 'as'".to_string())?;
                let fn_name = obj
                    .get("function")
                    .or_else(|| obj.get("op"))
                    .and_then(Value::as_str)
                    .unwrap_or("row_number");
                let fn_name = fn_name.strip_prefix("dtcs:").unwrap_or(fn_name);
                let value = match fn_name {
                    "row_number" => RuntimeValue::Integer((pos as i64) + 1),
                    "rank" => RuntimeValue::Integer(rank),
                    "dense_rank" => RuntimeValue::Integer(dense_rank),
                    "lag" | "lead" => {
                        let offset = obj
                            .get("offset")
                            .and_then(Value::as_i64)
                            .unwrap_or(1)
                            .unsigned_abs() as usize;
                        let expr = obj
                            .get("expr")
                            .or_else(|| obj.get("field"))
                            .and_then(Value::as_str)
                            .ok_or_else(|| format!("{fn_name} requires expr or field"))?;
                        let source_pos = if fn_name == "lag" {
                            pos.checked_sub(offset)
                        } else {
                            let next = pos + offset;
                            (next < ordered.len()).then_some(next)
                        };
                        match source_pos {
                            Some(sp) => {
                                let source_idx = ordered[sp];
                                eval_expression_on_row(expr, &dataset[source_idx])?
                            }
                            None => obj
                                .get("default")
                                .map(|v| match v {
                                    Value::Null => RuntimeValue::Null,
                                    Value::String(s) => RuntimeValue::String(s.clone()),
                                    Value::Number(n) => n
                                        .as_i64()
                                        .map(RuntimeValue::Integer)
                                        .or_else(|| n.as_f64().map(RuntimeValue::Decimal))
                                        .unwrap_or(RuntimeValue::Null),
                                    Value::Bool(b) => RuntimeValue::Boolean(*b),
                                    _ => RuntimeValue::Null,
                                })
                                .unwrap_or(RuntimeValue::Null),
                        }
                    }
                    other => {
                        return Err(format!("unsupported window function '{other}'"));
                    }
                };
                out[idx].insert(as_name.to_string(), value);
            }
        }
    }
    workspaces.insert(target.to_string(), out);
    Ok(())
}

fn apply_distinct(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let fields = parameters
        .get("fields")
        .map(|v| param_string_list_from_value(v, "fields"))
        .transpose()?;
    let dataset = workspaces
        .get(target)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for row in dataset {
        let key = row_fingerprint(&row, fields.as_deref());
        if seen.insert(key) {
            out.push(row);
        }
    }
    workspaces.insert(target.to_string(), out);
    Ok(())
}

fn apply_deduplicate(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let keys = param_string_list(parameters, "keys")?;
    let retain = parameters
        .get("retain")
        .and_then(Value::as_str)
        .unwrap_or("first");
    let dataset = workspaces
        .get(target)
        .cloned()
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    let mut map: IndexMap<String, Row> = IndexMap::new();
    for row in dataset {
        let key = row_fingerprint(&row, Some(&keys));
        match retain {
            "first" => {
                map.entry(key).or_insert(row);
            }
            "last" => {
                map.insert(key, row);
            }
            other => return Err(format!("unsupported deduplicate retain '{other}'")),
        }
    }
    workspaces.insert(target.to_string(), map.into_values().collect());
    Ok(())
}

fn apply_limit(
    target: &str,
    parameters: &IndexMap<String, Value>,
    workspaces: &mut BTreeMap<String, Dataset>,
) -> Result<(), String> {
    let count = parameters
        .get("count")
        .and_then(Value::as_u64)
        .ok_or_else(|| "limit requires integer 'count'".to_string())? as usize;
    let offset = parameters
        .get("offset")
        .and_then(Value::as_u64)
        .unwrap_or(0) as usize;
    let dataset = workspaces
        .get_mut(target)
        .ok_or_else(|| format!("unknown interface '{target}'"))?;
    let end = offset.saturating_add(count).min(dataset.len());
    let start = offset.min(dataset.len());
    *dataset = dataset[start..end].to_vec();
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum GroupKey {
    Null,
    Missing,
    Invalid,
    Present(String),
}

#[derive(Debug, Clone, Copy)]
enum Nulls {
    First,
    Last,
}

fn merge_rows(left: &Row, right: &Row) -> Row {
    let mut merged = left.clone();
    for (k, v) in right {
        merged.entry(k.clone()).or_insert_with(|| v.clone());
    }
    merged
}

/// Ordinary equality does not match null/missing/invalid keys unless `null_safe`.
fn keys_equal(a: &RuntimeValue, b: &RuntimeValue, null_safe: bool) -> bool {
    if matches!(
        a,
        RuntimeValue::Null | RuntimeValue::Missing(_) | RuntimeValue::Invalid(_)
    ) || matches!(
        b,
        RuntimeValue::Null | RuntimeValue::Missing(_) | RuntimeValue::Invalid(_)
    ) {
        if null_safe {
            return std::mem::discriminant(a) == std::mem::discriminant(b)
                && matches!(
                    a,
                    RuntimeValue::Null | RuntimeValue::Missing(_) | RuntimeValue::Invalid(_)
                );
        }
        return false;
    }
    a == b
}

fn param_string(parameters: &IndexMap<String, Value>, key: &str) -> Result<String, String> {
    parameters
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("missing string parameter '{key}'"))
}

fn param_string_list(
    parameters: &IndexMap<String, Value>,
    key: &str,
) -> Result<Vec<String>, String> {
    let value = parameters
        .get(key)
        .ok_or_else(|| format!("missing parameter '{key}'"))?;
    param_string_list_from_value(value, key)
}

fn param_string_list_from_value(value: &Value, key: &str) -> Result<Vec<String>, String> {
    match value {
        Value::Array(items) => items
            .iter()
            .map(|item| {
                item.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| format!("parameter '{key}' must be an array of strings"))
            })
            .collect(),
        _ => Err(format!("parameter '{key}' must be an array of strings")),
    }
}

fn values_equal(value: &RuntimeValue, expected: &Value) -> bool {
    match (value, expected) {
        (RuntimeValue::String(s), Value::String(e)) => s == e,
        (RuntimeValue::Integer(i), Value::Number(n)) => n.as_i64() == Some(*i),
        (RuntimeValue::Boolean(b), Value::Bool(e)) => b == e,
        (RuntimeValue::Null, Value::Null) => true,
        _ => false,
    }
}

fn compare_values(a: &RuntimeValue, b: &RuntimeValue) -> std::cmp::Ordering {
    compare_values_with_nulls(a, b, Nulls::Last)
}

fn compare_values_with_nulls(a: &RuntimeValue, b: &RuntimeValue, nulls: Nulls) -> std::cmp::Ordering {
    let a_null = matches!(
        a,
        RuntimeValue::Null | RuntimeValue::Missing(_) | RuntimeValue::Invalid(_)
    );
    let b_null = matches!(
        b,
        RuntimeValue::Null | RuntimeValue::Missing(_) | RuntimeValue::Invalid(_)
    );
    if a_null || b_null {
        return match (a_null, b_null, nulls) {
            (true, true, _) => std::cmp::Ordering::Equal,
            (true, false, Nulls::First) => std::cmp::Ordering::Less,
            (true, false, Nulls::Last) => std::cmp::Ordering::Greater,
            (false, true, Nulls::First) => std::cmp::Ordering::Greater,
            (false, true, Nulls::Last) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        };
    }
    match (a, b) {
        (RuntimeValue::Integer(x), RuntimeValue::Integer(y)) => x.cmp(y),
        (RuntimeValue::String(x), RuntimeValue::String(y)) => x.cmp(y),
        (RuntimeValue::Decimal(x), RuntimeValue::Decimal(y)) => {
            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
        }
        _ => std::cmp::Ordering::Equal,
    }
}

fn row_fingerprint(row: &Row, fields: Option<&[String]>) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let keys: Vec<&String> = match fields {
        Some(fields) => fields.iter().collect(),
        None => {
            let mut all: Vec<&String> = row.keys().collect();
            all.sort();
            all
        }
    };
    for key in keys {
        key.hash(&mut hasher);
        match row.get(key) {
            Some(v) => format!("{v:?}").hash(&mut hasher),
            None => "missing".hash(&mut hasher),
        }
    }
    format!("{:x}", hasher.finish())
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
    }
}
