//! Reference implementation of the Data Transformation Contract Standard (DTCS).
//!
//! [`SPEC.md`](../SPEC.md) at the repository root is the authoritative normative
//! specification. This crate implements the pipeline through contract analysis:
//!
//! ```text
//! DTCS Document → Parser → Canonical Object Model → Validator → Diagnostics
//!                                              ├→ Analyzer → Analysis reports
//!                                              └→ Registry (identifier resolution)
//! ```
//!
//! # Example
//!
//! ```
//! use dtcs::{parse, validate, DocumentFormat};
//!
//! let yaml = br#"
//! dtcsVersion: "1.0.0"
//! id: "example"
//! name: "Example"
//! version: "0.2.0"
//! inputs:
//!   - id: "in"
//!     schema:
//!       fields:
//!         - name: "value"
//!           type: "string"
//!           nullable: false
//! outputs:
//!   - id: "out"
//!     schema:
//!       fields:
//!         - name: "value"
//!           type: "string"
//!           nullable: false
//! lineage:
//!   mappings:
//!     - output: "out"
//!       inputs: ["in"]
//! "#;
//!
//! let result = parse(yaml, DocumentFormat::Yaml);
//! let contract = result.contract.expect("parse succeeded");
//! let report = validate(&contract);
//! assert!(report.is_valid());
//! ```

/// DTCS specification version this crate targets.
pub const SPEC_VERSION: &str = "1.0.0-draft";

pub mod compatibility;
pub mod diagnostics;
pub mod lineage;
pub mod metadata;
pub mod analysis;
pub mod model;
pub mod parser;
pub mod plan;
pub mod registry;
pub mod validation;
pub mod versioning;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "python")]
mod python;

pub use compatibility::{
    analyze as analyze_compatibility, analyze_evolution, ChangeCategory, ComparisonScope,
    CompatibilityLevel, CompatibilityReport, EvolutionReport,
};
pub use diagnostics::{
    codes, inspect_contract, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    Severity, ValidationReport,
};
pub use lineage::{analyze as analyze_lineage, LineageAnalysisReport, LineageGovernance};
pub use analysis::{check_contract, check_expression, AnalysisFinding, AnalysisReport};
pub use model::{
    parse_logical_type, type_compatible, ExtensionCompatibility, LogicalType, RegistryCategory,
    RegistryDocument, RegistryEntry, RegistryEntryStatus, RegistryPublicationStatus, RegistryRef,
    TransformationContract, TypeCompatibility, TypeParseError,
};
pub use parser::{parse, parse_file, parse_json, parse_yaml, DocumentFormat, ParseResult};
pub use registry::{
    default_registry, is_known_action, is_known_function, is_known_rule, load as load_registry,
    load_merged, resolve as resolve_registry, resolve_default,
};
pub use validation::{validate, validate_with_registry, ValidationPhase};

/// Parse and validate a DTCS document in one step.
#[must_use]
pub fn parse_and_validate(content: &[u8], format: DocumentFormat) -> ValidationReport {
    parse(content, format).validate()
}

impl TransformationContract {
    /// Parse a contract from YAML text.
    pub fn from_yaml(content: &str) -> ParseResult {
        parse(content.as_bytes(), DocumentFormat::Yaml)
    }

    /// Parse a contract from JSON text.
    pub fn from_json(content: &str) -> ParseResult {
        parse(content.as_bytes(), DocumentFormat::Json)
    }

    /// Parse a contract from a file path.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> miette::Result<ParseResult> {
        parse_file(path)
    }
}
