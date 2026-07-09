//! Transformation plan optimization (SPEC Ch 13 §9, Ch 8 §14, Ch 17–19 §11).

use std::collections::{BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::analysis::expr::ast::{BinaryOp, Expr, LiteralValue, Span, UnaryOp};
use crate::analysis::expr::{eval, format, parse};
use crate::diagnostics::{codes, optimization_error, Diagnostic, DiagnosticCategory};
use crate::model::{ActionOrdering, RegistryCategory, RegistryDocument};
use crate::registry;

use super::graph;
use super::model::{PlanNodeKind, TransformationPlan};
use super::validate::{plan_as_contract, validate_with_registry};

/// Options controlling optimization passes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeOptions {
    /// Run expression rewrites (constant folding and algebraic simplification).
    #[serde(default = "default_true")]
    pub expressions: bool,
    /// Run deterministic function-call evaluation in expressions.
    #[serde(default = "default_true")]
    pub functions: bool,
    /// Run semantic action fusion passes.
    #[serde(default = "default_true")]
    pub actions: bool,
    /// Run rule deduplication.
    #[serde(default = "default_true")]
    pub rules: bool,
    /// Remove unused expression nodes after other passes.
    #[serde(default = "default_true")]
    pub dead_expressions: bool,
    /// Validate the optimized plan before returning it.
    #[serde(default = "default_true")]
    pub validate: bool,
}

