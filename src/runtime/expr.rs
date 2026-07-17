//! Runtime expression evaluation.

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, UnaryOp};
use crate::analysis::expr::parse::parse_expression;
use crate::runtime::conversion::integer_to_decimal;
use crate::runtime::functions::call_function;
use crate::runtime::model::{
    lookup_field, parse_qualified_field_with_interfaces, FieldLookup, Row, RuntimeValue,
};

/// Parse and evaluate an expression string against a single row (unqualified field names).
pub fn eval_expression_on_row(source: &str, row: &Row) -> Result<RuntimeValue, String> {
    let expr =
        parse_expression(source).map_err(|e| format!("expression parse error: {}", e.message))?;
    evaluate_expr_on_row(&expr, row)
}

/// Evaluate an expression AST against a single row.
pub fn evaluate_expr_on_row(expr: &Expr, row: &Row) -> Result<RuntimeValue, String> {
    match expr {
        Expr::Literal { value, .. } => Ok(literal_to_runtime(value)),
        Expr::FieldRef { target, .. } => {
            let field = target.rsplit('.').next().unwrap_or(target);
            Ok(match lookup_field(row, field) {
                FieldLookup::Missing => RuntimeValue::missing(),
                FieldLookup::Present(value) => value,
            })
        }
        Expr::Unary { op, expr, .. } => {
            let inner = evaluate_expr_on_row(expr, row)?;
            match op {
                UnaryOp::Negate => match inner {
                    RuntimeValue::Integer(v) => Ok(RuntimeValue::Integer(-v)),
                    RuntimeValue::Decimal(v) => Ok(RuntimeValue::Decimal(-v)),
                    other => Err(format!("negate unsupported for {other:?}")),
                },
                UnaryOp::Not => match inner.as_bool() {
                    Some(v) => Ok(RuntimeValue::Boolean(!v)),
                    None => Err("not requires boolean".into()),
                },
            }
        }
        Expr::Binary {
            op, left, right, ..
        } => match op {
            BinaryOp::And => {
                let left_val = evaluate_expr_on_row(left, row)?;
                let left_bool = left_val
                    .as_bool()
                    .ok_or_else(|| "and requires boolean operands".to_string())?;
                if !left_bool {
                    return Ok(RuntimeValue::Boolean(false));
                }
                let right_val = evaluate_expr_on_row(right, row)?;
                let right_bool = right_val
                    .as_bool()
                    .ok_or_else(|| "and requires boolean operands".to_string())?;
                Ok(RuntimeValue::Boolean(right_bool))
            }
            BinaryOp::Or => {
                let left_val = evaluate_expr_on_row(left, row)?;
                let left_bool = left_val
                    .as_bool()
                    .ok_or_else(|| "or requires boolean operands".to_string())?;
                if left_bool {
                    return Ok(RuntimeValue::Boolean(true));
                }
                let right_val = evaluate_expr_on_row(right, row)?;
                let right_bool = right_val
                    .as_bool()
                    .ok_or_else(|| "or requires boolean operands".to_string())?;
                Ok(RuntimeValue::Boolean(right_bool))
            }
            _ => {
                let left_val = evaluate_expr_on_row(left, row)?;
                let right_val = evaluate_expr_on_row(right, row)?;
                evaluate_binary(*op, &left_val, &right_val)
            }
        },
        Expr::Call { callee, args, .. } => {
            if is_lambda_function(callee) {
                return evaluate_lambda_call(callee, args, row);
            }
            let evaluated_args: Result<Vec<_>, _> = args
                .iter()
                .map(|arg| evaluate_expr_on_row(arg, row))
                .collect();
            call_function(callee, &evaluated_args?)
        }
        Expr::Lambda { .. } => {
            Err("lambda expressions may only be evaluated by a lambda-enabled function".into())
        }
    }
}

/// Evaluate an expression AST against a row workspace context.
pub fn evaluate_expr(
    expr: &Expr,
    workspaces: &std::collections::BTreeMap<String, Vec<Row>>,
    row_index: usize,
) -> Result<RuntimeValue, String> {
    let interface_ids: Vec<String> = workspaces.keys().cloned().collect();
    evaluate_expr_with_interfaces(expr, workspaces, row_index, &interface_ids)
}

