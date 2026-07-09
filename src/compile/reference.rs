//! Reference backend compiler (`dtcs:reference`).

use super::CompileResult;
use crate::capability::{match_plan, reference_profile, EngineCapabilityDeclaration};
use crate::diagnostics::{codes, compilation_error, DiagnosticCategory};
use crate::model::RulePhase;
use crate::plan::{
    plan_as_contract, topological_order, validate, PlanNodeKind, TransformationPlan,
};

use super::compiler::Compiler;
use super::model::{ExecutionPlan, ExecutionStep, ExecutionStepKind, ExecutionTarget};

/// Reference compiler targeting the in-memory runtime.
#[derive(Debug, Clone, Copy, Default)]
pub struct ReferenceCompiler;

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
            step_index += 1;
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
                step_index += 1;
            }
        }

        for node_id in &order {
            let Some(node) = plan.nodes.iter().find(|n| &n.id == node_id) else {
                continue;
            };
            match &node.kind {
                PlanNodeKind::SemanticAction(action) => {
                    steps.push(ExecutionStep {
                        id: format!("step_{step_index}"),
                        kind: ExecutionStepKind::ApplyAction {
                            node_id: node.id.clone(),
                            action_id: action.action.clone(),
                            target: action.target.clone(),
                        },
                    });
                    step_index += 1;
                }
                PlanNodeKind::Expression(expression) => {
                    steps.push(ExecutionStep {
                        id: format!("step_{step_index}"),
                        kind: ExecutionStepKind::EvaluateExpression {
                            node_id: node.id.clone(),
                            expression_id: expression.id.clone(),
                        },
                    });
                    step_index += 1;
                }
                PlanNodeKind::Rule(rule) if rule.phase == RulePhase::Execution => {
                    steps.push(ExecutionStep {
                        id: format!("step_{step_index}"),
                        kind: ExecutionStepKind::ValidateRules {
                            phase: RulePhase::Execution,
                            rule_ids: vec![rule.id.clone()],
                        },
                    });
                    step_index += 1;
                }
                PlanNodeKind::Rule(_) => {}
            }
        }

        if let Some(lineage) = &plan.lineage {
            for mapping in &lineage.mappings {
                steps.push(ExecutionStep {
                    id: format!("step_{step_index}"),
                    kind: ExecutionStepKind::MaterializeOutput {
                        output_id: mapping.output.clone(),
                        input_ids: mapping.inputs.clone(),
                    },
                });
                step_index += 1;
            }
        }

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
            step_index += 1;
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
                step_index += 1;
            }
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

/// Compile using the reference backend and embedded profile.
#[must_use]
pub fn compile_reference(plan: &TransformationPlan) -> CompileResult {
    let capability = reference_profile();
    ReferenceCompiler.compile(plan, &capability)
}
