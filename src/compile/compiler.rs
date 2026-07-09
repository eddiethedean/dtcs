//! Compiler trait (SPEC Chapter 15).

use super::CompileResult;
use crate::capability::EngineCapabilityDeclaration;
use crate::plan::TransformationPlan;

/// A DTCS compiler lowering transformation plans to execution plans.
pub trait Compiler {
    /// Target engine identifier.
    fn target_id(&self) -> &str;

    /// Compile a transformation plan for this backend.
    fn compile(
        &self,
        plan: &TransformationPlan,
        capability: &EngineCapabilityDeclaration,
    ) -> CompileResult;
}
