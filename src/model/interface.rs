//! Input and output interface definitions (SPEC Chapter 6).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::metadata::Metadata;
use super::types::Schema;

/// Streaming mode for bounded or unbounded datasets (SPEC Chapter 6 §12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamingMode {
    /// Bounded dataset.
    Bounded,
    /// Unbounded data stream.
    Unbounded,
}

/// Streaming declaration for an input or output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingDeclaration {
    /// Streaming mode.
    pub mode: StreamingMode,
}

/// A precondition or postcondition referencing a rule instance (SPEC Chapter 6 §7–8).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfaceCondition {
    /// Rule instance identifier (`rules[].id`).
    pub rule: String,
}

/// A contract input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// Stable logical identifier.
    pub id: String,
    /// Optional logical schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// Whether the input is optional (SPEC Chapter 6 §11).
    #[serde(default)]
    pub optional: bool,
    /// Streaming declaration (SPEC Chapter 6 §12).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streaming: Option<StreamingDeclaration>,
    /// Preconditions (SPEC Chapter 6 §7).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions: Vec<InterfaceCondition>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Vendor extension fields (SPEC Chapter 6 §13).
    #[serde(default, flatten)]
    pub extensions: IndexMap<String, Value>,
}

/// A contract output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    /// Stable logical identifier.
    pub id: String,
    /// Optional logical schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// Streaming declaration (SPEC Chapter 6 §12).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streaming: Option<StreamingDeclaration>,
    /// Postconditions (SPEC Chapter 6 §8).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub postconditions: Vec<InterfaceCondition>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Vendor extension fields (SPEC Chapter 6 §13).
    #[serde(default, flatten)]
    pub extensions: IndexMap<String, Value>,
}
