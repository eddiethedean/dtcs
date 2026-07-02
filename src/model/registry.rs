//! Registry references.

use serde::{Deserialize, Serialize};

/// A registry reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Registry {
    /// Registry identifier.
    pub id: String,
    /// Registry URI or location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
