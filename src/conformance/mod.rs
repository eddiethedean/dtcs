//! Conformance profiles, declarations, and offline test suite (SPEC Chapter 23).

mod declare;
mod model;
mod profiles;
mod runner;
mod security;

pub use declare::{declare, declare_profile};
pub use model::{
    ConformanceAssertion, ConformanceManifest, ConformanceProfile, ConformanceReport,
    ConformanceTestCase, ConformanceTestResult, ImplementationCapabilityDeclaration,
    ImplementationClass,
};
pub use runner::{default_fixtures_dir, embedded_profiles, manifest, run_all, run_for_profiles};
pub use security::{run_security_probe, run_security_probes};
