//! Semantic equivalence checking for optimized plans (SPEC Ch 15 §9).

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::analysis::expr::{eval, format, parse};
use crate::model::{ActionOrdering, TransformationSemantics};
use crate::plan::graph;
use crate::plan::model::{PlanGuarantees, PlanNodeKind, TransformationPlan};
use crate::plan::validate::plan_as_contract;

/// Returns true when two plans are semantically equivalent.
#[must_use]
pub fn equivalent(before: &TransformationPlan, after: &TransformationPlan) -> bool {
    structural_invariants_equal(before, after) && semantic_effects_equal(before, after)
}

fn structural_invariants_equal(before: &TransformationPlan, after: &TransformationPlan) -> bool {
    before.identity == after.identity
        && before.inputs == after.inputs
        && before.outputs == after.outputs
        && before.functions == after.functions
        && before.lineage == after.lineage
        && normalized_guarantees(before) == normalized_guarantees(after)
        && before.versioning == after.versioning
        && before.extensions == after.extensions
}

fn normalized_guarantees(plan: &TransformationPlan) -> PlanGuarantees {
    let mut guarantees = plan.guarantees.clone();
    if let Some(semantics) = guarantees.semantics.as_mut() {
        normalize_ordering(semantics, plan);
    }
    guarantees
}

fn normalize_ordering(semantics: &mut TransformationSemantics, plan: &TransformationPlan) {
    let Some(ActionOrdering::Explicit { order }) = semantics.ordering.as_mut() else {
        return;
    };
    let signatures = action_signatures_by_id(plan);
    let mut normalized: Vec<String> = order
        .iter()
        .filter_map(|id| signatures.get(id.as_str()).cloned())
        .collect();
    normalized.sort();
    normalized.dedup();
    semantics.ordering = Some(ActionOrdering::Explicit { order: normalized });
}

fn action_signatures_by_id(plan: &TransformationPlan) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for node in &plan.nodes {
        if let PlanNodeKind::SemanticAction(action) = &node.kind {
            out.insert(
                node.id.clone(),
                format!("{}@{}", action.action, action.target),
            );
        }
    }
    out
}

fn semantic_effects_equal(before: &TransformationPlan, after: &TransformationPlan) -> bool {
    observable_effect_fingerprint(before) == observable_effect_fingerprint(after)
}

fn observable_effect_fingerprint(plan: &TransformationPlan) -> BTreeSet<String> {
    let contract = plan_as_contract(plan);
    let order = graph::topological_order(&contract, &plan.nodes, &plan.dependencies);
    let observable = observable_node_ids(plan);
    let mut node_by_id = BTreeMap::new();
    for node in &plan.nodes {
        node_by_id.insert(node.id.as_str(), node);
    }

    let mut effects = BTreeSet::new();
    for id in order {
        if !observable.contains(id.as_str()) {
            continue;
        }
        let Some(node) = node_by_id.get(id.as_str()) else {
            continue;
        };
        if let Some(key) = effect_key(node) {
            effects.insert(key);
        }
    }
    effects
}

fn observable_node_ids(plan: &TransformationPlan) -> HashSet<String> {
    let mut observable = HashSet::new();
    for node in &plan.nodes {
        match &node.kind {
            PlanNodeKind::SemanticAction(_) | PlanNodeKind::Rule(_) => {
                observable.insert(node.id.clone());
            }
            PlanNodeKind::Expression(_) => {}
        }
    }
    for edge in &plan.dependencies {
        if matches!(edge.reason, crate::plan::DependencyReason::FieldRead) {
            observable.insert(edge.to.clone());
        }
    }
    observable
}

fn effect_key(node: &crate::plan::model::PlanNode) -> Option<String> {
    match &node.kind {
        PlanNodeKind::SemanticAction(action) => {
            Some(format!("action:{}@{}", action.action, action.target))
        }
        PlanNodeKind::Rule(rule) => Some(format!(
            "rule:{}@{}:{}",
            rule.rule,
            rule.target,
            rule.phase.as_str()
        )),
        PlanNodeKind::Expression(expression) => {
            let body = expression.expr.as_deref()?;
            if body.trim().is_empty() {
                return None;
            }
            let normalized = normalize_expression_body(body);
            Some(format!("expr:{}@{}", normalized, expression.id))
        }
    }
}

fn normalize_expression_body(body: &str) -> String {
    let Ok(ast) = parse::parse_expression(body) else {
        return body.trim().to_string();
    };
    let simplified = simplify_for_compare(&ast);
    if let Some(value) = eval::evaluate(&simplified) {
        return format::format_literal_value(&value);
    }
    format::format_expression(&simplified)
}

fn simplify_for_compare(
    expr: &crate::analysis::expr::ast::Expr,
) -> crate::analysis::expr::ast::Expr {
    use crate::analysis::expr::ast::{BinaryOp, Expr};
    match expr {
        Expr::Binary {
            op,
            left,
            right,
            span,
        } => {
            let left = simplify_for_compare(left);
            let right = simplify_for_compare(right);
            match op {
                BinaryOp::Mul => {
                    if matches!(&right, Expr::Literal { value, .. } if eval::is_one(value)) {
                        return left;
                    }
                    if matches!(&left, Expr::Literal { value, .. } if eval::is_one(value)) {
                        return right;
                    }
                }
                BinaryOp::Add => {
                    if matches!(&right, Expr::Literal { value, .. } if eval::is_zero(value)) {
                        return left;
                    }
                    if matches!(&left, Expr::Literal { value, .. } if eval::is_zero(value)) {
                        return right;
                    }
                }
                _ => {}
            }
            Expr::Binary {
                op: *op,
                span: span.clone(),
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Expr::Unary { op, expr, span } => Expr::Unary {
            op: *op,
            span: span.clone(),
            expr: Box::new(simplify_for_compare(expr)),
        },
        Expr::Call { callee, args, span } => Expr::Call {
            callee: callee.clone(),
            span: span.clone(),
            args: args.iter().map(simplify_for_compare).collect(),
        },
        other => other.clone(),
    }
}
