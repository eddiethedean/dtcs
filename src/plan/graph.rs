//! Dependency graph construction for transformation plans (SPEC Ch 13 §8).

use std::collections::{HashMap, HashSet, VecDeque};

use crate::analysis::expr::{self, check_expression};
use crate::diagnostics::{codes, planning_error, Diagnostic, DiagnosticCategory};
use crate::model::{ActionOrdering, RegistryDocument, Rule, RulePhase, TransformationContract};
use crate::validation::field_index::FieldIndex;

use super::model::{DependencyReason, PlanDependency, PlanNode, PlanNodeKind};

/// Result of building a dependency graph.
#[derive(Debug, Clone, Default)]
pub struct GraphBuildResult {
    /// Directed dependency edges.
    pub dependencies: Vec<PlanDependency>,
    /// Diagnostics emitted during graph construction.
    pub diagnostics: Vec<Diagnostic>,
}

/// Build logical dependency edges for plan nodes.
#[must_use]
pub fn build(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    registry: &RegistryDocument,
) -> GraphBuildResult {
    let mut result = GraphBuildResult::default();
    let field_index = FieldIndex::from_contract(contract);

    add_lineage_edges(contract, &mut result.dependencies);
    add_action_edges(
        contract,
        nodes,
        &mut result.dependencies,
        &mut result.diagnostics,
    );
    add_expression_edges(
        contract,
        nodes,
        registry,
        &field_index,
        &mut result.dependencies,
    );
    add_rule_phase_edges(nodes, &mut result.dependencies);
    add_interface_condition_edges(contract, nodes, &mut result.dependencies);
    add_rule_field_edges(contract, nodes, &field_index, &mut result.dependencies);

    if let Some(cycle) = detect_cycle(nodes, contract, &result.dependencies) {
        result.diagnostics.push(
            planning_error(
                codes::CYCLIC_DEPENDENCY,
                DiagnosticCategory::Semantic,
                format!("dependency graph contains a cycle involving '{cycle}'"),
            )
            .with_object_ref("dependencies"),
        );
    }

    sort_dependencies(&mut result.dependencies);
    result
}

/// Topological order of node and interface ids (returns empty on cycle).
#[must_use]
pub fn topological_order(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    dependencies: &[PlanDependency],
) -> Vec<String> {
    let mut vertices = HashSet::new();
    for input in &contract.inputs {
        vertices.insert(input.id.clone());
    }
    for output in &contract.outputs {
        vertices.insert(output.id.clone());
    }
    for node in nodes {
        vertices.insert(node.id.clone());
    }

    let mut in_degree: HashMap<String, usize> = vertices.iter().map(|v| (v.clone(), 0)).collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();

    for edge in dependencies {
        if vertices.contains(&edge.from) && vertices.contains(&edge.to) {
            adj.entry(edge.from.clone())
                .or_default()
                .push(edge.to.clone());
            if let Some(deg) = in_degree.get_mut(&edge.to) {
                *deg += 1;
            }
        }
    }

    let mut queue: VecDeque<String> = in_degree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(id, _)| id.clone())
        .collect();
    queue.make_contiguous().sort();

    let mut order = Vec::new();
    while let Some(v) = queue.pop_front() {
        order.push(v.clone());
        if let Some(neighbors) = adj.get(&v) {
            let mut sorted_neighbors = neighbors.clone();
            sorted_neighbors.sort();
            for n in sorted_neighbors {
                if let Some(deg) = in_degree.get_mut(&n) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(n);
                    }
                }
            }
        }
    }

    if order.len() == vertices.len() {
        order
    } else {
        Vec::new()
    }
}

fn detect_cycle(
    nodes: &[PlanNode],
    contract: &TransformationContract,
    dependencies: &[PlanDependency],
) -> Option<String> {
    let order = topological_order(contract, nodes, dependencies);
    if order.is_empty() && !dependencies.is_empty() {
        dependencies.first().map(|e| e.from.clone())
    } else if order.len() < contract.inputs.len() + contract.outputs.len() + nodes.len()
        && !dependencies.is_empty()
    {
        dependencies.first().map(|e| e.from.clone())
    } else {
        None
    }
}

