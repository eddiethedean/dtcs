//! Compatibility and evolution type definitions (SPEC Ch 11–12).

use serde::{Deserialize, Serialize};

/// Compatibility classification (SPEC Chapter 11 §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompatibilityLevel {
    /// Contracts are semantically identical within the comparison scope.
    Identical,
    /// Target extends source without breaking required behavior.
    BackwardCompatible,
    /// Source extends target without breaking required behavior.
    ForwardCompatible,
    /// Compatible only under documented conditions.
    ConditionallyCompatible,
    /// Contracts cannot interoperate safely.
    Incompatible,
}

/// Contract change category (SPEC Chapter 12 §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeCategory {
    /// Metadata change.
    Metadata,
    /// Interface change.
    Interface,
    /// Logical type change.
    Type,
    /// Semantic action, expression, or function change.
    Semantic,
    /// Rule change.
    Rule,
    /// Lineage change.
    Lineage,
    /// Extension change.
    Extension,
}

/// Scoped comparison aspects (SPEC Chapter 11 §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonScope {
    /// Compare inputs and outputs.
    #[serde(default = "default_true")]
    pub interfaces: bool,
    /// Compare logical types within interfaces.
    #[serde(default = "default_true")]
    pub types: bool,
    /// Compare semantic actions, expressions, functions, and rules.
    #[serde(default = "default_true")]
    pub semantics: bool,
    /// Compare lineage mappings.
    #[serde(default = "default_true")]
    pub lineage: bool,
    /// Compare behavioral metadata (governance, identity).
    #[serde(default = "default_true")]
    pub metadata: bool,
    /// Compare vendor extension keys.
    #[serde(default = "default_true")]
    pub extensions: bool,
}

const fn default_true() -> bool {
    true
}

impl ComparisonScope {
    /// All comparison aspects enabled.
    #[must_use]
    pub const fn all() -> Self {
        Self {
            interfaces: true,
            types: true,
            semantics: true,
            lineage: true,
            metadata: true,
            extensions: true,
        }
    }

    /// Parse scope from CLI tokens such as `interfaces,types`.
    #[must_use]
    pub fn from_tokens(tokens: &[String]) -> Self {
        if tokens.is_empty() {
            return Self::all();
        }
        let mut scope = Self {
            interfaces: false,
            types: false,
            semantics: false,
            lineage: false,
            metadata: false,
            extensions: false,
        };
        for token in tokens {
            match token.as_str() {
                "interfaces" => scope.interfaces = true,
                "types" => scope.types = true,
                "semantics" => scope.semantics = true,
                "lineage" => scope.lineage = true,
                "metadata" => scope.metadata = true,
                "extensions" => scope.extensions = true,
                "all" => return Self::all(),
                _ => {}
            }
        }
        if !scope.interfaces
            && !scope.types
            && !scope.semantics
            && !scope.lineage
            && !scope.metadata
            && !scope.extensions
        {
            return Self::all();
        }
        scope
    }
}

/// Kind of difference detected during comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DiffKind {
    /// No difference.
    Neutral,
    /// Additive extension of the target relative to the source.
    Additive,
    /// Breaking change relative to the source.
    Breaking,
    /// Change that is breaking unless optional conditions apply.
    Conditional,
}

/// A single aspect comparison outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AspectResult {
    /// Aspect name (`interfaces`, `types`, etc.).
    pub aspect: String,
    /// Whether the aspect matched within scope.
    pub compatible: bool,
    /// Summary message.
    pub message: String,
}
