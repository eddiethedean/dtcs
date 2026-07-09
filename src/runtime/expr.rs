//! Runtime expression evaluation.

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, UnaryOp};
use crate::runtime::conversion::integer_to_decimal;
use crate::runtime::functions::call_function;
use crate::runtime::model::{parse_qualified_field_with_interfaces, Row, RuntimeValue};

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
            row.get(&qualified.field_name).cloned().ok_or_else(|| {
                format!(
                    "missing field '{}' on interface '{}'",
                    qualified.field_name, qualified.interface_id
                )
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
        BinaryOp::Eq => Ok(RuntimeValue::Boolean(values_equal(left, right))),
        BinaryOp::Neq => Ok(RuntimeValue::Boolean(!values_equal(left, right))),
        BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
            compare_ordered(op, left, right).map(RuntimeValue::Boolean)
        }
        BinaryOp::And | BinaryOp::Or => unreachable!("handled by short-circuit evaluation"),
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
