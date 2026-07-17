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
struct StdlibParameter {
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    #[serde(default)]
    optional: bool,
}

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
        #[serde(default)]
        #[serde(rename = "targetKind")]
        target_kind: Option<String>,
        /// Parameter names from the registry definition (string list form).
        #[serde(default)]
        parameters: Option<Vec<String>>,
        /// Legacy parameter subset that remains valid under widened schemas.
        #[serde(default)]
        #[serde(rename = "legacyParameters")]
        legacy_parameters: Option<Vec<String>>,
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
        #[serde(default)]
        parameters: Option<Vec<StdlibParameter>>,
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
        #[serde(default)]
        #[serde(rename = "returnNullable")]
        return_nullable: Option<bool>,
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

fn parse_stdlib_definition(
    definition: Option<&str>,
) -> Result<Option<StdlibDefinition>, serde_json::Error> {
    let Some(definition) = definition else {
        return Ok(None);
    };
    let definition = definition.trim();
    if !definition.starts_with('{') {
        return Ok(None);
    }
    serde_json::from_str(definition).map(Some)
}

fn validate_stdlib_action(
    ctx: &mut ValidationContext,
    action: &crate::model::SemanticAction,
    definition: Option<&str>,
    index: &FieldIndex,
) {
    let object_ref = format!("semanticActions.{}.target", action.id);
    let parsed = match parse_stdlib_definition(definition) {
        Ok(value) => value,
        Err(err) => {
            ctx.error(
                codes::INVALID_SEMANTIC_ACTION,
                DiagnosticCategory::Semantic,
                format!(
                    "registry definition for '{}' is invalid JSON: {err}",
                    action.action
                ),
                Some(&format!("registry.{}", action.action)),
                Some("Fix the registry entry definition JSON for this identifier"),
            );
            return;
        }
    };
    if action.action.starts_with("dtcs:") && parsed.is_none() {
        ctx.error(
            codes::INVALID_SEMANTIC_ACTION,
            DiagnosticCategory::Semantic,
            format!(
                "registry entry '{}' requires a JSON definition",
                action.action
            ),
            Some(&format!("registry.{}", action.action)),
            Some("Provide a JSON stdlib definition for standard semantic actions"),
        );
        return;
    }
    if action.action == "dtcs:derive" {
        ctx.error(
            codes::INVALID_SEMANTIC_ACTION,
            DiagnosticCategory::Semantic,
            "dtcs:derive is a lineage mapping operation, not a semantic action",
            Some(&format!("semanticActions.{}.action", action.id)),
            Some("Use dtcs:derive only in lineage.mappings.operation"),
        );
        return;
    }

    let Some(StdlibDefinition::SemanticAction {
        target_type,
        target_nullable_allowed,
        target_kind,
        parameters: catalog_params,
        legacy_parameters,
    }) = parsed
    else {
        return;
    };

    // Dataset operators target an interface id (for example `customer_raw`), not a field.
    if target_kind.as_deref() == Some("dataset") {
        match index.resolve(&action.target) {
            TargetResolution::Interface { .. } => {}
            TargetResolution::Field(_) => {
                ctx.error(
                    codes::INVALID_SEMANTIC_ACTION,
                    DiagnosticCategory::Semantic,
                    format!(
                        "{} targets a dataset interface; '{}' looks like a field path",
                        action.action, action.target
                    ),
                    Some(&object_ref),
                    Some("Use an input/output interface identifier as the target"),
                );
            }
            TargetResolution::Ambiguous(_) => {
                ctx.error(
                    codes::AMBIGUOUS_REFERENCE,
                    DiagnosticCategory::Semantic,
                    format!("target '{}' matches multiple declarations", action.target),
                    Some(&object_ref),
                    Some("Use a unique interface identifier"),
                );
            }
            TargetResolution::NotFound => {
                ctx.error(
                    codes::INVALID_SEMANTIC_ACTION,
                    DiagnosticCategory::Semantic,
                    format!(
                        "target '{}' is not a declared input or output interface",
                        action.target
                    ),
                    Some(&object_ref),
                    Some("Target a declared interface identifier"),
                );
            }
        }
        validate_dataset_action_parameters(
            ctx,
            action,
            catalog_params.as_deref(),
            legacy_parameters.as_deref(),
        );
        return;
    }

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
    let parsed = match parse_stdlib_definition(definition) {
        Ok(value) => value,
        Err(err) => {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!(
                    "registry definition for '{}' is invalid JSON: {err}",
                    rule.rule
                ),
                Some(&format!("registry.{}", rule.rule)),
                Some("Fix the registry entry definition JSON for this identifier"),
            );
            return;
        }
    };
    if rule.rule.starts_with("dtcs:") && parsed.is_none() {
        ctx.error(
            codes::INVALID_RULE,
            DiagnosticCategory::Semantic,
            format!("registry entry '{}' requires a JSON definition", rule.rule),
            Some(&format!("registry.{}", rule.rule)),
            Some("Provide a JSON stdlib definition for standard rules"),
        );
        return;
    }
    let Some(StdlibDefinition::Rule {
        phases,
        target_type,
        target_nullable_allowed,
        parameters,
    }) = parsed
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

    validate_rule_parameters(ctx, rule, parameters.as_deref());
}

