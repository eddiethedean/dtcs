//! Conformance model types (SPEC Chapter 23).

use serde::{Deserialize, Serialize};

/// DTCS implementation class (Ch 23 §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImplementationClass {
    /// Parser implementation class.
    Parser,
    /// Validator implementation class.
    Validator,
    /// Analyzer implementation class.
    Analyzer,
    /// Planner implementation class.
    Planner,
    /// Optimizer implementation class.
    Optimizer,
    /// Compiler implementation class.
    Compiler,
    /// Runtime implementation class.
    Runtime,
    /// Integrated platform implementation class.
    IntegratedPlatform,
}

impl ImplementationClass {
    /// Returns the stable profile identifier for this class.
    #[must_use]
    pub fn profile_id(self) -> &'static str {
        match self {
            Self::Parser => "parser",
            Self::Validator => "validator",
            Self::Analyzer => "analyzer",
            Self::Planner => "planner",
            Self::Optimizer => "optimizer",
            Self::Compiler => "compiler",
            Self::Runtime => "runtime",
            Self::IntegratedPlatform => "integrated-platform",
        }
    }

    /// All implementation classes.
    pub const ALL: [Self; 8] = [
        Self::Parser,
        Self::Validator,
        Self::Analyzer,
        Self::Planner,
        Self::Optimizer,
        Self::Compiler,
        Self::Runtime,
        Self::IntegratedPlatform,
    ];
}

/// Machine-readable conformance profile (Ch 23 §5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConformanceProfile {
    /// Stable profile identifier.
    pub id: String,
    /// Implementation class.
    pub implementation_class: ImplementationClass,
    /// DTCS specification version.
    pub dtcs_version: String,
    /// Implementation version when the profile was published.
    pub implementation_version: String,
    /// Supported registry identifiers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_registries: Vec<String>,
    /// Supported extension namespace prefixes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_extensions: Vec<String>,
    /// Optional capabilities beyond mandatory requirements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub optional_capabilities: Vec<String>,
}

/// Implementation capability declaration (Ch 23 §9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationCapabilityDeclaration {
    /// Implementation identity.
    pub implementation_id: String,
    /// Implementation version.
    pub implementation_version: String,
    /// DTCS specification version.
    pub dtcs_version: String,
    /// Declared conformance profiles.
    pub profiles: Vec<ConformanceProfile>,
    /// Primary profile identifier.
    pub primary_profile: String,
}

/// Assertion kind for a conformance test case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ConformanceAssertion {
    /// Parse succeeds and produces a contract.
    ParseValid,
    /// Parse fails.
    ParseInvalid,
    /// Validation succeeds.
    ValidateValid,
    /// Validation fails with optional diagnostic codes.
    ValidateInvalid {
        /// Expected diagnostic codes.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        codes: Vec<String>,
    },
    /// Plan lowering succeeds.
    PlanValid,
    /// Optimization preserves semantics.
    OptimizeEquivalent,
    /// Capability match reports supported.
    MatchSupported,
    /// Compilation succeeds.
    CompileValid,
    /// Runtime execution matches expected output JSON.
    RuntimeOutput {
        /// Runtime input fixture path relative to tests/fixtures.
        input: String,
        /// Expected output fixture path relative to tests/fixtures.
        #[serde(rename = "expectedOutput")]
        expected_output: String,
    },
    /// Runtime execution fails with expected diagnostic codes.
    RuntimeInvalid {
        /// Runtime input fixture path relative to tests/fixtures.
        input: String,
        /// Expected runtime diagnostic codes.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        codes: Vec<String>,
    },
    /// Security probe identifier.
    SecurityProbe {
        /// Probe id from security checklist.
        #[serde(rename = "probeId")]
        probe_id: String,
    },
}

/// A single conformance test definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConformanceTestCase {
    /// Stable test identifier.
    pub id: String,
    /// Profiles that must run this test.
    pub profiles: Vec<String>,
    /// Contract fixture path relative to tests/fixtures.
    pub fixture: String,
    /// Document format.
    #[serde(default = "default_yaml")]
    pub format: String,
    /// Assertion to verify.
    pub assertion: ConformanceAssertion,
}

fn default_yaml() -> String {
    "yaml".into()
}

/// Embedded conformance manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConformanceManifest {
    /// Manifest version.
    pub version: String,
    /// Test cases.
    pub tests: Vec<ConformanceTestCase>,
}

/// Result of a single conformance test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConformanceTestResult {
    /// Test identifier.
    pub id: String,
    /// Profile under test.
    pub profile: String,
    /// Whether the test passed.
    pub passed: bool,
    /// Failure message when not passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Conformance report (Ch 23 §12).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConformanceReport {
    /// Implementation identifier.
    pub implementation_id: String,
    /// Implementation version.
    pub implementation_version: String,
    /// Profiles exercised.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<String>,
    /// Individual test results.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<ConformanceTestResult>,
    /// Security checklist results.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<ConformanceTestResult>,
    /// Whether all tests passed.
    pub passed: bool,
}

impl ConformanceReport {
    /// Returns true when all tests and security probes passed.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.passed
    }
}
