//! Transformation semantics container.

use serde::{Deserialize, Serialize};

/// Semantic transformation definition (reserved for future fields).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformationSemantics {
    /// Whether the transformation is deterministic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deterministic: Option<bool>,
}
