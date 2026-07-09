//! Implementation capability declaration (Ch 23 §9).

use super::model::ImplementationCapabilityDeclaration;
use super::profiles;

/// Builds the implementation capability declaration for all profiles.
#[must_use]
pub fn declare() -> ImplementationCapabilityDeclaration {
    profiles::capability_declaration()
}

/// Builds a declaration filtered to a single profile identifier.
#[must_use]
pub fn declare_profile(profile_id: &str) -> Option<ImplementationCapabilityDeclaration> {
    let profile = profiles::profile_by_id(profile_id)?;
    Some(ImplementationCapabilityDeclaration {
        implementation_id: profiles::capability_declaration().implementation_id,
        implementation_version: env!("CARGO_PKG_VERSION").into(),
        dtcs_version: crate::SPEC_VERSION.into(),
        primary_profile: profile.id.clone(),
        profiles: vec![profile],
    })
}
