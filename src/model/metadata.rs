//! Contract metadata (SPEC Chapter 5).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Classification levels from SPEC Chapter 5 §9.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClassificationLevel {
    /// Public classification.
    Public,
    /// Internal classification.
    Internal,
    /// Confidential classification.
    Confidential,
    /// Restricted classification.
    Restricted,
}

/// Documentation metadata (SPEC Chapter 5 §6).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentationMetadata {
    /// Short summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Detailed description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Usage examples.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
    /// External references.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<String>,
}

/// Ownership metadata (SPEC Chapter 5 §4 / §7).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipMetadata {
    /// Owning party or team.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Accountable organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Contact URI or email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

/// Lifecycle metadata (SPEC Chapter 5 §4).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleMetadata {
    /// Lifecycle state (for example `draft`, `active`, `retired`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Effective-from timestamp (ISO-8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<String>,
    /// Effective-until timestamp (ISO-8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_until: Option<String>,
}

/// Governance metadata (SPEC Chapter 5 §7).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceMetadata {
    /// Owning party.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Data steward.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steward: Option<String>,
    /// Approval status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<String>,
    /// Last review date (ISO-8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_date: Option<String>,
    /// Policy references.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_refs: Vec<String>,
}

/// Provenance metadata (SPEC Chapter 5 §8).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceMetadata {
    /// Author.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Creation timestamp (ISO-8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Modification timestamp (ISO-8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// Originating system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_system: Option<String>,
}

/// Identity metadata (SPEC Chapter 5 §5).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityMetadata {
    /// Object identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Object name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Object version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Object metadata (SPEC Chapter 5).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Optional description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Identity metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityMetadata>,
    /// Documentation metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<DocumentationMetadata>,
    /// Governance metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub governance: Option<GovernanceMetadata>,
    /// Ownership metadata (SPEC Chapter 5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ownership: Option<OwnershipMetadata>,
    /// Lifecycle metadata (SPEC Chapter 5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<LifecycleMetadata>,
    /// Provenance metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProvenanceMetadata>,
    /// Classification level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<ClassificationLevel>,
    /// Whether this object revision is deprecated (evolution analysis).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// Recommended replacement contract id when deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// Anticipated removal version or date (SPEC Chapter 12 §10).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anticipated_removal: Option<String>,
    /// Custom namespaced metadata preserved verbatim.
    #[serde(default, flatten)]
    pub extensions: IndexMap<String, Value>,
}
