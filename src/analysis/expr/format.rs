//! Deterministic expression formatting for optimization rewrites.

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, UnaryOp};

/// Format an expression AST as canonical source text.
#[must_use]
pub fn format_expression(expr: &Expr) -> String {
    format_expr(expr, false)
}

/// Format a literal value as canonical source text.
#[must_use]
pub fn format_literal_value(value: &LiteralValue) -> String {
    format_literal(value)
}

fn format_expr(expr: &Expr, parent_is_binary: bool) -> String {
    match expr {
        Expr::Literal { value, .. } => format_literal(value),
        Expr::FieldRef { target, .. } => target.clone(),
        Expr::Unary { op, expr, .. } => {
            let inner = format_expr(expr, false);
            let op_str = match op {
                UnaryOp::Negate => "-",
                UnaryOp::Not => "!",
            };
            if matches!(expr.as_ref(), Expr::Binary { .. }) {
                format!("{op_str}({inner})")
            } else {
                format!("{op_str}{inner}")
            }
        }
        Expr::Binary {
            op, left, right, ..
        } => {
            let left_str = format_expr(left, true);
            let right_str = format_expr(right, true);
            let op_str = binary_op_str(*op);
            let formatted = format!("{left_str} {op_str} {right_str}");
            if parent_is_binary {
                format!("({formatted})")
            } else {
                formatted
            }
        }
        Expr::Call { callee, args, .. } => {
            let rendered: Vec<String> = args.iter().map(|arg| format_expr(arg, false)).collect();
            format!("{}({})", callee, rendered.join(", "))
        }
        Expr::Lambda {
            parameters, body, ..
        } => format!(
            "({}) -> {}",
            parameters.join(", "),
            format_expr(body, false)
        ),
    }
}

fn format_literal(value: &LiteralValue) -> String {
    match value {
        LiteralValue::Boolean(true) => "true".into(),
        LiteralValue::Boolean(false) => "false".into(),
        LiteralValue::String(s) => format!("\"{}\"", escape_string(s)),
        LiteralValue::Integer(v) => v.to_string(),
        LiteralValue::Decimal(v) => v.to_string(),
    }
}

fn escape_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn binary_op_str(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Mod => "%",
        BinaryOp::Eq => "==",
        BinaryOp::Neq => "!=",
        BinaryOp::NullSafeEq => "<=>",
        BinaryOp::Lt => "<",
        BinaryOp::Lte => "<=",
        BinaryOp::Gt => ">",
        BinaryOp::Gte => ">=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::In => "in",
        BinaryOp::Between => "between",
        BinaryOp::Contains => "contains",
    }
}
