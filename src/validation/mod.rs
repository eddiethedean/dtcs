//! Validation pipeline.

mod com;
pub(crate) mod context;
mod document;
mod expressions;
mod extensions;
mod field_index;
mod interfaces;
mod lineage;
mod phases;
mod references;
mod semantics;
mod structural;
mod types;

pub use phases::ValidationPhase;

use crate::diagnostics::ValidationReport;
use crate::metadata;
use crate::model::{RegistryDocument, TransformationContract};
use crate::registry;

use self::com::validate_com;
use self::context::ValidationContext;
use self::document::validate_document;
use self::expressions::validate_expressions;
use self::extensions::validate_extensions;
use self::interfaces::{
    validate_condition_rule_phases, validate_condition_rule_refs, validate_io_extensions,
    validate_optional_inputs,
};
use self::references::validate_references;
use self::semantics::validate_semantics;
use self::structural::validate_structural;
use self::types::validate_types;

/// Validate a transformation contract using the embedded standard registry.
#[must_use]
pub fn validate(contract: &TransformationContract) -> ValidationReport {
    validate_with_registry(contract, registry::default_registry())
}

/// Validate a transformation contract against a specific registry catalog.
#[must_use]
pub fn validate_with_registry(
    contract: &TransformationContract,
    registry: &RegistryDocument,
) -> ValidationReport {
    let mut ctx = ValidationContext::new();

    for phase in ValidationPhase::ORDER {
        match phase {
            ValidationPhase::Document => {
                validate_document(&mut ctx, contract);
                merge_versioning(&mut ctx, contract);
            }
            ValidationPhase::CanonicalObjectModel => {
                validate_com(&mut ctx, contract);
                metadata::validate_into(&mut ctx, contract);
            }
            ValidationPhase::Structural => {
                validate_structural(&mut ctx, contract);
                validate_optional_inputs(&mut ctx, contract);
            }
            ValidationPhase::Types => {
                validate_types(&mut ctx, contract);
                validate_expressions(&mut ctx, contract);
            }
            ValidationPhase::References => {
                validate_references(&mut ctx, contract);
                validate_condition_rule_refs(&mut ctx, contract);
            }
            ValidationPhase::Semantics => {
                validate_semantics(&mut ctx, contract, registry);
                validate_condition_rule_phases(&mut ctx, contract);
            }
            ValidationPhase::Extensions => {
                validate_extensions(&mut ctx, contract, registry);
                validate_io_extensions(&mut ctx, contract);
            }
        }
    }

    ctx.into_report()
}

fn merge_versioning(ctx: &mut ValidationContext, contract: &TransformationContract) {
    ctx.merge_report(crate::versioning::validate(contract));
}
