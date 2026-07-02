//! Versioning metadata.

use serde::{Deserialize, Serialize};

/// Contract versioning metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Versioning {
    /// Compatibility policy identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
}
