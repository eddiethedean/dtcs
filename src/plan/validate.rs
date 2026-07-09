//! Transformation plan validation (SPEC Ch 13 §11).

use crate::analysis::expr::check_expression;
use crate::diagnostics::{codes, planning_error, DiagnosticCategory, DiagnosticReport};
use crate::model::{RegistryDocument, RulePhase};
use crate::registry;
use crate::validation::field_index::FieldIndex;

use super::graph;
use super::model::{PlanNodeKind, TransformationPlan};

/// Validate a transformation plan.
#[must_use]
pub fn validate(plan: &TransformationPlan) -> DiagnosticReport {
    validate_with_registry(plan, registry::default_registry())
}

/// Validate a transformation plan with a registry catalog.
#[must_use]
pub fn validate_with_registry(
    plan: &TransformationPlan,
    registry_doc: &RegistryDocument,
) -> DiagnosticReport {
    let mut report = DiagnosticReport::default();
    let contract = plan_as_contract(plan);

    check_completeness(plan, &mut report);
    check_unique_node_ids(plan, &mut report);
    check_dependency_integrity(plan, &mut report);
    check_acyclic(plan, &mut report);
    check_lineage(plan, &mut report);
    check_types(plan, &contract, registry_doc, &mut report);
    check_guarantees(plan, &mut report);

    report
}

fn plan_as_contract(plan: &TransformationPlan) -> crate::model::TransformationContract {
    let mut semantic_actions = Vec::new();
    let mut expressions = Vec::new();
    let mut rules = Vec::new();
    for node in &plan.nodes {
        match &node.kind {
            PlanNodeKind::SemanticAction(a) => semantic_actions.push(a.clone()),
            PlanNodeKind::Expression(e) => expressions.push(e.clone()),
            PlanNodeKind::Rule(r) => rules.push(r.clone()),
        }
    }
    crate::model::TransformationContract {
        dtcs_version: plan.identity.dtcs_version.clone(),
        id: plan.identity.id.clone(),
        name: plan.identity.name.clone(),
        version: plan.identity.version.clone(),
        metadata: plan.metadata.clone(),
        inputs: plan.inputs.clone(),
        outputs: plan.outputs.clone(),
        semantic_actions,
        expressions,
        functions: plan.functions.clone(),
        rules,
        lineage: plan.lineage.clone(),
        versioning: plan.versioning.clone(),
        semantics: plan.guarantees.semantics.clone(),
        extensions: plan.extensions.clone(),
    }
}

fn check_completeness(plan: &TransformationPlan, report: &mut DiagnosticReport) {
    if plan.inputs.is_empty() {
        report.push(
            planning_error(
                codes::INCOMPLETE_PLAN,
                DiagnosticCategory::Structure,
                "plan is missing inputs",
            )
            .with_object_ref("inputs"),
        );
    }
    if plan.outputs.is_empty() {
        report.push(
            planning_error(
                codes::INCOMPLETE_PLAN,
                DiagnosticCategory::Structure,
                "plan is missing outputs",
            )
            .with_object_ref("outputs"),
        );
    }
}

fn check_unique_node_ids(plan: &TransformationPlan, report: &mut DiagnosticReport) {
    let mut seen = std::collections::HashSet::new();
    for node in &plan.nodes {
        if !seen.insert(node.id.as_str()) {
            report.push(
                planning_error(
                    codes::INVALID_PLAN,
                    DiagnosticCategory::Structure,
                    format!("duplicate plan node id '{}'", node.id),
                )
                .with_object_ref(node.object_ref.clone()),
            );
        }
    }
}

fn check_dependency_integrity(plan: &TransformationPlan, report: &mut DiagnosticReport) {
    let known: std::collections::HashSet<String> = plan
        .inputs
        .iter()
        .map(|i| i.id.clone())
        .chain(plan.outputs.iter().map(|o| o.id.clone()))
        .chain(plan.nodes.iter().map(|n| n.id.clone()))
        .collect();

    for (index, edge) in plan.dependencies.iter().enumerate() {
        if !known.contains(&edge.from) {
            report.push(
                planning_error(
                    codes::UNRESOLVED_PLAN_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!(
                        "dependency source '{}' is not a known node or interface",
                        edge.from
                    ),
                )
                .with_object_ref(format!("dependencies[{index}].from")),
            );
        }
        if !known.contains(&edge.to) {
            report.push(
                planning_error(
                    codes::UNRESOLVED_PLAN_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!(
                        "dependency target '{}' is not a known node or interface",
                        edge.to
                    ),
                )
                .with_object_ref(format!("dependencies[{index}].to")),
            );
        }
    }
}

fn check_acyclic(plan: &TransformationPlan, report: &mut DiagnosticReport) {
    let contract = plan_as_contract(plan);
    let order = graph::topological_order(&contract, &plan.nodes, &plan.dependencies);
    if !plan.dependencies.is_empty() && order.is_empty() {
        report.push(
            planning_error(
                codes::CYCLIC_DEPENDENCY,
                DiagnosticCategory::Semantic,
                "plan dependency graph contains a cycle",
            )
            .with_object_ref("dependencies"),
        );
    }
}

