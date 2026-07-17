//! Embedded `dtcs:reference` capability profile.

use std::collections::BTreeMap;

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
    "lambdaExpressions",
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
        capability_version: "2.0.0".into(),
        categories: CapabilityCategories {
            plan_protocols: vec![
                crate::plan::LEGACY_TRANSFORM_PLAN_IDENTITY.into(),
                crate::plan::TRANSFORM_PLAN_IDENTITY.into(),
            ],
            profiles: vec![
                crate::plan::KERNEL_PROFILE.into(),
                crate::plan::RELATIONAL_PROFILE.into(),
                crate::plan::WINDOW_PROFILE.into(),
                crate::plan::COMPLEX_VALUES_PROFILE.into(),
                crate::plan::STRING_ADVANCED_PROFILE.into(),
                crate::plan::CONVERSION_PROFILE.into(),
                crate::plan::STATISTICS_PROFILE.into(),
                crate::plan::RESHAPE_PROFILE.into(),
                crate::plan::RELATIONAL_EXTENDED_PROFILE.into(),
                crate::plan::TEMPORAL_IANA_PROFILE.into(),
                crate::plan::NONDETERMINISTIC_PROFILE.into(),
            ],
            partial_profiles: BTreeMap::new(),
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
            semantic_versions: BTreeMap::from([
                ("regexGrammar".into(), "dtcs-regex/1".into()),
                ("formatGrammar".into(), "dtcs-format/1".into()),
                ("unicode".into(), "unicode-15.1".into()),
                ("timezoneData".into(), "iana-2025b".into()),
                ("randomAlgorithm".into(), "xorshift64star/1".into()),
            ]),
            resource_limits: BTreeMap::from([
                (
                    "planBytes".into(),
                    crate::plan::MAX_PORTABLE_PLAN_BYTES as u64,
                ),
                (
                    "planDepth".into(),
                    crate::plan::MAX_PORTABLE_PLAN_DEPTH as u64,
                ),
                (
                    "planNodes".into(),
                    crate::plan::MAX_PORTABLE_PLAN_NODES as u64,
                ),
                ("collectionElements".into(), 1_000_000),
                ("generatedRows".into(), 1_000_000),
                ("regexInputBytes".into(), 1_048_576),
            ]),
            semantic_modes: BTreeMap::from([
                (
                    "error".into(),
                    vec![
                        "fail".into(),
                        "invalid".into(),
                        "null".into(),
                        "route".into(),
                    ],
                ),
                (
                    "determinism".into(),
                    vec![
                        "deterministic".into(),
                        "run-stable".into(),
                        "seeded-stable".into(),
                        "nondeterministic".into(),
                    ],
                ),
            ]),
            guarantees: vec![
                "canonicalPlanV2".into(),
                "stableOrderingWhenDeclared".into(),
                "partitionIndependentSeededRandom".into(),
            ],
        },
    }
}