fn evaluate_expr_with_interfaces(
    expr: &Expr,
    workspaces: &std::collections::BTreeMap<String, Vec<Row>>,
    row_index: usize,
    interface_ids: &[String],
) -> Result<RuntimeValue, String> {
    match expr {
        Expr::Literal { value, .. } => Ok(literal_to_runtime(value)),
        Expr::FieldRef { target, .. } => {
            let qualified = parse_qualified_field_with_interfaces(target, interface_ids)
                .ok_or_else(|| format!("invalid field reference '{target}'"))?;
            let rows = workspaces
                .get(&qualified.interface_id)
                .ok_or_else(|| format!("unknown interface '{}'", qualified.interface_id))?;
            let row = rows
                .get(row_index)
                .ok_or_else(|| format!("row index {row_index} out of range"))?;
            Ok(match lookup_field(row, &qualified.field_name) {
                FieldLookup::Missing => RuntimeValue::missing(),
                FieldLookup::Present(value) => value,
            })
        }
        Expr::Unary { op, expr, .. } => {
            let inner = evaluate_expr_with_interfaces(expr, workspaces, row_index, interface_ids)?;
            match op {
                UnaryOp::Negate => match inner {
                    RuntimeValue::Integer(v) => Ok(RuntimeValue::Integer(-v)),
                    RuntimeValue::Decimal(v) => Ok(RuntimeValue::Decimal(-v)),
                    other => Err(format!("negate unsupported for {other:?}")),
                },
                UnaryOp::Not => match inner.as_bool() {
                    Some(v) => Ok(RuntimeValue::Boolean(!v)),
                    None => Err("not requires boolean".into()),
                },
            }
        }
        Expr::Binary {
            op, left, right, ..
        } => match op {
            BinaryOp::And => {
                let left_val =
                    evaluate_expr_with_interfaces(left, workspaces, row_index, interface_ids)?;
                let left_bool = left_val
                    .as_bool()
                    .ok_or_else(|| "and requires boolean operands".to_string())?;
                if !left_bool {
                    return Ok(RuntimeValue::Boolean(false));
                }
                let right_val =
                    evaluate_expr_with_interfaces(right, workspaces, row_index, interface_ids)?;
                let right_bool = right_val
                    .as_bool()
                    .ok_or_else(|| "and requires boolean operands".to_string())?;
                Ok(RuntimeValue::Boolean(right_bool))
            }
            BinaryOp::Or => {
                let left_val =
                    evaluate_expr_with_interfaces(left, workspaces, row_index, interface_ids)?;
                let left_bool = left_val
                    .as_bool()
                    .ok_or_else(|| "or requires boolean operands".to_string())?;
                if left_bool {
                    return Ok(RuntimeValue::Boolean(true));
                }
                let right_val =
                    evaluate_expr_with_interfaces(right, workspaces, row_index, interface_ids)?;
                let right_bool = right_val
                    .as_bool()
                    .ok_or_else(|| "or requires boolean operands".to_string())?;
                Ok(RuntimeValue::Boolean(right_bool))
            }
            _ => {
                let left_val =
                    evaluate_expr_with_interfaces(left, workspaces, row_index, interface_ids)?;
                let right_val =
                    evaluate_expr_with_interfaces(right, workspaces, row_index, interface_ids)?;
                evaluate_binary(*op, &left_val, &right_val)
            }
        },
        Expr::Call { callee, args, .. } => {
            let evaluated_args: Result<Vec<_>, _> = args
                .iter()
                .map(|arg| evaluate_expr_with_interfaces(arg, workspaces, row_index, interface_ids))
                .collect();
            call_function(callee, &evaluated_args?)
        }
        Expr::Lambda { .. } => {
            Err("lambda expressions may only be evaluated by a lambda-enabled function".into())
        }
    }
}

fn is_lambda_function(callee: &str) -> bool {
    matches!(
        callee,
        "dtcs:transform" | "dtcs:filter_values" | "dtcs:exists" | "dtcs:forall"
    )
}

