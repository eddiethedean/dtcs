//! Diagnostic categories.

/// Standard diagnostic category per SPEC Chapter 20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticCategory {
    /// Syntax errors.
    Syntax,
    /// Structural errors.
    Structure,
    /// Type errors.
    Type,
    /// Reference errors.
    Reference,
    /// Semantic errors.
    Semantic,
    /// Compatibility errors.
    Compatibility,
    /// Capability errors.
    Capability,
    /// Runtime errors.
    Runtime,
    /// Extension errors.
    Extension,
}
