//! Capability declaration validation (SPEC Ch 14 §11).

use crate::diagnostics::{codes, compilation_error, DiagnosticCategory, DiagnosticReport};
use crate::model::is_namespaced_identifier;
use crate::registry;

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
    } else if !is_namespaced_identifier(&declaration.engine_id)
        && declaration.engine_id != "dtcs:reference"
    {
        // Allow `dtcs:reference` and other namespaced engine ids.
        if !declaration.engine_id.contains(':') {
            report.push(
                compilation_error(
                    codes::INVALID_CAPABILITY,
                    DiagnosticCategory::Capability,
                    format!(
                        "capability engineId '{}' must be a namespaced identifier",
                        declaration.engine_id
                    ),
                )
                .with_object_ref("engineId"),
            );
        }
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
    } else if !looks_like_version(&declaration.engine_version) {
        report.push(
            compilation_error(
                codes::INVALID_CAPABILITY,
                DiagnosticCategory::Capability,
                format!(
                    "capability engineVersion '{}' is not a valid version string",
                    declaration.engine_version
                ),
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
    } else if !looks_like_version(&declaration.capability_version) {
        report.push(
            compilation_error(
                codes::INVALID_CAPABILITY,
                DiagnosticCategory::Capability,
                format!(
                    "capabilityVersion '{}' is not a valid version string",
                    declaration.capability_version
                ),
            )
            .with_object_ref("capabilityVersion"),
        );
    }

    let registry = registry::default_registry();
    for action in &declaration.categories.semantic_actions {
        if action.starts_with("dtcs:") && registry::resolve(registry, action).is_none() {
            report.push(
                compilation_error(
                    codes::INVALID_CAPABILITY,
                    DiagnosticCategory::Capability,
                    format!("capability references unknown semantic action '{action}'"),
                )
                .with_object_ref("categories.semanticActions"),
            );
        }
    }
    for function in &declaration.categories.functions {
        if function.starts_with("dtcs:") && registry::resolve(registry, function).is_none() {
            report.push(
                compilation_error(
                    codes::INVALID_CAPABILITY,
                    DiagnosticCategory::Capability,
                    format!("capability references unknown function '{function}'"),
                )
                .with_object_ref("categories.functions"),
            );
        }
    }
    for rule in &declaration.categories.rules {
        if rule.starts_with("dtcs:") && registry::resolve(registry, rule).is_none() {
            report.push(
                compilation_error(
                    codes::INVALID_CAPABILITY,
                    DiagnosticCategory::Capability,
                    format!("capability references unknown rule '{rule}'"),
                )
                .with_object_ref("categories.rules"),
            );
        }
    }

    report
}

fn looks_like_version(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() {
        return false;
    }
    value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+' | '_'))
}
