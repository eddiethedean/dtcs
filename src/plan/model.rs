//! Transformation Plan IR types (SPEC Chapter 13).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::analysis::AnalysisFinding;
use crate::model::{
    Expression, Function, Input, Lineage, Metadata, Output, Rule, SemanticAction,
    TransformationSemantics, Versioning,
};

/// Contract identity carried into the plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanIdentity {
    /// DTCS specification version.
    pub dtcs_version: String,
    /// Contract identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Contract version.
    pub version: String,
}

/// A unified plan step node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanNode {
    /// Stable node identifier (matches COM object id).
    pub id: String,
    /// Node kind and payload.
    pub kind: PlanNodeKind,
    /// COM object reference path (for example `semanticActions.normalize_email`).
    pub object_ref: String,
}

/// Plan node payload kinds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PlanNodeKind {
    /// Semantic action step.
    SemanticAction(SemanticAction),
    /// Expression step.
    Expression(Expression),
    /// Rule step.
    Rule(Rule),
}

/// Reason for a dependency edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DependencyReason {
    /// Output depends on lineage input.
    Lineage,
    /// Node reads a schema field.
    FieldRead,
    /// Node writes after a prior writer on the same field.
    FieldWrite,
    /// Explicit semantics.ordering between actions.
    ExplicitOrder,
    /// Rule phase ordering (precondition → execution → postcondition).
    RulePhase,
    /// Interface precondition/postcondition linkage.
    InterfaceCondition,
}

/// Directed dependency edge (`from` must precede `to`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDependency {
    /// Source node or interface identifier.
    pub from: String,
    /// Target node or interface identifier.
    pub to: String,
    /// Why the dependency exists.
    pub reason: DependencyReason,
}

/// Reference to an interface condition rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceConditionRef {
    /// Declaring interface identifier.
    pub interface_id: String,
    /// Whether the interface is an input.
    pub is_input: bool,
    /// Referenced rule instance id.
    pub rule_id: String,
}

/// Contractual guarantees preserved in the plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanGuarantees {
    /// Transformation semantics (determinism, purity, ordering, side effects).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantics: Option<TransformationSemantics>,
    /// Input interface preconditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_preconditions: Vec<InterfaceConditionRef>,
    /// Output interface postconditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_postconditions: Vec<InterfaceConditionRef>,
}

/// Canonical semantic intermediate representation (SPEC Chapter 13).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformationPlan {
    /// Originating contract identity.
    pub identity: PlanIdentity,
    /// Declared inputs.
    pub inputs: Vec<Input>,
    /// Declared outputs.
    pub outputs: Vec<Output>,
    /// Function declarations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<Function>,
    /// Semantic step nodes (actions, expressions, rules).
    pub nodes: Vec<PlanNode>,
    /// Logical dependency edges.
    pub dependencies: Vec<PlanDependency>,
    /// Dataset lineage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Lineage>,
    /// Contractual guarantees.
    pub guarantees: PlanGuarantees,
    /// Contract metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Versioning policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versioning: Option<Versioning>,
    /// Vendor extensions.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub extensions: IndexMap<String, Value>,
    /// Analysis findings attached during lowering.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<AnalysisFinding>,
}
