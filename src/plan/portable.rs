//! Portable Transformation Plan serialization (`dtcs.transform-plan/1`).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::model::TransformationPlan;

/// Canonical portable plan serialization identity.
pub const TRANSFORM_PLAN_IDENTITY: &str = "dtcs.transform-plan/1";

/// Default portable relational kernel profile.
pub const KERNEL_PROFILE: &str = "dtcs:profile/portable-relational-kernel/1";

/// Full portable relational profile.
pub const RELATIONAL_PROFILE: &str = "dtcs:profile/portable-relational/1";

/// Portable window profile.
pub const WINDOW_PROFILE: &str = "dtcs:profile/portable-window/1";

/// Portable complex-types profile.
pub const COMPLEX_TYPES_PROFILE: &str = "dtcs:profile/portable-complex-types/1";

/// Default security budgets for portable plans (Chapter 13 §12.1).
pub const MAX_PORTABLE_PLAN_BYTES: usize = 8 * 1024 * 1024;
/// Maximum nesting depth for portable plan JSON.
pub const MAX_PORTABLE_PLAN_DEPTH: usize = 128;
/// Maximum action/expression nodes in a portable plan.
pub const MAX_PORTABLE_PLAN_NODES: usize = 100_000;

/// Pinned registry versions recorded in a portable plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegistryVersions {
    /// Semantic actions registry document version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    /// Functions registry document version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub functions: Option<String>,
    /// Operators registry document version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operators: Option<String>,
    /// Types registry / catalog version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
}

impl RegistryVersions {
    /// Builtin registry versions shipped with this crate.
    #[must_use]
    pub fn builtin() -> Self {
        Self {
            actions: Some("2.0.0".into()),
            functions: Some("2.0.0".into()),
            operators: Some("1.0.0".into()),
            types: Some("1.0.0".into()),
        }
    }
}

/// Canonical portable plan envelope (`dtcs.transform-plan/1`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortablePlan {
    /// Serialization identity (always `dtcs.transform-plan/1` when canonical).
    #[serde(default = "default_plan_identity")]
    pub plan_identity: String,
    /// Portable profile identifier.
    pub profile: String,
    /// Governing DTCS specification version.
    pub specification_version: String,
    /// Pinned registry versions.
    #[serde(default)]
    pub registry_versions: RegistryVersions,
    /// Originating transformation / contract identity.
    pub transformation: String,
    /// Named inputs (interface id → schema / metadata).
    #[serde(default)]
    pub inputs: IndexMap<String, Value>,
    /// Plan parameters (runtime values remain outside portable plans).
    #[serde(default)]
    pub parameters: IndexMap<String, Value>,
    /// Ordered semantic actions (data-only).
    #[serde(default)]
    pub actions: Vec<Value>,
    /// Named outputs.
    #[serde(default)]
    pub outputs: IndexMap<String, Value>,
    /// Rules.
    #[serde(default)]
    pub rules: Vec<Value>,
    /// Lineage mappings.
    #[serde(default)]
    pub lineage: Vec<Value>,
    /// Capability / engine requirements.
    #[serde(default)]
    pub requirements: IndexMap<String, Value>,
    /// Vendor extensions.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub extensions: IndexMap<String, Value>,
}

fn default_plan_identity() -> String {
    TRANSFORM_PLAN_IDENTITY.into()
}

impl PortablePlan {
    /// Export an internal COM plan to the portable envelope.
    #[must_use]
    pub fn from_transformation_plan(plan: &TransformationPlan, profile: &str) -> Self {
        let mut inputs = IndexMap::new();
        for input in &plan.inputs {
            if let Ok(value) = serde_json::to_value(input) {
                inputs.insert(input.id.clone(), value);
            }
        }
        let mut outputs = IndexMap::new();
        for output in &plan.outputs {
            if let Ok(value) = serde_json::to_value(output) {
                outputs.insert(output.id.clone(), value);
            }
        }
        let mut actions = Vec::new();
        let mut rules = Vec::new();
        for node in &plan.nodes {
            if let Ok(value) = serde_json::to_value(node) {
                match &node.kind {
                    super::model::PlanNodeKind::Rule(_) => rules.push(value),
                    _ => actions.push(value),
                }
            }
        }
        let lineage = plan
            .lineage
            .as_ref()
            .and_then(|l| serde_json::to_value(l).ok())
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default();

        let mut requirements = IndexMap::new();
        requirements.insert(
            "planIdentity".into(),
            Value::String(TRANSFORM_PLAN_IDENTITY.into()),
        );
        if !plan.dependencies.is_empty() {
            if let Ok(deps) = serde_json::to_value(&plan.dependencies) {
                requirements.insert("dependencies".into(), deps);
            }
        }

        Self {
            plan_identity: TRANSFORM_PLAN_IDENTITY.into(),
            profile: profile.to_string(),
            specification_version: plan.identity.dtcs_version.clone(),
            registry_versions: RegistryVersions::builtin(),
            transformation: plan.identity.id.clone(),
            inputs,
            parameters: IndexMap::new(),
            actions: actions.into_iter().map(lower_action_value_exprs).collect(),
            outputs,
            rules,
            lineage,
            requirements,
            extensions: plan.extensions.clone(),
        }
    }

