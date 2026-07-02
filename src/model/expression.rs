//! Expression model.

use serde::{Deserialize, Serialize};

use super::metadata::Metadata;

/// A DTCS expression declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    /// Stable expression identifier.
    pub id: String,
    /// Expression body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    /// Declared logical type (SPEC Chapter 4 §11).
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
