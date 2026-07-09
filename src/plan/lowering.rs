//! COM → Transformation Plan lowering (SPEC Ch 13 §5–6).

use crate::analysis::{self, AnalysisReport};
use crate::diagnostics::{codes, planning_error, Diagnostic, DiagnosticCategory, DiagnosticReport};
use crate::model::{RegistryDocument, TransformationContract};
use crate::registry;

use super::graph;
use super::model::{
    InterfaceConditionRef, PlanGuarantees, PlanIdentity, PlanNode, PlanNodeKind, TransformationPlan,
};
use super::validate;

/// Result of lowering a contract to a plan.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanResult {
    /// Lowered plan when lowering succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<TransformationPlan>,
    /// Diagnostics from lowering and plan validation.
    pub diagnostics: Vec<Diagnostic>,
}

impl PlanResult {
    /// Returns `true` when no error-level diagnostics are present.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.diagnostics.iter().any(|d| d.severity.is_error())
    }
}

/// Lower a validated contract into a transformation plan.
#[must_use]
pub fn lower(
    contract: &TransformationContract,
    registry_doc: Option<&RegistryDocument>,
    analysis: Option<&AnalysisReport>,
) -> PlanResult {
    let registry_doc = registry_doc.unwrap_or(registry::default_registry());
    let mut result = PlanResult::default();

    if let Some(err) = check_completeness(contract) {
        result.diagnostics.push(err);
        return result;
    }

    let owned_analysis;
    let analysis_ref = match analysis {
        Some(report) => report,
        None => {
            owned_analysis = analysis::check_contract(contract, Some(registry_doc));
            &owned_analysis
        }
    };

    let mut nodes = build_nodes(contract);
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let graph_result = graph::build(contract, &nodes, registry_doc);
    result.diagnostics.extend(graph_result.diagnostics);

    if result.diagnostics.iter().any(|d| d.severity.is_error()) {
        return result;
    }

    let guarantees = build_guarantees(contract);
    let findings = analysis_ref.findings.clone();

    let plan = TransformationPlan {
        identity: PlanIdentity {
            dtcs_version: contract.dtcs_version.clone(),
            id: contract.id.clone(),
            name: contract.name.clone(),
            version: contract.version.clone(),
        },
        inputs: contract.inputs.clone(),
        outputs: contract.outputs.clone(),
        functions: contract.functions.clone(),
        nodes,
        dependencies: graph_result.dependencies,
        lineage: contract.lineage.clone(),
        guarantees,
        metadata: contract.metadata.clone(),
        versioning: contract.versioning.clone(),
        extensions: contract.extensions.clone(),
        findings,
    };

    let validation = validate::validate_with_registry(&plan, registry_doc);
    result.diagnostics.extend(validation.diagnostics);

    if result.diagnostics.iter().any(|d| d.severity.is_error()) {
        return result;
    }

    result.plan = Some(plan);
    result
}

fn check_completeness(contract: &TransformationContract) -> Option<Diagnostic> {
    if contract.inputs.is_empty() {
        return Some(
            planning_error(
                codes::INCOMPLETE_PLAN,
                DiagnosticCategory::Structure,
                "plan requires at least one input",
            )
            .with_object_ref("inputs"),
        );
    }
    if contract.outputs.is_empty() {
        return Some(
            planning_error(
                codes::INCOMPLETE_PLAN,
                DiagnosticCategory::Structure,
                "plan requires at least one output",
            )
            .with_object_ref("outputs"),
        );
    }
    let lineage = contract.lineage.as_ref()?;
    let mapped: std::collections::HashSet<_> =
        lineage.mappings.iter().map(|m| m.output.as_str()).collect();
    for output in &contract.outputs {
        if !mapped.contains(output.id.as_str()) {
            return Some(
                planning_error(
                    codes::INCOMPLETE_PLAN,
                    DiagnosticCategory::Structure,
                    format!("output '{}' is not covered by lineage mappings", output.id),
                )
                .with_object_ref("lineage.mappings"),
            );
        }
    }
    None
}

fn build_nodes(contract: &TransformationContract) -> Vec<PlanNode> {
    let mut nodes = Vec::new();
    for action in &contract.semantic_actions {
        nodes.push(PlanNode {
            id: action.id.clone(),
            kind: PlanNodeKind::SemanticAction(action.clone()),
            object_ref: format!("semanticActions.{}", action.id),
        });
    }
    for expression in &contract.expressions {
        nodes.push(PlanNode {
            id: expression.id.clone(),
            kind: PlanNodeKind::Expression(expression.clone()),
            object_ref: format!("expressions.{}", expression.id),
        });
    }
    for rule in &contract.rules {
        nodes.push(PlanNode {
            id: rule.id.clone(),
            kind: PlanNodeKind::Rule(rule.clone()),
            object_ref: format!("rules.{}", rule.id),
        });
    }
    nodes
}

fn build_guarantees(contract: &TransformationContract) -> PlanGuarantees {
    let mut input_preconditions = Vec::new();
    for input in &contract.inputs {
        for cond in &input.preconditions {
            input_preconditions.push(InterfaceConditionRef {
                interface_id: input.id.clone(),
                is_input: true,
                rule_id: cond.rule.clone(),
            });
        }
    }
    let mut output_postconditions = Vec::new();
    for output in &contract.outputs {
        for cond in &output.postconditions {
            output_postconditions.push(InterfaceConditionRef {
                interface_id: output.id.clone(),
                is_input: false,
                rule_id: cond.rule.clone(),
            });
        }
    }
    PlanGuarantees {
        semantics: contract.semantics.clone(),
        input_preconditions,
        output_postconditions,
    }
}

/// Lower and return a diagnostic report (for callers that only need validation output).
#[must_use]
pub fn lower_report(
    contract: &TransformationContract,
    registry_doc: Option<&RegistryDocument>,
    analysis: Option<&AnalysisReport>,
) -> DiagnosticReport {
    let result = lower(contract, registry_doc, analysis);
    DiagnosticReport {
        diagnostics: result.diagnostics,
    }
}
