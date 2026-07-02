//! Canonical Object Model validation phase.

use crate::diagnostics::{codes, DiagnosticCategory, DiagnosticStage};
use crate::model::TransformationContract;

use super::context::ValidationContext;

pub(crate) fn validate_com(ctx: &mut ValidationContext, contract: &TransformationContract) {
    if contract.id.contains(' ') {
        ctx.error_with_stage(
            codes::INVALID_IDENTIFIER,
            DiagnosticCategory::Structure,
            "contract id should not contain whitespace",
            Some("id"),
            Some("Use a stable dotted or slashed identifier"),
            DiagnosticStage::CanonicalObjectModel,
        );
    }
}
