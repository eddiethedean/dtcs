//! Portable semantic-family differential fixtures (proposal R3).

use std::collections::BTreeMap;
use std::path::Path;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::analysis::expr::{format_expression, from_structured_node, to_structured_node};
use crate::runtime::actions::apply_dataset_action;
use crate::runtime::{Dataset, Row, RuntimeValue};

use super::fixtures::read_fixture;
use super::model::ConformanceTestResult;

/// One action step in a portable differential fixture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableActionStep {
    /// Registry action id (`dtcs:…`).
    pub action: String,
    /// Target interface / dataset id.
    pub target: String,
    /// Action parameters.
    #[serde(default)]
    pub parameters: IndexMap<String, Value>,
}

/// Portable differential fixture document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableDifferentialFixture {
    /// Stable fixture id.
    pub id: String,
    /// Action sequence to apply.
    pub actions: Vec<PortableActionStep>,
    /// Input datasets keyed by interface id.
    pub input: BTreeMap<String, Vec<Value>>,
    /// Expected datasets keyed by interface id.
    pub expected: BTreeMap<String, Vec<Value>>,
    /// Optional expected error substring (negative cases).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expect_error: Option<String>,
}

/// Evaluation mode for dual-path conformance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortableEvalMode {
    /// Evaluate expression strings directly.
    Direct,
    /// Lower strings to structured nodes and reformat before evaluation.
    StructuredLowering,
}

/// Run a portable differential fixture under one evaluation mode.
pub fn run_portable_fixture(
    fixture: &PortableDifferentialFixture,
    mode: PortableEvalMode,
) -> Result<BTreeMap<String, Dataset>, String> {
    let mut workspaces: BTreeMap<String, Dataset> = BTreeMap::new();
    for (name, rows) in &fixture.input {
        let mut dataset = Vec::new();
        for row in rows {
            dataset.push(json_row_to_runtime(row)?);
        }
        workspaces.insert(name.clone(), dataset);
    }

    for step in &fixture.actions {
        let params = match mode {
            PortableEvalMode::Direct => step.parameters.clone(),
            PortableEvalMode::StructuredLowering => lower_expr_params(&step.parameters)?,
        };
        apply_dataset_action(&step.action, &step.target, &params, &mut workspaces)?;
    }
    Ok(workspaces)
}

/// Compare runtime datasets to expected JSON rows.
pub fn datasets_match_expected(
    actual: &BTreeMap<String, Dataset>,
    expected: &BTreeMap<String, Vec<Value>>,
) -> Result<(), String> {
    for (name, expected_rows) in expected {
        let actual_rows = actual
            .get(name)
            .ok_or_else(|| format!("missing output dataset '{name}'"))?;
        let expected_runtime: Dataset = expected_rows
            .iter()
            .map(json_row_to_runtime)
            .collect::<Result<_, _>>()?;
        if actual_rows != &expected_runtime {
            return Err(format!(
                "dataset '{name}' mismatch: got {actual_rows:?}, expected {expected_runtime:?}"
            ));
        }
    }
    Ok(())
}

