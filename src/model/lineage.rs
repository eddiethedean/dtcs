//! Lineage model (SPEC Chapter 10).

use serde::{Deserialize, Serialize};

/// Logical information-flow kind (SPEC Chapter 10 §7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum InformationFlow {
    /// Values preserved without derivation.
    Preserved,
    /// Values derived from sources.
    #[default]
    Derived,
    /// Values aggregated from sources.
    Aggregated,
    /// Values filtered from sources.
    Filtered,
    /// Values partitioned from sources.
    Partitioned,
    /// Values explicitly discarded (information loss).
    Discarded,
}

impl InformationFlow {
    /// Serialized flow name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Derived => "derived",
            Self::Aggregated => "aggregated",
            Self::Filtered => "filtered",
            Self::Partitioned => "partitioned",
            Self::Discarded => "discarded",
        }
    }
}

/// Data lineage metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lineage {
    /// Output-to-input provenance mappings.
    #[serde(default)]
    pub mappings: Vec<LineageMapping>,
}

fn default_lineage_operation() -> String {
    "dtcs:derive".into()
}

/// Maps an output to contributing inputs with semantic operation and flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageMapping {
    /// Optional stable mapping identity (SPEC Chapter 10 §5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Output identifier (destination).
    pub output: String,
    /// Contributing input identifiers (sources).
    #[serde(default)]
    pub inputs: Vec<String>,
    /// Semantic operation relating sources to destination (SPEC Chapter 10 §6).
    /// Defaults to `dtcs:derive` when omitted for backward compatibility.
    #[serde(default = "default_lineage_operation")]
    pub operation: String,
    /// Information-flow kind (SPEC Chapter 10 §7).
    #[serde(default)]
    pub flow: InformationFlow,
}
