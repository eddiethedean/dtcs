//! DTCS document parsers.

mod json;
mod yaml;

use std::path::Path;

pub use json::parse_json;
pub use yaml::parse_yaml;

use crate::diagnostics::{
    codes, emit, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage, Severity,
};
use crate::model::TransformationContract;

/// Result of parsing a DTCS document.
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Parsed contract when parsing succeeded.
    pub contract: Option<TransformationContract>,
    /// Parse-time diagnostics.
    pub report: DiagnosticReport,
}

impl ParseResult {
    /// Returns the parsed contract when parsing succeeded without parse errors.
    pub fn into_contract(self) -> Result<TransformationContract, DiagnosticReport> {
        match (self.contract, self.report.is_valid()) {
            (Some(contract), true) => Ok(contract),
            (_, false) => Err(self.report),
            (None, true) => {
                let mut report = self.report;
                emit(
                    &mut report,
                    Diagnostic::new(
                        codes::PARSE_ERROR,
                        Severity::Error,
                        DiagnosticStage::Parse,
                        DiagnosticCategory::Syntax,
                        "parse succeeded but no contract was produced",
                    ),
                );
                Err(report)
            }
        }
    }

    /// Parses and validates in one step.
    #[must_use]
    pub fn validate(self) -> DiagnosticReport {
        let mut report = self.report;
        if let Some(contract) = self.contract {
            report.merge(crate::validate(&contract));
        }
        report
    }
}

/// Supported DTCS document serialization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    /// YAML encoding.
    Yaml,
    /// JSON encoding.
    Json,
}

impl DocumentFormat {
    /// Infers format from a file extension.
    #[must_use]
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "yaml" | "yml" => Some(Self::Yaml),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Maximum DTCS document size accepted by the parser (16 MiB).
pub const MAX_DOCUMENT_BYTES: usize = 16 * 1024 * 1024;

/// Parse a DTCS document from bytes.
#[must_use]
pub fn parse(content: &[u8], format: DocumentFormat) -> ParseResult {
    if content.len() > MAX_DOCUMENT_BYTES {
        return failure(format!(
            "document exceeds maximum size of {} bytes",
            MAX_DOCUMENT_BYTES
        ));
    }
    match format {
        DocumentFormat::Yaml => parse_yaml(content),
        DocumentFormat::Json => parse_json(content),
    }
}

/// Parse a DTCS document from a file path.
pub fn parse_file(path: impl AsRef<Path>) -> miette::Result<ParseResult> {
    let path = path.as_ref();
    let content = std::fs::read(path)
        .map_err(|e| miette::miette!("failed to read {}: {e}", path.display()))?;
    let format = DocumentFormat::from_path(path).ok_or_else(|| {
        miette::miette!(
            "unsupported file extension for {}; use .yaml, .yml, or .json",
            path.display()
        )
    })?;
    Ok(parse(&content, format))
}

fn parse_error(message: impl Into<String>) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    emit(
        &mut report,
        Diagnostic::new(
            codes::PARSE_ERROR,
            Severity::Error,
            DiagnosticStage::Parse,
            DiagnosticCategory::Syntax,
            message,
        ),
    );
    report
}

pub(crate) fn success(contract: TransformationContract) -> ParseResult {
    ParseResult {
        contract: Some(contract),
        report: DiagnosticReport::new(),
    }
}

pub(crate) fn failure(message: impl Into<String>) -> ParseResult {
    ParseResult {
        contract: None,
        report: parse_error(message),
    }
}
