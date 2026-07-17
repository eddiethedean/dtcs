//! Transformation plan lowering and validation (SPEC Chapter 13).

mod equivalence;
mod graph;
mod lowering;
mod model;
mod optimize;
mod portable;
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
pub use portable::{
    export_portable_plan, PortablePlan, RegistryVersions, COMPLEX_TYPES_PROFILE,
    COMPLEX_VALUES_PROFILE, CONVERSION_PROFILE, KERNEL_PROFILE, LEGACY_TRANSFORM_PLAN_IDENTITY,
    MAX_PORTABLE_PLAN_BYTES, MAX_PORTABLE_PLAN_DEPTH, MAX_PORTABLE_PLAN_NODES,
    NONDETERMINISTIC_PROFILE, RELATIONAL_EXTENDED_PROFILE, RELATIONAL_PROFILE, RESHAPE_PROFILE,
    STATISTICS_PROFILE, STRING_ADVANCED_PROFILE, TEMPORAL_IANA_PROFILE, TRANSFORM_PLAN_IDENTITY,
    WINDOW_PROFILE,
};
pub use validate::{plan_as_contract, validate, validate_with_registry};
