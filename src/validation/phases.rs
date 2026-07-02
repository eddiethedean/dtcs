//! Validation phases defined in `SPEC.md` Chapter 9.

/// A validation phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationPhase {
    /// Document validation.
    Document,
    /// Canonical Object Model validation.
    CanonicalObjectModel,
    /// Structural validation.
    Structural,
    /// Type validation.
    Types,
    /// Reference validation.
    References,
    /// Semantic validation.
    Semantics,
    /// Extension validation.
    Extensions,
}

impl ValidationPhase {
    /// All phases in normative order.
    pub const ORDER: [Self; 7] = [
        Self::Document,
        Self::CanonicalObjectModel,
        Self::Structural,
        Self::Types,
        Self::References,
        Self::Semantics,
        Self::Extensions,
    ];
}
