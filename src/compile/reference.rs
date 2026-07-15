//! Reference backend compiler (`dtcs:reference`).

use super::CompileResult;
use crate::capability::{match_plan, reference_profile, EngineCapabilityDeclaration};
use crate::diagnostics::{codes, compilation_error, Diagnostic, DiagnosticCategory};
use crate::model::RulePhase;
use crate::plan::{
    plan_as_contract, topological_order, validate, PlanNode, PlanNodeKind, TransformationPlan,
};
use crate::runtime::parse_qualified_field_with_interfaces;

use super::compiler::Compiler;
use super::model::{ExecutionPlan, ExecutionStep, ExecutionStepKind, ExecutionTarget};

/// Reference compiler targeting the in-memory runtime.
#[derive(Debug, Clone, Copy, Default)]
pub struct ReferenceCompiler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterfaceKind {
    Input,
    Output,
}

impl Compiler for ReferenceCompiler {
    fn target_id(&self) -> &str {
        crate::capability::REFERENCE_ENGINE_ID
    }

    fn compile(
        &self,
        plan: &TransformationPlan,
        capability: &EngineCapabilityDeclaration,
    ) -> CompileResult {
        let mut result = CompileResult::default();

        let plan_validation = validate(plan);
        result
            .diagnostics
            .extend(plan_validation.diagnostics.clone());
        if !plan_validation.is_valid() {
            return result;
        }

        let match_report = match_plan(plan, capability);
        if !match_report.is_valid() {
            result.diagnostics.extend(match_report.diagnostics);
            return result;
        }

        let contract = plan_as_contract(plan);
        let order = topological_order(&contract, &plan.nodes, &plan.dependencies);

        let mut steps = Vec::new();
        let mut step_index = 0usize;

        push_precondition_steps(plan, &mut steps, &mut step_index);
        push_ordered_node_steps(
            plan,
            &order,
            InterfaceKind::Input,
            &mut steps,
            &mut step_index,
            &mut result.diagnostics,
        );
        push_materialize_steps(plan, &mut steps, &mut step_index);
        push_ordered_node_steps(
            plan,
            &order,
            InterfaceKind::Output,
            &mut steps,
            &mut step_index,
            &mut result.diagnostics,
        );
        push_postcondition_steps(plan, &mut steps, &mut step_index);

        if result.diagnostics.iter().any(|d| d.severity.is_error()) {
            return result;
        }

        if steps.is_empty() {
            result.diagnostics.push(
                compilation_error(
                    codes::COMPILATION_FAILED,
                    DiagnosticCategory::Semantic,
                    "compilation produced no execution steps",
                )
                .with_object_ref("steps"),
            );
            return result;
        }

        let execution_plan = ExecutionPlan {
            target: ExecutionTarget {
                engine_id: capability.engine_id.clone(),
                engine_version: capability.engine_version.clone(),
                capability_version: capability.capability_version.clone(),
            },
            identity: plan.identity.clone(),
            inputs: plan.inputs.clone(),
            outputs: plan.outputs.clone(),
            nodes: plan.nodes.clone(),
            steps,
            guarantees: plan.guarantees.clone(),
            lineage: plan.lineage.clone(),
        };

        let validation = super::validate::validate(&execution_plan);
        if !validation.is_valid() {
            result.diagnostics.extend(validation.diagnostics);
            return result;
        }

        result.plan = Some(execution_plan);
        result
    }
}

fn push_precondition_steps(
    plan: &TransformationPlan,
    steps: &mut Vec<ExecutionStep>,
    step_index: &mut usize,
) {
    let precondition_rules: Vec<String> = plan
        .nodes
        .iter()
        .filter_map(|node| {
            if let PlanNodeKind::Rule(rule) = &node.kind {
                if rule.phase == RulePhase::Precondition {
                    return Some(rule.id.clone());
                }
            }
            None
        })
        .collect();
    if !precondition_rules.is_empty() {
        steps.push(ExecutionStep {
            id: format!("step_{step_index}"),
            kind: ExecutionStepKind::ValidateRules {
                phase: RulePhase::Precondition,
                rule_ids: precondition_rules,
            },
        });
        *step_index += 1;
    }

    for input in &plan.inputs {
        for precondition in &input.preconditions {
            steps.push(ExecutionStep {
                id: format!("step_{step_index}"),
                kind: ExecutionStepKind::ValidateRules {
                    phase: RulePhase::Precondition,
                    rule_ids: vec![precondition.rule.clone()],
                },
            });
            *step_index += 1;
        }
    }
}

fn push_materialize_steps(
    plan: &TransformationPlan,
    steps: &mut Vec<ExecutionStep>,
    step_index: &mut usize,
) {
    if let Some(lineage) = &plan.lineage {
        for mapping in &lineage.mappings {
            steps.push(ExecutionStep {
                id: format!("step_{step_index}"),
                kind: ExecutionStepKind::MaterializeOutput {
                    output_id: mapping.output.clone(),
                    input_ids: mapping.inputs.clone(),
                },
            });
            *step_index += 1;
        }
    }
}

