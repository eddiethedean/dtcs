//! Extract capability requirements from a transformation plan.

use std::collections::BTreeSet;

use crate::analysis::expr::{self, ast::BinaryOp};
use crate::model::{parse_logical_type, LogicalType, COMPOSITE_TYPES, PRIMITIVE_TYPES};
use crate::plan::{PlanNodeKind, TransformationPlan};

use super::model::CapabilityGap;

/// Requirements extracted from a transformation plan.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanRequirements {
    /// Required logical types.
    pub logical_types: BTreeSet<String>,
    /// Required semantic actions.
    pub semantic_actions: BTreeSet<String>,
    /// Required functions.
    pub functions: BTreeSet<String>,
    /// Required rules.
    pub rules: BTreeSet<String>,
    /// Required operators.
    pub operators: BTreeSet<String>,
}

impl PlanRequirements {
    /// Extract requirements from a transformation plan.
    #[must_use]
    pub fn from_plan(plan: &TransformationPlan) -> Self {
        let mut req = Self::default();

        for input in &plan.inputs {
            collect_schema_types(input.schema.as_ref(), &mut req.logical_types);
        }
        for output in &plan.outputs {
            collect_schema_types(output.schema.as_ref(), &mut req.logical_types);
        }

        for function in &plan.functions {
            req.functions.insert(function.function.clone());
            if let Some(type_name) = &function.type_name {
                collect_type_name(type_name, &mut req.logical_types);
            }
        }

        for node in &plan.nodes {
            match &node.kind {
                PlanNodeKind::SemanticAction(action) => {
                    req.semantic_actions.insert(action.action.clone());
                }
                PlanNodeKind::Rule(rule) => {
                    req.rules.insert(rule.rule.clone());
                }
                PlanNodeKind::Expression(expression) => {
                    if let Some(type_name) = &expression.type_name {
                        collect_type_name(type_name, &mut req.logical_types);
                    }
                    if let Some(body) = expression.expr.as_deref() {
                        if let Ok(ast) = crate::analysis::expr::parse::parse_expression(body) {
                            collect_expr_requirements(&ast, &mut req);
                        }
                    }
                }
            }
        }

        req
    }

    /// Convert unmet requirements into capability gaps.
    #[must_use]
    pub fn gaps_against(
        &self,
        declaration: &super::model::EngineCapabilityDeclaration,
    ) -> Vec<CapabilityGap> {
        let mut gaps = Vec::new();
        let categories = &declaration.categories;

        for required in &self.logical_types {
            if !supports_type(required, &categories.logical_types) {
                gaps.push(CapabilityGap {
                    category: "logicalTypes".into(),
                    required: required.clone(),
                });
            }
        }
        for required in &self.semantic_actions {
            if !categories.semantic_actions.iter().any(|s| s == required) {
                gaps.push(CapabilityGap {
                    category: "semanticActions".into(),
                    required: required.clone(),
                });
            }
        }
        for required in &self.functions {
            if !categories.functions.iter().any(|s| s == required) {
                gaps.push(CapabilityGap {
                    category: "functions".into(),
                    required: required.clone(),
                });
            }
        }
        for required in &self.rules {
            if !categories.rules.iter().any(|s| s == required) {
                gaps.push(CapabilityGap {
                    category: "rules".into(),
                    required: required.clone(),
                });
            }
        }
        for required in &self.operators {
            if !categories.operators.iter().any(|s| s == required) {
                gaps.push(CapabilityGap {
                    category: "operators".into(),
                    required: required.clone(),
                });
            }
        }
        gaps
    }
}

fn collect_schema_types(schema: Option<&crate::model::Schema>, out: &mut BTreeSet<String>) {
    let Some(schema) = schema else {
        return;
    };
    for field in &schema.fields {
        collect_type_name(&field.type_name, out);
    }
}

fn collect_type_name(type_name: &str, out: &mut BTreeSet<String>) {
    out.insert(type_name.to_string());
    if let Ok(parsed) = parse_logical_type(type_name) {
        match parsed {
            LogicalType::Primitive(name) => {
                out.insert(name);
            }
            LogicalType::Composite { kind, params } => {
                out.insert(kind);
                for param in params {
                    collect_type_name(&param, out);
                }
            }
            LogicalType::Extension(name) => {
                out.insert(name);
            }
        }
    }
}

fn supports_type(required: &str, supported: &[String]) -> bool {
    if supported.iter().any(|s| s == required) {
        return true;
    }
    if PRIMITIVE_TYPES.contains(&required) || COMPOSITE_TYPES.contains(&required) {
        return supported.iter().any(|s| s == required);
    }
    if let Ok(parsed) = parse_logical_type(required) {
        return match parsed {
            LogicalType::Primitive(name) => supported.iter().any(|s| s == &name),
            LogicalType::Composite { kind, params } => {
                supported.iter().any(|s| s == &kind)
                    && params.iter().all(|p| supports_type(p, supported))
            }
            LogicalType::Extension(name) => supported.iter().any(|s| s == &name),
        };
    }
    false
}

fn collect_expr_requirements(ast: &expr::ast::Expr, req: &mut PlanRequirements) {
    match ast {
        expr::ast::Expr::Literal { .. } => {}
        expr::ast::Expr::FieldRef { .. } => {}
        expr::ast::Expr::Unary { op, expr, .. } => {
            req.operators.insert(unary_op_name(*op).into());
            collect_expr_requirements(expr, req);
        }
        expr::ast::Expr::Binary {
            op, left, right, ..
        } => {
            req.operators.insert(binary_op_name(*op).into());
            collect_expr_requirements(left, req);
            collect_expr_requirements(right, req);
        }
        expr::ast::Expr::Call { callee, args, .. } => {
            if callee.contains(':') {
                req.functions.insert(callee.clone());
            }
            for arg in args {
                collect_expr_requirements(arg, req);
            }
        }
    }
}

fn unary_op_name(op: expr::ast::UnaryOp) -> &'static str {
    match op {
        expr::ast::UnaryOp::Negate => "negate",
        expr::ast::UnaryOp::Not => "not",
    }
}

fn binary_op_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Sub => "sub",
        BinaryOp::Mul => "mul",
        BinaryOp::Div => "div",
        BinaryOp::Eq => "eq",
        BinaryOp::Neq => "neq",
        BinaryOp::Lt => "lt",
        BinaryOp::Lte => "lte",
        BinaryOp::Gt => "gt",
        BinaryOp::Gte => "gte",
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
    }
}
