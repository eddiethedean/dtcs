//! Compatibility and evolution report types.

use serde::{Deserialize, Serialize};

use crate::diagnostics::Diagnostic;

use super::types::{AspectResult, ChangeCategory, CompatibilityLevel};

/// Result of compatibility analysis (SPEC Chapter 11).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityReport {
    /// Overall compatibility classification.
    pub level: CompatibilityLevel,
    /// Per-aspect summaries.
    pub aspects: Vec<AspectResult>,
    /// Diagnostics including violations and notes.
    pub diagnostics: Vec<Diagnostic>,
}

impl CompatibilityReport {
    /// Returns `true` when the pair is not incompatible.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        !matches!(self.level, CompatibilityLevel::Incompatible)
    }
}

/// A categorized contract change detected during evolution analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractChange {
    /// Change category (SPEC Chapter 12 §6).
    pub category: ChangeCategory,
    /// Human-readable description.
    pub message: String,
    /// Affected object reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ref: Option<String>,
}

/// Result of evolution analysis (SPEC Chapter 12).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionReport {
    /// Older revision contract id.
    pub source_id: String,
    /// Newer revision contract id.
    pub target_id: String,
    /// Whether both revisions share the same logical identity.
    pub same_identity: bool,
    /// Underlying compatibility level.
    pub compatibility: CompatibilityLevel,
    /// Detected changes by category.
    pub changes: Vec<ContractChange>,
    /// Informative migration guidance (SPEC Chapter 12 §9).
    pub migration_hints: Vec<String>,
    /// Diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}
