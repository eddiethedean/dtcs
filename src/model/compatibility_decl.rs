//! Contract-level compatibility declaration (SPEC Chapter 3 §9).

use serde::{Deserialize, Serialize};

/// Compatibility policy declared on the contract COM.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityDeclaration {
    /// Compatibility policy identifier or profile reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Human-readable notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Whether forward compatibility is claimed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward: Option<bool>,
    /// Whether backward compatibility is claimed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backward: Option<bool>,
}
