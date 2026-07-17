//! Expression type inference and null propagation.

use std::collections::HashMap;

use serde::Deserialize;

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, UnaryOp};
use crate::diagnostics::{codes, Diagnostic, DiagnosticCategory, DiagnosticStage, Severity};
use crate::model::{
    parse_logical_type, type_compatible, types_assignable, Function, LogicalType, RegistryCategory,
    RegistryDocument, TransformationContract, TypeCompatibility,
};
use crate::registry;
use crate::validation::field_index::{FieldIndex, TargetResolution};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferredExprType {
    pub logical: LogicalType,
    /// Whether the expression may evaluate to null.
    pub nullable: bool,
    /// Whether nullability is introduced by referencing nullable fields.
    ///
    /// This is used by validation to enforce declared non-null expression typing,
    /// without treating function-level `returnNullable` as a typing error.
    pub nullable_from_fields: bool,
}

pub fn infer_expression_type(
    expr: &Expr,
    contract: &TransformationContract,
    registry_doc: &RegistryDocument,
) -> Result<InferredExprType, Diagnostic> {
    let index = FieldIndex::from_contract(contract);
    let functions: HashMap<&str, &Function> = contract
        .functions
        .iter()
        .map(|function| (function.id.as_str(), function))
        .collect();

    infer_expr(expr, &index, &functions, registry_doc).map_err(|(id, category, message)| {
        Diagnostic {
            id: id.to_string(),
            severity: Severity::Error,
            stage: DiagnosticStage::Analysis,
            category,
            message,
            object_ref: None,
            remediation: None,
        }
    })
}

fn infer_expr(
    expr: &Expr,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
    registry_doc: &RegistryDocument,
) -> Result<InferredExprType, (&'static str, DiagnosticCategory, String)> {
    match expr {
        Expr::Literal { value, .. } => Ok(non_null(literal_type(value))),
        Expr::FieldRef { target, .. } => resolve_field_type(target, index),
        Expr::Unary { op, expr, .. } => {
            let inner = infer_expr(expr, index, functions, registry_doc)?;
            let logical = match op {
                UnaryOp::Negate => negate_type(&inner.logical)?,
                UnaryOp::Not => not_type(&inner.logical)?,
            };
            Ok(InferredExprType {
                logical,
                nullable: inner.nullable,
                nullable_from_fields: inner.nullable_from_fields,
            })
        }
        Expr::Binary {
            op, left, right, ..
        } => {
            let left_type = infer_expr(left, index, functions, registry_doc)?;
            let right_type = infer_expr(right, index, functions, registry_doc)?;
            let logical = infer_binary_type(*op, &left_type.logical, &right_type.logical)?;
            let nullable_from_fields =
                left_type.nullable_from_fields || right_type.nullable_from_fields;
            Ok(InferredExprType {
                logical,
                nullable: left_type.nullable || right_type.nullable,
                nullable_from_fields,
            })
        }
        Expr::Call { callee, args, .. } => {
            infer_call_type(callee, args, index, functions, registry_doc)
        }
    }
}

fn literal_type(value: &LiteralValue) -> LogicalType {
    match value {
        LiteralValue::Boolean(_) => LogicalType::Primitive("boolean".into()),
        LiteralValue::String(_) => LogicalType::Primitive("string".into()),
        LiteralValue::Integer(_) => LogicalType::Primitive("integer".into()),
        LiteralValue::Decimal(_) => LogicalType::Primitive("decimal".into()),
    }
}

fn non_null(logical: LogicalType) -> InferredExprType {
    InferredExprType {
        logical,
        nullable: false,
        nullable_from_fields: false,
    }
}

fn resolve_field_type(
    target: &str,
    index: &FieldIndex,
) -> Result<InferredExprType, (&'static str, DiagnosticCategory, String)> {
    match index.resolve(target) {
        TargetResolution::Field(field) => {
            let logical = parse_logical_type(&field.type_name).map_err(|_| {
                (
                    codes::INVALID_TYPE,
                    DiagnosticCategory::Type,
                    format!("field '{target}' has invalid logical type"),
                )
            })?;
            Ok(InferredExprType {
                logical,
                nullable: field.nullable,
                nullable_from_fields: field.nullable,
            })
        }
        TargetResolution::Ambiguous(_) => Err((
            codes::AMBIGUOUS_REFERENCE,
            DiagnosticCategory::Reference,
            format!("field reference '{target}' is ambiguous"),
        )),
        TargetResolution::Interface { id, .. } => Err((
            codes::INVALID_TYPE,
            DiagnosticCategory::Type,
            format!("expression reference '{id}' must target a schema field"),
        )),
        TargetResolution::NotFound => Err((
            codes::UNRESOLVED_REFERENCE,
            DiagnosticCategory::Reference,
            format!("unresolved field reference '{target}'"),
        )),
    }
}

