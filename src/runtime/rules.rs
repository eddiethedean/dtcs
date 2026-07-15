//! Rule evaluation.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use indexmap::IndexMap;
use regex::Regex;
use serde_json::Value;

use crate::model::{Rule, RuleOutcome};
use crate::runtime::model::{
    lookup_field, parse_qualified_field_with_interfaces, FieldLookup, RuntimeValue,
};

/// Evaluate a rule against a workspace value, returning a typed outcome.
pub fn evaluate_rule_outcome(
    rule: &Rule,
    value: &RuntimeValue,
    parameters: &IndexMap<String, Value>,
) -> Result<RuleOutcome, String> {
    match evaluate_rule(rule, value, parameters) {
        Ok(()) => Ok(RuleOutcome::Satisfied),
        Err(message) if message.contains("indeterminate") => Ok(RuleOutcome::Indeterminate),
        Err(message) if message.contains("violated") => Ok(RuleOutcome::Violated),
        Err(message) => Err(message),
    }
}

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
            if value.is_invalid() {
                return Err(format!("rule '{}' violated: value is invalid", rule.id));
            }
            Ok(())
        }
        "dtcs:min_length" => {
            let min = param_integer(parameters, "min")?;
            match value {
                RuntimeValue::Null => Err(format!("rule '{}' violated: value is null", rule.id)),
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
                RuntimeValue::Null => Err(format!("rule '{}' violated: value is null", rule.id)),
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
                RuntimeValue::Null => Err(format!("rule '{}' violated: value is null", rule.id)),
                RuntimeValue::String(s) => {
                    let re = cached_regex(&pattern)?;
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
                RuntimeValue::Null => Err(format!("rule '{}' violated: value is null", rule.id)),
                RuntimeValue::Integer(v) => check_range(&rule.id, *v, min, max),
                RuntimeValue::Decimal(v) => {
                    if v.fract() != 0.0 {
                        return Err(format!(
                            "rule '{}' violated: decimal {v} is not an integer value",
                            rule.id
                        ));
                    }
                    check_range(&rule.id, *v as i64, min, max)
                }
                other => Err(format!("dtcs:range requires numeric value, got {other:?}")),
            }
        }
        "dtcs:one_of" => {
            let values = parameters
                .get("values")
                .and_then(Value::as_array)
                .ok_or_else(|| "missing values parameter".to_string())?;
            match value {
                RuntimeValue::Null => Err(format!("rule '{}' violated: value is null", rule.id)),
                RuntimeValue::String(s) => {
                    if values.iter().any(|v| v.as_str() == Some(s.as_str())) {
                        Ok(())
                    } else {
                        Err(format!(
                            "rule '{}' violated: value not in allowed set",
                            rule.id
                        ))
                    }
                }
                other => Err(format!("dtcs:one_of requires string, got {other:?}")),
            }
        }
        "dtcs:equals" => {
            let expected = param_string(parameters, "value")?;
            match value {
                RuntimeValue::Null if rule.allow_indeterminate => {
                    Err(format!("rule '{}' indeterminate: value is null", rule.id))
                }
                RuntimeValue::Null => Err(format!("rule '{}' violated: value is null", rule.id)),
                RuntimeValue::String(s) if s == &expected => Ok(()),
                RuntimeValue::String(_) => Err(format!(
                    "rule '{}' violated: value does not equal expected",
                    rule.id
                )),
                other => Err(format!("dtcs:equals requires string, got {other:?}")),
            }
        }
        other => Err(format!("unsupported rule '{other}'")),
    }
}

fn check_range(rule_id: &str, v: i64, min: Option<i64>, max: Option<i64>) -> Result<(), String> {
    if let Some(min) = min {
        if v < min {
            return Err(format!("rule '{rule_id}' violated: {v} < min {min}"));
        }
    }
    if let Some(max) = max {
        if v > max {
            return Err(format!("rule '{rule_id}' violated: {v} > max {max}"));
        }
    }
    Ok(())
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
    value
        .as_i64()
        .or_else(|| value.as_u64().and_then(|v| v.try_into().ok()))
}

fn regex_cache() -> &'static Mutex<HashMap<String, Result<Regex, String>>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Result<Regex, String>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cached_regex(pattern: &str) -> Result<Regex, String> {
    let mut cache = regex_cache()
        .lock()
        .map_err(|_| "regex cache lock poisoned".to_string())?;
    if let Some(entry) = cache.get(pattern) {
        return match entry {
            Ok(regex) => Ok(regex.clone()),
            Err(message) => Err(message.clone()),
        };
    }
    let compiled =
        Regex::new(pattern).map_err(|e| format!("invalid regex pattern '{pattern}': {e}"));
    cache.insert(pattern.to_string(), compiled.clone());
    compiled
}

/// Resolve a rule target value from workspaces.
pub fn resolve_target(
    workspaces: &BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>>,
    target: &str,
    row_index: usize,
) -> Result<RuntimeValue, String> {
    let interface_ids: Vec<String> = workspaces.keys().cloned().collect();
    let qualified = parse_qualified_field_with_interfaces(target, &interface_ids)
        .ok_or_else(|| format!("invalid target '{target}'"))?;
    let rows = workspaces
        .get(&qualified.interface_id)
        .ok_or_else(|| format!("unknown interface '{}'", qualified.interface_id))?;
    let row = rows
        .get(row_index)
        .ok_or_else(|| format!("row index {row_index} out of range"))?;
    match lookup_field(row, &qualified.field_name) {
        FieldLookup::Missing => Err(format!(
            "missing field '{}' on interface '{}'",
            qualified.field_name, qualified.interface_id
        )),
        FieldLookup::Present(value) => Ok(value),
    }
}