fn validate_rule_parameters(
    ctx: &mut ValidationContext,
    rule: &crate::model::Rule,
    schema: Option<&[StdlibParameter]>,
) {
    let Some(schema) = schema else {
        return;
    };

    let object_ref = format!("rules.{}.parameters", rule.id);

    for param_schema in schema {
        if !param_schema.optional && !rule.parameters.contains_key(&param_schema.name) {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!("{} requires parameter '{}'", rule.rule, param_schema.name),
                Some(&object_ref),
                Some("Provide all required rule parameters"),
            );
        }
    }

    for (name, value) in &rule.parameters {
        let Some(param_schema) = schema.iter().find(|p| p.name == *name) else {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!("{} does not accept parameter '{name}'", rule.rule),
                Some(&format!("{object_ref}.{name}")),
                Some("Remove unknown parameters or use a supported rule identifier"),
            );
            continue;
        };
        if !parameter_value_matches_type(value, &param_schema.type_name) {
            ctx.error(
                codes::INVALID_RULE,
                DiagnosticCategory::Semantic,
                format!(
                    "parameter '{name}' for {} must be of type '{}'",
                    rule.rule, param_schema.type_name
                ),
                Some(&format!("{object_ref}.{name}")),
                Some("Use a value compatible with the parameter type"),
            );
        }
    }

    if rule.rule == "dtcs:range"
        && !rule.parameters.contains_key("min")
        && !rule.parameters.contains_key("max")
    {
        ctx.error(
            codes::INVALID_RULE,
            DiagnosticCategory::Semantic,
            "dtcs:range requires at least one of 'min' or 'max' parameters".to_string(),
            Some(&object_ref),
            Some("Provide min and/or max bounds"),
        );
    }
}

fn parameter_value_matches_type(value: &serde_json::Value, expected: &str) -> bool {
    match expected {
        "integer" => value_as_integer(value).is_some(),
        "string" => value.as_str().is_some(),
        "decimal" => value.as_f64().is_some() || value_as_integer(value).is_some(),
        "boolean" => value.as_bool().is_some(),
        "list<string>" => value
            .as_array()
            .is_some_and(|items| items.iter().all(|item| item.as_str().is_some())),
        "list<integer>" => value
            .as_array()
            .is_some_and(|items| items.iter().all(|item| value_as_integer(item).is_some())),
        _ => false,
    }
}

