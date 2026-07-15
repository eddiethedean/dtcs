//! Null-behavior declarations (SPEC Chapters 8 and 18).

use serde::{Deserialize, Serialize};

/// Declared null-propagation behavior for expressions and functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum NullBehavior {
    /// Null / missing inputs propagate as null.
    #[default]
    Propagate,
    /// Null inputs are rejected (diagnostics / invalid).
    Reject,
    /// Null inputs are coalesced to a default / empty value.
    Coalesce,
    /// Behavior is function-specific and documented in the registry definition.
    Defined,
}

impl NullBehavior {
    /// Serialized name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Propagate => "propagate",
            Self::Reject => "reject",
            Self::Coalesce => "coalesce",
            Self::Defined => "defined",
        }
    }

    /// Parse from registry / document string.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "propagate" => Some(Self::Propagate),
            "reject" => Some(Self::Reject),
            "coalesce" => Some(Self::Coalesce),
            "defined" => Some(Self::Defined),
            _ => None,
        }
    }
}
