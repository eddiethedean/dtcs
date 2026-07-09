//! Capability declaration validation (SPEC Ch 14 §11).

use crate::diagnostics::{codes, compilation_error, DiagnosticCategory, DiagnosticReport};

use super::model::EngineCapabilityDeclaration;

/// Validate an engine capability declaration.
#[must_use]
pub fn validate(declaration: &EngineCapabilityDeclaration) -> DiagnosticReport {
    let mut report = DiagnosticReport::default();

    if declaration.engine_id.trim().is_empty() {
        report.push(
            compilation_error(
                codes::INVALID_CAPABILITY,
                DiagnosticCategory::Capability,
                "capability declaration is missing engineId",
            )
            .with_object_ref("engineId"),
        );
    }
    if declaration.engine_version.trim().is_empty() {
        report.push(
            compilation_error(
                codes::INVALID_CAPABILITY,
                DiagnosticCategory::Capability,
                "capability declaration is missing engineVersion",
            )
            .with_object_ref("engineVersion"),
        );
    }
    if declaration.capability_version.trim().is_empty() {
        report.push(
            compilation_error(
                codes::INVALID_CAPABILITY,
                DiagnosticCategory::Capability,
                "capability declaration is missing capabilityVersion",
            )
            .with_object_ref("capabilityVersion"),
        );
    }

    report
}
