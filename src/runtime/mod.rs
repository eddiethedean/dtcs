//! Runtime execution (SPEC Chapter 16).

pub mod actions;
mod conversion;
mod expr;
pub mod functions;
mod lineage;
mod model;
mod reference;
pub mod rules;
mod runtime_trait;
mod validate;

pub use model::{
    parse_qualified_field, parse_qualified_field_with_interfaces, Dataset, QualifiedField, Row,
    RuntimeInputs, RuntimeOutputs, RuntimeValue,
};
pub use reference::ReferenceRuntime;
pub use runtime_trait::Runtime;

use crate::compile::ExecutionPlan;
use crate::diagnostics::Diagnostic;

/// Result of executing an execution plan.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResult {
    /// Produced outputs when execution succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<RuntimeOutputs>,
    /// Diagnostics from execution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}

impl ExecuteResult {
    /// Returns `true` when no error-level diagnostics are present.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.outputs.is_some() && !self.diagnostics.iter().any(|d| d.severity.is_error())
    }
}

/// Execute an execution plan using the reference runtime.
#[must_use]
pub fn execute(plan: &ExecutionPlan, inputs: &RuntimeInputs) -> ExecuteResult {
    ReferenceRuntime.execute(plan, inputs)
}
