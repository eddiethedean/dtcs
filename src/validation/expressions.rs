//! Expression and function typing validation (SPEC Chapter 4 §11).

use std::collections::HashSet;

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{
    parse_logical_type, type_compatible, Function, LogicalType, RegistryDocument,
    TransformationContract, TypeCompatibility, TypeParseError,
};

use super::context::ValidationContext;

pub(crate) fn validate_expressions(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
    registry: &RegistryDocument,
) {
    for function in &contract.functions {
        validate_function_declaration(ctx, function);
    }

    for expression in &contract.expressions {
        let object_ref = format!("expressions.{}", expression.id);
        let has_body = expression
            .expr
            .as_ref()
            .is_some_and(|body| !body.trim().is_empty());

        if let Some(declared_type) = expression.type_name.as_deref() {
            if let Err(error) = parse_logical_type(declared_type) {
                emit_type_error(ctx, &format!("{object_ref}.type"), declared_type, error);
                if !has_body {
                    continue;
                }
            }
        }

        if !has_body {
            continue;
        }

        let Some(declared_type) = expression.type_name.as_deref() else {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Type,
                "expression type is required when an expression body is declared",
                Some(&format!("{object_ref}.type")),
                Some("Declare the logical type of the expression"),
            );
            continue;
        };

        let declared = match parse_logical_type(declared_type) {
            Ok(parsed) => parsed,
            Err(error) => {
                emit_type_error(ctx, &format!("{object_ref}.type"), declared_type, error);
                continue;
            }
        };

        let body = expression.expr.as_deref().unwrap_or("");
        let parsed = match crate::analysis::expr::parse::parse_expression(body) {
            Ok(expr) => expr,
            Err(err) => {
                ctx.error(
                    codes::INVALID_EXPRESSION,
                    DiagnosticCategory::Syntax,
                    err.message,
                    Some(object_ref.as_str()),
                    Some("Fix expression syntax (operators, parentheses, or string literals)"),
                );
                continue;
            }
        };

        match crate::analysis::expr::types::infer_expression_type(&parsed, contract, registry) {
            Ok(inferred) => {
                if inferred.nullable_from_fields {
                    ctx.error(
                        codes::INVALID_TYPE,
                        DiagnosticCategory::Type,
                        "expression references nullable fields but declares a non-null type",
                        Some(&format!("{object_ref}.type")),
                        Some("Use nullable-compatible typing or target non-nullable fields"),
                    );
                } else if !types_assignable(&inferred.logical, &declared) {
                    ctx.error(
                        codes::INVALID_TYPE,
                        DiagnosticCategory::Type,
                        format!(
                            "expression type '{declared_type}' does not match inferred type '{}'",
                            format_logical_type(&inferred.logical)
                        ),
                        Some(&format!("{object_ref}.type")),
                        Some("Align the declared type with the expression semantics"),
                    );
                }
            }
            Err(diag) => {
                ctx.error(
                    codes::INVALID_TYPE,
                    DiagnosticCategory::Type,
                    diag.message,
                    Some(object_ref.as_str()),
                    Some("Fix field references, operators, or function calls in the expression"),
                );
            }
        }
    }
}

fn validate_function_declaration(ctx: &mut ValidationContext, function: &Function) {
    let object_ref = format!("functions.{}", function.id);
    let Some(return_type) = function.type_name.as_deref() else {
        ctx.error(
            codes::MISSING_REQUIRED_FIELD,
            DiagnosticCategory::Type,
            "function return type is required",
            Some(&format!("{object_ref}.type")),
            Some("Declare the logical return type of the function"),
        );
        return;
    };

    if let Err(error) = parse_logical_type(return_type) {
        emit_type_error(ctx, &format!("{object_ref}.type"), return_type, error);
    }

    let mut seen = HashSet::new();
    let mut optional_started = false;
    for (index, parameter) in function.parameters.iter().enumerate() {
        let param_ref = format!("{object_ref}.parameters[{index}]");
        if !seen.insert(parameter.name.as_str()) {
            ctx.error(
                codes::DUPLICATE_IDENTIFIER,
                DiagnosticCategory::Type,
                format!("function parameter '{}' is duplicated", parameter.name),
                Some(&format!("{param_ref}.name")),
                Some("Use unique parameter names within a function declaration"),
            );
        }
        if parameter.optional {
            optional_started = true;
        } else if optional_started {
            ctx.error(
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Type,
                format!(
                    "required parameter '{}' must appear before optional parameters",
                    parameter.name
                ),
                Some(&format!("{param_ref}.name")),
                Some("Declare optional parameters as a trailing suffix"),
            );
        }
        if let Err(error) = parse_logical_type(&parameter.type_name) {
            emit_type_error(
                ctx,
                &format!("{param_ref}.type"),
                &parameter.type_name,
                error,
            );
        }
    }
}

fn types_assignable(inferred: &LogicalType, declared: &LogicalType) -> bool {
    match type_compatible(inferred, declared) {
        TypeCompatibility::Identical => true,
        TypeCompatibility::Compatible => matches!(
            (inferred, declared),
            (LogicalType::Primitive(a), LogicalType::Primitive(b)) if a == "integer" && b == "decimal"
        ),
        TypeCompatibility::Incompatible => false,
    }
}

fn format_logical_type(logical_type: &LogicalType) -> String {
    match logical_type {
        LogicalType::Primitive(name) => name.clone(),
        LogicalType::Composite { kind, params } => {
            format!("{kind}<{}>", params.join(","))
        }
        LogicalType::Extension(name) => name.clone(),
    }
}

fn emit_type_error(ctx: &mut ValidationContext, object_ref: &str, type_name: &str, error: TypeParseError) {
    let message = match error {
        TypeParseError::Unknown(unknown) => format!("unknown logical type '{unknown}'"),
        TypeParseError::BareComposite(kind) => {
            format!("composite type '{kind}' is missing parameters")
        }
        TypeParseError::Malformed(detail) => format!("malformed logical type '{type_name}': {detail}"),
        TypeParseError::UnknownParameter(unknown) => format!("unknown nested logical type '{unknown}'"),
        TypeParseError::InvalidArity { kind, expected, actual } => {
            format!("composite type '{kind}' expects {expected}, found {actual}")
        }
    };

    ctx.error(
        codes::INVALID_TYPE,
        DiagnosticCategory::Type,
        message,
        Some(object_ref),
        Some("Use a valid DTCS logical type declaration"),
    );
}

