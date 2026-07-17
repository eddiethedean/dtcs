//! Embedded `dtcs:reference` capability profile.

use crate::model::{COMPOSITE_TYPES, PRIMITIVE_TYPES};
use crate::registry::default_registry;

use super::model::{CapabilityCategories, EngineCapabilityDeclaration};

/// Reference engine identifier.
pub const REFERENCE_ENGINE_ID: &str = "dtcs:reference";

/// Supported expression operators in the reference runtime.
pub const REFERENCE_OPERATORS: &[&str] = &[
    "add",
    "sub",
    "mul",
    "div",
    "eq",
    "neq",
    "lt",
    "lte",
    "gt",
    "gte",
    "and",
    "or",
    "negate",
    "not",
    "in",
    "contains",
    "dtcs:add",
    "dtcs:subtract",
    "dtcs:multiply",
    "dtcs:divide",
    "dtcs:eq",
    "dtcs:not_eq",
    "dtcs:lt",
    "dtcs:lte",
    "dtcs:gt",
    "dtcs:gte",
    "dtcs:and",
    "dtcs:or",
    "dtcs:not",
    "dtcs:negate",
    "dtcs:in",
    "dtcs:between",
    "dtcs:null_safe_eq",
    "dtcs:field",
    "dtcs:index",
    "dtcs:element_at",
    "dtcs:modulo",
];

/// Supported language features for the reference engine.
pub const REFERENCE_LANGUAGE_FEATURES: &[&str] = &[
    "nullMissingInvalidDistinction",
    "collectionOperators",
    "temporalLiterals",
];

/// Supported optimization capabilities.
pub const REFERENCE_OPTIMIZATION: &[&str] = &[
    "constantFolding",
    "actionFusion",
    "ruleDedup",
    "deadExpressionElimination",
];

/// Supported runtime features for the reference engine.
pub const REFERENCE_RUNTIME_FEATURES: &[&str] = &[
    "rowOriented",
    "prePostconditions",
    "lineageMaterialization",
    "deterministic",
    "nonDeterminismSourceReporting",
    "datasetActions",
];

/// Returns the embedded reference engine capability profile.
#[must_use]
pub fn reference_profile() -> EngineCapabilityDeclaration {
    let registry = default_registry();
    let mut semantic_actions = Vec::new();
    let mut functions = Vec::new();
    let mut rules = Vec::new();

    for entry in registry.entries.values() {
        match entry.category {
            crate::model::RegistryCategory::SemanticAction => {
                semantic_actions.push(entry.id.clone());
            }
            crate::model::RegistryCategory::Function => {
                functions.push(entry.id.clone());
            }
            crate::model::RegistryCategory::Rule => {
                rules.push(entry.id.clone());
            }
            _ => {}
        }
    }
    semantic_actions.sort();
    functions.sort();
    rules.sort();

    let mut logical_types: Vec<String> = PRIMITIVE_TYPES
        .iter()
        .chain(COMPOSITE_TYPES.iter())
        .map(|t| (*t).to_string())
        .collect();
    logical_types.sort();

    EngineCapabilityDeclaration {
        engine_id: REFERENCE_ENGINE_ID.into(),
        engine_version: env!("CARGO_PKG_VERSION").into(),
        capability_version: "1.0.0".into(),
        categories: CapabilityCategories {
            language_features: REFERENCE_LANGUAGE_FEATURES
                .iter()
                .map(|f| (*f).to_string())
                .collect(),
            logical_types,
            semantic_actions,
            functions,
            rules,
            operators: REFERENCE_OPERATORS
                .iter()
                .map(|op| (*op).to_string())
                .collect(),
            optimization: REFERENCE_OPTIMIZATION
                .iter()
                .map(|f| (*f).to_string())
                .collect(),
            runtime_features: REFERENCE_RUNTIME_FEATURES
                .iter()
                .map(|f| (*f).to_string())
                .collect(),
            extension_support: vec!["vendorNamespacedPreserve".into()],
        },
    }
}