fn validate_dataset_action_parameters(
    ctx: &mut ValidationContext,
    action: &crate::model::SemanticAction,
    catalog: Option<&[String]>,
    legacy: Option<&[String]>,
) {
    let Some(catalog) = catalog else {
        return;
    };
    // Present in the catalog list but optional at runtime / rich-form alternatives.
    const OPTIONAL: &[&str] = &[
        "equals",
        "descending",
        "rightKey",
        "predicate",
        "type",
        "on",
        "nullSafe",
        "collisionPolicy",
        "keys",
        "mode",
        "missingColumnPolicy",
        "duplicatePolicy",
        "aggregates",
        "filter",
        "missingPolicy",
        "offset",
        "retain",
        "partitionBy",
        "orderBy",
        "frame",
        "functions",
    ];
    let object_ref = format!("semanticActions.{}.parameters", action.id);

    // Rich alternative forms that satisfy the action without the legacy key set.
    let rich_ok = match action.action.as_str() {
        "dtcs:filter" | "dtcs:select" => action.parameters.contains_key("predicate"),
        "dtcs:sort" => action.parameters.contains_key("keys"),
        "dtcs:aggregate" | "dtcs:group" => action.parameters.contains_key("aggregates"),
        "dtcs:join" => {
            action.parameters.contains_key("on")
                || action.parameters.contains_key("predicate")
                || action.parameters.get("type").and_then(|v| v.as_str()) == Some("cross")
        }
        "dtcs:with_fields" => action.parameters.contains_key("assignments"),
        "dtcs:rename_fields" => action.parameters.contains_key("mapping"),
        "dtcs:drop_fields" => action.parameters.contains_key("fields"),
        "dtcs:distinct" => true, // fields optional
        "dtcs:deduplicate" => {
            action.parameters.contains_key("keys") && action.parameters.contains_key("retain")
        }
        "dtcs:limit" => action.parameters.contains_key("count"),
        "dtcs:window" => action.parameters.contains_key("functions"),
        _ => false,
    };
    if rich_ok {
        return;
    }

    let required = legacy.unwrap_or(catalog);
    for name in required {
        if OPTIONAL.contains(&name.as_str()) {
            continue;
        }
        if !action.parameters.contains_key(name) {
            ctx.error(
                codes::INVALID_SEMANTIC_ACTION,
                DiagnosticCategory::Semantic,
                format!(
                    "{} requires parameter '{}' (got keys {:?})",
                    action.action,
                    name,
                    action.parameters.keys().cloned().collect::<Vec<_>>()
                ),
                Some(&object_ref),
                Some("Supply the documented parameters map for this dataset operator"),
            );
        }
    }
}

fn value_as_integer(value: &serde_json::Value) -> Option<i64> {
    value
        .as_i64()
        .or_else(|| value.as_u64().and_then(|v| v.try_into().ok()))
}