fn evaluate_lambda_call(callee: &str, args: &[Expr], row: &Row) -> Result<RuntimeValue, String> {
    if args.len() != 2 {
        return Err(format!("{callee} requires a collection and lambda"));
    }
    let collection = evaluate_expr_on_row(&args[0], row)?;
    let Expr::Lambda {
        parameters, body, ..
    } = &args[1]
    else {
        return Err(format!("{callee} requires a lambda as its second argument"));
    };
    if parameters.is_empty() || parameters.len() > 2 {
        return Err("lambda requires one or two parameters".into());
    }
    let RuntimeValue::List(values) = collection else {
        return Err(format!("{callee} requires a list"));
    };
    let mut mapped = Vec::with_capacity(values.len());
    for (index, value) in values.iter().cloned().enumerate() {
        let mut lambda_row = row.clone();
        lambda_row.insert(parameters[0].clone(), value);
        if let Some(index_name) = parameters.get(1) {
            lambda_row.insert(index_name.clone(), RuntimeValue::Integer(index as i64));
        }
        mapped.push(evaluate_expr_on_row(body, &lambda_row)?);
    }
    match callee {
        "dtcs:transform" => Ok(RuntimeValue::List(mapped)),
        "dtcs:filter_values" => {
            let mut out = Vec::new();
            for (index, predicate) in mapped.into_iter().enumerate() {
                if predicate.as_bool() == Some(true) {
                    out.push(values[index].clone());
                }
            }
            Ok(RuntimeValue::List(out))
        }
        "dtcs:exists" => Ok(RuntimeValue::Boolean(
            mapped.iter().any(|value| value.as_bool() == Some(true)),
        )),
        "dtcs:forall" => Ok(RuntimeValue::Boolean(
            mapped.iter().all(|value| value.as_bool() == Some(true)),
        )),
        _ => unreachable!(),
    }
}

fn literal_to_runtime(value: &LiteralValue) -> RuntimeValue {
    match value {
        LiteralValue::Boolean(v) => RuntimeValue::Boolean(*v),
        LiteralValue::String(v) => RuntimeValue::String(v.clone()),
        LiteralValue::Integer(v) => RuntimeValue::Integer(*v),
        LiteralValue::Decimal(v) => RuntimeValue::Decimal(*v),
    }
}

fn values_equal(left: &RuntimeValue, right: &RuntimeValue) -> bool {
    if left == right {
        return true;
    }
    match (left, right) {
        (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) => {
            integer_to_decimal(*a).is_ok_and(|promoted| promoted == *b)
        }
        (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) => {
            integer_to_decimal(*b).is_ok_and(|promoted| *a == promoted)
        }
        _ => false,
    }
}

