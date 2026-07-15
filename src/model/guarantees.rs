//! Contractual guarantees (SPEC Chapter 2 §3).

use serde::{Deserialize, Serialize};

use super::semantics::TransformationSemantics;

/// First-class contractual guarantees on a Transformation Contract.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractGuarantees {
    /// Transformation semantics (determinism, purity, ordering, side effects).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantics: Option<TransformationSemantics>,
    /// Declared information-loss policy summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub information_loss: Option<String>,
    /// Free-form guarantee statements preserved for documentation/analysis.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statements: Vec<String>,
}
