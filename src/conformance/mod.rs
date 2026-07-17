//! Conformance profiles, declarations, and offline test suite (SPEC Chapter 23).

mod declare;
mod fixtures;
mod model;
mod portable;
mod profiles;
mod runner;
mod security;

pub use declare::{declare, declare_profile};
pub use fixtures::default_fixtures_dir;
pub use model::{
    ConformanceAssertion, ConformanceManifest, ConformanceProfile, ConformanceReport,
    ConformanceTestCase, ConformanceTestResult, ImplementationCapabilityDeclaration,
    ImplementationClass,
};
pub use portable::{
    datasets_match_expected, run_portable_differential_case, run_portable_fixture,
    PortableDifferentialFixture, PortableEvalMode,
};
pub use runner::{embedded_profiles, manifest, run_all, run_for_profiles};
pub use security::{run_security_probe, run_security_probes};