fn validate_stdlib_function(
    ctx: &mut ValidationContext,
    function: &crate::model::Function,
    definition: Option<&str>,
) {
    let parsed = match parse_stdlib_definition(definition) {
        Ok(value) => value,
        Err(err) => {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Semantic,
                format!(
                    "registry definition for '{}' is invalid JSON: {err}",
                    function.function
                ),
                Some(&format!("registry.{}", function.function)),
                Some("Fix the registry entry definition JSON for this identifier"),
            );
            return;
        }
    };
    if function.function.starts_with("dtcs:") && parsed.is_none() {
        ctx.error(
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Semantic,
            format!(
                "registry entry '{}' requires a JSON definition",
                function.function
            ),
            Some(&format!("registry.{}", function.function)),
            Some("Provide a JSON stdlib definition for standard functions"),
        );
        return;
    }
    let Some(StdlibDefinition::Function {
        min_args,
        max_args,
        arg_types,
        return_type,
        return_nullable,
    }) = parsed
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
            if return_type == "sameAsArgs" {
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
                        Some("Use homogeneous parameter types allowed by the standard function"),
                    );
                }
            } else if uses_positional_arg_types(min_args, &arg_types) {
                let Some(expected) = expected_arg_type(&arg_types, idx, max_args) else {
                    ctx.error(
                        codes::INVALID_FUNCTION,
                        DiagnosticCategory::Semantic,
                        format!(
                            "function '{}' has too many parameters for its signature",
                            function.function
                        ),
                        Some(&format!("{object_ref}.parameters[{idx}]")),
                        Some("Remove extra parameters"),
                    );
                    continue;
                };
                if name != expected {
                    ctx.error(
                        codes::INVALID_FUNCTION,
                        DiagnosticCategory::Semantic,
                        format!(
                            "function '{}' parameter {} '{}' has type '{}', expected '{expected}'",
                            function.function,
                            idx + 1,
                            param.name,
                            name,
                        ),
                        Some(&format!("{object_ref}.parameters[{idx}].type")),
                        Some("Align parameter types with the standard function signature"),
                    );
                }
            } else if !arg_types.iter().any(|allowed| allowed == name) {
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

    if return_type == "sameAsArgs" {
        validate_same_as_args_return(ctx, function, &object_ref, &arg_types);
    } else if let Some(declared_return) = function.type_name.as_deref() {
        let Ok(declared) = parse_logical_type(declared_return) else {
            return;
        };
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

    if return_nullable == Some(false) && function.nullable {
        ctx.error(
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Semantic,
            format!(
                "function '{}' declares a nullable return type but the standard signature is non-nullable",
                function.function
            ),
            Some(&format!("{object_ref}.nullable")),
            Some("Set nullable: false on the function return type"),
        );
    }
}

fn uses_positional_arg_types(min_args: Option<usize>, arg_types: &[String]) -> bool {
    matches!(min_args, Some(min) if min >= 2 && arg_types.len() >= 2)
}

fn expected_arg_type(arg_types: &[String], idx: usize, max_args: Option<usize>) -> Option<&str> {
    if arg_types.is_empty() {
        return None;
    }
    if idx < arg_types.len() {
        return Some(arg_types[idx].as_str());
    }
    if max_args.is_none() {
        return Some(arg_types.last()?.as_str());
    }
    if let Some(max) = max_args {
        if idx < max {
            return Some(arg_types.last()?.as_str());
        }
    }
    None
}

fn validate_same_as_args_return(
    ctx: &mut ValidationContext,
    function: &crate::model::Function,
    object_ref: &str,
    allowed_types: &[String],
) {
    if function.parameters.is_empty() {
        ctx.error(
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Semantic,
            format!(
                "function '{}' requires at least one parameter",
                function.function
            ),
            Some(&format!("{object_ref}.parameters")),
            Some("Declare at least one parameter"),
        );
        return;
    }

    let mut param_types = Vec::new();
    for param in &function.parameters {
        let Ok(param_type) = parse_logical_type(&param.type_name) else {
            continue;
        };
        let Some(name) = matches_primitive(&param_type) else {
            continue;
        };
        if !allowed_types.iter().any(|allowed| allowed == name) {
            return;
        }
        param_types.push(name.clone());
    }

    if param_types.is_empty() {
        return;
    }

    let first = &param_types[0];
    if !param_types.iter().all(|t| t == first) {
        ctx.error(
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Semantic,
            format!(
                "function '{}' parameters must share the same primitive type",
                function.function
            ),
            Some(&format!("{object_ref}.parameters")),
            Some("Use homogeneous parameter types for coalesce"),
        );
        return;
    }

    if let Some(declared_return) = function.type_name.as_deref() {
        let Ok(declared) = parse_logical_type(declared_return) else {
            return;
        };
        let Ok(expected) = parse_logical_type(first) else {
            return;
        };
        if declared != expected {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Semantic,
                format!(
                    "function '{}' declares return type '{declared_return}', expected '{first}'",
                    function.function
                ),
                Some(&format!("{object_ref}.type")),
                Some("Align the declared return type with parameter types"),
            );
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
        TargetResolution::NotFound => {
            ctx.error(
                codes::UNRESOLVED_REFERENCE,
                category,
                format!("target '{target}' is not declared"),
                Some(object_ref),
                Some("Target a declared schema field"),
            );
            None
        }
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
            Some(StdlibDefinition::SemanticAction {
                target_type,
                target_nullable_allowed,
                target_kind,
                parameters,
                legacy_parameters,
            }) => {
                assert_eq!(target_type.as_deref(), Some("string"));
                assert_eq!(target_nullable_allowed, Some(false));
                assert_eq!(target_kind, None);
                assert_eq!(parameters, None);
                assert_eq!(legacy_parameters, None);
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn expected_arg_type_uses_positional_and_variadic_tail() {
        let types = vec!["string".into(), "integer".into()];
        assert_eq!(expected_arg_type(&types, 0, Some(3)), Some("string"));
        assert_eq!(expected_arg_type(&types, 1, Some(3)), Some("integer"));
        assert_eq!(expected_arg_type(&types, 2, Some(3)), Some("integer"));
        assert_eq!(
            expected_arg_type(&["string".into()], 2, None),
            Some("string")
        );
        assert_eq!(
            expected_arg_type(&["string".into()], 2, Some(3)),
            Some("string")
        );
    }
}
