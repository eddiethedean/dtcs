//! Engine capability model and matching (SPEC Chapter 14).

mod builtin;
#[path = "match.rs"]
mod matching;
mod model;
mod requirements;
mod validate;

pub use builtin::{reference_profile, REFERENCE_ENGINE_ID};
pub use matching::{match_plan, match_plan_with_registry};
pub use model::{
    CapabilityCategories, CapabilityGap, CapabilityMatchReport, EngineCapabilityDeclaration,
};
pub use requirements::PlanRequirements;
pub use validate::validate;

/// Discover available capability profiles programmatically (Ch 14 §9).
#[must_use]
pub fn discover() -> Vec<EngineCapabilityDeclaration> {
    vec![reference_profile()]
}