impl Default for OptimizeOptions {
    fn default() -> Self {
        Self {
            expressions: true,
            functions: true,
            actions: true,
            rules: true,
            dead_expressions: true,
            validate: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Record of an applied optimization rewrite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformRecord {
    /// Optimization pass name.
    pub pass: String,
    /// Affected plan node identifier.
    pub node_id: String,
    /// Human-readable description of the rewrite.
    pub description: String,
}

/// Result of optimizing a transformation plan.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeResult {
    /// Optimized plan when optimization succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<TransformationPlan>,
    /// Diagnostics from optimization and validation.
    pub diagnostics: Vec<Diagnostic>,
    /// Informational log of applied rewrites.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<TransformRecord>,
}

impl OptimizeResult {
    /// Returns `true` when no error-level diagnostics are present.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.diagnostics.iter().any(|d| d.severity.is_error())
    }
}

/// Optimize a validated transformation plan.
#[must_use]
pub fn optimize(plan: &TransformationPlan) -> OptimizeResult {
    optimize_with_registry(
        plan,
        registry::default_registry(),
        &OptimizeOptions::default(),
    )
}

/// Optimize a transformation plan with a registry catalog and options.
#[must_use]
pub fn optimize_with_registry(
    plan: &TransformationPlan,
    registry_doc: &RegistryDocument,
    options: &OptimizeOptions,
) -> OptimizeResult {
    let mut result = OptimizeResult::default();
    let mut working = plan.clone();
    let contract = plan_as_contract(&working);

    if options.expressions {
        optimize_expressions(
            &mut working,
            &contract,
            registry_doc,
            &mut result.transforms,
        );
    }
    if options.functions {
        optimize_functions(
            &mut working,
            &contract,
            registry_doc,
            &mut result.transforms,
        );
    }
    if options.actions {
        optimize_actions(&mut working, &mut result.transforms);
    }
    if options.rules {
        optimize_rules(&mut working, &mut result.transforms);
    }
    if options.dead_expressions {
        eliminate_dead_expressions(&mut working, &mut result.transforms);
    }

    rebuild_dependencies(&mut working, registry_doc, &mut result);

    if !result.is_valid() {
        return result;
    }

    if options.validate {
        let validation = validate_with_registry(&working, registry_doc);
        let validation_failed = !validation.is_valid();
        result.diagnostics.extend(validation.diagnostics);
        if validation_failed {
            result.diagnostics.push(
                optimization_error(
                    codes::INVALID_OPTIMIZATION,
                    DiagnosticCategory::Semantic,
                    "optimized plan failed validation",
                )
                .with_object_ref("plan"),
            );
            return result;
        }
    }

    result.plan = Some(working);
    result
}

fn rebuild_dependencies(
    plan: &mut TransformationPlan,
    registry_doc: &RegistryDocument,
    result: &mut OptimizeResult,
) {
    let contract = plan_as_contract(plan);
    let graph_result = graph::build(&contract, &plan.nodes, registry_doc);
    let graph_failed = graph_result
        .diagnostics
        .iter()
        .any(|d| d.severity.is_error());
    result.diagnostics.extend(graph_result.diagnostics);
    if graph_failed {
        result.diagnostics.push(
            optimization_error(
                codes::INVALID_OPTIMIZATION,
                DiagnosticCategory::Semantic,
                "failed to rebuild dependency graph after optimization",
            )
            .with_object_ref("dependencies"),
        );
        return;
    }
    plan.dependencies = graph_result.dependencies;
}

fn optimize_expressions(
    plan: &mut TransformationPlan,
    contract: &crate::model::TransformationContract,
    registry_doc: &RegistryDocument,
    transforms: &mut Vec<TransformRecord>,
) {
    for node in &mut plan.nodes {
        let PlanNodeKind::Expression(expression) = &mut node.kind else {
            continue;
        };
        let Some(body) = expression.expr.as_deref() else {
            continue;
        };
        if body.trim().is_empty() {
            continue;
        }
        let original_body = body.to_string();
        let Ok(mut ast) = parse::parse_expression(body) else {
            continue;
        };
        let before = format::format_expression(&ast);
        ast = simplify_expression(&ast);
        ast = fold_expression_calls(&ast, registry_doc);
        ast = simplify_expression(&ast);
        if let Some(value) = eval::evaluate(&ast) {
            ast = eval::literal_expr(value, ast_span(&ast));
        }
        let after = format::format_expression(&ast);
        if after != before {
            let mut accepted = true;
            if let Some(type_name) = expression.type_name.as_deref() {
                if let Ok(inferred) = crate::analysis::expr::types::infer_expression_type(
                    &ast,
                    contract,
                    registry_doc,
                ) {
                    if let Ok(declared) = crate::model::parse_logical_type(type_name) {
                        if declared != inferred.logical {
                            accepted = false;
                        }
                    }
                }
            }
            if accepted {
                expression.expr = Some(after);
                transforms.push(TransformRecord {
                    pass: "expression".into(),
                    node_id: node.id.clone(),
                    description: format!(
                        "rewrote expression '{before}' to '{}'",
                        expression.expr.as_deref().unwrap_or("")
                    ),
                });
                plan.findings
                    .retain(|finding| finding.object_ref != node.object_ref);
            } else {
                expression.expr = Some(original_body);
            }
        }
    }
}

fn optimize_functions(
    plan: &mut TransformationPlan,
    contract: &crate::model::TransformationContract,
    registry_doc: &RegistryDocument,
    transforms: &mut Vec<TransformRecord>,
) {
    for node in &mut plan.nodes {
        let PlanNodeKind::Expression(expression) = &mut node.kind else {
            continue;
        };
        let Some(body) = expression.expr.as_deref() else {
            continue;
        };
        let original_body = body.to_string();
        let Ok(mut ast) = parse::parse_expression(body) else {
            continue;
        };
        let before = format::format_expression(&ast);
        ast = fold_expression_calls(&ast, registry_doc);
        if let Some(value) = eval::evaluate(&ast) {
            ast = eval::literal_expr(value, ast_span(&ast));
        }
        let after = format::format_expression(&ast);
        if after != before {
            let mut accepted = true;
            if let Some(type_name) = expression.type_name.as_deref() {
                if let Ok(inferred) = crate::analysis::expr::types::infer_expression_type(
                    &ast,
                    contract,
                    registry_doc,
                ) {
                    if let Ok(declared) = crate::model::parse_logical_type(type_name) {
                        if declared != inferred.logical {
                            accepted = false;
                        }
                    }
                }
            }
            if accepted {
                expression.expr = Some(after.clone());
                transforms.push(TransformRecord {
                    pass: "function".into(),
                    node_id: node.id.clone(),
                    description: format!("evaluated function expression '{before}' to '{after}'"),
                });
                plan.findings
                    .retain(|finding| finding.object_ref != node.object_ref);
            } else {
                expression.expr = Some(original_body);
            }
        }
    }
}

fn fold_expression_calls(expr: &Expr, registry_doc: &RegistryDocument) -> Expr {
    match expr {
        Expr::Call { callee, args, span } => {
            let folded_args: Vec<Expr> = args
                .iter()
                .map(|arg| fold_expression_calls(arg, registry_doc))
                .collect();
            if callee.starts_with("dtcs:")
                && is_deterministic_registry_function(callee, registry_doc)
            {
                let const_args: Option<Vec<LiteralValue>> =
                    folded_args.iter().map(eval::evaluate).collect();
                if let Some(values) = const_args {
                    if let Some(result) = eval::evaluate_registry_call(callee, &values) {
                        return eval::literal_expr(result, span.clone());
                    }
                }
            }
            Expr::Call {
                callee: callee.clone(),
                args: folded_args,
                span: span.clone(),
            }
        }
        Expr::Unary { op, expr, span } => Expr::Unary {
            op: *op,
            span: span.clone(),
            expr: Box::new(fold_expression_calls(expr, registry_doc)),
        },
        Expr::Binary {
            op,
            left,
            right,
            span,
        } => Expr::Binary {
            op: *op,
            span: span.clone(),
            left: Box::new(fold_expression_calls(left, registry_doc)),
            right: Box::new(fold_expression_calls(right, registry_doc)),
        },
        other => other.clone(),
    }
}

fn simplify_expression(expr: &Expr) -> Expr {
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
            if let Expr::Literal { value, .. } = left {
                if eval::is_false(value) {
                    return Some(left.clone());
                }
                if eval::is_true(value) {
                    return Some(right.clone());
                }
            }
            if let Expr::Literal { value, .. } = right {
                if eval::is_false(value) {
                    return Some(right.clone());
                }
                if eval::is_true(value) {
                    return Some(left.clone());
                }
            }
        }
        BinaryOp::Or => {
            if let Expr::Literal { value, .. } = left {
                if eval::is_true(value) {
                    return Some(left.clone());
                }
                if eval::is_false(value) {
                    return Some(right.clone());
                }
            }
            if let Expr::Literal { value, .. } = right {
                if eval::is_true(value) {
                    return Some(right.clone());
                }
                if eval::is_false(value) {
                    return Some(left.clone());
                }
            }
        }
        _ => {}
    }
    None
}

