//! Semantic validation phase.

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{
    is_vendor_namespaced_identifier, parse_logical_type, LogicalType, RegistryCategory,
    RegistryDocument, TransformationContract,
};
use crate::registry;

use super::context::ValidationContext;
use super::field_index::{FieldIndex, TargetResolution};

pub(crate) fn validate_semantics(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
    registry_doc: &RegistryDocument,
) {
    let index = FieldIndex::from_contract(contract);

    for action in &contract.semantic_actions {
        if !action.action.starts_with("dtcs:") && !is_vendor_namespaced_identifier(&action.action) {
            ctx.error(
                codes::INVALID_SEMANTIC_ACTION,
                DiagnosticCategory::Semantic,
                format!("semantic action '{}' must be namespaced", action.action),
                Some(&format!("semanticActions.{}.action", action.id)),
                Some("Use a dtcs: identifier or vendor namespace"),
            );
            continue;
        }
        if action.action.starts_with("dtcs:")
            && !registry::resolve(registry_doc, &action.action)
                .is_some_and(|entry| entry.category == RegistryCategory::SemanticAction)
        {
            ctx.error(
                codes::INVALID_SEMANTIC_ACTION,
                DiagnosticCategory::Semantic,
                format!("unsupported standard semantic action '{}'", action.action),
                Some(&format!("semanticActions.{}.action", action.id)),
                Some("Use a standardized semantic action identifier"),
            );
            continue;
        }

        if action.action == "dtcs:lowercase" {
            validate_lowercase_target(ctx, &action.target, &action.id, &index);
        }
    }

    for rule in &contract.rules {
        if !rule.rule.starts_with("dtcs:") && !is_vendor_namespaced_identifier(&rule.rule) {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!("rule '{}' must be namespaced", rule.rule),
                Some(&format!("rules.{}.rule", rule.id)),
                Some("Use a dtcs: identifier or vendor namespace"),
            );
            continue;
        }
        if rule.rule.starts_with("dtcs:")
            && !registry::resolve(registry_doc, &rule.rule)
                .is_some_and(|entry| entry.category == RegistryCategory::Rule)
        {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!("unsupported standard rule '{}'", rule.rule),
                Some(&format!("rules.{}.rule", rule.id)),
                Some("Use a standardized rule identifier"),
            );
            continue;
        }

        if rule.rule == "dtcs:not_null" {
            validate_not_null_target(ctx, &rule.target, &rule.id, &index);
        }
    }

    for expression in &contract.expressions {
        let missing_body = expression
            .expr
            .as_ref()
            .map_or(true, |e| e.trim().is_empty());
        if missing_body {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Semantic,
                "expression body is required when an expression is declared",
                Some(&format!("expressions.{}", expression.id)),
                Some("Provide an expression body or remove the declaration"),
            );
        }
    }

    for function in &contract.functions {
        if !function.function.starts_with("dtcs:")
            && !is_vendor_namespaced_identifier(&function.function)
        {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Semantic,
                format!("function '{}' must be namespaced", function.function),
                Some(&format!("functions.{}.function", function.id)),
                Some("Use a dtcs: identifier or vendor namespace"),
            );
            continue;
        }
        if function.function.starts_with("dtcs:")
            && !registry::resolve(registry_doc, &function.function)
                .is_some_and(|entry| entry.category == RegistryCategory::Function)
        {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Semantic,
                format!("unsupported standard function '{}'", function.function),
                Some(&format!("functions.{}.function", function.id)),
                Some("Use a standardized function identifier"),
            );
        }
    }
}

fn validate_lowercase_target(
    ctx: &mut ValidationContext,
    target: &str,
    action_id: &str,
    index: &FieldIndex,
) {
    let object_ref = format!("semanticActions.{action_id}.target");
    let Some(field) = resolve_field(
        index,
        target,
        &object_ref,
        ctx,
        codes::INVALID_SEMANTIC_ACTION,
        DiagnosticCategory::Semantic,
    ) else {
        return;
    };
    if !matches!(
        parse_logical_type(&field.type_name),
        Ok(LogicalType::Primitive(name)) if name == "string"
    ) {
        ctx.error(
            codes::INVALID_SEMANTIC_ACTION,
            DiagnosticCategory::Semantic,
            format!(
                "dtcs:lowercase requires a string field; '{}' is '{}'",
                field.field_name, field.type_name
            ),
            Some(&object_ref),
            Some("Target a non-nullable string schema field"),
        );
        return;
    }
    if field.nullable {
        ctx.error(
            codes::INVALID_SEMANTIC_ACTION,
            DiagnosticCategory::Semantic,
            format!(
                "dtcs:lowercase cannot target nullable field '{}'",
                field.field_name
            ),
            Some(&object_ref),
            Some("Target a non-nullable string schema field"),
        );
    }
}

fn validate_not_null_target(
    ctx: &mut ValidationContext,
    target: &str,
    rule_id: &str,
    index: &FieldIndex,
) {
    let object_ref = format!("rules.{rule_id}.target");
    let Some(field) = resolve_field(
        index,
        target,
        &object_ref,
        ctx,
        codes::INVALID_RULE,
        DiagnosticCategory::Semantic,
    ) else {
        return;
    };
    if field.nullable {
        ctx.error(
            codes::INVALID_RULE,
            DiagnosticCategory::Semantic,
            format!(
                "dtcs:not_null cannot target nullable field '{}'",
                field.field_name
            ),
            Some(&object_ref),
            Some("Target a non-nullable schema field"),
        );
    }
}

fn resolve_field<'a>(
    index: &'a FieldIndex,
    target: &str,
    object_ref: &str,
    ctx: &mut ValidationContext,
    interface_error_code: &str,
    category: DiagnosticCategory,
) -> Option<&'a super::field_index::FieldLocation> {
    match index.resolve(target) {
        TargetResolution::Field(field) => Some(field),
        TargetResolution::Ambiguous(_) => {
            ctx.error(
                codes::AMBIGUOUS_REFERENCE,
                category,
                format!("target '{target}' matches multiple schema fields"),
                Some(object_ref),
                Some("Qualify the target with an interface identifier"),
            );
            None
        }
        TargetResolution::Interface { .. } => {
            ctx.error(
                interface_error_code,
                category,
                format!("target '{target}' must reference a schema field"),
                Some(object_ref),
                Some("Target a declared schema field"),
            );
            None
        }
        TargetResolution::NotFound => None,
    }
}
