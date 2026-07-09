//! Dependency graph construction for transformation plans (SPEC Ch 13 §8).

use std::collections::{HashMap, HashSet, VecDeque};

use crate::analysis::expr::{self, check_expression};
use crate::diagnostics::{codes, planning_error, Diagnostic, DiagnosticCategory};
use crate::model::{ActionOrdering, RegistryDocument, RulePhase, TransformationContract};
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
    add_field_write_edges(contract, nodes, &mut result.dependencies);
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

/// Count unique vertices in the dependency graph.
#[must_use]
pub fn vertex_count(contract: &TransformationContract, nodes: &[PlanNode]) -> usize {
    let mut vertices = HashSet::new();
    for input in &contract.inputs {
        vertices.insert(input.id.as_str());
    }
    for output in &contract.outputs {
        vertices.insert(output.id.as_str());
    }
    for node in nodes {
        vertices.insert(node.id.as_str());
    }
    vertices.len()
}

/// Returns `true` when the dependency graph is acyclic.
#[must_use]
pub fn is_acyclic(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    dependencies: &[PlanDependency],
) -> bool {
    if dependencies.is_empty() {
        return true;
    }
    let order = topological_order(contract, nodes, dependencies);
    order.len() == vertex_count(contract, nodes)
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
    if is_acyclic(contract, nodes, dependencies) {
        None
    } else {
        dependencies.first().map(|e| e.from.clone())
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

    let action_ids: HashSet<_> = action_nodes.iter().map(|n| n.id.as_str()).collect();

    let explicit_order = contract
        .semantics
        .as_ref()
        .and_then(|s| s.ordering.as_ref())
        .and_then(|o| match o {
            ActionOrdering::Explicit { order } => Some(order.as_slice()),
            ActionOrdering::Unordered => None,
        });

    if let Some(order) = explicit_order {
        let mut seen = HashSet::new();
        for (index, id) in order.iter().enumerate() {
            if !action_ids.contains(id.as_str()) {
                diagnostics.push(
                    planning_error(
                        codes::UNRESOLVED_PLAN_REFERENCE,
                        DiagnosticCategory::Reference,
                        format!("semantics.ordering references unknown semantic action '{id}'"),
                    )
                    .with_object_ref(format!("semantics.ordering.order[{index}]"))
                    .with_remediation(
                        "Reference only declared semantic action identifiers in semantics.ordering",
                    ),
                );
            } else if !seen.insert(id.as_str()) {
                diagnostics.push(
                    planning_error(
                        codes::INVALID_PLAN,
                        DiagnosticCategory::Semantic,
                        format!("semantics.ordering contains duplicate action id '{id}'"),
                    )
                    .with_object_ref(format!("semantics.ordering.order[{index}]")),
                );
            }
        }
        for id in &action_ids {
            if !seen.contains(id) {
                diagnostics.push(
                    planning_error(
                        codes::INVALID_PLAN,
                        DiagnosticCategory::Semantic,
                        format!("semantics.ordering is missing semantic action '{id}'"),
                    )
                    .with_object_ref("semantics.ordering")
                    .with_remediation("Include all semantic actions in the explicit order list"),
                );
            }
        }
        for pair in order.windows(2) {
            push_edge(
                edges,
                pair[0].clone(),
                pair[1].clone(),
                DependencyReason::ExplicitOrder,
            );
        }
    } else {
        let mut target_counts: HashMap<&str, usize> = HashMap::new();
        for node in &action_nodes {
            if let PlanNodeKind::SemanticAction(action) = &node.kind {
                *target_counts.entry(action.target.as_str()).or_default() += 1;
            }
        }
        for (target, count) in target_counts {
            if count > 1 {
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
    }

    let output_ids: HashSet<_> = contract.outputs.iter().map(|o| o.id.as_str()).collect();

    for node in &action_nodes {
        let PlanNodeKind::SemanticAction(action) = &node.kind else {
            continue;
        };
        let Some((iface, _)) = action.target.split_once('.') else {
            continue;
        };

        if contract.inputs.iter().any(|i| i.id == iface) {
            push_edge(
                edges,
                iface.to_string(),
                node.id.clone(),
                DependencyReason::FieldRead,
            );
        } else if output_ids.contains(iface) {
            for input in lineage_inputs_for_output(contract, iface) {
                push_edge(edges, input, node.id.clone(), DependencyReason::Lineage);
            }
        }
    }
}

fn add_field_write_edges(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    edges: &mut Vec<PlanDependency>,
) {
    let writers_by_target = writers_per_target(nodes);
    let explicit_order = contract
        .semantics
        .as_ref()
        .and_then(|s| s.ordering.as_ref())
        .and_then(|o| match o {
            ActionOrdering::Explicit { order } => Some(order.as_slice()),
            ActionOrdering::Unordered => None,
        });

    for (target, writers) in writers_by_target {
        if writers.len() < 2 {
            continue;
        }
        let ordered: Vec<_> = if let Some(order) = explicit_order {
            order
                .iter()
                .filter(|id| writers.contains(id))
                .cloned()
                .collect()
        } else {
            writers
        };
        for pair in ordered.windows(2) {
            push_edge(
                edges,
                pair[0].clone(),
                pair[1].clone(),
                DependencyReason::FieldWrite,
            );
        }
        let _ = target;
    }
}

fn writers_per_target(nodes: &[PlanNode]) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes {
        if let PlanNodeKind::SemanticAction(action) = &node.kind {
            map.entry(action.target.clone())
                .or_default()
                .push(node.id.clone());
        }
    }
    for ids in map.values_mut() {
        ids.sort();
    }
    map
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
            for from in dependency_sources_for_target(contract, nodes, field_index, &target) {
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
        for from in dependency_sources_for_target(contract, nodes, field_index, &rule.target) {
            push_edge(edges, from, node.id.clone(), DependencyReason::FieldRead);
        }
    }
}

fn dependency_sources_for_target(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    field_index: &FieldIndex,
    target: &str,
) -> Vec<String> {
    match field_index.resolve(target) {
        crate::validation::field_index::TargetResolution::Field(loc) => {
            if loc.is_input {
                return vec![loc.interface_id.clone()];
            }
            if let Some(writer) = last_writer_for_target(contract, nodes, target) {
                return vec![writer];
            }
            lineage_inputs_for_output(contract, &loc.interface_id)
        }
        crate::validation::field_index::TargetResolution::Interface { id, is_input } => {
            if is_input {
                vec![id]
            } else {
                lineage_inputs_for_output(contract, &id)
            }
        }
        _ => Vec::new(),
    }
}

fn lineage_inputs_for_output(contract: &TransformationContract, output_id: &str) -> Vec<String> {
    contract
        .lineage
        .as_ref()
        .map(|lineage| {
            lineage
                .mappings
                .iter()
                .find(|m| m.output == output_id)
                .map(|m| m.inputs.clone())
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

fn last_writer_for_target(
    contract: &TransformationContract,
    nodes: &[PlanNode],
    target: &str,
) -> Option<String> {
    let writers: HashSet<_> = nodes
        .iter()
        .filter_map(|node| {
            if let PlanNodeKind::SemanticAction(action) = &node.kind {
                if action.target == target {
                    return Some(node.id.as_str());
                }
            }
            None
        })
        .collect();

    if writers.is_empty() {
        return None;
    }

    if let Some(ActionOrdering::Explicit { order }) = contract
        .semantics
        .as_ref()
        .and_then(|s| s.ordering.as_ref())
    {
        for id in order.iter().rev() {
            if writers.contains(id.as_str()) {
                return Some(id.clone());
            }
        }
        return None;
    }

    if writers.len() == 1 {
        writers.into_iter().next().map(str::to_string)
    } else {
        None
    }
}

fn add_rule_phase_edges(nodes: &[PlanNode], edges: &mut Vec<PlanDependency>) {
    let rules: Vec<_> = nodes
        .iter()
        .filter_map(|n| {
            if let PlanNodeKind::Rule(rule) = &n.kind {
                Some((n.id.as_str(), rule.phase, rule.target.as_str()))
            } else {
                None
            }
        })
        .collect();

    for (id_a, phase_a, target_a) in &rules {
        for (id_b, phase_b, target_b) in &rules {
            if id_a == id_b || target_a != target_b {
                continue;
            }
            let ordered = matches!(
                (phase_a, phase_b),
                (RulePhase::Precondition, RulePhase::Execution)
                    | (RulePhase::Precondition, RulePhase::Postcondition)
                    | (RulePhase::Execution, RulePhase::Postcondition)
            );
            if ordered {
                push_edge(
                    edges,
                    (*id_a).to_string(),
                    (*id_b).to_string(),
                    DependencyReason::RulePhase,
                );
            }
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
