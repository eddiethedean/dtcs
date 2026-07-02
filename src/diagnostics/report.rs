//! Aggregated diagnostic report.

use super::{Diagnostic, Severity};

/// Collection of diagnostics produced during parsing or validation.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticReport {
    /// Diagnostics emitted during processing.
    pub diagnostics: Vec<Diagnostic>,
}

/// Alias aligned with implementation guides.
pub type ValidationReport = DiagnosticReport;

impl DiagnosticReport {
    /// Creates an empty report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a diagnostic.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Merges another report into this one.
    pub fn merge(&mut self, other: DiagnosticReport) {
        self.diagnostics.extend(other.diagnostics);
    }

    /// Returns `true` when no error-level diagnostics are present.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.diagnostics.iter().any(|d| d.severity.is_error())
    }

    /// Returns diagnostics at or above the given severity.
    #[must_use]
    pub fn with_min_severity(&self, min: Severity) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity >= min)
            .collect()
    }

    /// Returns error-level diagnostics.
    #[must_use]
    pub fn errors(&self) -> Vec<&Diagnostic> {
        self.with_min_severity(Severity::Error)
    }
}
