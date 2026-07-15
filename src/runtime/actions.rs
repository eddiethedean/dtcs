//! Semantic action execution.

use std::collections::BTreeMap;

use indexmap::IndexMap;
use serde_json::Value;

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
                // Distinguish missing from null: treat as missing → null for null-tolerant
                // transforms; reject for reject-null actions.
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
            | "dtcs:aggregate"
            | "dtcs:group"
            | "dtcs:join"
            | "dtcs:sort"
            | "dtcs:union"
            | "dtcs:partition"
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
        "dtcs:project" => {
            let fields = param_string_list(parameters, "fields")?;
            let dataset = workspaces
                .get_mut(target)
                .ok_or_else(|| format!("unknown interface '{target}'"))?;
            for row in dataset.iter_mut() {
                row.retain(|k, _| fields.iter().any(|f| f == k));
            }
            Ok(())
        }
        "dtcs:select" | "dtcs:filter" => {
            let field = param_string(parameters, "field")?;
            let equals = parameters.get("equals");
            let dataset = workspaces
                .get_mut(target)
                .ok_or_else(|| format!("unknown interface '{target}'"))?;
            dataset.retain(|row| match lookup_field(row, &field) {
                FieldLookup::Missing => false,
                FieldLookup::Present(value) => match equals {
                    None => !value.is_null() && !value.is_invalid(),
                    Some(expected) => values_equal(&value, expected),
                },
            });
            Ok(())
        }
        "dtcs:sort" => {
            let field = param_string(parameters, "field")?;
            let descending = parameters
                .get("descending")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let dataset = workspaces
                .get_mut(target)
                .ok_or_else(|| format!("unknown interface '{target}'"))?;
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
        "dtcs:union" => {
            let other = param_string(parameters, "other")?;
            let other_ds = workspaces
                .get(&other)
                .cloned()
                .ok_or_else(|| format!("unknown interface '{other}'"))?;
            let dataset = workspaces
                .get_mut(target)
                .ok_or_else(|| format!("unknown interface '{target}'"))?;
            dataset.extend(other_ds);
            Ok(())
        }
        "dtcs:join" => {
            let right = param_string(parameters, "right")?;
            let left_key = param_string(parameters, "leftKey")?;
            let right_key = parameters
                .get("rightKey")
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| left_key.clone());
            let right_ds = workspaces
                .get(&right)
                .cloned()
                .ok_or_else(|| format!("unknown interface '{right}'"))?;
            let left_ds = workspaces
                .get(target)
                .cloned()
                .ok_or_else(|| format!("unknown interface '{target}'"))?;
            let mut out = Vec::new();
            for left_row in &left_ds {
                let lk = left_row
                    .get(&left_key)
                    .cloned()
                    .unwrap_or(RuntimeValue::Null);
                for right_row in &right_ds {
                    let rk = right_row
                        .get(&right_key)
                        .cloned()
                        .unwrap_or(RuntimeValue::Null);
                    if values_equal_runtime(&lk, &rk) {
                        let mut merged = left_row.clone();
                        for (k, v) in right_row {
                            merged.entry(k.clone()).or_insert_with(|| v.clone());
                        }
                        out.push(merged);
                    }
                }
            }
            workspaces.insert(target.to_string(), out);
            Ok(())
        }
        "dtcs:aggregate" | "dtcs:group" => {
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
            let mut groups: BTreeMap<String, Vec<RuntimeValue>> = BTreeMap::new();
            for row in dataset {
                let key = match row.get(&group_by) {
                    Some(RuntimeValue::String(s)) => s.clone(),
                    Some(RuntimeValue::Integer(i)) => i.to_string(),
                    Some(other) => format!("{other:?}"),
                    None => String::new(),
                };
                let value = row.get(&value_field).cloned().unwrap_or(RuntimeValue::Null);
                groups.entry(key).or_default().push(value);
            }
            let mut out = Vec::new();
            for (key, values) in groups {
                let mut row = BTreeMap::new();
                row.insert(group_by.clone(), RuntimeValue::String(key));
                let agg = match op {
                    "sum" => RuntimeValue::Decimal(
                        values.iter().filter_map(RuntimeValue::as_decimal).sum(),
                    ),
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
                    _ => RuntimeValue::Integer(values.len() as i64),
                };
                row.insert(value_field.clone(), agg);
                out.push(row);
            }
            workspaces.insert(target.to_string(), out);
            Ok(())
        }
        "dtcs:partition" => {
            let field = param_string(parameters, "field")?;
            let dataset = workspaces
                .get(target)
                .cloned()
                .ok_or_else(|| format!("unknown interface '{target}'"))?;
            // Partition keeps rows but annotates a partition key field.
            let mut out = Vec::new();
            for mut row in dataset {
                let key = row.get(&field).cloned().unwrap_or(RuntimeValue::Null);
                row.insert("_partition".into(), key);
                out.push(row);
            }
            workspaces.insert(target.to_string(), out);
            Ok(())
        }
        other => Err(format!("unsupported dataset semantic action '{other}'")),
    }
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

fn values_equal_runtime(a: &RuntimeValue, b: &RuntimeValue) -> bool {
    a == b
}

fn compare_values(a: &RuntimeValue, b: &RuntimeValue) -> std::cmp::Ordering {
    match (a, b) {
        (RuntimeValue::Integer(x), RuntimeValue::Integer(y)) => x.cmp(y),
        (RuntimeValue::String(x), RuntimeValue::String(y)) => x.cmp(y),
        (RuntimeValue::Decimal(x), RuntimeValue::Decimal(y)) => {
            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
        }
        _ => std::cmp::Ordering::Equal,
    }
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
    }
}
