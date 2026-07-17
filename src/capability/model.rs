//! Engine capability model (SPEC Chapter 14).

use std::collections::BTreeMap;

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
    /// Accepted portable Transformation Plan protocol identities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plan_protocols: Vec<String>,
    /// Complete semantic-family profile claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<String>,
    /// Partial profile claims. Each value lists supported requirements.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub partial_profiles: BTreeMap<String, Vec<String>>,
    /// Language features (expression subset, null distinction, etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_features: Vec<String>,
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
    /// Supported optimization passes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub optimization: Vec<String>,
    /// Supported runtime features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_features: Vec<String>,
    /// Extension support declarations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension_support: Vec<String>,
    /// Versioned semantic environments and grammars.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub semantic_versions: BTreeMap<String, String>,
    /// Resource budgets keyed by the normative budget name.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub resource_limits: BTreeMap<String, u64>,
    /// Supported semantic modes keyed by registry entry or mode family.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub semantic_modes: BTreeMap<String, Vec<String>>,
    /// Ordering and deterministic-execution guarantees.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guarantees: Vec<String>,
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