fn negate_type(
    logical: &LogicalType,
) -> Result<LogicalType, (&'static str, DiagnosticCategory, String)> {
    match logical {
        LogicalType::Primitive(name) if is_numeric_primitive(name) => Ok(logical.clone()),
        _ => Err((
            codes::INVALID_TYPE,
            DiagnosticCategory::Type,
            format!(
                "unary '-' requires a numeric operand, found '{}'",
                format_logical_type(logical)
            ),
        )),
    }
}

fn not_type(
    logical: &LogicalType,
) -> Result<LogicalType, (&'static str, DiagnosticCategory, String)> {
    match logical {
        LogicalType::Primitive(name) if name == "boolean" => Ok(logical.clone()),
        _ => Err((
            codes::INVALID_TYPE,
            DiagnosticCategory::Type,
            format!(
                "unary '!' requires a boolean operand, found '{}'",
                format_logical_type(logical)
            ),
        )),
    }
}

fn infer_binary_type(
    op: BinaryOp,
    left: &LogicalType,
    right: &LogicalType,
) -> Result<LogicalType, (&'static str, DiagnosticCategory, String)> {
    match op {
        BinaryOp::Eq
        | BinaryOp::Neq
        | BinaryOp::NullSafeEq
        | BinaryOp::Lt
        | BinaryOp::Lte
        | BinaryOp::Gt
        | BinaryOp::Gte
        | BinaryOp::Between => {
            if type_compatible(left, right) == TypeCompatibility::Incompatible {
                return Err((
                    codes::INVALID_TYPE,
                    DiagnosticCategory::Type,
                    format!(
                        "comparison operator cannot compare '{}' and '{}'",
                        format_logical_type(left),
                        format_logical_type(right)
                    ),
                ));
            }
            Ok(LogicalType::Primitive("boolean".into()))
        }
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
            infer_arithmetic_type(op, left, right)
        }
        BinaryOp::In | BinaryOp::Contains => Ok(LogicalType::Primitive("boolean".into())),
        BinaryOp::And | BinaryOp::Or => {
            let left_name = primitive_name(left)?;
            let right_name = primitive_name(right)?;
            if left_name != "boolean" || right_name != "boolean" {
                return Err((
                    codes::INVALID_TYPE,
                    DiagnosticCategory::Type,
                    format!(
                        "logical operator requires boolean operands, found '{}' and '{}'",
                        format_logical_type(left),
                        format_logical_type(right)
                    ),
                ));
            }
            Ok(LogicalType::Primitive("boolean".into()))
        }
    }
}

fn infer_arithmetic_type(
    op: BinaryOp,
    left: &LogicalType,
    right: &LogicalType,
) -> Result<LogicalType, (&'static str, DiagnosticCategory, String)> {
    let left_name = primitive_name(left)?;
    let right_name = primitive_name(right)?;

    if left_name == "string" || right_name == "string" {
        if matches!(op, BinaryOp::Add) && left_name == "string" && right_name == "string" {
            return Ok(LogicalType::Primitive("string".into()));
        }
        return Err((
            codes::INVALID_TYPE,
            DiagnosticCategory::Type,
            format!(
                "operator is not valid for '{}' and '{}'",
                format_logical_type(left),
                format_logical_type(right)
            ),
        ));
    }

    if !is_numeric_primitive(left_name) || !is_numeric_primitive(right_name) {
        return Err((
            codes::INVALID_TYPE,
            DiagnosticCategory::Type,
            format!(
                "operator requires numeric operands, found '{}' and '{}'",
                format_logical_type(left),
                format_logical_type(right)
            ),
        ));
    }

    if left_name == "decimal" || right_name == "decimal" {
        Ok(LogicalType::Primitive("decimal".into()))
    } else {
        Ok(LogicalType::Primitive("integer".into()))
    }
}

fn infer_call_type(
    name: &str,
    args: &[Expr],
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
    registry_doc: &RegistryDocument,
) -> Result<InferredExprType, (&'static str, DiagnosticCategory, String)> {
    if name.starts_with("dtcs:") {
        return infer_registry_call_type(name, args, index, functions, registry_doc);
    }
    infer_contract_call_type(name, args, index, functions, registry_doc)
}

