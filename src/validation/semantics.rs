//! Semantic validation phase.

use serde::Deserialize;

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{
    is_vendor_namespaced_identifier, parse_logical_type, LogicalType, RegistryCategory,
    RegistryDocument, TransformationContract,
};
use crate::registry;

use super::context::ValidationContext;
use super::field_index::{FieldIndex, TargetResolution};

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
enum StdlibDefinition {
    #[serde(rename = "semanticAction")]
    SemanticAction {
        #[serde(default)]
        #[serde(rename = "targetType")]
        target_type: Option<String>,
        #[serde(default)]
        #[serde(rename = "targetNullableAllowed")]
        target_nullable_allowed: Option<bool>,
    },
    #[serde(rename = "rule")]
    Rule {
        #[serde(default)]
        phases: Option<Vec<String>>,
        #[serde(default)]
        #[serde(rename = "targetType")]
        target_type: Option<String>,
        #[serde(default)]
        #[serde(rename = "targetNullableAllowed")]
        target_nullable_allowed: Option<bool>,
    },
    #[serde(rename = "function")]
    Function {
        #[serde(default)]
        #[serde(rename = "minArgs")]
        min_args: Option<usize>,
        #[serde(default)]
        #[serde(rename = "maxArgs")]
        max_args: Option<usize>,
        #[serde(default)]
        #[serde(rename = "argTypes")]
        arg_types: Vec<String>,
        #[serde(rename = "returnType")]
        return_type: String,
    },
}

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
        let Some(entry) = registry::resolve(registry_doc, &action.action) else {
            ctx.error(
                codes::INVALID_SEMANTIC_ACTION,
                DiagnosticCategory::Semantic,
                format!("unsupported standard semantic action '{}'", action.action),
                Some(&format!("semanticActions.{}.action", action.id)),
                Some("Use a standardized semantic action identifier"),
            );
            continue;
        };
        if entry.category != RegistryCategory::SemanticAction {
            ctx.error(
                codes::INVALID_SEMANTIC_ACTION,
                DiagnosticCategory::Semantic,
                format!("unsupported standard semantic action '{}'", action.action),
                Some(&format!("semanticActions.{}.action", action.id)),
                Some("Use a standardized semantic action identifier"),
            );
            continue;
        }
        validate_stdlib_action(ctx, action, entry.definition.as_deref(), &index);
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
        let Some(entry) = registry::resolve(registry_doc, &rule.rule) else {
            if rule.rule.starts_with("dtcs:") {
                ctx.error(
                    codes::INVALID_RULE,
                    DiagnosticCategory::Semantic,
                    format!("unsupported standard rule '{}'", rule.rule),
                    Some(&format!("rules.{}.rule", rule.id)),
                    Some("Use a standardized rule identifier"),
                );
            }
            continue;
        };
        if entry.category != RegistryCategory::Rule {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!("unsupported standard rule '{}'", rule.rule),
                Some(&format!("rules.{}.rule", rule.id)),
                Some("Use a standardized rule identifier"),
            );
            continue;
        }
        validate_stdlib_rule(ctx, rule, entry.definition.as_deref(), &index);
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
        let Some(entry) = registry::resolve(registry_doc, &function.function) else {
            if function.function.starts_with("dtcs:") {
                ctx.error(
                    codes::INVALID_FUNCTION,
                    DiagnosticCategory::Semantic,
                    format!("unsupported standard function '{}'", function.function),
                    Some(&format!("functions.{}.function", function.id)),
                    Some("Use a standardized function identifier"),
                );
            }
            continue;
        };
        if entry.category != RegistryCategory::Function {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Semantic,
                format!("unsupported standard function '{}'", function.function),
                Some(&format!("functions.{}.function", function.id)),
                Some("Use a standardized function identifier"),
            );
            continue;
        }
        validate_stdlib_function(ctx, function, entry.definition.as_deref());
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
}

fn parse_stdlib_definition(definition: Option<&str>) -> Option<StdlibDefinition> {
    let definition = definition?.trim();
    if !definition.starts_with('{') {
        return None;
    }
    serde_json::from_str(definition).ok()
}

fn validate_stdlib_action(
    ctx: &mut ValidationContext,
    action: &crate::model::SemanticAction,
    definition: Option<&str>,
    index: &FieldIndex,
) {
    let object_ref = format!("semanticActions.{}.target", action.id);
    let Some(field) = resolve_field(
        index,
        &action.target,
        &object_ref,
        ctx,
        codes::INVALID_SEMANTIC_ACTION,
        DiagnosticCategory::Semantic,
    ) else {
        return;
    };
    let Some(StdlibDefinition::SemanticAction {
        target_type,
        target_nullable_allowed,
    }) = parse_stdlib_definition(definition)
    else {
        return;
    };

    if let Some(expected) = target_type {
        match parse_logical_type(&field.type_name) {
            Ok(LogicalType::Primitive(name)) if name == expected => {}
            Ok(_) | Err(_) => {
                ctx.error(
                    codes::INVALID_SEMANTIC_ACTION,
                    DiagnosticCategory::Semantic,
                    format!(
                        "{} requires a '{}' target field; '{}' is '{}'",
                        action.action, expected, field.field_name, field.type_name
                    ),
                    Some(&object_ref),
                    Some("Target a compatible schema field"),
                );
            }
        }
    }
    if target_nullable_allowed == Some(false) && field.nullable {
        ctx.error(
            codes::INVALID_SEMANTIC_ACTION,
            DiagnosticCategory::Semantic,
            format!(
                "{} cannot target nullable field '{}'",
                action.action, field.field_name
            ),
            Some(&object_ref),
            Some("Target a non-nullable schema field"),
        );
    }
}

