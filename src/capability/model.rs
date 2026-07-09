//! Engine capability model (SPEC Chapter 14).

use serde::{Deserialize, Serialize};

/// Engine capability declaration (Ch 14 §3–4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineCapabilityDeclaration {
    /// Stable engine identifier (for example `dtcs:reference`).
    pub engine_id: String,
    /// Engine implementation version.
    pub engine_version: String,
    /// Capability declaration version.
    pub capability_version: String,
    /// Grouped capability categories (Ch 14 §5).
    pub categories: CapabilityCategories,
}

/// Capability categories supported by an engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityCategories {
    /// Supported logical type expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logical_types: Vec<String>,
    /// Supported semantic action identifiers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_actions: Vec<String>,
    /// Supported function identifiers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<String>,
    /// Supported rule identifiers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
    /// Supported expression operators.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<String>,
    /// Supported runtime features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_features: Vec<String>,
}

/// A missing capability required by a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityGap {
    /// Category of the missing capability.
    pub category: String,
    /// Required identifier or feature.
    pub required: String,
}

/// Result of matching a plan against engine capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityMatchReport {
    /// Whether all mandatory requirements are satisfied.
    pub supported: bool,
    /// Missing capabilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<CapabilityGap>,
    /// Diagnostics from matching.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<crate::diagnostics::Diagnostic>,
}

impl CapabilityMatchReport {
    /// Returns `true` when no error-level diagnostics are present.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.supported && !self.diagnostics.iter().any(|d| d.severity.is_error())
    }
}
