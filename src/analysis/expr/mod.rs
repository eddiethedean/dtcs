//! Expression analysis (SPEC Chapter 8).

#![allow(missing_docs)]

use crate::analysis::AnalysisFinding;
use crate::diagnostics::Diagnostic;
use crate::model::{Expression, RegistryDocument, TransformationContract};

pub mod ast;
mod constants;
pub(crate) mod parse;
pub(crate) mod types;

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

    let Some(body) = expression.expr.as_deref() else {
        return out;
    };
    if body.trim().is_empty() {
        return out;
    }

    match parse::parse_expression(body) {
        Ok(expr) => {
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
                    diag.remediation = Some(
                        "Fix field references, operators, or function calls in the expression"
                            .into(),
                    );
                    out.diagnostics.push(diag);
                }
            }
        }
        Err(err) => {
            out.diagnostics.push(parse::to_diagnostic(expression, err));
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
    }
}
