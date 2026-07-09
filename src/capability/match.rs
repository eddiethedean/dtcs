//! Capability matching (SPEC Ch 14 §6).

use crate::diagnostics::{codes, compilation_error, DiagnosticCategory};
use crate::model::RegistryDocument;
use crate::plan::{validate_with_registry, TransformationPlan};
use crate::registry;

use super::model::{CapabilityMatchReport, EngineCapabilityDeclaration};
use super::requirements::PlanRequirements;
use super::validate;

/// Match a transformation plan against an engine capability declaration.
#[must_use]
pub fn match_plan(
    plan: &TransformationPlan,
    declaration: &EngineCapabilityDeclaration,
) -> CapabilityMatchReport {
    match_plan_with_registry(plan, declaration, registry::default_registry())
}

/// Match a transformation plan against an engine capability declaration with a registry catalog.
#[must_use]
pub fn match_plan_with_registry(
    plan: &TransformationPlan,
    declaration: &EngineCapabilityDeclaration,
    registry_doc: &RegistryDocument,
) -> CapabilityMatchReport {
    let mut report = CapabilityMatchReport::default();

    let plan_validation = validate_with_registry(plan, registry_doc);
    if !plan_validation.is_valid() {
        report.diagnostics = plan_validation.diagnostics;
        report.supported = false;
        return report;
    }

    let validation = validate(declaration);
    if !validation.is_valid() {
        report.diagnostics = validation.diagnostics;
        report.supported = false;
        return report;
    }

    let requirements = PlanRequirements::from_plan(plan);
    report.missing = requirements.gaps_against(declaration);

    for gap in &report.missing {
        report.diagnostics.push(
            compilation_error(
                codes::UNSUPPORTED_CAPABILITY,
                DiagnosticCategory::Capability,
                format!(
                    "engine '{}' does not support {} '{}'",
                    declaration.engine_id, gap.category, gap.required
                ),
            )
            .with_object_ref(&gap.required),
        );
    }

    report.supported = report.missing.is_empty();
    report
}
