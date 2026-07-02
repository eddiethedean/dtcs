//! Metadata validation (SPEC Chapter 5).

mod validate;

use crate::diagnostics::ValidationReport;
use crate::model::TransformationContract;
use crate::validation::context::ValidationContext;

/// Validate metadata for a contract and collect diagnostics.
#[must_use]
pub fn validate(contract: &TransformationContract) -> ValidationReport {
    let mut ctx = ValidationContext::new();
    validate_into(&mut ctx, contract);
    ctx.into_report()
}

/// Validate metadata into an existing validation context.
pub(crate) fn validate_into(ctx: &mut ValidationContext, contract: &TransformationContract) {
    validate::validate_metadata(ctx, contract);
}