fn optimize_actions(plan: &mut TransformationPlan, transforms: &mut Vec<TransformRecord>) {
    let contract = plan_as_contract(plan);
    let order = graph::topological_order(&contract, &plan.nodes, &plan.dependencies);
    let mut action_nodes: Vec<(String, String, String)> = Vec::new();
    for id in &order {
        let Some(node) = plan.nodes.iter().find(|n| &n.id == id) else {
            continue;
        };
        if let PlanNodeKind::SemanticAction(action) = &node.kind {
            action_nodes.push((
                node.id.clone(),
                action.action.clone(),
                action.target.clone(),
            ));
        }
    }

    let mut remove_ids = HashSet::new();
    let mut previous_by_target: HashMap<String, (String, String)> = HashMap::new();

    for (id, action, target) in action_nodes {
        if let Some((prev_id, prev_action)) = previous_by_target.get(&target) {
            if prev_action == &action && is_idempotent_action(&action) {
                remove_ids.insert(id.clone());
                transforms.push(TransformRecord {
                    pass: "action".into(),
                    node_id: id,
                    description: format!(
                        "removed redundant idempotent action '{action}' on target '{target}' after '{prev_id}'"
                    ),
                });
                continue;
            }
        }
        previous_by_target.insert(target, (id, action));
    }

    if remove_ids.is_empty() {
        return;
    }

    plan.nodes.retain(|node| !remove_ids.contains(&node.id));
    update_ordering_after_removals(plan, &remove_ids);
}

