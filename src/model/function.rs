//! Function model (declarations only for MVP).

use serde::{Deserialize, Serialize};

use super::metadata::Metadata;

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
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
