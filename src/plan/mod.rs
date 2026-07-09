//! Transformation plan lowering and validation (SPEC Chapter 13).

mod graph;
mod lowering;
mod model;
mod validate;

pub use graph::{is_acyclic, topological_order, vertex_count};
pub use lowering::{lower, lower_report, PlanResult};
pub use model::{
    DependencyReason, InterfaceConditionRef, PlanDependency, PlanGuarantees, PlanIdentity,
    PlanNode, PlanNodeKind, TransformationPlan,
};
pub use validate::{validate, validate_with_registry};
