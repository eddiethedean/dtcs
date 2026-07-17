//! Portable engine capability manifests (proposal §13 / Ch 14).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::registry::default_registry;

use super::builtin::{
    reference_profile, REFERENCE_ENGINE_ID, REFERENCE_LANGUAGE_FEATURES, REFERENCE_OPERATORS,
    REFERENCE_OPTIMIZATION, REFERENCE_RUNTIME_FEATURES,
};
use super::model::EngineCapabilityDeclaration;

/// Per-entry capability detail for a registry identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryCapability {
    /// Whether the entry is supported.
    pub supported: bool,
    /// Entry version this declaration covers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Semantic modes (for example filter invalid handling).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_modes: Vec<String>,
    /// Known limits.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub limits: IndexMap<String, String>,
    /// Notes / unsupported optional features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

/// Machine-readable portable capability manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableCapabilityManifest {
    /// Portable profile this declaration targets.
    pub profile: String,
    /// Implementation class (Compiler, Runtime, …).
    pub implementation_class: String,
    /// Engine identifier.
    pub engine: String,
    /// Engine version.
    pub engine_version: String,
    /// Supported plan serialization profiles.
    pub supported_plan_profiles: Vec<String>,
    /// Actions keyed by registry id.
    pub actions: IndexMap<String, EntryCapability>,
    /// Operators keyed by registry id or short name.
    pub operators: IndexMap<String, EntryCapability>,
    /// Functions keyed by registry id.
    pub functions: IndexMap<String, EntryCapability>,
    /// Types keyed by logical type name.
    pub types: IndexMap<String, EntryCapability>,
    /// Semantic modes at engine level.
    pub semantic_modes: IndexMap<String, String>,
    /// Engine limits.
    pub limits: IndexMap<String, String>,
    /// Legacy flat capability declaration for compatibility.
    pub legacy: EngineCapabilityDeclaration,
}

/// Build the reference portable capability manifest for a profile.
#[must_use]
pub fn reference_portable_manifest(profile: &str) -> PortableCapabilityManifest {
    let legacy = reference_profile();
    let registry = default_registry();
    let mut actions = IndexMap::new();
    let mut functions = IndexMap::new();
    let mut operators = IndexMap::new();
    let mut types = IndexMap::new();

    for entry in registry.entries.values() {
        match entry.category {
            crate::model::RegistryCategory::SemanticAction => {
                actions.insert(
                    entry.id.clone(),
                    EntryCapability {
                        supported: true,
                        version: Some(entry.version.clone()),
                        semantic_modes: Vec::new(),
                        limits: IndexMap::new(),
                        notes: Vec::new(),
                    },
                );
            }
            crate::model::RegistryCategory::Function => {
                let experimental = entry.status == crate::model::RegistryEntryStatus::Experimental;
                functions.insert(
                    entry.id.clone(),
                    EntryCapability {
                        supported: true,
                        version: Some(entry.version.clone()),
                        semantic_modes: if experimental {
                            vec!["experimental".into()]
                        } else {
                            Vec::new()
                        },
                        limits: IndexMap::new(),
                        notes: if experimental {
                            vec!["experimental registry status".into()]
                        } else {
                            Vec::new()
                        },
                    },
                );
            }
            crate::model::RegistryCategory::Operator => {
                operators.insert(
                    entry.id.clone(),
                    EntryCapability {
                        supported: true,
                        version: Some(entry.version.clone()),
                        semantic_modes: Vec::new(),
                        limits: IndexMap::new(),
                        notes: Vec::new(),
                    },
                );
            }
            _ => {}
        }
    }
    for op in REFERENCE_OPERATORS {
        operators.entry((*op).to_string()).or_insert(EntryCapability {
            supported: true,
            version: Some("1.0.0".into()),
            semantic_modes: Vec::new(),
            limits: IndexMap::new(),
            notes: Vec::new(),
        });
    }
    for ty in &legacy.categories.logical_types {
        types.insert(
            ty.clone(),
            EntryCapability {
                supported: true,
                version: Some("1.0.0".into()),
                semantic_modes: Vec::new(),
                limits: IndexMap::new(),
                notes: Vec::new(),
            },
        );
    }

    let mut semantic_modes = IndexMap::new();
    semantic_modes.insert("filterInvalid".into(), "fail".into());
    semantic_modes.insert("joinNullKeys".into(), "neverMatch".into());
    semantic_modes.insert("missingGroupingKeys".into(), "distinctFromNull".into());
    for feature in REFERENCE_LANGUAGE_FEATURES {
        semantic_modes.insert((*feature).to_string(), "supported".into());
    }
    for feature in REFERENCE_OPTIMIZATION {
        semantic_modes.insert((*feature).to_string(), "supported".into());
    }
    for feature in REFERENCE_RUNTIME_FEATURES {
        semantic_modes.insert((*feature).to_string(), "supported".into());
    }

    let mut limits = IndexMap::new();
    limits.insert(
        "maxPortablePlanBytes".into(),
        crate::plan::MAX_PORTABLE_PLAN_BYTES.to_string(),
    );
    limits.insert(
        "maxPortablePlanDepth".into(),
        crate::plan::MAX_PORTABLE_PLAN_DEPTH.to_string(),
    );
    limits.insert(
        "maxPortablePlanNodes".into(),
        crate::plan::MAX_PORTABLE_PLAN_NODES.to_string(),
    );

    PortableCapabilityManifest {
        profile: profile.to_string(),
        implementation_class: "Compiler".into(),
        engine: REFERENCE_ENGINE_ID.into(),
        engine_version: env!("CARGO_PKG_VERSION").into(),
        supported_plan_profiles: vec![crate::plan::TRANSFORM_PLAN_IDENTITY.into()],
        actions,
        operators,
        functions,
        types,
        semantic_modes,
        limits,
        legacy,
    }
}