fn evaluate_binary(
    op: BinaryOp,
    left: &RuntimeValue,
    right: &RuntimeValue,
) -> Result<RuntimeValue, String> {
    match op {
        BinaryOp::Add => match (left, right) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => a
                .checked_add(*b)
                .map(RuntimeValue::Integer)
                .ok_or_else(|| "integer overflow".into()),
            (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) => {
                Ok(RuntimeValue::Decimal(a + b))
            }
            (RuntimeValue::String(a), RuntimeValue::String(b)) => {
                Ok(RuntimeValue::String(format!("{a}{b}")))
            }
            (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) => {
                Ok(RuntimeValue::Decimal(integer_to_decimal(*a)? + b))
            }
            (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) => {
                Ok(RuntimeValue::Decimal(a + integer_to_decimal(*b)?))
            }
            _ => Err("add type mismatch".into()),
        },
        BinaryOp::Sub => match (left, right) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => a
                .checked_sub(*b)
                .map(RuntimeValue::Integer)
                .ok_or_else(|| "integer overflow".into()),
            (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) => {
                Ok(RuntimeValue::Decimal(a - b))
            }
            (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) => {
                Ok(RuntimeValue::Decimal(integer_to_decimal(*a)? - b))
            }
            (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) => {
                Ok(RuntimeValue::Decimal(a - integer_to_decimal(*b)?))
            }
            _ => Err("sub type mismatch".into()),
        },
        BinaryOp::Mul => match (left, right) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => a
                .checked_mul(*b)
                .map(RuntimeValue::Integer)
                .ok_or_else(|| "integer overflow".into()),
            (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) => {
                Ok(RuntimeValue::Decimal(a * b))
            }
            (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) => {
                Ok(RuntimeValue::Decimal(integer_to_decimal(*a)? * b))
            }
            (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) => {
                Ok(RuntimeValue::Decimal(a * integer_to_decimal(*b)?))
            }
            _ => Err("mul type mismatch".into()),
        },
        BinaryOp::Div => match (left, right) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) if *b != 0 => a
                .checked_div(*b)
                .map(RuntimeValue::Integer)
                .ok_or_else(|| "integer division overflow".into()),
            (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) if *b != 0.0 => {
                Ok(RuntimeValue::Decimal(a / b))
            }
            (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) if *b != 0.0 => {
                Ok(RuntimeValue::Decimal(integer_to_decimal(*a)? / b))
            }
            (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) if *b != 0 => {
                Ok(RuntimeValue::Decimal(a / integer_to_decimal(*b)?))
            }
            _ => Err("div type mismatch or division by zero".into()),
        },
        BinaryOp::Mod => match (left, right) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) if *b != 0 => {
                Ok(RuntimeValue::Integer(a % b))
            }
            (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) if *b != 0.0 => {
                Ok(RuntimeValue::Decimal(a % b))
            }
            _ => Err("mod type mismatch or division by zero".into()),
        },
        BinaryOp::Eq => Ok(RuntimeValue::Boolean(values_equal(left, right))),
        BinaryOp::Neq => Ok(RuntimeValue::Boolean(!values_equal(left, right))),
        BinaryOp::NullSafeEq => Ok(RuntimeValue::Boolean(null_safe_equal(left, right))),
        BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
            compare_ordered(op, left, right).map(RuntimeValue::Boolean)
        }
        BinaryOp::Between => {
            Err("between is ternary; use `value between lo and hi` (parsed as dtcs:between)".into())
        }
        BinaryOp::In => match right {
            RuntimeValue::List(items) => Ok(RuntimeValue::Boolean(
                items.iter().any(|item| values_equal(left, item)),
            )),
            RuntimeValue::String(haystack) => match left.as_str() {
                Some(needle) => Ok(RuntimeValue::Boolean(haystack.contains(needle))),
                None if left.is_null() || left.is_missing() => Ok(RuntimeValue::Null),
                None => Err("in with string right-hand side requires string left".into()),
            },
            _ => Err("in requires list or string on the right".into()),
        },
        BinaryOp::Contains => match left {
            RuntimeValue::List(items) => Ok(RuntimeValue::Boolean(
                items.iter().any(|item| values_equal(item, right)),
            )),
            RuntimeValue::String(haystack) => match right.as_str() {
                Some(needle) => Ok(RuntimeValue::Boolean(haystack.contains(needle))),
                None if right.is_null() || right.is_missing() => Ok(RuntimeValue::Null),
                None => Err("contains with string left-hand side requires string right".into()),
            },
            _ => Err("contains requires list or string on the left".into()),
        },
        BinaryOp::And | BinaryOp::Or => unreachable!("handled by short-circuit evaluation"),
    }
}

fn null_safe_equal(left: &RuntimeValue, right: &RuntimeValue) -> bool {
    match (left, right) {
        (RuntimeValue::Null, RuntimeValue::Null)
        | (RuntimeValue::Missing(_), RuntimeValue::Missing(_)) => true,
        (RuntimeValue::Null, _)
        | (_, RuntimeValue::Null)
        | (RuntimeValue::Missing(_), _)
        | (_, RuntimeValue::Missing(_)) => false,
        _ => values_equal(left, right),
    }
}

fn compare_ordered(
    op: BinaryOp,
    left: &RuntimeValue,
    right: &RuntimeValue,
) -> Result<bool, String> {
    use BinaryOp::{Gt, Gte, Lt, Lte};
    let ordering = match (left, right) {
        (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => a.cmp(b),
        (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) => a
            .partial_cmp(b)
            .ok_or_else(|| "decimal comparison failed".to_string())?,
        (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) => integer_to_decimal(*a)?
            .partial_cmp(b)
            .ok_or_else(|| "decimal comparison failed".to_string())?,
        (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) => a
            .partial_cmp(&integer_to_decimal(*b)?)
            .ok_or_else(|| "decimal comparison failed".to_string())?,
        (RuntimeValue::String(a), RuntimeValue::String(b)) => a.cmp(b),
        (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => a.cmp(b),
        _ => return Err("comparison type mismatch".into()),
    };
    Ok(match op {
        Lt => ordering == std::cmp::Ordering::Less,
        Lte => ordering != std::cmp::Ordering::Greater,
        Gt => ordering == std::cmp::Ordering::Greater,
        Gte => ordering != std::cmp::Ordering::Less,
        _ => return Err("invalid comparison operator".into()),
    })
}
