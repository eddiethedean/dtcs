//! Constant-expression detection.

use crate::analysis::expr::ast::Expr;

#[must_use]
pub fn is_constant(expr: &Expr) -> bool {
    match expr {
        Expr::Literal { .. } => true,
        Expr::FieldRef { .. } => false,
        Expr::Unary { expr, .. } => is_constant(expr),
        Expr::Binary { left, right, .. } => is_constant(left) && is_constant(right),
        Expr::Call { .. } => false,
    }
}

