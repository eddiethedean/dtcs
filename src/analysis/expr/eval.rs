//! Constant expression evaluation for optimization (SPEC Ch 8 §14).

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, Span, UnaryOp};

/// Evaluate a fully constant expression subtree.
#[must_use]
pub fn evaluate(expr: &Expr) -> Option<LiteralValue> {
    match expr {
        Expr::Literal { value, .. } => Some(value.clone()),
        Expr::FieldRef { .. } | Expr::Call { .. } | Expr::Lambda { .. } => None,
        Expr::Unary { op, expr, .. } => {
            let inner = evaluate(expr)?;
            match op {
                UnaryOp::Negate => match inner {
                    LiteralValue::Integer(v) => Some(LiteralValue::Integer(-v)),
                    LiteralValue::Decimal(v) => Some(LiteralValue::Decimal(-v)),
                    _ => None,
                },
                UnaryOp::Not => match inner {
                    LiteralValue::Boolean(v) => Some(LiteralValue::Boolean(!v)),
                    _ => None,
                },
            }
        }
        Expr::Binary {
            op, left, right, ..
        } => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            evaluate_binary(*op, &left_val, &right_val)
        }
    }
}

/// Evaluate a deterministic `dtcs:` function call with constant arguments.
#[must_use]
pub fn evaluate_registry_call(callee: &str, args: &[LiteralValue]) -> Option<LiteralValue> {
    match callee {
        "dtcs:lower" | "dtcs:upper" => {
            let LiteralValue::String(s) = args.first()? else {
                return None;
            };
            let out = if callee == "dtcs:lower" {
                s.to_lowercase()
            } else {
                s.to_uppercase()
            };
            Some(LiteralValue::String(out))
        }
        "dtcs:concat" => {
            let mut out = String::new();
            for arg in args {
                let LiteralValue::String(s) = arg else {
                    return None;
                };
                out.push_str(s);
            }
            Some(LiteralValue::String(out))
        }
        "dtcs:length" => {
            let value = args.first()?;
            let len = match value {
                LiteralValue::String(s) => s.chars().count() as i64,
                _ => return None,
            };
            Some(LiteralValue::Integer(len))
        }
        _ => None,
    }
}

