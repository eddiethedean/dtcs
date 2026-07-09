//! Shared expression rewrite helpers for optimization and equivalence.

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, Span, UnaryOp};
use crate::analysis::expr::{eval, format, parse};

/// Algebraically simplify an expression AST.
#[must_use]
pub(crate) fn simplify_expression(expr: &Expr) -> Expr {
    match expr {
        Expr::Unary { op, expr, span } => {
            let inner = simplify_expression(expr);
            match (op, &inner) {
                (
                    UnaryOp::Not,
                    Expr::Unary {
                        op: UnaryOp::Not,
                        expr: inner2,
                        ..
                    },
                ) => simplify_expression(inner2),
                (
                    UnaryOp::Negate,
                    Expr::Unary {
                        op: UnaryOp::Negate,
                        expr: inner2,
                        ..
                    },
                ) => simplify_expression(inner2),
                _ => Expr::Unary {
                    op: *op,
                    span: span.clone(),
                    expr: Box::new(inner),
                },
            }
        }
        Expr::Binary {
            op,
            left,
            right,
            span,
        } => {
            let left = simplify_expression(left);
            let right = simplify_expression(right);
            if let Some(simplified) = simplify_binary(*op, &left, &right, span) {
                return simplified;
            }
            Expr::Binary {
                op: *op,
                span: span.clone(),
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        other => other.clone(),
    }
}

fn simplify_binary(op: BinaryOp, left: &Expr, right: &Expr, span: &Span) -> Option<Expr> {
    match op {
        BinaryOp::Add => {
            if let Expr::Literal { value, .. } = right {
                if eval::is_zero(value) {
                    return Some(left.clone());
                }
            }
            if let Expr::Literal { value, .. } = left {
                if eval::is_zero(value) {
                    return Some(right.clone());
                }
            }
        }
        BinaryOp::Sub => {
            if let Expr::Literal { value, .. } = right {
                if eval::is_zero(value) {
                    return Some(left.clone());
                }
            }
        }
        BinaryOp::Mul => {
            if let Expr::Literal { value, .. } = right {
                if eval::is_one(value) {
                    return Some(left.clone());
                }
                if eval::is_zero(value) {
                    return Some(eval::literal_expr(value.clone(), span.clone()));
                }
            }
            if let Expr::Literal { value, .. } = left {
                if eval::is_one(value) {
                    return Some(right.clone());
                }
                if eval::is_zero(value) {
                    return Some(eval::literal_expr(value.clone(), span.clone()));
                }
            }
        }
        BinaryOp::Div => {
            if let Expr::Literal { value, .. } = right {
                if eval::is_one(value) {
                    return Some(left.clone());
                }
            }
        }
        BinaryOp::And => {
            if let (
                Expr::Literal {
                    value: left_val, ..
                },
                Expr::Literal {
                    value: right_val, ..
                },
            ) = (&left, &right)
            {
                if !matches!(left_val, LiteralValue::Boolean(_))
                    || !matches!(right_val, LiteralValue::Boolean(_))
                {
                    return None;
                }
                if eval::is_false(left_val) {
                    return Some(left.clone());
                }
                if eval::is_true(left_val) {
                    return Some(right.clone());
                }
                if eval::is_false(right_val) {
                    return Some(right.clone());
                }
                if eval::is_true(right_val) {
                    return Some(left.clone());
                }
            }
        }
        BinaryOp::Or => {
            if let (
                Expr::Literal {
                    value: left_val, ..
                },
                Expr::Literal {
                    value: right_val, ..
                },
            ) = (&left, &right)
            {
                if !matches!(left_val, LiteralValue::Boolean(_))
                    || !matches!(right_val, LiteralValue::Boolean(_))
                {
                    return None;
                }
                if eval::is_true(left_val) {
                    return Some(left.clone());
                }
                if eval::is_false(left_val) {
                    return Some(right.clone());
                }
                if eval::is_true(right_val) {
                    return Some(right.clone());
                }
                if eval::is_false(right_val) {
                    return Some(left.clone());
                }
            }
        }
        _ => {}
    }
    None
}

/// Normalize an expression body for semantic comparison.
#[must_use]
pub(crate) fn normalize_expression_body(body: &str) -> String {
    let Ok(ast) = parse::parse_expression(body) else {
        return body.trim().to_string();
    };
    let simplified = simplify_expression(&ast);
    if let Some(value) = eval::evaluate(&simplified) {
        return format::format_literal_value(&value);
    }
    format::format_expression(&simplified)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::expr::ast::LiteralValue;

    #[test]
    fn simplify_add_zero() {
        let expr = Expr::Binary {
            op: BinaryOp::Add,
            span: Span { start: 0, end: 3 },
            left: Box::new(Expr::FieldRef {
                target: "in.value".into(),
                span: Span { start: 0, end: 8 },
            }),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Integer(0),
                span: Span { start: 9, end: 10 },
            }),
        };
        let simplified = simplify_expression(&expr);
        assert!(matches!(simplified, Expr::FieldRef { .. }));
    }
}