fn add_lineage_edges(contract: &TransformationContract, edges: &mut Vec<PlanDependency>) {
    let Some(lineage) = contract.lineage.as_ref() else {
        return;
    };
    for mapping in &lineage.mappings {
        for input in &mapping.inputs {
            push_edge(
                edges,
                input.clone(),
                mapping.output.clone(),
                DependencyReason::Lineage,
            );
        }
    }
}

fn add_action_edges(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    edges: &mut Vec<PlanDependency>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let action_nodes: Vec<_> = nodes
        .iter()
        .filter(|n| matches!(n.kind, PlanNodeKind::SemanticAction(_)))
        .collect();

    // Detect overlapping targets without explicit ordering.
    let has_explicit = contract
        .semantics
        .as_ref()
        .and_then(|s| s.ordering.as_ref())
        .is_some_and(|o| matches!(o, ActionOrdering::Explicit { .. }));

    let mut target_counts: HashMap<&str, usize> = HashMap::new();
    for node in &action_nodes {
        if let PlanNodeKind::SemanticAction(action) = &node.kind {
            *target_counts.entry(action.target.as_str()).or_default() += 1;
        }
    }
    for (target, count) in target_counts {
        if count > 1 && !has_explicit {
            diagnostics.push(
                planning_error(
                    codes::INVALID_PLAN,
                    DiagnosticCategory::Semantic,
                    format!(
                        "multiple semantic actions target '{target}' without an explicit ordering declaration"
                    ),
                )
                .with_object_ref("semantics.ordering")
                .with_remediation(
                    "Declare semantics.ordering or avoid overlapping semantic action targets",
                ),
            );
        }
    }

    // Explicit ordering edges.
    if let Some(ActionOrdering::Explicit { order }) = contract
        .semantics
        .as_ref()
        .and_then(|s| s.ordering.as_ref())
    {
        for pair in order.windows(2) {
            push_edge(
                edges,
                pair[0].clone(),
                pair[1].clone(),
                DependencyReason::ExplicitOrder,
            );
        }
    }

    // Actions depend on their target's input interface.
    for node in &action_nodes {
        if let PlanNodeKind::SemanticAction(action) = &node.kind {
            if let Some((iface, _)) = action.target.split_once('.') {
                if contract.inputs.iter().any(|i| i.id == iface) {
                    push_edge(
                        edges,
                        iface.to_string(),
                        node.id.clone(),
                        DependencyReason::FieldRead,
                    );
                }
            }
        }
    }
}

fn add_expression_edges(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    registry: &RegistryDocument,
    field_index: &FieldIndex,
    edges: &mut Vec<PlanDependency>,
) {
    for node in nodes {
        let PlanNodeKind::Expression(expression) = &node.kind else {
            continue;
        };
        let analysis = check_expression(contract, expression, registry);
        let Some(ast) = analysis.ast else {
            continue;
        };
        for target in expr::collect_field_refs(&ast) {
            let source = dependency_source_for_target(contract, nodes, field_index, &target);
            if let Some(from) = source {
                push_edge(edges, from, node.id.clone(), DependencyReason::FieldRead);
            }
        }
    }
}

fn add_rule_field_edges(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    field_index: &FieldIndex,
    edges: &mut Vec<PlanDependency>,
) {
    for node in nodes {
        let PlanNodeKind::Rule(rule) = &node.kind else {
            continue;
        };
        let source = dependency_source_for_target(contract, nodes, field_index, &rule.target);
        if let Some(from) = source {
            push_edge(edges, from, node.id.clone(), DependencyReason::FieldRead);
        }
    }
}

fn dependency_source_for_target(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    field_index: &FieldIndex,
    target: &str,
) -> Option<String> {
    match field_index.resolve(target) {
        crate::validation::field_index::TargetResolution::Field(loc) => {
            if loc.is_input {
                return Some(loc.interface_id.clone());
            }
            // Output field: depend on lineage inputs or writing actions.
            find_writer_for_target(nodes, target)
                .or_else(|| lineage_input_for_output(contract, &loc.interface_id))
        }
        crate::validation::field_index::TargetResolution::Interface { id, is_input } => {
            if is_input {
                Some(id)
            } else {
                lineage_input_for_output(contract, &id)
            }
        }
        _ => None,
    }
}

