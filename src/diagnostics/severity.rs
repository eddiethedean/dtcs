//! Diagnostic severity levels.

/// Diagnostic severity per SPEC Chapter 20.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Informational observation.
    #[serde(rename = "information")]
    Information,
    /// Non-fatal observation.
    Warning,
    /// Blocks validation or processing.
    Error,
}

impl Severity {
    /// Returns `true` when this severity blocks successful validation.
    pub fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}
