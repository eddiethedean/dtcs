//! Post-compile execution plan validation (SPEC Ch 15 §11).

use crate::diagnostics::{codes, compilation_error, DiagnosticCategory, DiagnosticReport};

use super::model::{ExecutionPlan, ExecutionStepKind};

/// Validate a compiled execution plan.
#[must_use]
pub fn validate(plan: &ExecutionPlan) -> DiagnosticReport {
    let mut report = DiagnosticReport::default();

    if plan.target.engine_id.trim().is_empty() {
        report.push(
            compilation_error(
                codes::INVALID_EXECUTION_PLAN,
                DiagnosticCategory::Structure,
                "execution plan is missing target.engineId",
            )
            .with_object_ref("target.engineId"),
        );
    }
    if plan.inputs.is_empty() {
        report.push(
            compilation_error(
                codes::INVALID_EXECUTION_PLAN,
                DiagnosticCategory::Structure,
                "execution plan is missing inputs",
            )
            .with_object_ref("inputs"),
        );
    }
    if plan.outputs.is_empty() {
        report.push(
            compilation_error(
                codes::INVALID_EXECUTION_PLAN,
                DiagnosticCategory::Structure,
                "execution plan is missing outputs",
            )
            .with_object_ref("outputs"),
        );
    }
    if plan.steps.is_empty() {
        report.push(
            compilation_error(
                codes::INVALID_EXECUTION_PLAN,
                DiagnosticCategory::Structure,
                "execution plan is missing steps",
            )
            .with_object_ref("steps"),
        );
    }

    let mut step_ids = std::collections::BTreeSet::new();
    for step in &plan.steps {
        if !step_ids.insert(step.id.clone()) {
            report.push(
                compilation_error(
                    codes::INVALID_EXECUTION_PLAN,
                    DiagnosticCategory::Structure,
                    format!("duplicate execution step id '{}'", step.id),
                )
                .with_object_ref(&step.id),
            );
        }
        match &step.kind {
            ExecutionStepKind::ValidateRules { rule_ids, .. } if rule_ids.is_empty() => {
                report.push(
                    compilation_error(
                        codes::INVALID_EXECUTION_PLAN,
                        DiagnosticCategory::Structure,
                        format!("execution step '{}' has no ruleIds", step.id),
                    )
                    .with_object_ref(&step.id),
                );
            }
            ExecutionStepKind::ApplyAction {
                action_id, target, ..
            } if action_id.is_empty() || target.is_empty() => {
                report.push(
                    compilation_error(
                        codes::INVALID_EXECUTION_PLAN,
                        DiagnosticCategory::Structure,
                        format!(
                            "execution step '{}' has incomplete action metadata",
                            step.id
                        ),
                    )
                    .with_object_ref(&step.id),
                );
            }
            ExecutionStepKind::MaterializeOutput {
                output_id,
                input_ids,
            } if output_id.is_empty() || input_ids.is_empty() => {
                report.push(
                    compilation_error(
                        codes::INVALID_EXECUTION_PLAN,
                        DiagnosticCategory::Structure,
                        format!(
                            "execution step '{}' has incomplete materialize metadata",
                            step.id
                        ),
                    )
                    .with_object_ref(&step.id),
                );
            }
            _ => {}
        }
    }

    report
}