fn validate_stdlib_rule(
    ctx: &mut ValidationContext,
    rule: &crate::model::Rule,
    definition: Option<&str>,
    index: &FieldIndex,
) {
    let object_ref = format!("rules.{}.target", rule.id);
    let Some(field) = resolve_field(
        index,
        &rule.target,
        &object_ref,
        ctx,
        codes::INVALID_RULE,
        DiagnosticCategory::Semantic,
    ) else {
        return;
    };
    let Some(StdlibDefinition::Rule {
        phases,
        target_type,
        target_nullable_allowed,
    }) = parse_stdlib_definition(definition)
    else {
        return;
    };

    if let Some(phases) = phases {
        let phase = rule.phase.as_str();
        if !phases.iter().any(|p| p == phase) {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!("{} is not valid in phase '{phase}'", rule.rule),
                Some(&format!("rules.{}.phase", rule.id)),
                Some("Use a supported rule evaluation phase"),
            );
        }
    }

    if let Some(expected) = target_type {
        match parse_logical_type(&field.type_name) {
            Ok(LogicalType::Primitive(name)) if name == expected => {}
            Ok(_) | Err(_) => {
                ctx.error(
                    codes::INVALID_RULE,
                    DiagnosticCategory::Semantic,
                    format!(
                        "{} requires a '{}' target field; '{}' is '{}'",
                        rule.rule, expected, field.field_name, field.type_name
                    ),
                    Some(&object_ref),
                    Some("Target a compatible schema field"),
                );
            }
        }
    }

    if target_nullable_allowed == Some(false) && field.nullable {
        ctx.error(
            codes::INVALID_RULE,
            DiagnosticCategory::Semantic,
            format!(
                "{} cannot target nullable field '{}'",
                rule.rule, field.field_name
            ),
            Some(&object_ref),
            Some("Target a non-nullable schema field"),
        );
    }
}

fn validate_stdlib_function(
    ctx: &mut ValidationContext,
    function: &crate::model::Function,
    definition: Option<&str>,
) {
    let Some(StdlibDefinition::Function {
        min_args,
        max_args,
        arg_types,
        return_type,
    }) = parse_stdlib_definition(definition)
    else {
        return;
    };

    let object_ref = format!("functions.{}", function.id);
    let actual = function.parameters.len();
    if let Some(min) = min_args {
        if actual < min {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Semantic,
                format!(
                    "function '{}' expects at least {min} parameter(s), found {actual}",
                    function.function
                ),
                Some(&format!("{object_ref}.parameters")),
                Some("Declare the required number of parameters"),
            );
        }
    }
    if let Some(max) = max_args {
        if actual > max {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Semantic,
                format!(
                    "function '{}' expects at most {max} parameter(s), found {actual}",
                    function.function
                ),
                Some(&format!("{object_ref}.parameters")),
                Some("Remove extra parameters"),
            );
        }
    }

    if !arg_types.is_empty() {
        for (idx, param) in function.parameters.iter().enumerate() {
            let Ok(param_type) = parse_logical_type(&param.type_name) else {
                continue;
            };
            let Some(name) = matches_primitive(&param_type) else {
                ctx.error(
                    codes::INVALID_FUNCTION,
                    DiagnosticCategory::Semantic,
                    format!(
                        "function '{}' parameter '{}' must be a primitive type",
                        function.function, param.name
                    ),
                    Some(&format!("{object_ref}.parameters[{idx}].type")),
                    Some("Use a supported primitive type"),
                );
                continue;
            };
            if !arg_types.iter().any(|allowed| allowed == name) {
                ctx.error(
                    codes::INVALID_FUNCTION,
                    DiagnosticCategory::Semantic,
                    format!(
                        "function '{}' parameter '{}' has type '{}', expected one of {}",
                        function.function,
                        param.name,
                        name,
                        arg_types.join(", ")
                    ),
                    Some(&format!("{object_ref}.parameters[{idx}].type")),
                    Some("Align parameter types with the standard function signature"),
                );
            }
        }
    }

    if let Some(declared_return) = function.type_name.as_deref() {
        let Ok(declared) = parse_logical_type(declared_return) else {
            return;
        };
        if return_type != "sameAsArgs" {
            let Ok(expected) = parse_logical_type(&return_type) else {
                return;
            };
            if declared != expected {
                ctx.error(
                    codes::INVALID_FUNCTION,
                    DiagnosticCategory::Semantic,
                    format!(
                        "function '{}' declares return type '{declared_return}', expected '{return_type}'",
                        function.function
                    ),
                    Some(&format!("{object_ref}.type")),
                    Some("Align the declared return type with the standard function signature"),
                );
            }
        }
    }
}

fn matches_primitive(logical: &LogicalType) -> Option<&String> {
    match logical {
        LogicalType::Primitive(name) => Some(name),
        _ => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_stdlib_semantic_action_definition() {
        let def =
            r#"{"kind":"semanticAction","targetType":"string","targetNullableAllowed":false}"#;
        let parsed = parse_stdlib_definition(Some(def)).expect("parsed");
        match parsed {
            StdlibDefinition::SemanticAction {
                target_type,
                target_nullable_allowed,
            } => {
                assert_eq!(target_type.as_deref(), Some("string"));
                assert_eq!(target_nullable_allowed, Some(false));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }
}
