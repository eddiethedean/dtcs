//! Expression model (SPEC Chapter 8).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::metadata::Metadata;
use super::null_behavior::NullBehavior;

/// A DTCS expression declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    /// Stable expression identifier.
    pub id: String,
    /// Expression body as authoring sugar (string grammar).
    ///
    /// Portable plans prefer [`Self::body`] structured nodes; when both are
    /// present, `body` is authoritative.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    /// Structured expression node tree (SPEC Chapter 8 §3.1).
    ///
    /// Canonical portable representation. Node shapes use a `kind` discriminator
    /// (`literal`, `fieldRef`, `unary`, `binary`, `call`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
    /// Declared logical type (SPEC Chapter 4 §11).
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Declared null behavior (SPEC Chapter 8 §5 / §9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_behavior: Option<NullBehavior>,
    /// Whether the expression is deterministic. Defaults to true when omitted.
    #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
    pub deterministic: bool,
    /// Source of non-determinism when [`Self::deterministic`] is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_determinism_source: Option<String>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Vendor extension fields preserved verbatim (SPEC Chapter 21 §8).
    #[serde(default, flatten)]
    pub extensions: IndexMap<String, Value>,
}

fn default_true() -> bool {
    true
}