fn lineage_input_for_output(contract: &TransformationContract, output_id: &str) -> Option<String> {
    contract
        .lineage
        .as_ref()?
        .mappings
        .iter()
        .find(|m| m.output == output_id)
        .and_then(|m| m.inputs.first().cloned())
}

fn find_writer_for_target(nodes: &[PlanNode], target: &str) -> Option<String> {
    nodes.iter().find_map(|node| {
        if let PlanNodeKind::SemanticAction(action) = &node.kind {
            if action.target == target {
                return Some(node.id.clone());
            }
        }
        None
    })
}

fn add_rule_phase_edges(nodes: &[PlanNode], edges: &mut Vec<PlanDependency>) {
    let pre: Vec<_> = nodes
        .iter()
        .filter(|n| {
            matches!(
                n.kind,
                PlanNodeKind::Rule(Rule {
                    phase: RulePhase::Precondition,
                    ..
                })
            )
        })
        .map(|n| n.id.as_str())
        .collect();
    let exec: Vec<_> = nodes
        .iter()
        .filter(|n| {
            matches!(
                n.kind,
                PlanNodeKind::Rule(Rule {
                    phase: RulePhase::Execution,
                    ..
                })
            )
        })
        .map(|n| n.id.as_str())
        .collect();
    let post: Vec<_> = nodes
        .iter()
        .filter(|n| {
            matches!(
                n.kind,
                PlanNodeKind::Rule(Rule {
                    phase: RulePhase::Postcondition,
                    ..
                })
            )
        })
        .map(|n| n.id.as_str())
        .collect();

    for p in &pre {
        for e in &exec {
            push_edge(
                edges,
                (*p).to_string(),
                (*e).to_string(),
                DependencyReason::RulePhase,
            );
        }
        for po in &post {
            push_edge(
                edges,
                (*p).to_string(),
                (*po).to_string(),
                DependencyReason::RulePhase,
            );
        }
    }
    for e in &exec {
        for po in &post {
            push_edge(
                edges,
                (*e).to_string(),
                (*po).to_string(),
                DependencyReason::RulePhase,
            );
        }
    }
}

fn add_interface_condition_edges(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    edges: &mut Vec<PlanDependency>,
) {
    let rule_ids: HashSet<_> = nodes
        .iter()
        .filter(|n| matches!(n.kind, PlanNodeKind::Rule(_)))
        .map(|n| n.id.as_str())
        .collect();

    for input in &contract.inputs {
        for cond in &input.preconditions {
            if rule_ids.contains(cond.rule.as_str()) {
                push_edge(
                    edges,
                    input.id.clone(),
                    cond.rule.clone(),
                    DependencyReason::InterfaceCondition,
                );
            }
        }
    }
    for output in &contract.outputs {
        for cond in &output.postconditions {
            if rule_ids.contains(cond.rule.as_str()) {
                // Postcondition rules depend on output being produced via lineage.
                if let Some(lineage) = contract.lineage.as_ref() {
                    if let Some(mapping) = lineage.mappings.iter().find(|m| m.output == output.id) {
                        for input in &mapping.inputs {
                            push_edge(
                                edges,
                                input.clone(),
                                cond.rule.clone(),
                                DependencyReason::InterfaceCondition,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn push_edge(edges: &mut Vec<PlanDependency>, from: String, to: String, reason: DependencyReason) {
    if from == to {
        return;
    }
    if !edges
        .iter()
        .any(|e| e.from == from && e.to == to && e.reason == reason)
    {
        edges.push(PlanDependency { from, to, reason });
    }
}

fn sort_dependencies(edges: &mut [PlanDependency]) {
    edges.sort_by(|a, b| (&a.from, &a.to, a.reason).cmp(&(&b.from, &b.to, b.reason)));
}
