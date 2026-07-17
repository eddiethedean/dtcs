//! Expression analysis (SPEC Chapter 8).

#![allow(missing_docs)]

use crate::analysis::AnalysisFinding;
use crate::diagnostics::Diagnostic;
use crate::model::{Expression, RegistryDocument, TransformationContract};

pub mod ast;
mod constants;
pub(crate) mod eval;
pub(crate) mod format;
pub(crate) mod parse;
pub(crate) mod rewrite;
pub(crate) mod types;

pub use format::format_expression;

use serde_json::Value;

/// Lower a string expression to a structured portable node (SPEC Chapter 8 §3.1).
pub fn to_structured_node(source: &str) -> Result<Value, String> {
    let expr = parse::parse_expression(source).map_err(|e| e.message)?;
    serde_json::to_value(&expr).map_err(|e| e.to_string())
}

/// Parse a structured expression node into the analysis AST.
pub fn from_structured_node(value: &Value) -> Result<ast::Expr, String> {
    serde_json::from_value(value.clone()).map_err(|e| e.to_string())
}

/// Resolve an expression declaration to an AST (body preferred over string expr).
pub fn resolve_expression_ast(expression: &Expression) -> Result<Option<ast::Expr>, String> {
    if let Some(body) = &expression.body {
        return Ok(Some(from_structured_node(body)?));
    }
    let Some(source) = expression.expr.as_deref() else {
        return Ok(None);
    };
    if source.trim().is_empty() {
        return Ok(None);
    }
    parse::parse_expression(source)
        .map(Some)
        .map_err(|e| e.message)
}

/// Output of analyzing a single expression body.
#[derive(Debug, Clone, Default)]
pub struct ExpressionAnalysis {
    pub diagnostics: Vec<Diagnostic>,
    pub findings: Vec<AnalysisFinding>,
    pub ast: Option<ast::Expr>,
    pub inferred_type: Option<crate::model::LogicalType>,
    pub inferred_nullable: Option<bool>,
}

/// Analyze one expression declaration.
#[must_use]
pub fn check_expression(
    _contract: &TransformationContract,
    expression: &Expression,
    registry_doc: &RegistryDocument,
) -> ExpressionAnalysis {
    let mut out = ExpressionAnalysis::default();

    let expr = match resolve_expression_ast(expression) {
        Ok(Some(expr)) => expr,
        Ok(None) => return out,
        Err(message) => {
            out.diagnostics.push(Diagnostic {
                id: crate::diagnostics::codes::INVALID_EXPRESSION.into(),
                severity: crate::diagnostics::Severity::Error,
                stage: crate::diagnostics::DiagnosticStage::Analysis,
                category: crate::diagnostics::DiagnosticCategory::Semantic,
                message,
                object_ref: Some(format!("expressions.{}", expression.id)),
                remediation: Some(
                    "Fix field references, operators, or structured body nodes".into(),
                ),
            });
            return out;
        }
    };

    if constants::is_constant(&expr) {
        out.findings.push(crate::analysis::AnalysisFinding {
            object_ref: format!("expressions.{}", expression.id),
            kind: "constantExpression".into(),
            message: "expression is constant and may be evaluated during planning".into(),
            attributes: Default::default(),
        });
    }

    out.ast = Some(expr.clone());

    match types::infer_expression_type(&expr, _contract, registry_doc) {
        Ok(inferred) => {
            out.inferred_type = Some(inferred.logical);
            out.inferred_nullable = Some(inferred.nullable);
        }
        Err(mut diag) => {
            diag.object_ref = Some(format!("expressions.{}", expression.id));
            diag.remediation =
                Some("Fix field references, operators, or function calls in the expression".into());
            out.diagnostics.push(diag);
        }
    }

    out
}

/// Collect qualified field reference targets from an expression AST.
#[must_use]
pub fn collect_field_refs(expr: &ast::Expr) -> Vec<String> {
    let mut refs = Vec::new();
    collect_field_refs_inner(expr, &mut refs);
    refs.sort();
    refs.dedup();
    refs
}

fn collect_field_refs_inner(expr: &ast::Expr, out: &mut Vec<String>) {
    match expr {
        ast::Expr::Literal { .. } => {}
        ast::Expr::FieldRef { target, .. } => out.push(target.clone()),
        ast::Expr::Unary { expr, .. } => collect_field_refs_inner(expr, out),
        ast::Expr::Binary { left, right, .. } => {
            collect_field_refs_inner(left, out);
            collect_field_refs_inner(right, out);
        }
        ast::Expr::Call { args, .. } => {
            for arg in args {
                collect_field_refs_inner(arg, out);
            }
        }
        ast::Expr::Lambda { body, .. } => collect_field_refs_inner(body, out),
    }
}