fn infer_contract_call_type(
    name: &str,
    args: &[Expr],
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
    registry_doc: &RegistryDocument,
) -> Result<InferredExprType, (&'static str, DiagnosticCategory, String)> {
    let Some(function) = functions.get(name) else {
        return Err((
            codes::UNRESOLVED_REFERENCE,
            DiagnosticCategory::Reference,
            format!("unresolved function reference '{name}'"),
        ));
    };
    let Some(return_type) = function.type_name.as_deref() else {
        return Err((
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!("function '{name}' is missing a return type"),
        ));
    };
    let return_type = parse_logical_type(return_type).map_err(|_| {
        (
            codes::INVALID_TYPE,
            DiagnosticCategory::Type,
            format!("function '{name}' has invalid return type"),
        )
    })?;

    if args.len() > function.parameters.len() {
        return Err((
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!(
                "function '{name}' expects at most {} parameter(s), found {}",
                function.parameters.len(),
                args.len()
            ),
        ));
    }

    for (param_index, parameter) in function.parameters.iter().enumerate() {
        if !parameter.optional && param_index >= args.len() {
            return Err((
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Type,
                format!(
                    "function '{name}' missing required argument for parameter '{}'",
                    parameter.name
                ),
            ));
        }
    }

    let mut any_nullable_from_fields = false;

    for (arg_index, arg) in args.iter().enumerate() {
        let arg_type = infer_expr(arg, index, functions, registry_doc)?;
        any_nullable_from_fields |= arg_type.nullable_from_fields;
        let Some(parameter) = function.parameters.get(arg_index) else {
            return Err((
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Type,
                format!("function '{name}' received too many arguments"),
            ));
        };
        let param_type = parse_logical_type(&parameter.type_name).map_err(|_| {
            (
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!(
                    "function '{name}' parameter '{}' has invalid type",
                    parameter.name
                ),
            )
        })?;
        if !types_assignable(&arg_type.logical, &param_type) {
            return Err((
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!(
                    "argument {} to function '{name}' has type '{}', expected '{}'",
                    arg_index + 1,
                    format_logical_type(&arg_type.logical),
                    parameter.type_name
                ),
            ));
        }
        if arg_type.nullable_from_fields && !parameter.optional {
            return Err((
                codes::NULL_SEMANTICS_VIOLATION,
                DiagnosticCategory::Type,
                format!(
                    "argument {} to function '{name}' references nullable fields but parameter '{}' is required",
                    arg_index + 1,
                    parameter.name
                ),
            ));
        }
    }

    let return_nullable = function_return_nullable(function, registry_doc);
    Ok(InferredExprType {
        logical: return_type,
        nullable: any_nullable_from_fields || return_nullable,
        nullable_from_fields: any_nullable_from_fields,
    })
}

#[derive(Debug, Deserialize)]
struct RegistryFunctionDef {
    #[serde(rename = "minArgs")]
    min_args: Option<usize>,
    #[serde(rename = "maxArgs")]
    max_args: Option<usize>,
    #[serde(default, rename = "argTypes")]
    arg_types: Vec<String>,
    #[serde(rename = "returnType")]
    return_type: String,
    #[serde(rename = "returnNullable")]
    return_nullable: Option<bool>,
    #[allow(dead_code)]
    deterministic: Option<bool>,
}

