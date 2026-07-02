//! A single diagnostic message.

use super::{DiagnosticCategory, DiagnosticStage, Severity};

/// A spec-level diagnostic record.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Diagnostic {
    /// Stable `dtcs:` diagnostic identifier.
    pub id: String,
    /// Diagnostic severity.
    pub severity: Severity,
    /// Originating processing stage.
    pub stage: DiagnosticStage,
    /// Diagnostic category.
    pub category: DiagnosticCategory,
    /// Human-readable message.
    pub message: String,
    /// Affected object reference, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ref: Option<String>,
    /// Suggested remediation, when practical.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
}

impl Diagnostic {
    /// Creates a new diagnostic.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        severity: Severity,
        stage: DiagnosticStage,
        category: DiagnosticCategory,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            stage,
            category,
            message: message.into(),
            object_ref: None,
            remediation: None,
        }
    }

    /// Sets the affected object reference.
    #[must_use]
    pub fn with_object_ref(mut self, object_ref: impl Into<String>) -> Self {
        self.object_ref = Some(object_ref.into());
        self
    }

    /// Sets remediation guidance.
    #[must_use]
    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }
}
