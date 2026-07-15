//! Function model (SPEC Chapter 18).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::metadata::Metadata;
use super::null_behavior::NullBehavior;

/// A declared function parameter (SPEC Chapter 18 §5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParameter {
    /// Parameter name.
    pub name: String,
    /// Parameter logical type.
    #[serde(rename = "type")]
    pub type_name: String,
    /// Whether the parameter is optional.
    #[serde(default)]
    pub optional: bool,
}

/// A reusable function definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    /// Stable function identifier.
    pub id: String,
    /// Function registry identifier.
    pub function: String,
    /// Declared return logical type (SPEC Chapter 4 §11).
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Whether the declared return type is nullable.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub nullable: bool,
    /// Declared parameters.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<FunctionParameter>,
    /// Declared null behavior (SPEC Chapter 18 §3 / §7).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_behavior: Option<NullBehavior>,
    /// Whether the function is deterministic. Defaults to true when omitted.
    #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
    pub deterministic: bool,
    /// Source of non-determinism when [`Self::deterministic`] is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_determinism_source: Option<String>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Vendor extension fields preserved verbatim (SPEC Chapter 21 §8).
    #[serde(default, flatten)]
    pub extensions: IndexMap<String, Value>,
}

fn default_true() -> bool {
    true
}