/// Load and execute a portable differential fixture for conformance.
pub fn run_portable_differential_case(
    fixtures_dir: &Path,
    relative: &str,
    test_id: &str,
    profile_id: &str,
) -> ConformanceTestResult {
    let bytes = match read_fixture(fixtures_dir, relative) {
        Ok(bytes) => bytes,
        Err(err) => {
            return ConformanceTestResult {
                id: test_id.into(),
                profile: profile_id.into(),
                passed: false,
                message: Some(err),
            };
        }
    };
    let fixture: PortableDifferentialFixture = match serde_json::from_slice(&bytes) {
        Ok(f) => f,
        Err(err) => {
            return ConformanceTestResult {
                id: test_id.into(),
                profile: profile_id.into(),
                passed: false,
                message: Some(format!("parse portable fixture: {err}")),
            };
        }
    };

    for mode in [
        PortableEvalMode::Direct,
        PortableEvalMode::StructuredLowering,
    ] {
        let result = run_portable_fixture(&fixture, mode);
        if let Some(expect_err) = &fixture.expect_error {
            match result {
                Err(err) if err.contains(expect_err) => continue,
                Err(err) => {
                    return ConformanceTestResult {
                        id: test_id.into(),
                        profile: profile_id.into(),
                        passed: false,
                        message: Some(format!(
                            "mode {mode:?}: expected error containing '{expect_err}', got '{err}'"
                        )),
                    };
                }
                Ok(_) => {
                    return ConformanceTestResult {
                        id: test_id.into(),
                        profile: profile_id.into(),
                        passed: false,
                        message: Some(format!(
                            "mode {mode:?}: expected error containing '{expect_err}'"
                        )),
                    };
                }
            }
        } else {
            let outputs = match result {
                Ok(outputs) => outputs,
                Err(err) => {
                    return ConformanceTestResult {
                        id: test_id.into(),
                        profile: profile_id.into(),
                        passed: false,
                        message: Some(format!("mode {mode:?}: {err}")),
                    };
                }
            };
            if let Err(err) = datasets_match_expected(&outputs, &fixture.expected) {
                return ConformanceTestResult {
                    id: test_id.into(),
                    profile: profile_id.into(),
                    passed: false,
                    message: Some(format!("mode {mode:?}: {err}")),
                };
            }
        }
    }

    ConformanceTestResult {
        id: test_id.into(),
        profile: profile_id.into(),
        passed: true,
        message: None,
    }
}

fn lower_expr_params(
    parameters: &IndexMap<String, Value>,
) -> Result<IndexMap<String, Value>, String> {
    let mut out = IndexMap::new();
    for (key, value) in parameters {
        if matches!(
            key.as_str(),
            "expr" | "condition" | "predicate" | "on" | "filter"
        ) && value.is_string()
        {
            let s = value.as_str().unwrap();
            let node = to_structured_node(s)?;
            let expr = from_structured_node(&node)?;
            out.insert(key.clone(), Value::String(format_expression(&expr)));
        } else {
            out.insert(key.clone(), lower_value(value)?);
        }
    }
    Ok(out)
}

fn lower_value(value: &Value) -> Result<Value, String> {
    match value {
        Value::Array(items) => {
            let lowered: Result<Vec<_>, _> = items.iter().map(lower_value).collect();
            Ok(Value::Array(lowered?))
        }
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                if matches!(
                    k.as_str(),
                    "expr" | "condition" | "predicate" | "on" | "filter"
                ) && v.is_string()
                {
                    let s = v.as_str().unwrap();
                    let node = to_structured_node(s)?;
                    let expr = from_structured_node(&node)?;
                    out.insert(k.clone(), Value::String(format_expression(&expr)));
                } else {
                    out.insert(k.clone(), lower_value(v)?);
                }
            }
            Ok(Value::Object(out))
        }
        other => Ok(other.clone()),
    }
}

fn json_row_to_runtime(value: &Value) -> Result<Row, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "portable fixture rows must be objects".to_string())?;
    let mut row = BTreeMap::new();
    for (k, v) in obj {
        row.insert(k.clone(), json_to_runtime(v)?);
    }
    Ok(row)
}

fn json_to_runtime(value: &Value) -> Result<RuntimeValue, String> {
    Ok(match value {
        Value::Null => RuntimeValue::Null,
        Value::Bool(b) => RuntimeValue::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                RuntimeValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                RuntimeValue::Decimal(f)
            } else {
                return Err(format!("unsupported number {n}"));
            }
        }
        Value::String(s) => RuntimeValue::String(s.clone()),
        Value::Array(items) => {
            let values: Result<Vec<_>, _> = items.iter().map(json_to_runtime).collect();
            RuntimeValue::List(values?)
        }
        Value::Object(map) => {
            if map.len() == 1 {
                if let Some(token) = map.get("$missing") {
                    let _ = token;
                    return Ok(RuntimeValue::missing());
                }
                if let Some(reason) = map.get("$invalid").and_then(Value::as_str) {
                    return Ok(RuntimeValue::invalid(reason));
                }
            }
            let mut out = BTreeMap::new();
            for (k, v) in map {
                out.insert(k.clone(), json_to_runtime(v)?);
            }
            RuntimeValue::Map(out)
        }
    })
}
