//! Interface validation (SPEC Chapter 6).

use std::collections::{HashMap, HashSet};

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{RulePhase, TransformationContract};

use super::context::{is_vendor_namespaced_identifier, ValidationContext};

const MISPLACED_IO_KEYS: &[(&str, &str)] = &[
    ("description", "metadata.description"),
    ("tags", "metadata.tags"),
    ("classification", "metadata.classification"),
    ("identity", "metadata.identity"),
    ("governance", "metadata.governance"),
    ("provenance", "metadata.provenance"),
    ("documentation", "metadata.documentation"),
    ("precondition", "preconditions"),
    ("postcondition", "postconditions"),
    ("preconditions", "preconditions (inputs only)"),
    ("postconditions", "postconditions (outputs only)"),
];

pub(crate) fn validate_optional_inputs(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
) {
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

pub(crate) fn validate_io_extensions(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
) {
    for input in &contract.inputs {
        validate_io_extension_keys(ctx, &format!("inputs.{}", input.id), &input.extensions);
    }
    for output in &contract.outputs {
        validate_io_extension_keys(ctx, &format!("outputs.{}", output.id), &output.extensions);
    }
}

fn validate_io_extension_keys(
    ctx: &mut ValidationContext,
    object_prefix: &str,
    extensions: &indexmap::IndexMap<String, serde_json::Value>,
) {
    for key in extensions.keys() {
        if key == "extensions" {
            if extensions.get(key).is_some_and(|v| v.is_object()) {
                ctx.error(
                    codes::INVALID_EXTENSION,
                    DiagnosticCategory::Structure,
                    format!(
                        "vendor keys must be flattened on {object_prefix}, not nested under 'extensions'"
                    ),
                    Some(&format!("{object_prefix}.extensions")),
                    Some("Use vendor:fieldName directly on the input or output object"),
                );
            }
            continue;
        }
        if let Some(suggestion) = misplaced_io_key_suggestion(key) {
            ctx.error(
                codes::INVALID_INTERFACE,
                DiagnosticCategory::Structure,
                format!("'{key}' must be declared under {suggestion}"),
                Some(&format!("{object_prefix}.{key}")),
                Some(&format!("Move this field to {suggestion}")),
            );
            continue;
        }
        if !is_vendor_namespaced_identifier(key) {
            ctx.error(
                codes::INVALID_EXTENSION,
                DiagnosticCategory::Structure,
                format!("extension key '{key}' must use a vendor namespace"),
                Some(&format!("{object_prefix}.{key}")),
                Some("Use vendor:fieldName for input and output extensions"),
            );
        }
    }
}

fn misplaced_io_key_suggestion(key: &str) -> Option<&'static str> {
    MISPLACED_IO_KEYS
        .iter()
        .find(|(misplaced, _)| *misplaced == key)
        .map(|(_, suggestion)| *suggestion)
}

pub(crate) fn validate_condition_rule_refs(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
) {
    let rule_ids: HashSet<&str> = contract.rules.iter().map(|rule| rule.id.as_str()).collect();

    for input in &contract.inputs {
        for (index, condition) in input.preconditions.iter().enumerate() {
            let object_ref = format!("inputs.{}.preconditions[{index}].rule", input.id);
            let rule_id = condition.rule.trim();
            if rule_id.is_empty() {
                ctx.error(
                    codes::INVALID_INTERFACE,
                    DiagnosticCategory::Reference,
                    "precondition rule reference is required",
                    Some(&object_ref),
                    Some("Reference a rule instance id from rules[]"),
                );
                continue;
            }
            if !rule_ids.contains(rule_id) {
                ctx.error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("precondition references unknown rule '{rule_id}'"),
                    Some(&object_ref),
                    Some("Declare the rule in rules[] or fix the reference"),
                );
            }
        }
    }

    for output in &contract.outputs {
        for (index, condition) in output.postconditions.iter().enumerate() {
            let object_ref = format!("outputs.{}.postconditions[{index}].rule", output.id);
            let rule_id = condition.rule.trim();
            if rule_id.is_empty() {
                ctx.error(
                    codes::INVALID_INTERFACE,
                    DiagnosticCategory::Reference,
                    "postcondition rule reference is required",
                    Some(&object_ref),
                    Some("Reference a rule instance id from rules[]"),
                );
                continue;
            }
            if !rule_ids.contains(rule_id) {
                ctx.error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("postcondition references unknown rule '{rule_id}'"),
                    Some(&object_ref),
                    Some("Declare the rule in rules[] or fix the reference"),
                );
            }
        }
    }
}

pub(crate) fn validate_condition_rule_phases(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
) {
    let rule_phases: HashMap<&str, RulePhase> = contract
        .rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule.phase))
        .collect();

    for input in &contract.inputs {
        for (index, condition) in input.preconditions.iter().enumerate() {
            let rule_id = condition.rule.trim();
            if rule_id.is_empty() {
                continue;
            }
            let object_ref = format!("inputs.{}.preconditions[{index}].rule", input.id);
            if let Some(phase) = rule_phases.get(rule_id) {
                if *phase != RulePhase::Precondition {
                    ctx.error(
                        codes::INVALID_INTERFACE,
                        DiagnosticCategory::Semantic,
                        format!(
                            "precondition references rule '{rule_id}' with phase '{}', expected precondition",
                            phase.as_str()
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
            let rule_id = condition.rule.trim();
            if rule_id.is_empty() {
                continue;
            }
            let object_ref = format!("outputs.{}.postconditions[{index}].rule", output.id);
            if let Some(phase) = rule_phases.get(rule_id) {
                if *phase != RulePhase::Postcondition {
                    ctx.error(
                        codes::INVALID_INTERFACE,
                        DiagnosticCategory::Semantic,
                        format!(
                            "postcondition references rule '{rule_id}' with phase '{}', expected postcondition",
                            phase.as_str()
                        ),
                        Some(&object_ref),
                        Some("Use a rule with phase: postcondition for output postconditions"),
                    );
                }
            }
        }
    }
}
