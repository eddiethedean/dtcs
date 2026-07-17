//! Expression AST types (analysis-only).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Expr {
    Literal {
        value: LiteralValue,
        span: Span,
    },
    FieldRef {
        target: String,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum LiteralValue {
    Boolean(bool),
    String(String),
    Integer(i64),
    Decimal(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BinaryOp {
/// Binary operators.
    Add,
    Sub,
    Mul,
    Div,
    /// Integer / numeric modulo.
    Mod,
    Eq,
    Neq,
    /// Null-safe equality (null == null is true).
    NullSafeEq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    /// Membership test (`value in collection`).
    In,
    /// Inclusive between (`value between lo and hi`) — desugared to comparisons when needed.
    Between,
    /// Collection/string contains (`collection contains value`).
    Contains,
}
