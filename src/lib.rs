//! Reference implementation of the Data Transformation Contract Standard (DTCS).
//!
//! [`SPEC.md`](../SPEC.md) at the repository root is the authoritative normative
//! specification. This crate implements the foundational pipeline:
//!
//! ```text
//! DTCS Document → Parser → Canonical Object Model → Validator → Diagnostics
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
//! version: "0.1.0"
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
pub mod metadata;
pub mod model;
pub mod parser;
pub mod plan;
pub mod validation;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "python")]
mod python;

pub use diagnostics::{
    codes, inspect_contract, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    Severity, ValidationReport,
};
pub use model::{
    parse_logical_type, type_compatible, LogicalType, TransformationContract, TypeCompatibility,
    TypeParseError,
};
pub use parser::{parse, parse_file, parse_json, parse_yaml, DocumentFormat, ParseResult};
pub use validation::{validate, ValidationPhase};

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
