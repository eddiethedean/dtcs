//! Validation pipeline.

mod com;
mod context;
mod document;
mod extensions;
mod field_index;
mod lineage;
mod phases;
mod references;
mod semantics;
mod structural;
mod types;

pub use phases::ValidationPhase;

use crate::diagnostics::ValidationReport;
use crate::model::TransformationContract;

use self::com::validate_com;
use self::context::ValidationContext;
use self::document::validate_document;
use self::extensions::validate_extensions;
use self::references::validate_references;
use self::semantics::validate_semantics;
use self::structural::validate_structural;
use self::types::validate_types;

/// Validate a transformation contract and collect diagnostics.
#[must_use]
pub fn validate(contract: &TransformationContract) -> ValidationReport {
    let mut ctx = ValidationContext::new();

    for phase in ValidationPhase::ORDER {
        match phase {
            ValidationPhase::Document => validate_document(&mut ctx, contract),
            ValidationPhase::CanonicalObjectModel => validate_com(&mut ctx, contract),
            ValidationPhase::Structural => validate_structural(&mut ctx, contract),
            ValidationPhase::Types => validate_types(&mut ctx, contract),
            ValidationPhase::References => validate_references(&mut ctx, contract),
            ValidationPhase::Semantics => validate_semantics(&mut ctx, contract),
            ValidationPhase::Extensions => validate_extensions(&mut ctx, contract),
        }
    }

    ctx.into_report()
}
