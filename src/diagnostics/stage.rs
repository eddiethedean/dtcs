//! Processing stages that produce diagnostics.

/// Originating processing stage per SPEC Chapter 20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticStage {
    /// Document parsing.
    Parse,
    /// Canonical Object Model construction.
    CanonicalObjectModel,
    /// Validation.
    Validation,
    /// Analysis.
    Analysis,
    /// Planning.
    Planning,
    /// Compilation.
    Compilation,
    /// Runtime.
    Runtime,
}
