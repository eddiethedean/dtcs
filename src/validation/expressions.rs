//! Expression and function typing validation (SPEC Chapter 4 §11).

use std::collections::{HashMap, HashSet};

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{
    parse_logical_type, type_compatible, Function, LogicalType, TransformationContract,
    TypeCompatibility, TypeParseError,
};

use super::context::ValidationContext;
use super::field_index::{FieldIndex, TargetResolution};

/// Inferred expression type including nullability from field references.
#[derive(Debug, Clone, PartialEq, Eq)]
struct InferredExprType {
    logical: LogicalType,
    nullable: bool,
}

pub(crate) fn validate_expressions(ctx: &mut ValidationContext, contract: &TransformationContract) {
    let index = FieldIndex::from_contract(contract);
    let functions: HashMap<&str, &Function> = contract
        .functions
        .iter()
        .map(|function| (function.id.as_str(), function))
        .collect();

    for function in &contract.functions {
        validate_function_declaration(ctx, function);
    }

    for expression in &contract.expressions {
        let has_body = expression
            .expr
            .as_ref()
            .is_some_and(|body| !body.trim().is_empty());
        if !has_body {
            continue;
        }

        let object_ref = format!("expressions.{}", expression.id);
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

        if let Err(error) = parse_logical_type(declared_type) {
            emit_type_error(ctx, &format!("{object_ref}.type"), declared_type, error);
            continue;
        }

        match infer_expression_type(expression.expr.as_deref().unwrap_or(""), &index, &functions) {
            Ok(inferred) => {
                let declared = parse_logical_type(declared_type).expect("validated above");
                if inferred.nullable {
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
            Err(message) => {
                ctx.error(
                    codes::INVALID_TYPE,
                    DiagnosticCategory::Type,
                    message,
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
        if parameter.name.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Type,
                "function parameter name is required",
                Some(&format!("{param_ref}.name")),
                None,
            );
            continue;
        }
        if !seen.insert(parameter.name.clone()) {
            ctx.error(
                codes::DUPLICATE_IDENTIFIER,
                DiagnosticCategory::Type,
                format!("duplicate function parameter '{}'", parameter.name),
                Some(&format!("{param_ref}.name")),
                Some("Use unique parameter names within each function"),
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
            (
                LogicalType::Primitive(a),
                LogicalType::Primitive(b)
            ) if a == "integer" && b == "decimal"
        ),
        TypeCompatibility::Incompatible => false,
    }
}

fn infer_expression_type(
    expr: &str,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
) -> Result<InferredExprType, String> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Err("expression body is empty".into());
    }
    infer_comparison(expr, index, functions)
}

fn infer_comparison(
    expr: &str,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
) -> Result<InferredExprType, String> {
    if let Some((left, op, right)) =
        split_binary_leftmost(expr, &["==", "!=", "<=", ">=", "<", ">"])
    {
        let left_type = infer_additive(left, index, functions)?;
        let right_type = infer_additive(right, index, functions)?;
        let logical = infer_binary_type(op, &left_type.logical, &right_type.logical)?;
        return Ok(InferredExprType {
            logical,
            nullable: false,
        });
    }
    infer_additive(expr, index, functions)
}

fn infer_additive(
    expr: &str,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
) -> Result<InferredExprType, String> {
    if let Some((left, op, right)) = split_binary_leftmost(expr, &["+", "-"]) {
        let left_type = infer_additive(left, index, functions)?;
        let right_type = infer_multiplicative(right, index, functions)?;
        let logical = infer_binary_type(op, &left_type.logical, &right_type.logical)?;
        return Ok(InferredExprType {
            logical,
            nullable: left_type.nullable || right_type.nullable,
        });
    }
    infer_multiplicative(expr, index, functions)
}

fn infer_multiplicative(
    expr: &str,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
) -> Result<InferredExprType, String> {
    if let Some((left, op, right)) = split_binary_leftmost(expr, &["*", "/"]) {
        let left_type = infer_multiplicative(left, index, functions)?;
        let right_type = infer_unary(right, index, functions)?;
        let logical = infer_binary_type(op, &left_type.logical, &right_type.logical)?;
        return Ok(InferredExprType {
            logical,
            nullable: left_type.nullable || right_type.nullable,
        });
    }
    infer_unary(expr, index, functions)
}

fn infer_unary(
    expr: &str,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
) -> Result<InferredExprType, String> {
    let expr = expr.trim();
    if let Some(rest) = expr.strip_prefix('-') {
        let inner = infer_unary(rest, index, functions)?;
        let logical = negate_type(&inner.logical)?;
        return Ok(InferredExprType {
            logical,
            nullable: inner.nullable,
        });
    }
    if let Some(rest) = expr.strip_prefix('+') {
        return infer_unary(rest, index, functions);
    }
    infer_primary(expr, index, functions)
}

fn infer_primary(
    expr: &str,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
) -> Result<InferredExprType, String> {
    let expr = expr.trim();
    if let Some(inner) = strip_outer_parens(expr) {
        return infer_comparison(inner, index, functions);
    }
    if let Some((name, args_source)) = split_call(expr) {
        let logical = infer_call_type(name, args_source, index, functions)?;
        return Ok(InferredExprType {
            logical,
            nullable: false,
        });
    }
    infer_atom(expr, index)
}

fn negate_type(logical: &LogicalType) -> Result<LogicalType, String> {
    match logical {
        LogicalType::Primitive(name) if is_numeric_primitive(name) => Ok(logical.clone()),
        _ => Err(format!(
            "unary '-' requires a numeric operand, found '{}'",
            format_logical_type(logical)
        )),
    }
}

fn infer_atom(expr: &str, index: &FieldIndex) -> Result<InferredExprType, String> {
    let expr = expr.trim();
    if expr.eq_ignore_ascii_case("true") || expr.eq_ignore_ascii_case("false") {
        return Ok(non_null(LogicalType::Primitive("boolean".into())));
    }
    if (expr.starts_with('"') && expr.ends_with('"'))
        || (expr.starts_with('\'') && expr.ends_with('\''))
    {
        return Ok(non_null(LogicalType::Primitive("string".into())));
    }
    if expr.parse::<i64>().is_ok() {
        return Ok(non_null(LogicalType::Primitive("integer".into())));
    }
    if expr.parse::<f64>().is_ok() && expr.contains('.') {
        return Ok(non_null(LogicalType::Primitive("decimal".into())));
    }

    resolve_field_type(expr, index)
}

fn non_null(logical: LogicalType) -> InferredExprType {
    InferredExprType {
        logical,
        nullable: false,
    }
}

fn resolve_field_type(target: &str, index: &FieldIndex) -> Result<InferredExprType, String> {
    match index.resolve(target) {
        TargetResolution::Field(field) => {
            let logical = parse_logical_type(&field.type_name)
                .map_err(|_| format!("field '{target}' has invalid logical type"))?;
            Ok(InferredExprType {
                logical,
                nullable: field.nullable,
            })
        }
        TargetResolution::Ambiguous(_) => Err(format!("field reference '{target}' is ambiguous")),
        TargetResolution::Interface { id, .. } => Err(format!(
            "expression reference '{id}' must target a schema field"
        )),
        TargetResolution::NotFound => Err(format!("unresolved field reference '{target}'")),
    }
}

fn infer_binary_type(
    op: &str,
    left: &LogicalType,
    right: &LogicalType,
) -> Result<LogicalType, String> {
    match op {
        "==" | "!=" | "<" | ">" | "<=" | ">=" => {
            if type_compatible(left, right) == TypeCompatibility::Incompatible {
                return Err(format!(
                    "comparison operator '{op}' cannot compare '{}' and '{}'",
                    format_logical_type(left),
                    format_logical_type(right)
                ));
            }
            Ok(LogicalType::Primitive("boolean".into()))
        }
        "+" | "-" | "*" | "/" => infer_arithmetic_type(op, left, right),
        _ => Err(format!("unsupported operator '{op}'")),
    }
}

fn infer_arithmetic_type(
    op: &str,
    left: &LogicalType,
    right: &LogicalType,
) -> Result<LogicalType, String> {
    let left_name = primitive_name(left)?;
    let right_name = primitive_name(right)?;

    if left_name == "string" || right_name == "string" {
        if op == "+" && left_name == "string" && right_name == "string" {
            return Ok(LogicalType::Primitive("string".into()));
        }
        return Err(format!(
            "operator '{op}' is not valid for '{}' and '{}'",
            format_logical_type(left),
            format_logical_type(right)
        ));
    }

    if !is_numeric_primitive(left_name) || !is_numeric_primitive(right_name) {
        return Err(format!(
            "operator '{op}' requires numeric operands, found '{}' and '{}'",
            format_logical_type(left),
            format_logical_type(right)
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
    args_source: &str,
    index: &FieldIndex,
    functions: &HashMap<&str, &Function>,
) -> Result<LogicalType, String> {
    let Some(function) = functions.get(name) else {
        return Err(format!("unresolved function reference '{name}'"));
    };
    let Some(return_type) = function.type_name.as_deref() else {
        return Err(format!("function '{name}' is missing a return type"));
    };
    let return_type = parse_logical_type(return_type)
        .map_err(|_| format!("function '{name}' has invalid return type"))?;

    let args = split_args(args_source);
    if args.len() > function.parameters.len() {
        return Err(format!(
            "function '{name}' expects at most {} parameter(s), found {}",
            function.parameters.len(),
            args.len()
        ));
    }

    for (param_index, parameter) in function.parameters.iter().enumerate() {
        if !parameter.optional && param_index >= args.len() {
            return Err(format!(
                "function '{name}' missing required argument for parameter '{}'",
                parameter.name
            ));
        }
    }

    for (arg_index, arg) in args.iter().enumerate() {
        let arg_type = infer_comparison(arg, index, functions)?;
        let Some(parameter) = function.parameters.get(arg_index) else {
            return Err(format!("function '{name}' received too many arguments"));
        };
        let param_type = parse_logical_type(&parameter.type_name).map_err(|_| {
            format!(
                "function '{name}' parameter '{}' has invalid type",
                parameter.name
            )
        })?;
        if !types_assignable(&arg_type.logical, &param_type) {
            return Err(format!(
                "argument {} to function '{name}' has type '{}', expected '{}'",
                arg_index + 1,
                format_logical_type(&arg_type.logical),
                parameter.type_name
            ));
        }
        if arg_type.nullable && !parameter.optional {
            return Err(format!(
                "argument {} to function '{name}' references nullable fields but parameter '{}' is required",
                arg_index + 1,
                parameter.name
            ));
        }
    }

    Ok(return_type)
}

fn primitive_name(logical_type: &LogicalType) -> Result<&str, String> {
    match logical_type {
        LogicalType::Primitive(name) => Ok(name.as_str()),
        _ => Err(format!(
            "expected primitive type, found '{}'",
            format_logical_type(logical_type)
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

fn strip_outer_parens(expr: &str) -> Option<&str> {
    let expr = expr.trim();
    if !expr.starts_with('(') || !expr.ends_with(')') {
        return None;
    }
    let mut depth = 0;
    for (index, ch) in expr.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && index != expr.len() - 1 {
                    return None;
                }
            }
            _ => {}
        }
    }
    if depth == 0 {
        Some(&expr[1..expr.len() - 1])
    } else {
        None
    }
}

fn split_binary_leftmost<'a>(
    expr: &'a str,
    operators: &[&'a str],
) -> Option<(&'a str, &'a str, &'a str)> {
    let mut depth = 0;
    let mut in_string = false;
    let mut quote = '\0';
    let bytes = expr.as_bytes();
    let mut index = 0;
    while index < expr.len() {
        let ch = expr[index..].chars().next()?;
        if in_string {
            if ch == quote && (index == 0 || bytes[index - 1] != b'\\') {
                in_string = false;
            }
            index += ch.len_utf8();
            continue;
        }
        if ch == '"' || ch == '\'' {
            in_string = true;
            quote = ch;
            index += ch.len_utf8();
            continue;
        }
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ if depth == 0 => {
                for op in operators {
                    if expr[index..].starts_with(op) {
                        // Skip unary + / - at expression start.
                        if index == 0 && (*op == "+" || *op == "-") {
                            continue;
                        }
                        let left = expr[..index].trim();
                        let right = expr[index + op.len()..].trim();
                        if !left.is_empty() && !right.is_empty() {
                            return Some((left, op, right));
                        }
                    }
                }
            }
            _ => {}
        }
        index += ch.len_utf8();
    }
    None
}

fn split_call(expr: &str) -> Option<(&str, &str)> {
    let open = expr.find('(')?;
    if !expr.ends_with(')') {
        return None;
    }
    let name = expr[..open].trim();
    if name.is_empty() || name.contains(' ') {
        return None;
    }
    Some((name, &expr[open + 1..expr.len() - 1]))
}

fn split_args(args_source: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut quote = '\0';
    let mut start = 0;
    for (index, ch) in args_source.char_indices() {
        if in_string {
            if ch == quote {
                in_string = false;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            in_string = true;
            quote = ch;
            continue;
        }
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                let part = args_source[start..index].trim();
                if !part.is_empty() {
                    args.push(part);
                }
                start = index + 1;
            }
            _ => {}
        }
    }
    let part = args_source[start..].trim();
    if !part.is_empty() {
        args.push(part);
    }
    args
}

fn emit_type_error(
    ctx: &mut ValidationContext,
    object_ref: &str,
    type_name: &str,
    error: TypeParseError,
) {
    let message = match error {
        TypeParseError::BareComposite(kind) => {
            format!("composite type '{kind}' must declare type parameters")
        }
        TypeParseError::InvalidArity {
            kind,
            expected,
            actual,
        } => format!("composite type '{kind}' requires {expected} type parameters, found {actual}"),
        TypeParseError::Unknown(unknown) => format!("unknown logical type '{unknown}'"),
        TypeParseError::UnknownParameter(param) => {
            format!("unknown nested type parameter '{param}' in '{type_name}'")
        }
        TypeParseError::Malformed(detail) => {
            format!("malformed logical type '{type_name}': {detail}")
        }
    };
    ctx.error(
        codes::INVALID_TYPE,
        DiagnosticCategory::Type,
        message,
        Some(object_ref),
        None,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_index() -> FieldIndex {
        let contract = crate::model::TransformationContract::from_yaml(
            r#"
dtcsVersion: "1.0.0"
id: "test"
name: "Test"
version: "0.1.0"
inputs:
  - id: "in"
    schema:
      fields:
        - name: "a"
          type: "integer"
          nullable: false
        - name: "b"
          type: "integer"
          nullable: false
        - name: "value"
          type: "integer"
          nullable: false
outputs:
  - id: "out"
    schema:
      fields:
        - name: "value"
          type: "integer"
          nullable: false
lineage:
  mappings:
    - output: "out"
      inputs: ["in"]
"#,
        )
        .into_contract()
        .expect("contract");
        FieldIndex::from_contract(&contract)
    }

    #[test]
    fn infers_multiplication_before_addition() {
        let index = test_index();
        let inferred =
            infer_expression_type("in.a + in.b * 2", &index, &HashMap::new()).expect("type");
        assert_eq!(inferred.logical, LogicalType::Primitive("integer".into()));
    }

    #[test]
    fn infers_comparison_after_addition() {
        let index = test_index();
        let inferred =
            infer_expression_type("in.a < in.b + 1", &index, &HashMap::new()).expect("type");
        assert_eq!(inferred.logical, LogicalType::Primitive("boolean".into()));
    }

    #[test]
    fn infers_unary_minus() {
        let index = test_index();
        let inferred = infer_expression_type("-in.value", &index, &HashMap::new()).expect("type");
        assert_eq!(inferred.logical, LogicalType::Primitive("integer".into()));
    }
}
