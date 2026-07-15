//! Reference in-memory runtime.

use std::collections::BTreeMap;

use crate::compile::{ExecutionPlan, ExecutionStepKind};
use crate::diagnostics::{codes, runtime_error, DiagnosticCategory};
use crate::model::RulePhase;
use crate::plan::{plan_as_contract, TransformationPlan};

use super::actions::{apply_action_to_rows, apply_dataset_action, is_dataset_action};
use super::expr::evaluate_expr;
use super::lineage::materialize_output;
use super::model::{Dataset, RuntimeInputs, RuntimeOutputs};
use super::rules::resolve_target;
use super::runtime_trait::Runtime;
use super::validate::validate_inputs;
use super::ExecuteResult;

/// Reference row-oriented runtime.
#[derive(Debug, Clone, Copy, Default)]
pub struct ReferenceRuntime;

impl Runtime for ReferenceRuntime {
    fn target_id(&self) -> &str {
        crate::capability::REFERENCE_ENGINE_ID
    }

    fn execute(&self, plan: &ExecutionPlan, inputs: &RuntimeInputs) -> ExecuteResult {
        let mut result = ExecuteResult::default();

        let validation = super::validate::validate_execution_plan(plan);
        result.diagnostics.extend(validation.diagnostics.clone());
        if !validation.is_valid() {
            return result;
        }

        if let Err(message) = validate_inputs(&plan.inputs, inputs) {
            result.diagnostics.push(
                runtime_error(
                    codes::INVALID_RUNTIME_INPUT,
                    DiagnosticCategory::Runtime,
                    message,
                )
                .with_object_ref("inputs"),
            );
            return result;
        }

        let contract = plan_as_contract_from_execution(plan);
        let mut workspaces: BTreeMap<String, Dataset> = BTreeMap::new();

        for input in &plan.inputs {
            if let Some(dataset) = inputs.get(&input.id) {
                workspaces.insert(input.id.clone(), dataset.clone());
            } else if !input.optional {
                result.diagnostics.push(
                    runtime_error(
                        codes::INVALID_RUNTIME_INPUT,
                        DiagnosticCategory::Runtime,
                        format!("missing required input '{}'", input.id),
                    )
                    .with_object_ref(&input.id),
                );
                return result;
            } else {
                workspaces.insert(input.id.clone(), Vec::new());
            }
        }

        for output in &plan.outputs {
            workspaces.entry(output.id.clone()).or_default();
        }

        let row_count = plan
            .inputs
            .iter()
            .find_map(|input| workspaces.get(&input.id).map(|dataset| dataset.len()))
            .unwrap_or(0);

        for step in &plan.steps {
            if let Err(message) = execute_step(step, plan, &contract, &mut workspaces, row_count) {
                let code = match &step.kind {
                    ExecutionStepKind::ValidateRules { phase, .. } => match phase {
                        RulePhase::Precondition => codes::PRECONDITION_VIOLATION,
                        RulePhase::Postcondition => codes::POSTCONDITION_VIOLATION,
                        RulePhase::Execution => codes::RUNTIME_ERROR,
                    },
                    _ => codes::RUNTIME_ERROR,
                };
                result.diagnostics.push(
                    runtime_error(code, DiagnosticCategory::Runtime, message)
                        .with_object_ref(&step.id),
                );
                return result;
            }
        }

        let mut outputs = RuntimeOutputs::new();
        for output in &plan.outputs {
            if let Some(dataset) = workspaces.get(&output.id) {
                outputs.insert(output.id.clone(), dataset.clone());
            }
        }
        result.outputs = Some(outputs);
        result
    }
}

fn plan_as_contract_from_execution(plan: &ExecutionPlan) -> crate::model::TransformationContract {
    let transformation_plan = TransformationPlan {
        identity: plan.identity.clone(),
        inputs: plan.inputs.clone(),
        outputs: plan.outputs.clone(),
        functions: Vec::new(),
        nodes: plan.nodes.clone(),
        dependencies: Vec::new(),
        lineage: plan.lineage.clone(),
        guarantees: plan.guarantees.clone(),
        metadata: None,
        versioning: None,
        extensions: Default::default(),
        findings: Vec::new(),
    };
    plan_as_contract(&transformation_plan)
}

