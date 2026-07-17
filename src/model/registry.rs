//! Registry model (SPEC Chapter 22).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A contract-level reference to an external registry catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryRef {
    /// Registry identifier.
    pub id: String,
    /// Registry URI or location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Publication status for a registry document (Ch 22 §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistryPublicationStatus {
    /// Work in progress.
    Draft,
    /// Published for experimental use.
    Experimental,
    /// Normative standard publication.
    Standard,
    /// Still valid but discouraged.
    Deprecated,
    /// No longer supported.
    Obsolete,
}

impl RegistryPublicationStatus {
    /// Returns the serialized status name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Experimental => "experimental",
            Self::Standard => "standard",
            Self::Deprecated => "deprecated",
            Self::Obsolete => "obsolete",
        }
    }
}

/// Status of an individual registry entry (Ch 22 §10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistryEntryStatus {
    /// Work in progress.
    Draft,
    /// Published for experimental use.
    Experimental,
    /// Normative standard entry.
    Standard,
    /// Still valid but discouraged.
    Deprecated,
    /// No longer supported; identifier remains reserved.
    Obsolete,
}

impl RegistryEntryStatus {
    /// Returns the serialized status name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Experimental => "experimental",
            Self::Standard => "standard",
            Self::Deprecated => "deprecated",
            Self::Obsolete => "obsolete",
        }
    }
}

/// Category of a registry entry (Ch 22 §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RegistryCategory {
    /// Semantic action identifier.
    SemanticAction,
    /// Function identifier.
    Function,
    /// Operator identifier (`dtcs:eq`, …).
    Operator,
    /// Rule identifier.
    Rule,
    /// Logical type identifier.
    LogicalType,
    /// Diagnostic code.
    Diagnostic,
    /// Extension namespace.
    ExtensionNamespace,
    /// Conformance profile.
    Profile,
    /// Engine capability.
    Capability,
}

impl RegistryCategory {
    /// Returns the serialized category name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SemanticAction => "semanticAction",
            Self::Function => "function",
            Self::Operator => "operator",
            Self::Rule => "rule",
            Self::LogicalType => "logicalType",
            Self::Diagnostic => "diagnostic",
            Self::ExtensionNamespace => "extensionNamespace",
            Self::Profile => "profile",
            Self::Capability => "capability",
        }
    }
}

/// Compatibility requirement for an extension entry (Ch 21 §7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionCompatibility {
    /// Unsupported mandatory extensions prevent successful processing.
    Mandatory,
    /// Optional extensions may be preserved without interpretation.
    Optional,
}

impl ExtensionCompatibility {
    /// Returns the serialized compatibility name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mandatory => "mandatory",
            Self::Optional => "optional",
        }
    }
}

/// A single registry entry (Ch 22 §5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryEntry {
    /// Stable identifier (for example `dtcs:lowercase`).
    ///
    /// When omitted in a map-keyed registry document, the map key is used.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Entry category.
    pub category: RegistryCategory,
    /// Entry version.
    pub version: String,
    /// Publication status.
    pub status: RegistryEntryStatus,
    /// Compatibility requirement (primarily for extension namespaces).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<ExtensionCompatibility>,
    /// Semantic definition summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// Normative references.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<String>,
    /// Whether this implementation supports the entry.
    #[serde(default = "default_supported")]
    pub supported: bool,
}

fn default_supported() -> bool {
    true
}

/// An authoritative registry catalog document (Ch 22 §4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryDocument {
    /// Registry identifier.
    pub id: String,
    /// Registry version.
    pub version: String,
    /// Governing specification version.
    pub governing_specification: String,
    /// Publication status of the registry document.
    pub publication_status: RegistryPublicationStatus,
    /// Catalog entries keyed by stable identifier.
    #[serde(default)]
    pub entries: IndexMap<String, RegistryEntry>,
}

impl RegistryDocument {
    /// Returns the entry for `id`, if present.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&RegistryEntry> {
        self.entries.get(id)
    }

    /// Inserts or replaces an entry, keyed by `entry.id`.
    pub fn insert(&mut self, entry: RegistryEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    /// Merges `other` into this document.
    ///
    /// Non-`dtcs:` entries from `other` override existing entries. `dtcs:` entries
    /// already present in this document are preserved (builtin authority). Novel
    /// `dtcs:` keys from `other` are rejected.
    pub fn merge(
        &mut self,
        other: &RegistryDocument,
    ) -> Result<(), crate::diagnostics::DiagnosticReport> {
        for (id, entry) in &other.entries {
            if id.starts_with("dtcs:") {
                if self.entries.contains_key(id) {
                    continue;
                }
                let mut report = crate::diagnostics::DiagnosticReport::new();
                report.push(
                    crate::diagnostics::Diagnostic::new(
                        crate::diagnostics::codes::INVALID_REGISTRY,
                        crate::diagnostics::Severity::Error,
                        crate::diagnostics::DiagnosticStage::Validation,
                        crate::diagnostics::DiagnosticCategory::Structure,
                        format!(
                            "registry merge rejected novel standard entry '{id}'; vendor catalogs cannot extend the dtcs: namespace"
                        ),
                    )
                    .with_object_ref(format!("entries.{id}"))
                    .with_remediation(
                        "Use vendor namespaces for custom identifiers; only builtin dtcs: entries are authoritative",
                    ),
                );
                return Err(report);
            }
            self.entries.insert(id.clone(), entry.clone());
        }
        Ok(())
    }

    /// Merges `other` into this document without rejecting novel `dtcs:` keys.
    ///
    /// Reserved for trusted builtin catalog assembly.
    pub(crate) fn merge_trusted(&mut self, other: &RegistryDocument) {
        for (id, entry) in &other.entries {
            if id.starts_with("dtcs:") && self.entries.contains_key(id) {
                continue;
            }
            self.entries.insert(id.clone(), entry.clone());
        }
    }

    /// Returns all entries in insertion order.
    #[must_use]
    pub fn list(&self) -> Vec<&RegistryEntry> {
        self.entries.values().collect()
    }
}