fn evaluate_binary(
    op: BinaryOp,
    left: &LiteralValue,
    right: &LiteralValue,
) -> Option<LiteralValue> {
    match op {
        BinaryOp::Add => match (left, right) {
            (LiteralValue::Integer(a), LiteralValue::Integer(b)) => {
                Some(LiteralValue::Integer(a.checked_add(*b)?))
            }
            (LiteralValue::Decimal(a), LiteralValue::Decimal(b)) => {
                Some(LiteralValue::Decimal(a + b))
            }
            (LiteralValue::String(a), LiteralValue::String(b)) => {
                Some(LiteralValue::String(format!("{a}{b}")))
            }
            (LiteralValue::Integer(a), LiteralValue::Decimal(b)) => {
                Some(LiteralValue::Decimal(*a as f64 + b))
            }
            (LiteralValue::Decimal(a), LiteralValue::Integer(b)) => {
                Some(LiteralValue::Decimal(a + *b as f64))
            }
            _ => None,
        },
        BinaryOp::Sub => match (left, right) {
            (LiteralValue::Integer(a), LiteralValue::Integer(b)) => {
                Some(LiteralValue::Integer(a.checked_sub(*b)?))
            }
            (LiteralValue::Decimal(a), LiteralValue::Decimal(b)) => {
                Some(LiteralValue::Decimal(a - b))
            }
            (LiteralValue::Integer(a), LiteralValue::Decimal(b)) => {
                Some(LiteralValue::Decimal(*a as f64 - b))
            }
            (LiteralValue::Decimal(a), LiteralValue::Integer(b)) => {
                Some(LiteralValue::Decimal(a - *b as f64))
            }
            _ => None,
        },
        BinaryOp::Mul => match (left, right) {
            (LiteralValue::Integer(a), LiteralValue::Integer(b)) => {
                Some(LiteralValue::Integer(a.checked_mul(*b)?))
            }
            (LiteralValue::Decimal(a), LiteralValue::Decimal(b)) => {
                Some(LiteralValue::Decimal(a * b))
            }
            (LiteralValue::Integer(a), LiteralValue::Decimal(b)) => {
                Some(LiteralValue::Decimal(*a as f64 * b))
            }
            (LiteralValue::Decimal(a), LiteralValue::Integer(b)) => {
                Some(LiteralValue::Decimal(a * *b as f64))
            }
            _ => None,
        },
        BinaryOp::Div => match (left, right) {
            (LiteralValue::Integer(a), LiteralValue::Integer(b)) if *b != 0 => {
                Some(LiteralValue::Integer(a / b))
            }
            (LiteralValue::Decimal(a), LiteralValue::Decimal(b)) if *b != 0.0 => {
                Some(LiteralValue::Decimal(a / b))
            }
            (LiteralValue::Integer(a), LiteralValue::Decimal(b)) if *b != 0.0 => {
                Some(LiteralValue::Decimal(*a as f64 / b))
            }
            (LiteralValue::Decimal(a), LiteralValue::Integer(b)) if *b != 0 => {
                Some(LiteralValue::Decimal(a / *b as f64))
            }
            _ => None,
        },
        BinaryOp::Eq => Some(LiteralValue::Boolean(left == right)),
        BinaryOp::Neq => Some(LiteralValue::Boolean(left != right)),
        BinaryOp::NullSafeEq => Some(LiteralValue::Boolean(left == right)),
        BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
            compare_ordered(op, left, right).map(LiteralValue::Boolean)
        }
        BinaryOp::And => match (left, right) {
            (LiteralValue::Boolean(a), LiteralValue::Boolean(b)) => {
                Some(LiteralValue::Boolean(*a && *b))
            }
            _ => None,
        },
        BinaryOp::Or => match (left, right) {
            (LiteralValue::Boolean(a), LiteralValue::Boolean(b)) => {
                Some(LiteralValue::Boolean(*a || *b))
            }
            _ => None,
        },
        BinaryOp::Mod => match (left, right) {
            (LiteralValue::Integer(a), LiteralValue::Integer(b)) if *b != 0 => {
                Some(LiteralValue::Integer(a % b))
            }
            _ => None,
        },
        BinaryOp::In | BinaryOp::Contains | BinaryOp::Between => None,
    }
}

fn compare_ordered(op: BinaryOp, left: &LiteralValue, right: &LiteralValue) -> Option<bool> {
    use BinaryOp::{Gt, Gte, Lt, Lte};
    let ordering = match (left, right) {
        (LiteralValue::Integer(a), LiteralValue::Integer(b)) => a.cmp(b),
        (LiteralValue::Decimal(a), LiteralValue::Decimal(b)) => a.partial_cmp(b)?,
        (LiteralValue::String(a), LiteralValue::String(b)) => a.cmp(b),
        (LiteralValue::Boolean(a), LiteralValue::Boolean(b)) => a.cmp(b),
        _ => return None,
    };
    Some(match op {
        Lt => ordering == std::cmp::Ordering::Less,
        Lte => ordering != std::cmp::Ordering::Greater,
        Gt => ordering == std::cmp::Ordering::Greater,
        Gte => ordering != std::cmp::Ordering::Less,
        _ => return None,
    })
}

/// Returns true when the literal is numeric zero.
#[must_use]
pub fn is_zero(value: &LiteralValue) -> bool {
    match value {
        LiteralValue::Integer(0) => true,
        LiteralValue::Decimal(v) => *v == 0.0,
        _ => false,
    }
}

/// Returns true when the literal is numeric one.
#[must_use]
pub fn is_one(value: &LiteralValue) -> bool {
    match value {
        LiteralValue::Integer(1) => true,
        LiteralValue::Decimal(v) => *v == 1.0,
        _ => false,
    }
}

/// Returns true when the literal is boolean true.
#[must_use]
pub fn is_true(value: &LiteralValue) -> bool {
    matches!(value, LiteralValue::Boolean(true))
}

/// Returns true when the literal is boolean false.
#[must_use]
pub fn is_false(value: &LiteralValue) -> bool {
    matches!(value, LiteralValue::Boolean(false))
}

/// Build a literal expression node.
#[must_use]
pub fn literal_expr(value: LiteralValue, span: Span) -> Expr {
    Expr::Literal { value, span }
}
