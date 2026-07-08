//! Static semantic analysis (SPEC Chapters 7–8).

#![allow(missing_docs)]

mod contract;
mod semantics;

pub mod expr;

use serde::{Deserialize, Serialize};

use crate::diagnostics::Diagnostic;

pub use contract::check_contract;
pub use expr::check_expression;

/// Analysis report containing diagnostics and attachable findings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisReport {
    /// Diagnostics produced during analysis.
    pub diagnostics: Vec<Diagnostic>,
    /// Findings attachable to downstream plan nodes.
    pub findings: Vec<AnalysisFinding>,
}

impl AnalysisReport {
    /// Returns `true` when no error-level diagnostics are present.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.diagnostics.iter().any(|d| d.severity.is_error())
    }

    pub(crate) fn merge(&mut self, mut other: AnalysisReport) {
        self.diagnostics.append(&mut other.diagnostics);
        self.findings.append(&mut other.findings);
    }
}

/// A structured analysis finding keyed by object reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisFinding {
    /// The object reference this finding attaches to (for example `expressions.double_value`).
    pub object_ref: String,
    /// Finding kind identifier (stable, machine-readable).
    pub kind: String,
    /// Human-readable message.
    pub message: String,
    /// Optional structured attributes.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub attributes: std::collections::BTreeMap<String, serde_json::Value>,
}

