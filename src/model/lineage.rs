//! Lineage model.

use serde::{Deserialize, Serialize};

/// Data lineage metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lineage {
    /// Output-to-input provenance mappings.
    #[serde(default)]
    pub mappings: Vec<LineageMapping>,
}

/// Maps an output to contributing inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageMapping {
    /// Output identifier.
    pub output: String,
    /// Contributing input identifiers.
    #[serde(default)]
    pub inputs: Vec<String>,
}
