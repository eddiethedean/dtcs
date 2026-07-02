//! Reference validation phase.

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::TransformationContract;

use super::context::ValidationContext;
use super::field_index::{FieldIndex, TargetResolution};
use super::lineage::validate_lineage;

pub(crate) fn validate_references(ctx: &mut ValidationContext, contract: &TransformationContract) {
    let index = FieldIndex::from_contract(contract);

    validate_lineage(ctx, contract);

    for action in &contract.semantic_actions {
        if action.action.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Reference,
                "semantic action registry identifier is required",
                Some(&format!("semanticActions.{}", action.id)),
                Some("Set action to a namespaced semantic action identifier"),
            );
        }
        if action.target.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Reference,
                "semantic action target is required",
                Some(&format!("semanticActions.{}", action.id)),
                None,
            );
            continue;
        }
        resolve_target(
            ctx,
            &action.target,
            &format!("semanticActions.{}.target", action.id),
            true,
            &index,
        );
    }

    for rule in &contract.rules {
        if rule.rule.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Reference,
                "rule registry identifier is required",
                Some(&format!("rules.{}", rule.id)),
                None,
            );
        }
        if rule.target.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Reference,
                "rule target is required",
                Some(&format!("rules.{}", rule.id)),
                None,
            );
            continue;
        }
        resolve_target(
            ctx,
            &rule.target,
            &format!("rules.{}.target", rule.id),
            false,
            &index,
        );
    }

    for function in &contract.functions {
        if function.function.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Reference,
                "function registry identifier is required",
                Some(&format!("functions.{}", function.id)),
                None,
            );
        }
    }
}

fn resolve_target(
    ctx: &mut ValidationContext,
    target: &str,
    object_ref: &str,
    allow_interface: bool,
    index: &FieldIndex,
) {
    match index.resolve(target) {
        TargetResolution::Field(_) => {}
        TargetResolution::Interface { .. } if allow_interface => {}
        TargetResolution::Interface { .. } => {
            ctx.error(
                codes::UNRESOLVED_REFERENCE,
                DiagnosticCategory::Reference,
                format!("target '{target}' must reference a schema field"),
                Some(object_ref),
                Some("Target a declared schema field or use interface.field syntax"),
            );
        }
        TargetResolution::Ambiguous(_) => {
            ctx.error(
                codes::AMBIGUOUS_REFERENCE,
                DiagnosticCategory::Reference,
                format!("target '{target}' matches multiple schema fields"),
                Some(object_ref),
                Some("Qualify the target with an interface identifier"),
            );
        }
        TargetResolution::NotFound => {
            ctx.error(
                codes::UNRESOLVED_REFERENCE,
                DiagnosticCategory::Reference,
                format!("target '{target}' is not declared"),
                Some(object_ref),
                Some("Target a declared schema field or interface identifier"),
            );
        }
    }
}
