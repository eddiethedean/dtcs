//! Input and output interface definitions.

use serde::{Deserialize, Serialize};

use super::types::Schema;

/// A contract input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    /// Stable logical identifier.
    pub id: String,
    /// Optional logical schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// Whether the input is optional.
    #[serde(default)]
    pub optional: bool,
}

/// A contract output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    /// Stable logical identifier.
    pub id: String,
    /// Optional logical schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}