    /// Serialize to canonical JSON bytes (sorted object keys via `serde_json` + IndexMap order).
    pub fn to_canonical_json(&self) -> Result<Vec<u8>, serde_json::Error> {
        let value = serde_json::to_value(self)?;
        let canonical = canonicalize_value(&value);
        serde_json::to_vec(&canonical)
    }

    /// Semantic fingerprint (SHA-256 hex of canonical JSON).
    pub fn fingerprint(&self) -> Result<String, serde_json::Error> {
        let bytes = self.to_canonical_json()?;
        let digest = Sha256::digest(&bytes);
        Ok(digest.iter().map(|b| format!("{b:02x}")).collect())
    }

    /// Validate security budgets and mandatory fields.
    pub fn validate_budgets(&self) -> Result<(), String> {
        if self.profile.trim().is_empty() {
            return Err("portable plan profile is required".into());
        }
        if self.plan_identity != TRANSFORM_PLAN_IDENTITY {
            return Err(format!(
                "unsupported plan identity '{}'; expected '{TRANSFORM_PLAN_IDENTITY}'",
                self.plan_identity
            ));
        }
        let bytes = self
            .to_canonical_json()
            .map_err(|e| format!("portable plan serialization failed: {e}"))?;
        if bytes.len() > MAX_PORTABLE_PLAN_BYTES {
            return Err(format!(
                "portable plan exceeds byte budget ({} > {MAX_PORTABLE_PLAN_BYTES})",
                bytes.len()
            ));
        }
        let depth = value_depth(&serde_json::from_slice(&bytes).unwrap_or(Value::Null));
        if depth > MAX_PORTABLE_PLAN_DEPTH {
            return Err(format!(
                "portable plan exceeds depth budget ({depth} > {MAX_PORTABLE_PLAN_DEPTH})"
            ));
        }
        let nodes = self.actions.len() + self.rules.len();
        if nodes > MAX_PORTABLE_PLAN_NODES {
            return Err(format!(
                "portable plan exceeds node budget ({nodes} > {MAX_PORTABLE_PLAN_NODES})"
            ));
        }
        if contains_executable_marker(&serde_json::from_slice(&bytes).unwrap_or(Value::Null)) {
            return Err("portable plan rejects executable or host-language objects".into());
        }
        Ok(())
    }
}

fn canonicalize_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut out = serde_json::Map::new();
            for key in keys {
                if let Some(v) = map.get(key) {
                    out.insert(key.clone(), canonicalize_value(v));
                }
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonicalize_value).collect()),
        other => other.clone(),
    }
}

fn value_depth(value: &Value) -> usize {
    match value {
        Value::Array(items) => 1 + items.iter().map(value_depth).max().unwrap_or(0),
        Value::Object(map) => 1 + map.values().map(value_depth).max().unwrap_or(0),
        _ => 1,
    }
}

fn contains_executable_marker(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            if map.contains_key("$executable")
                || map.contains_key("__repr__")
                || map.contains_key("pyObject")
                || map.contains_key("sqlText")
            {
                return true;
            }
            map.values().any(contains_executable_marker)
        }
        Value::Array(items) => items.iter().any(contains_executable_marker),
        _ => false,
    }
}

/// Export a transformation plan to a validated portable plan.
pub fn export_portable_plan(
    plan: &TransformationPlan,
    profile: &str,
) -> Result<PortablePlan, String> {
    let portable = PortablePlan::from_transformation_plan(plan, profile);
    portable.validate_budgets()?;
    Ok(portable)
}

/// Lower string expression parameters inside action JSON to structured nodes.
fn lower_action_value_exprs(value: Value) -> Value {
    match value {
        Value::Object(mut map) => {
            // Direct expression string fields commonly used in portable actions.
            for key in ["expr", "predicate"] {
                if let Some(Value::String(source)) = map.get(key).cloned() {
                    if let Ok(node) = crate::analysis::expr::to_structured_node(&source) {
                        map.insert(key.to_string(), node);
                    }
                }
            }
            // Nested arrays of assignment/aggregate/window objects.
            for key in ["fields", "assignments", "aggregates", "functions", "keys"] {
                if let Some(Value::Array(items)) = map.get(key).cloned() {
                    let lowered = items.into_iter().map(lower_action_value_exprs).collect();
                    map.insert(key.to_string(), Value::Array(lowered));
                }
            }
            // Expression COM nodes may carry string `expr` alongside optional body.
            if let Some(Value::String(source)) = map.get("expr").cloned() {
                if !map.contains_key("body") {
                    if let Ok(node) = crate::analysis::expr::to_structured_node(&source) {
                        map.insert("body".into(), node);
                    }
                }
            }
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                out.insert(k, lower_action_value_exprs(v));
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(lower_action_value_exprs).collect()),
        other => other,
    }
}
