//! Semantic equivalence checking for optimized plans (SPEC Ch 15 §9).

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::analysis::expr::rewrite;
use crate::model::{ActionOrdering, TransformationSemantics};
use crate::plan::graph;
use crate::plan::model::{PlanGuarantees, PlanNodeKind, TransformationPlan};
use crate::plan::rule_key;
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
    normalized = collapse_consecutive_idempotent_signatures(normalized);
    semantics.ordering = Some(ActionOrdering::Explicit { order: normalized });
}

fn collapse_consecutive_idempotent_signatures(order: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for signature in order {
        if let Some(last) = out.last() {
            if last == &signature && idempotent_action_signature(&signature) {
                continue;
            }
        }
        out.push(signature);
    }
    out
}

fn idempotent_action_signature(signature: &str) -> bool {
    signature.split('@').next().is_some_and(|action| {
        matches!(
            action,
            "dtcs:lowercase" | "dtcs:uppercase" | "dtcs:trim" | "dtcs:normalize_whitespace"
        )
    })
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

#[derive(Debug, PartialEq, Eq)]
struct SemanticFingerprint {
    ordered_effects: Vec<String>,
    rules: BTreeSet<String>,
}

fn semantic_effects_equal(before: &TransformationPlan, after: &TransformationPlan) -> bool {
    semantic_fingerprint(before) == semantic_fingerprint(after)
}

fn semantic_fingerprint(plan: &TransformationPlan) -> SemanticFingerprint {
    let contract = plan_as_contract(plan);
    let order = graph::topological_order(&contract, &plan.nodes, &plan.dependencies);
    let observable = observable_node_ids(plan);
    let mut node_by_id = BTreeMap::new();
    for node in &plan.nodes {
        node_by_id.insert(node.id.as_str(), node);
    }

    let mut ordered_effects: Vec<String> = Vec::new();
    let mut rules = BTreeSet::new();
    for id in order {
        if !observable.contains(id.as_str()) {
            continue;
        }
        let Some(node) = node_by_id.get(id.as_str()) else {
            continue;
        };
        match &node.kind {
            PlanNodeKind::Rule(rule) => {
                rules.insert(format!(
                    "rule:{}@{}:{}",
                    rule.rule,
                    rule.target,
                    rule_key::rule_effect_identity(rule)
                ));
            }
            PlanNodeKind::SemanticAction(_) | PlanNodeKind::Expression(_) => {
                if let Some(key) = ordered_effect_key(node) {
                    if let Some(last) = ordered_effects.last() {
                        if consecutive_idempotent_action(last, &key) {
                            continue;
                        }
                    }
                    ordered_effects.push(key);
                }
            }
        }
    }
    SemanticFingerprint {
        ordered_effects,
        rules,
    }
}

fn observable_node_ids(plan: &TransformationPlan) -> HashSet<String> {
    let consumed: HashSet<String> = plan
        .dependencies
        .iter()
        .map(|edge| edge.from.clone())
        .collect();

    let mut observable = HashSet::new();
    for node in &plan.nodes {
        match &node.kind {
            PlanNodeKind::SemanticAction(_) | PlanNodeKind::Rule(_) => {
                observable.insert(node.id.clone());
            }
            PlanNodeKind::Expression(_) => {
                if consumed.contains(&node.id) {
                    observable.insert(node.id.clone());
                }
            }
        }
    }
    observable
}

fn ordered_effect_key(node: &crate::plan::model::PlanNode) -> Option<String> {
    match &node.kind {
        PlanNodeKind::SemanticAction(action) => {
            Some(format!("action:{}@{}", action.action, action.target))
        }
        PlanNodeKind::Rule(_) => None,
        PlanNodeKind::Expression(expression) => {
            let body = expression.expr.as_deref()?;
            if body.trim().is_empty() {
                return None;
            }
            let normalized = rewrite::normalize_expression_body(body);
            Some(format!("expr:{}@{}", normalized, expression.id))
        }
    }
}

fn consecutive_idempotent_action(previous: &str, current: &str) -> bool {
    if previous != current {
        return false;
    }
    let Some(action) = previous
        .strip_prefix("action:")
        .and_then(|rest| rest.split('@').next())
    else {
        return false;
    };
    matches!(
        action,
        "dtcs:lowercase" | "dtcs:uppercase" | "dtcs:trim" | "dtcs:normalize_whitespace"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis;
    use crate::plan::{lower, validate_with_registry};
    use crate::registry;
    use crate::{parse, validate, DocumentFormat};
    use std::fs;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    fn lower_fixture(name: &str) -> TransformationPlan {
        let content = fs::read(fixture(name)).expect("read fixture");
        let contract = parse(&content, DocumentFormat::Yaml)
            .into_contract()
            .expect("contract");
        let report = validate(&contract);
        assert!(report.is_valid(), "{name}: {:?}", report.diagnostics);
        let analysis = analysis::check_contract(&contract, None);
        let result = lower(&contract, None, Some(&analysis));
        assert!(result.is_valid(), "{name}: {:?}", result.diagnostics);
        result.plan.expect("plan")
    }

    #[test]
    fn equivalence_respects_action_order() {
        let trim_first = lower_fixture("plan_field_write_chain.yaml");
        let lower_first = lower_fixture("plan_action_order_reversed.yaml");
        assert!(
            !equivalent(&trim_first, &lower_first),
            "plans with reversed action order must not compare as equivalent"
        );
    }

    #[test]
    fn equivalence_distinguishes_rule_parameters() {
        let with_both = lower_fixture("optimize_rule_dedup_params.yaml");
        let mut without_second = with_both.clone();
        without_second.nodes.retain(|node| node.id != "code_min_b");
        assert!(
            !equivalent(&with_both, &without_second),
            "rule parameters must affect equivalence fingerprints"
        );
    }

    #[test]
    fn equivalence_matches_after_algebraic_optimize() {
        let original = lower_fixture("optimize_algebraic.yaml");
        let optimized = crate::plan::optimize(&original).plan.expect("optimized");
        assert!(equivalent(&original, &optimized));
        let registry_doc = registry::default_registry();
        assert!(validate_with_registry(&optimized, registry_doc).is_valid());
    }
}
