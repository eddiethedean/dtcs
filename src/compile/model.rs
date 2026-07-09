//! Execution plan IR types (SPEC Chapter 15).

use serde::{Deserialize, Serialize};

use crate::model::{Input, Lineage, Output, RulePhase};
use crate::plan::{PlanGuarantees, PlanIdentity, PlanNode};

/// Execution engine target metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionTarget {
    /// Target engine identifier.
    pub engine_id: String,
    /// Engine version used for compilation.
    pub engine_version: String,
    /// Capability declaration version.
    pub capability_version: String,
}

/// Backend-specific execution plan (SPEC Ch 15 §5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPlan {
    /// Target execution engine.
    pub target: ExecutionTarget,
    /// Originating contract identity.
    pub identity: PlanIdentity,
    /// Declared inputs.
    pub inputs: Vec<Input>,
    /// Declared outputs.
    pub outputs: Vec<Output>,
    /// Semantic nodes from the originating transformation plan.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<PlanNode>,
    /// Ordered execution steps.
    pub steps: Vec<ExecutionStep>,
    /// Contractual guarantees.
    pub guarantees: PlanGuarantees,
    /// Dataset lineage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Lineage>,
}

/// A single execution step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionStep {
    /// Stable step identifier.
    pub id: String,
    /// Step payload.
    pub kind: ExecutionStepKind,
}

/// Execution step kinds for the reference backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ExecutionStepKind {
    /// Evaluate rules at a given phase.
    ValidateRules {
        /// Rule evaluation phase.
        phase: RulePhase,
        /// Rule instance identifiers.
        rule_ids: Vec<String>,
    },
    /// Apply a semantic action.
    ApplyAction {
        /// Plan node identifier.
        node_id: String,
        /// Semantic action registry identifier.
        action_id: String,
        /// Qualified target field.
        target: String,
    },
    /// Evaluate an expression declaration.
    EvaluateExpression {
        /// Plan node identifier.
        node_id: String,
        /// Expression instance identifier.
        expression_id: String,
    },
    /// Materialize an output dataset from lineage inputs.
    MaterializeOutput {
        /// Output interface identifier.
        output_id: String,
        /// Contributing input identifiers.
        input_ids: Vec<String>,
    },
}
