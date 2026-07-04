//! Semantic action model.

use serde::{Deserialize, Serialize};

use super::metadata::Metadata;

/// A standardized semantic action declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticAction {
    /// Stable action instance identifier.
    pub id: String,
    /// Semantic action registry identifier (for example `dtcs:lowercase`).
    pub action: String,
    /// Target field or object reference.
    pub target: String,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
