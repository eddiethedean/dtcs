//! Runtime trait (SPEC Chapter 16).

use super::model::RuntimeInputs;
use super::ExecuteResult;
use crate::compile::ExecutionPlan;

/// A DTCS runtime executing execution plans.
pub trait Runtime {
    /// Target engine identifier.
    fn target_id(&self) -> &str;

    /// Execute a validated execution plan.
    fn execute(&self, plan: &ExecutionPlan, inputs: &RuntimeInputs) -> ExecuteResult;
}
