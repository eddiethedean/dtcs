//! Embedded conformance profiles (Ch 23 §5).

use super::model::{ConformanceProfile, ImplementationCapabilityDeclaration, ImplementationClass};

const IMPLEMENTATION_ID: &str = "dtcs:reference";

fn base_profile(class: ImplementationClass, optional: &[&str]) -> ConformanceProfile {
    ConformanceProfile {
        id: class.profile_id().to_string(),
        implementation_class: class,
        dtcs_version: crate::SPEC_VERSION.to_string(),
        implementation_version: env!("CARGO_PKG_VERSION").to_string(),
        supported_registries: vec!["dtcs:builtin".into()],
        supported_extensions: vec!["acme".into()],
        optional_capabilities: optional.iter().map(|s| (*s).to_string()).collect(),
    }
}

/// Returns all embedded conformance profiles for this implementation.
#[must_use]
pub fn all_profiles() -> Vec<ConformanceProfile> {
    let mut profiles = vec![
        base_profile(ImplementationClass::Parser, &[]),
        base_profile(ImplementationClass::Validator, &[]),
        base_profile(
            ImplementationClass::Analyzer,
            &[
                "compatibilityAnalysis",
                "evolutionAnalysis",
                "lineageAnalysis",
            ],
        ),
        base_profile(ImplementationClass::Planner, &[]),
        base_profile(ImplementationClass::Optimizer, &["planOptimization"]),
        base_profile(ImplementationClass::Compiler, &[]),
        base_profile(ImplementationClass::Runtime, &["referenceRuntime"]),
        base_profile(
            ImplementationClass::IntegratedPlatform,
            &[
                "compatibilityAnalysis",
                "evolutionAnalysis",
                "lineageAnalysis",
                "planOptimization",
                "referenceRuntime",
            ],
        ),
    ];
    // Semantic-family profiles (orthogonal to implementation class).
    profiles.push(semantic_profile(
        "dtcs:profile/portable-relational-kernel/1",
        ImplementationClass::Compiler,
        &[
            "portablePlanSerialization",
            "structuredExpressions",
            "operatorRegistry",
            "fieldShaping",
        ],
    ));
    profiles.push(semantic_profile(
        "dtcs:profile/portable-relational/1",
        ImplementationClass::Compiler,
        &[
            "portablePlanSerialization",
            "richJoins",
            "unionByName",
            "multiAggregate",
            "distinctLimit",
        ],
    ));
    profiles.push(semantic_profile(
        "dtcs:profile/portable-window/1",
        ImplementationClass::Compiler,
        &["windowFunctions"],
    ));
    profiles.push(semantic_profile(
        "dtcs:profile/portable-complex-types/1",
        ImplementationClass::Compiler,
        &["complexTypes"],
    ));
    // DTCS 3.0 family profiles (Experimental / Candidate — reference surface).
    for (id, caps) in [
        (
            "dtcs:profile/portable-relational-kernel/2",
            &["portablePlanSerialization", "structuredExpressions"][..],
        ),
        (
            "dtcs:profile/portable-relational/2",
            &["portablePlanSerialization", "richJoins"][..],
        ),
        (
            "dtcs:profile/portable-window/2",
            &["windowFunctions", "windowV2"][..],
        ),
        (
            "dtcs:profile/portable-string-advanced/1",
            &["advancedStrings", "dtcsRegex"][..],
        ),
        (
            "dtcs:profile/portable-conversion/1",
            &["portableConversion"][..],
        ),
        (
            "dtcs:profile/portable-statistics/1",
            &["portableStatistics"][..],
        ),
        (
            "dtcs:profile/portable-complex-values/1",
            &["complexValues", "lambdaHof"][..],
        ),
        ("dtcs:profile/portable-reshape/1", &["reshape"][..]),
        (
            "dtcs:profile/portable-relational-extended/1",
            &["setOperations", "sampling"][..],
        ),
        (
            "dtcs:profile/portable-temporal-iana/1",
            &["ianaTimezone"][..],
        ),
        (
            "dtcs:profile/portable-nondeterministic/1",
            &["seededNondeterminism"][..],
        ),
    ] {
        profiles.push(semantic_profile(id, ImplementationClass::Compiler, caps));
    }
    profiles
}

fn semantic_profile(id: &str, class: ImplementationClass, optional: &[&str]) -> ConformanceProfile {
    ConformanceProfile {
        id: id.to_string(),
        implementation_class: class,
        dtcs_version: crate::SPEC_VERSION.to_string(),
        implementation_version: env!("CARGO_PKG_VERSION").to_string(),
        supported_registries: vec!["dtcs:builtin".into()],
        supported_extensions: vec!["acme".into()],
        optional_capabilities: optional.iter().map(|s| (*s).to_string()).collect(),
    }
}

/// Returns a profile by identifier.
#[must_use]
pub fn profile_by_id(id: &str) -> Option<ConformanceProfile> {
    all_profiles().into_iter().find(|p| p.id == id)
}

/// Builds the implementation capability declaration (Ch 23 §9).
#[must_use]
pub fn capability_declaration() -> ImplementationCapabilityDeclaration {
    let profiles = all_profiles();
    ImplementationCapabilityDeclaration {
        implementation_id: IMPLEMENTATION_ID.into(),
        implementation_version: env!("CARGO_PKG_VERSION").into(),
        dtcs_version: crate::SPEC_VERSION.into(),
        primary_profile: ImplementationClass::IntegratedPlatform.profile_id().into(),
        profiles,
    }
}