fn push_postcondition_steps(
    plan: &TransformationPlan,
    steps: &mut Vec<ExecutionStep>,
    step_index: &mut usize,
) {
    let postcondition_rules: Vec<String> = plan
        .nodes
        .iter()
        .filter_map(|node| {
            if let PlanNodeKind::Rule(rule) = &node.kind {
                if rule.phase == RulePhase::Postcondition {
                    return Some(rule.id.clone());
                }
            }
            None
        })
        .collect();
    if !postcondition_rules.is_empty() {
        steps.push(ExecutionStep {
            id: format!("step_{step_index}"),
            kind: ExecutionStepKind::ValidateRules {
                phase: RulePhase::Postcondition,
                rule_ids: postcondition_rules,
            },
        });
        *step_index += 1;
    }

    for output in &plan.outputs {
        for postcondition in &output.postconditions {
            steps.push(ExecutionStep {
                id: format!("step_{step_index}"),
                kind: ExecutionStepKind::ValidateRules {
                    phase: RulePhase::Postcondition,
                    rule_ids: vec![postcondition.rule.clone()],
                },
            });
            *step_index += 1;
        }
    }
}

fn push_ordered_node_steps(
    plan: &TransformationPlan,
    order: &[String],
    interface_kind: InterfaceKind,
    steps: &mut Vec<ExecutionStep>,
    step_index: &mut usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for node_id in order {
        let Some(node) = plan.nodes.iter().find(|n| &n.id == node_id) else {
            continue;
        };
        match node_step(plan, node, interface_kind, *step_index) {
            Ok(Some(step)) => {
                steps.push(step);
                *step_index += 1;
            }
            Ok(None) => {}
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }
}

fn node_step(
    plan: &TransformationPlan,
    node: &PlanNode,
    interface_kind: InterfaceKind,
    step_index: usize,
) -> Result<Option<ExecutionStep>, Diagnostic> {
    match &node.kind {
        PlanNodeKind::SemanticAction(action) => {
            let kind = target_interface(plan, &action.target).ok_or_else(|| {
                compilation_error(
                    codes::COMPILATION_FAILED,
                    DiagnosticCategory::Semantic,
                    format!(
                        "cannot resolve semantic action target '{}' to an input or output interface",
                        action.target
                    ),
                )
                .with_object_ref(format!("semanticActions.{}", node.id))
            })?;
            if kind != interface_kind {
                return Ok(None);
            }
            Ok(Some(ExecutionStep {
                id: format!("step_{step_index}"),
                kind: ExecutionStepKind::ApplyAction {
                    node_id: node.id.clone(),
                    action_id: action.action.clone(),
                    target: action.target.clone(),
                    parameters: action.parameters.clone(),
                },
            }))
        }
        PlanNodeKind::Rule(rule) if rule.phase == RulePhase::Execution => {
            let kind = target_interface(plan, &rule.target).ok_or_else(|| {
                compilation_error(
                    codes::COMPILATION_FAILED,
                    DiagnosticCategory::Semantic,
                    format!(
                        "cannot resolve rule target '{}' to an input or output interface",
                        rule.target
                    ),
                )
                .with_object_ref(format!("rules.{}", node.id))
            })?;
            if kind != interface_kind {
                return Ok(None);
            }
            Ok(Some(ExecutionStep {
                id: format!("step_{step_index}"),
                kind: ExecutionStepKind::ValidateRules {
                    phase: RulePhase::Execution,
                    rule_ids: vec![rule.id.clone()],
                },
            }))
        }
        // Expression write targets are not defined in COM yet; omit until modeled.
        PlanNodeKind::Expression(_) | PlanNodeKind::Rule(_) => Ok(None),
    }
}

fn target_interface(plan: &TransformationPlan, target: &str) -> Option<InterfaceKind> {
    let interface_ids: Vec<String> = plan
        .inputs
        .iter()
        .map(|i| i.id.clone())
        .chain(plan.outputs.iter().map(|o| o.id.clone()))
        .collect();
    let interface_id = parse_qualified_field_with_interfaces(target, &interface_ids)
        .map(|qualified| qualified.interface_id)
        .or_else(|| {
            if interface_ids.iter().any(|id| id == target) {
                Some(target.to_string())
            } else {
                None
            }
        })?;
    if plan.inputs.iter().any(|input| input.id == interface_id) {
        Some(InterfaceKind::Input)
    } else if plan.outputs.iter().any(|output| output.id == interface_id) {
        Some(InterfaceKind::Output)
    } else {
        None
    }
}

/// Compile using the reference backend and embedded profile.
#[must_use]
pub fn compile_reference(plan: &TransformationPlan) -> CompileResult {
    let capability = reference_profile();
    ReferenceCompiler.compile(plan, &capability)
}
