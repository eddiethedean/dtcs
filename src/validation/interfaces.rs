//! Interface validation phase (SPEC Chapter 6).

use std::collections::HashSet;

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{RulePhase, TransformationContract};

use super::context::{is_namespaced_identifier, ValidationContext};

pub(crate) fn validate_interfaces(ctx: &mut ValidationContext, contract: &TransformationContract) {
    validate_optional_inputs(ctx, contract);
    validate_io_extensions(ctx, contract);
    validate_streaming(ctx, contract);
    validate_conditions(ctx, contract);
}

fn validate_optional_inputs(ctx: &mut ValidationContext, contract: &TransformationContract) {
    if contract.inputs.is_empty() {
        return;
    }
    let all_optional = contract.inputs.iter().all(|input| input.optional);
    if all_optional {
        ctx.error(
            codes::INVALID_INTERFACE,
            DiagnosticCategory::Structure,
            "at least one input must be required",
            Some("inputs"),
            Some("Set optional: false on at least one input or remove optional: true"),
        );
    }
}

fn validate_io_extensions(ctx: &mut ValidationContext, contract: &TransformationContract) {
    for input in &contract.inputs {
        for key in input.extensions.keys() {
            if !is_namespaced_identifier(key) {
                ctx.error(
                    codes::INVALID_INTERFACE,
                    DiagnosticCategory::Structure,
                    format!("input extension key '{key}' must be namespaced"),
                    Some(&format!("inputs.{}.{}", input.id, key)),
                    Some("Use vendor:fieldName for input extensions"),
                );
            }
        }
    }

    for output in &contract.outputs {
        for key in output.extensions.keys() {
            if !is_namespaced_identifier(key) {
                ctx.error(
                    codes::INVALID_INTERFACE,
                    DiagnosticCategory::Structure,
                    format!("output extension key '{key}' must be namespaced"),
                    Some(&format!("outputs.{}.{}", output.id, key)),
                    Some("Use vendor:fieldName for output extensions"),
                );
            }
        }
    }
}

fn validate_streaming(_ctx: &mut ValidationContext, _contract: &TransformationContract) {
    // StreamingMode enum deserialization rejects invalid values at parse time.
}

fn validate_conditions(ctx: &mut ValidationContext, contract: &TransformationContract) {
    let rule_ids: HashSet<&str> = contract.rules.iter().map(|rule| rule.id.as_str()).collect();
    let rule_phases: std::collections::HashMap<&str, RulePhase> = contract
        .rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule.phase))
        .collect();

    for input in &contract.inputs {
        for (index, condition) in input.preconditions.iter().enumerate() {
            let object_ref = format!("inputs.{}.preconditions[{index}].rule", input.id);
            if condition.rule.trim().is_empty() {
                ctx.error(
                    codes::INVALID_INTERFACE,
                    DiagnosticCategory::Reference,
                    "precondition rule reference is required",
                    Some(&object_ref),
                    Some("Reference a rule instance id from rules[]"),
                );
                continue;
            }
            if !rule_ids.contains(condition.rule.as_str()) {
                ctx.error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("precondition references unknown rule '{}'", condition.rule),
                    Some(&object_ref),
                    Some("Declare the rule in rules[] or fix the reference"),
                );
                continue;
            }
            if let Some(phase) = rule_phases.get(condition.rule.as_str()) {
                if *phase != RulePhase::Precondition {
                    ctx.error(
                        codes::INVALID_INTERFACE,
                        DiagnosticCategory::Reference,
                        format!(
                            "precondition references rule '{}' with phase '{phase:?}', expected precondition",
                            condition.rule
                        ),
                        Some(&object_ref),
                        Some("Use a rule with phase: precondition for input preconditions"),
                    );
                }
            }
        }
    }

    for output in &contract.outputs {
        for (index, condition) in output.postconditions.iter().enumerate() {
            let object_ref = format!("outputs.{}.postconditions[{index}].rule", output.id);
            if condition.rule.trim().is_empty() {
                ctx.error(
                    codes::INVALID_INTERFACE,
                    DiagnosticCategory::Reference,
                    "postcondition rule reference is required",
                    Some(&object_ref),
                    Some("Reference a rule instance id from rules[]"),
                );
                continue;
            }
            if !rule_ids.contains(condition.rule.as_str()) {
                ctx.error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("postcondition references unknown rule '{}'", condition.rule),
                    Some(&object_ref),
                    Some("Declare the rule in rules[] or fix the reference"),
                );
                continue;
            }
            if let Some(phase) = rule_phases.get(condition.rule.as_str()) {
                if *phase != RulePhase::Postcondition {
                    ctx.error(
                        codes::INVALID_INTERFACE,
                        DiagnosticCategory::Reference,
                        format!(
                            "postcondition references rule '{}' with phase '{phase:?}', expected postcondition",
                            condition.rule
                        ),
                        Some(&object_ref),
                        Some("Use a rule with phase: postcondition for output postconditions"),
                    );
                }
            }
        }
    }
}
