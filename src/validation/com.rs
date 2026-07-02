//! Canonical Object Model validation phase.

use crate::diagnostics::{codes, DiagnosticCategory, DiagnosticStage};
use crate::model::TransformationContract;

use super::context::ValidationContext;

pub(crate) fn validate_com(ctx: &mut ValidationContext, contract: &TransformationContract) {
    check_identifier_whitespace(ctx, "id", &contract.id);

    for input in &contract.inputs {
        check_identifier_whitespace(ctx, &format!("inputs.{}.id", input.id), &input.id);
    }
    for output in &contract.outputs {
        check_identifier_whitespace(ctx, &format!("outputs.{}.id", output.id), &output.id);
    }
    for action in &contract.semantic_actions {
        check_identifier_whitespace(
            ctx,
            &format!("semanticActions.{}.id", action.id),
            &action.id,
        );
    }
    for expression in &contract.expressions {
        check_identifier_whitespace(
            ctx,
            &format!("expressions.{}.id", expression.id),
            &expression.id,
        );
    }
    for function in &contract.functions {
        check_identifier_whitespace(ctx, &format!("functions.{}.id", function.id), &function.id);
    }
    for rule in &contract.rules {
        check_identifier_whitespace(ctx, &format!("rules.{}.id", rule.id), &rule.id);
    }
}

fn check_identifier_whitespace(ctx: &mut ValidationContext, object_ref: &str, id: &str) {
    if id.chars().any(char::is_whitespace) {
        ctx.error_with_stage(
            codes::INVALID_IDENTIFIER,
            DiagnosticCategory::Structure,
            format!("identifier '{id}' should not contain whitespace"),
            Some(object_ref),
            Some("Use a stable identifier without spaces"),
            DiagnosticStage::CanonicalObjectModel,
        );
    }
}
