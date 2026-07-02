//! Semantic action model.

use serde::{Deserialize, Serialize};

/// A standardized semantic action declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticAction {
    /// Stable action instance identifier.
    pub id: String,
    /// Semantic action registry identifier (for example `dtcs:lowercase`).
    pub action: String,
    /// Target field or object reference.
    pub target: String,
}

/// Known standard semantic actions for MVP validation.
pub const KNOWN_ACTIONS: &[&str] = &["dtcs:lowercase"];

/// Returns `true` when the action identifier is recognized.
#[must_use]
pub fn is_known_action(action: &str) -> bool {
    KNOWN_ACTIONS.contains(&action) || is_namespaced(action)
}

/// Returns `true` for `vendor:action` style identifiers.
#[must_use]
pub fn is_namespaced(identifier: &str) -> bool {
    identifier.contains(':') && !identifier.starts_with("dtcs:")
}