fn optimize_rules(plan: &mut TransformationPlan, transforms: &mut Vec<TransformRecord>) {
    let protected: BTreeSet<String> = plan
        .guarantees
        .input_preconditions
        .iter()
        .map(|c| c.rule_id.clone())
        .chain(
            plan.guarantees
                .output_postconditions
                .iter()
                .map(|c| c.rule_id.clone()),
        )
        .collect();

    let mut seen = HashSet::new();
    let mut remove_ids = HashSet::new();

    for node in &plan.nodes {
        let PlanNodeKind::Rule(rule) = &node.kind else {
            continue;
        };
        let key = (rule.rule.as_str(), rule.target.as_str(), rule.phase);
        if !seen.insert(key) && !protected.contains(&node.id) {
            remove_ids.insert(node.id.clone());
            transforms.push(TransformRecord {
                pass: "rule".into(),
                node_id: node.id.clone(),
                description: format!(
                    "removed duplicate rule '{}' on target '{}' ({})",
                    rule.rule,
                    rule.target,
                    rule.phase.as_str()
                ),
            });
        }
    }

    if remove_ids.is_empty() {
        return;
    }

    plan.nodes.retain(|node| !remove_ids.contains(&node.id));
}

fn eliminate_dead_expressions(
    plan: &mut TransformationPlan,
    transforms: &mut Vec<TransformRecord>,
) {
    let protected: BTreeSet<String> = plan
        .guarantees
        .input_preconditions
        .iter()
        .map(|c| c.rule_id.clone())
        .chain(
            plan.guarantees
                .output_postconditions
                .iter()
                .map(|c| c.rule_id.clone()),
        )
        .collect();

    let referenced: HashSet<String> = plan
        .dependencies
        .iter()
        .map(|edge| edge.to.clone())
        .chain(protected)
        .collect();

    let mut remove_ids = HashSet::new();
    for node in &plan.nodes {
        let PlanNodeKind::Expression(_) = &node.kind else {
            continue;
        };
        if !referenced.contains(&node.id) {
            remove_ids.insert(node.id.clone());
            transforms.push(TransformRecord {
                pass: "deadExpression".into(),
                node_id: node.id.clone(),
                description: "removed unused expression node".into(),
            });
        }
    }

    if remove_ids.is_empty() {
        return;
    }

    plan.nodes.retain(|node| !remove_ids.contains(&node.id));
    plan.findings
        .retain(|finding| !remove_ids.iter().any(|id| finding.object_ref.contains(id)));
}

fn update_ordering_after_removals(plan: &mut TransformationPlan, removed: &HashSet<String>) {
    let Some(semantics) = plan.guarantees.semantics.as_mut() else {
        return;
    };
    let Some(ordering) = semantics.ordering.as_mut() else {
        return;
    };
    let ActionOrdering::Explicit { order } = ordering else {
        return;
    };
    order.retain(|id| !removed.contains(id));
}

fn is_idempotent_action(action: &str) -> bool {
    matches!(
        action,
        "dtcs:lowercase" | "dtcs:uppercase" | "dtcs:trim" | "dtcs:normalize_whitespace"
    )
}

fn is_deterministic_registry_function(name: &str, registry_doc: &RegistryDocument) -> bool {
    let Some(entry) = registry::resolve(registry_doc, name) else {
        return false;
    };
    if entry.category != RegistryCategory::Function {
        return false;
    }
    let Some(definition) = entry.definition.as_deref() else {
        return false;
    };
    #[derive(Deserialize)]
    struct Def {
        deterministic: Option<bool>,
    }
    serde_json::from_str::<Def>(definition)
        .ok()
        .and_then(|def| def.deterministic)
        .unwrap_or(false)
}

fn ast_span(expr: &Expr) -> Span {
    match expr {
        Expr::Literal { span, .. }
        | Expr::FieldRef { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Call { span, .. } => span.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