fn check_lineage(plan: &TransformationPlan, report: &mut DiagnosticReport) {
    let Some(lineage) = plan.lineage.as_ref() else {
        for output in &plan.outputs {
            report.push(
                planning_error(
                    codes::INCOMPLETE_PLAN,
                    DiagnosticCategory::Structure,
                    format!("output '{}' has no lineage mapping", output.id),
                )
                .with_object_ref("lineage"),
            );
        }
        return;
    };

    let input_ids: std::collections::HashSet<_> =
        plan.inputs.iter().map(|i| i.id.as_str()).collect();
    let output_ids: std::collections::HashSet<_> =
        plan.outputs.iter().map(|o| o.id.as_str()).collect();
    let mapped: std::collections::HashSet<_> =
        lineage.mappings.iter().map(|m| m.output.as_str()).collect();

    for output in &plan.outputs {
        if !mapped.contains(output.id.as_str()) {
            report.push(
                planning_error(
                    codes::INCOMPLETE_PLAN,
                    DiagnosticCategory::Structure,
                    format!("output '{}' is not covered by lineage mappings", output.id),
                )
                .with_object_ref("lineage.mappings"),
            );
        }
    }

    for mapping in &lineage.mappings {
        if !output_ids.contains(mapping.output.as_str()) {
            report.push(
                planning_error(
                    codes::UNRESOLVED_PLAN_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("lineage references unknown output '{}'", mapping.output),
                )
                .with_object_ref("lineage.mappings"),
            );
        }
        for input in &mapping.inputs {
            if !input_ids.contains(input.as_str()) {
                report.push(
                    planning_error(
                        codes::UNRESOLVED_PLAN_REFERENCE,
                        DiagnosticCategory::Reference,
                        format!("lineage references unknown input '{input}'"),
                    )
                    .with_object_ref("lineage.mappings"),
                );
            }
        }
    }
}

fn check_types(
    plan: &TransformationPlan,
    contract: &crate::model::TransformationContract,
    registry_doc: &RegistryDocument,
    report: &mut DiagnosticReport,
) {
    let field_index = FieldIndex::from_contract(contract);

    for node in &plan.nodes {
        match &node.kind {
            PlanNodeKind::Expression(expression) => {
                let analysis = check_expression(contract, expression, registry_doc);
                report.diagnostics.extend(analysis.diagnostics);

                if let (Some(declared), Some(inferred)) = (
                    expression.type_name.as_deref(),
                    analysis.inferred_type.as_ref(),
                ) {
                    if let Ok(declared_type) = crate::model::parse_logical_type(declared) {
                        if declared_type != *inferred {
                            report.push(
                                planning_error(
                                    codes::PLAN_TYPE_MISMATCH,
                                    DiagnosticCategory::Type,
                                    format!(
                                        "expression declared type '{declared}' does not match inferred type"
                                    ),
                                )
                                .with_object_ref(node.object_ref.clone()),
                            );
                        }
                    }
                }
            }
            PlanNodeKind::SemanticAction(action) => {
                if matches!(
                    field_index.resolve(&action.target),
                    crate::validation::field_index::TargetResolution::NotFound
                ) {
                    report.push(
                        planning_error(
                            codes::UNRESOLVED_PLAN_REFERENCE,
                            DiagnosticCategory::Reference,
                            format!(
                                "semantic action target '{}' could not be resolved",
                                action.target
                            ),
                        )
                        .with_object_ref(node.object_ref.clone()),
                    );
                }
            }
            PlanNodeKind::Rule(rule) => {
                if matches!(
                    field_index.resolve(&rule.target),
                    crate::validation::field_index::TargetResolution::NotFound
                ) {
                    report.push(
                        planning_error(
                            codes::UNRESOLVED_PLAN_REFERENCE,
                            DiagnosticCategory::Reference,
                            format!("rule target '{}' could not be resolved", rule.target),
                        )
                        .with_object_ref(node.object_ref.clone()),
                    );
                }
                // Phase must be valid enum (already typed); ensure rule id in interface conditions exists.
                let _ = rule.phase;
                if !matches!(
                    rule.phase,
                    RulePhase::Precondition | RulePhase::Execution | RulePhase::Postcondition
                ) {
                    report.push(
                        planning_error(
                            codes::INVALID_PLAN,
                            DiagnosticCategory::Semantic,
                            format!("rule '{}' has invalid phase", rule.id),
                        )
                        .with_object_ref(node.object_ref.clone()),
                    );
                }
            }
        }
    }
}

fn check_guarantees(plan: &TransformationPlan, report: &mut DiagnosticReport) {
    let rule_ids: std::collections::HashSet<_> = plan.nodes.iter().map(|n| n.id.as_str()).collect();

    for cond in &plan.guarantees.input_preconditions {
        if !rule_ids.contains(cond.rule_id.as_str()) {
            report.push(
                planning_error(
                    codes::UNRESOLVED_PLAN_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!(
                        "input precondition references unknown rule '{}'",
                        cond.rule_id
                    ),
                )
                .with_object_ref(format!("inputs.{}.preconditions", cond.interface_id)),
            );
        }
    }
    for cond in &plan.guarantees.output_postconditions {
        if !rule_ids.contains(cond.rule_id.as_str()) {
            report.push(
                planning_error(
                    codes::UNRESOLVED_PLAN_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!(
                        "output postcondition references unknown rule '{}'",
                        cond.rule_id
                    ),
                )
                .with_object_ref(format!("outputs.{}.postconditions", cond.interface_id)),
            );
        }
    }
}
