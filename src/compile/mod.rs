//! Compilation of transformation plans to execution plans (SPEC Chapter 15).

mod compiler;
mod model;
mod reference;
mod validate;

pub use compiler::Compiler;
pub use model::{ExecutionPlan, ExecutionStep, ExecutionStepKind, ExecutionTarget};
pub use reference::{compile_reference, ReferenceCompiler};
pub use validate::validate;

use crate::capability::{match_plan, reference_profile};
use crate::diagnostics::Diagnostic;
use crate::plan::TransformationPlan;

/// Result of compiling a transformation plan.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileResult {
    /// Compiled execution plan when compilation succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<ExecutionPlan>,
    /// Diagnostics from compilation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}

impl CompileResult {
    /// Returns `true` when no error-level diagnostics are present.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.plan.is_some() && !self.diagnostics.iter().any(|d| d.severity.is_error())
    }
}

/// Compile a transformation plan using the reference backend.
#[must_use]
pub fn compile(plan: &TransformationPlan) -> CompileResult {
    reference::compile_reference(plan)
}

/// Compile with an explicit capability declaration.
#[must_use]
pub fn compile_with_capability(
    plan: &TransformationPlan,
    capability: &crate::capability::EngineCapabilityDeclaration,
) -> CompileResult {
    reference::ReferenceCompiler.compile(plan, capability)
}

/// Match then compile, returning match diagnostics on failure.
#[must_use]
pub fn compile_after_match(plan: &TransformationPlan) -> CompileResult {
    let capability = reference_profile();
    let match_report = match_plan(plan, &capability);
    if !match_report.is_valid() {
        return CompileResult {
            diagnostics: match_report.diagnostics,
            ..CompileResult::default()
        };
    }
    reference::ReferenceCompiler.compile(plan, &capability)
}
