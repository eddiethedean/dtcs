//! Convenience builders for diagnostics at specific processing stages.

use super::{Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage, Severity};

/// Emit a diagnostic into a report.
pub(crate) fn emit(report: &mut DiagnosticReport, diagnostic: Diagnostic) {
    report.push(diagnostic);
}

/// Build an error diagnostic for the given processing stage.
pub(crate) fn stage_error(
    id: &str,
    stage: DiagnosticStage,
    category: DiagnosticCategory,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::new(id, Severity::Error, stage, category, message)
}

/// Convenience builder for validation-stage errors.
pub(crate) fn validation_error(
    id: &str,
    category: DiagnosticCategory,
    message: impl Into<String>,
) -> Diagnostic {
    stage_error(id, DiagnosticStage::Validation, category, message)
}

/// Convenience builder for Canonical Object Model stage errors.
pub(crate) fn com_error(
    id: &str,
    category: DiagnosticCategory,
    message: impl Into<String>,
) -> Diagnostic {
    stage_error(id, DiagnosticStage::CanonicalObjectModel, category, message)
}