fn infer_registry_call_type(
    name: &str,
    args: &[Expr],
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
    registry_doc: &RegistryDocument,
) -> Result<InferredExprType, (&'static str, DiagnosticCategory, String)> {
    let Some(entry) = registry::resolve(registry_doc, name) else {
        return Err((
            codes::UNKNOWN_REGISTRY_ENTRY,
            DiagnosticCategory::Reference,
            format!("unresolved registry function '{name}'"),
        ));
    };
    if entry.category != RegistryCategory::Function {
        return Err((
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!("'{name}' is not a function identifier"),
        ));
    }
    let Some(definition) = entry.definition.as_deref() else {
        return Err((
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!("registry function '{name}' is missing a definition block"),
        ));
    };
    let definition = definition.trim();
    if !definition.starts_with('{') {
        return Err((
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!("registry function '{name}' has a non-JSON definition"),
        ));
    }

    let def = serde_json::from_str::<RegistryFunctionDef>(definition).map_err(|_| {
        (
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!("registry function '{name}' has an invalid definition schema"),
        )
    })?;

    let min_args = def.min_args.unwrap_or(0);
    if args.len() < min_args {
        return Err((
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!(
                "function '{name}' expects at least {min_args} argument(s), found {}",
                args.len()
            ),
        ));
    }
    if let Some(max) = def.max_args {
        if args.len() > max {
            return Err((
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Type,
                format!(
                    "function '{name}' expects at most {max} argument(s), found {}",
                    args.len()
                ),
            ));
        }
    }

    let mut arg_inferred = Vec::new();
    let mut any_nullable_from_fields = false;
    for arg in args {
        let inferred = infer_expr(arg, index, functions, registry_doc)?;
        any_nullable_from_fields |= inferred.nullable_from_fields;
        arg_inferred.push(inferred);
    }

    if def.return_type == "sameAsArgs" {
        if arg_inferred.is_empty() {
            return Err((
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Type,
                format!("function '{name}' requires at least one argument"),
            ));
        }
        let first = &arg_inferred[0].logical;
        for (idx, other) in arg_inferred.iter().enumerate().skip(1) {
            if type_compatible(first, &other.logical) == TypeCompatibility::Incompatible {
                return Err((
                    codes::INVALID_TYPE,
                    DiagnosticCategory::Type,
                    format!(
                        "function '{name}' requires all arguments to share the same type; argument {} is '{}', expected '{}'",
                        idx + 1,
                        format_logical_type(&other.logical),
                        format_logical_type(first)
                    ),
                ));
            }
        }
        if !type_allowed_for_same_as_args(first, &def.arg_types) {
            return Err((
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!(
                    "function '{name}' does not accept arguments of type '{}'",
                    format_logical_type(first)
                ),
            ));
        }
        return Ok(InferredExprType {
            logical: first.clone(),
            nullable: any_nullable_from_fields || def.return_nullable.unwrap_or(false),
            nullable_from_fields: any_nullable_from_fields,
        });
    }

    // Check per-argument expected types.
    for (i, inferred) in arg_inferred.iter().enumerate() {
        let expected = expected_arg_type(&def.arg_types, i).ok_or_else(|| {
            (
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Type,
                format!("function '{name}' is missing argument type rules in registry"),
            )
        })?;

        let expected = parse_logical_type(expected).map_err(|_| {
            (
                codes::INVALID_FUNCTION,
                DiagnosticCategory::Type,
                format!(
                    "function '{name}' registry declares an invalid argument type '{expected}'"
                ),
            )
        })?;

        if !types_assignable(&inferred.logical, &expected) {
            return Err((
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!(
                    "argument {} to function '{name}' has type '{}', expected '{}'",
                    i + 1,
                    format_logical_type(&inferred.logical),
                    format_logical_type(&expected)
                ),
            ));
        }
    }

    let return_type = parse_logical_type(&def.return_type).map_err(|_| {
        (
            codes::INVALID_FUNCTION,
            DiagnosticCategory::Type,
            format!(
                "function '{name}' registry declares an invalid return type '{}'",
                def.return_type
            ),
        )
    })?;

    Ok(InferredExprType {
        logical: return_type,
        nullable: any_nullable_from_fields || def.return_nullable.unwrap_or(false),
        nullable_from_fields: any_nullable_from_fields,
    })
}

fn expected_arg_type(arg_types: &[String], index: usize) -> Option<&str> {
    if arg_types.is_empty() {
        return None;
    }
    if arg_types.len() == 1 {
        return Some(arg_types[0].as_str());
    }
    if index < arg_types.len() {
        Some(arg_types[index].as_str())
    } else {
        Some(arg_types[arg_types.len() - 1].as_str())
    }
}

fn type_allowed_for_same_as_args(logical: &LogicalType, allowed: &[String]) -> bool {
    let Ok(name) = primitive_name(logical) else {
        return false;
    };
    allowed.iter().any(|candidate| candidate == name)
}

fn function_return_nullable(function: &Function, registry_doc: &RegistryDocument) -> bool {
    if function.nullable {
        return true;
    }
    if !function.function.starts_with("dtcs:") {
        return false;
    }
    let Some(entry) = registry::resolve(registry_doc, &function.function) else {
        return false;
    };
    let Some(definition) = entry.definition.as_deref() else {
        return false;
    };
    let definition = definition.trim();
    if !definition.starts_with('{') {
        return false;
    }
    #[derive(serde::Deserialize)]
    struct FunctionDef {
        #[serde(rename = "returnNullable")]
        return_nullable: Option<bool>,
    }
    serde_json::from_str::<FunctionDef>(definition)
        .ok()
        .and_then(|def| def.return_nullable)
        .unwrap_or(false)
}

fn primitive_name(
    logical_type: &LogicalType,
) -> Result<&str, (&'static str, DiagnosticCategory, String)> {
    match logical_type {
        LogicalType::Primitive(name) => Ok(name.as_str()),
        _ => Err((
            codes::INVALID_TYPE,
            DiagnosticCategory::Type,
            format!(
                "expected primitive type, found '{}'",
                format_logical_type(logical_type)
            ),
        )),
    }
}

fn is_numeric_primitive(name: &str) -> bool {
    matches!(name, "integer" | "decimal")
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
