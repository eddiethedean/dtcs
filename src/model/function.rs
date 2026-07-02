//! Function model (declarations only for MVP).

use serde::{Deserialize, Serialize};

use super::identifiers::is_vendor_namespaced_identifier;
use super::metadata::Metadata;

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
    /// Declared parameters.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<FunctionParameter>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Known standard functions for Phase 0.2 validation.
pub const KNOWN_FUNCTIONS: &[&str] = &[];

/// Returns `true` when the function identifier is recognized.
#[must_use]
pub fn is_known_function(function: &str) -> bool {
    KNOWN_FUNCTIONS.contains(&function) || is_vendor_namespaced_identifier(function)
}
