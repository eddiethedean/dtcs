//! Transformation plan lowering and validation (SPEC Chapter 13).

mod equivalence;
mod graph;
mod lowering;
mod model;
mod optimize;
mod rule_key;
mod validate;

pub use equivalence::equivalent;
pub use graph::{is_acyclic, topological_order, vertex_count};
pub use lowering::{lower, lower_report, PlanResult};
pub use model::{
    DependencyReason, InterfaceConditionRef, PlanDependency, PlanGuarantees, PlanIdentity,
    PlanNode, PlanNodeKind, TransformationPlan,
};
pub use optimize::{
    optimize, optimize_with_registry, OptimizeOptions, OptimizeResult, TransformRecord,
};
pub use validate::{plan_as_contract, validate, validate_with_registry};
