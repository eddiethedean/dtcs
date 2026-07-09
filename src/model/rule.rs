//! Rule model.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::metadata::Metadata;

/// Rule evaluation phase per SPEC Chapter 19.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RulePhase {
    /// Evaluated before execution.
    Precondition,
    /// Evaluated during execution.
    Execution,
    /// Evaluated after execution.
    Postcondition,
}

impl RulePhase {
    /// Returns the serialized phase name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Precondition => "precondition",
            Self::Execution => "execution",
            Self::Postcondition => "postcondition",
        }
    }
}

/// A declarative invariant rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    /// Stable rule instance identifier.
    pub id: String,
    /// Rule registry identifier (for example `dtcs:not_null`).
    pub rule: String,
    /// Evaluation target.
    pub target: String,
    /// Evaluation phase.
    pub phase: RulePhase,
    /// Rule parameters (for example `min` for `dtcs:min_length`).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<String, serde_json::Value>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
