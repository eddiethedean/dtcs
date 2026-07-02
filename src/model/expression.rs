//! Expression model.

use serde::{Deserialize, Serialize};

/// A DTCS expression declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    /// Stable expression identifier.
    pub id: String,
    /// Expression body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}
