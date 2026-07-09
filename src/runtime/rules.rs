//! Rule evaluation.

use std::collections::BTreeMap;

use indexmap::IndexMap;
use regex::Regex;
use serde_json::Value;

use crate::model::Rule;
use crate::runtime::model::{parse_qualified_field, RuntimeValue};

/// Evaluate a rule against a workspace value.
pub fn evaluate_rule(
    rule: &Rule,
    value: &RuntimeValue,
    parameters: &IndexMap<String, Value>,
) -> Result<(), String> {
    match rule.rule.as_str() {
        "dtcs:not_null" => {
            if value.is_null() {
                return Err(format!("rule '{}' violated: value is null", rule.id));
            }
            Ok(())
        }
        "dtcs:min_length" => {
            let min = param_integer(parameters, "min")?;
            match value {
                RuntimeValue::Null => Ok(()),
                RuntimeValue::String(s) if (s.chars().count() as i64) >= min => Ok(()),
                RuntimeValue::String(s) => Err(format!(
                    "rule '{}' violated: length {} < min {}",
                    rule.id,
                    s.chars().count(),
                    min
                )),
                other => Err(format!("dtcs:min_length requires string, got {other:?}")),
            }
        }
        "dtcs:max_length" => {
            let max = param_integer(parameters, "max")?;
            match value {
                RuntimeValue::Null => Ok(()),
                RuntimeValue::String(s) if (s.chars().count() as i64) <= max => Ok(()),
                RuntimeValue::String(s) => Err(format!(
                    "rule '{}' violated: length {} > max {}",
                    rule.id,
                    s.chars().count(),
                    max
                )),
                other => Err(format!("dtcs:max_length requires string, got {other:?}")),
            }
        }
        "dtcs:regex_match" => {
            let pattern = param_string(parameters, "pattern")?;
            match value {
                RuntimeValue::Null => Ok(()),
                RuntimeValue::String(s) => {
                    let re = Regex::new(&pattern)
                        .map_err(|e| format!("invalid regex pattern '{pattern}': {e}"))?;
                    if re.is_match(s) {
                        Ok(())
                    } else {
                        Err(format!(
                            "rule '{}' violated: value does not match pattern '{pattern}'",
                            rule.id
                        ))
                    }
                }
                other => Err(format!("dtcs:regex_match requires string, got {other:?}")),
            }
        }
        "dtcs:range" => {
            let min = parameters.get("min").and_then(value_as_integer);
            let max = parameters.get("max").and_then(value_as_integer);
            match value {
                RuntimeValue::Null => Ok(()),
                RuntimeValue::Integer(v) => {
                    if let Some(min) = min {
                        if *v < min {
                            return Err(format!("rule '{}' violated: {v} < min {min}", rule.id));
                        }
                    }
                    if let Some(max) = max {
                        if *v > max {
                            return Err(format!("rule '{}' violated: {v} > max {max}", rule.id));
                        }
                    }
                    Ok(())
                }
                other => Err(format!("dtcs:range requires integer, got {other:?}")),
            }
        }
        other => Err(format!("unsupported rule '{other}'")),
    }
}

fn param_integer(parameters: &IndexMap<String, Value>, name: &str) -> Result<i64, String> {
    parameters
        .get(name)
        .and_then(value_as_integer)
        .ok_or_else(|| format!("missing integer parameter '{name}'"))
}

fn param_string(parameters: &IndexMap<String, Value>, name: &str) -> Result<String, String> {
    parameters
        .get(name)
        .and_then(|v| v.as_str().map(str::to_string))
        .ok_or_else(|| format!("missing string parameter '{name}'"))
}

fn value_as_integer(value: &Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_u64().map(|v| v as i64))
}

/// Resolve a rule target value from workspaces.
pub fn resolve_target(
    workspaces: &BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>>,
    target: &str,
    row_index: usize,
) -> Result<RuntimeValue, String> {
    let qualified =
        parse_qualified_field(target).ok_or_else(|| format!("invalid target '{target}'"))?;
    let rows = workspaces
        .get(&qualified.interface_id)
        .ok_or_else(|| format!("unknown interface '{}'", qualified.interface_id))?;
    let row = rows
        .get(row_index)
        .ok_or_else(|| format!("row index {row_index} out of range"))?;
    Ok(row
        .get(&qualified.field_name)
        .cloned()
        .unwrap_or(RuntimeValue::Null))
}
