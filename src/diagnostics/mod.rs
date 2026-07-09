//! DTCS diagnostics.

mod builders;
mod category;
pub mod codes;
mod diagnostic;
mod report;
mod severity;
mod stage;

pub use category::DiagnosticCategory;
pub use diagnostic::Diagnostic;
pub use report::{DiagnosticReport, ValidationReport};
pub use severity::Severity;
pub use stage::DiagnosticStage;

use crate::model::TransformationContract;

pub(crate) use builders::{com_error, emit, planning_error, validation_error};

/// Returns a short human-readable contract summary.
#[must_use]
pub fn inspect_contract(contract: &TransformationContract) -> String {
    format!(
        "id: {}\nname: {}\nversion: {}\ndtcsVersion: {}\ninputs: {}\noutputs: {}\nsemanticActions: {}\nrules: {}\nexpressions: {}\nfunctions: {}\n",
        contract.id,
        contract.name,
        contract.version,
        contract.dtcs_version,
        contract.inputs.len(),
        contract.outputs.len(),
        contract.semantic_actions.len(),
        contract.rules.len(),
        contract.expressions.len(),
        contract.functions.len(),
    )
}
