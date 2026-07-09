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
    vec![
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
    ]
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