fn execute_step(
    step: &crate::compile::ExecutionStep,
    plan: &ExecutionPlan,
    contract: &crate::model::TransformationContract,
    workspaces: &mut BTreeMap<String, Dataset>,
    row_count: usize,
) -> Result<(), String> {
    match &step.kind {
        ExecutionStepKind::ValidateRules { phase, rule_ids } => {
            for rule_id in rule_ids {
                let rule = find_rule(contract, rule_id)?;
                if rule.phase != *phase {
                    continue;
                }
                // Use current workspace size for the rule target so filters/projects
                // that shrink datasets do not over-iterate the original input count.
                let rule_row_count =
                    rule_target_row_count(workspaces, &rule.target).unwrap_or(row_count);
                for row_index in 0..rule_row_count {
                    let value = resolve_target(workspaces, &rule.target, row_index)?;
                    match crate::runtime::rules::evaluate_rule_outcome(
                        rule,
                        &value,
                        &rule.parameters,
                    )? {
                        crate::model::RuleOutcome::Satisfied => {}
                        crate::model::RuleOutcome::Indeterminate if rule.allow_indeterminate => {}
                        crate::model::RuleOutcome::Indeterminate => {
                            return Err(format!(
                                "rule '{}' returned indeterminate outcome",
                                rule.id
                            ));
                        }
                        crate::model::RuleOutcome::Violated => {
                            return Err(format!("rule '{}' violated", rule.id));
                        }
                    }
                }
            }
            Ok(())
        }
        ExecutionStepKind::ApplyAction {
            action_id,
            target,
            parameters,
            ..
        } => {
            if is_dataset_action(action_id) {
                return apply_dataset_action(action_id, target, parameters, workspaces);
            }
            let interface_ids: Vec<String> = workspaces.keys().cloned().collect();
            let qualified =
                super::model::parse_qualified_field_with_interfaces(target, &interface_ids)
                    .ok_or_else(|| format!("invalid action target '{target}'"))?;
            let rows = workspaces
                .get_mut(&qualified.interface_id)
                .ok_or_else(|| format!("unknown interface '{}'", qualified.interface_id))?;
            apply_action_to_rows(action_id, rows, &qualified.field_name)
        }
        ExecutionStepKind::EvaluateExpression { expression_id, .. } => {
            let expression = contract
                .expressions
                .iter()
                .find(|e| &e.id == expression_id)
                .ok_or_else(|| format!("unknown expression '{expression_id}'"))?;
            let body = expression
                .expr
                .as_deref()
                .ok_or_else(|| format!("expression '{expression_id}' has no body"))?;
            let ast =
                crate::analysis::expr::parse::parse_expression(body).map_err(|e| e.message)?;
            for row_index in 0..row_count {
                let _value = evaluate_expr(&ast, workspaces, row_index)?;
            }
            Ok(())
        }
        ExecutionStepKind::MaterializeOutput {
            output_id,
            input_ids,
        } => {
            let output = plan
                .outputs
                .iter()
                .find(|o| &o.id == output_id)
                .ok_or_else(|| format!("unknown output '{output_id}'"))?;
            let dataset = materialize_output(output, input_ids, &plan.inputs, workspaces)?;
            workspaces.insert(output_id.clone(), dataset);
            Ok(())
        }
    }
}

fn find_rule<'a>(
    contract: &'a crate::model::TransformationContract,
    rule_id: &str,
) -> Result<&'a crate::model::Rule, String> {
    contract
        .rules
        .iter()
        .find(|rule| rule.id == rule_id)
        .ok_or_else(|| format!("unknown rule '{rule_id}'"))
}

fn rule_target_row_count(workspaces: &BTreeMap<String, Dataset>, target: &str) -> Option<usize> {
    let interface_ids: Vec<String> = workspaces.keys().cloned().collect();
    let qualified = super::model::parse_qualified_field_with_interfaces(target, &interface_ids)?;
    workspaces.get(&qualified.interface_id).map(Dataset::len)
}
