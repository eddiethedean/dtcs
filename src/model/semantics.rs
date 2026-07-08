//! Transformation semantics container.

use serde::{Deserialize, Serialize};

/// Semantic transformation definition (reserved for future fields).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformationSemantics {
    /// Whether the transformation is deterministic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deterministic: Option<bool>,
    /// Whether the transformation is pure (side-effect free).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pure: Option<bool>,
    /// Declared ordering semantics for semantic actions (when required).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordering: Option<ActionOrdering>,
    /// Declared externally observable side effects (for impure transformations).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<SideEffectDeclaration>,
}

/// Ordering declaration for semantic actions (SPEC Chapter 7 §12).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "mode")]
pub enum ActionOrdering {
    /// Semantic action ordering is irrelevant.
    Unordered,
    /// Semantic actions are ordered as declared by the `order` list.
    Explicit { order: Vec<String> },
}

/// Declared side effect metadata (SPEC Chapter 7 §8).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SideEffectDeclaration {
    /// Stable side-effect identifier.
    pub id: String,
    /// Side-effect category (for example `network`, `filesystem`, `metrics`).
    pub kind: String,
    /// Human-readable description of the side effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
