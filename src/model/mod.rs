//! Canonical Object Model types derived from `SPEC.md`.

mod action;
mod contract;
mod expression;
mod extension;
mod function;
mod interface;
mod lineage;
mod metadata;
mod registry;
mod rule;
mod semantics;
mod types;
mod versioning;

pub use action::{is_known_action, SemanticAction};
pub use contract::{TransformationContract, SUPPORTED_DTCS_VERSIONS};
pub use expression::Expression;
pub use extension::ExtensionBlock;
pub use function::Function;
pub use interface::{Input, Output};
pub use lineage::{Lineage, LineageMapping};
pub use metadata::Metadata;
pub use registry::Registry;
pub use rule::{is_known_rule, Rule, RulePhase};
pub use semantics::TransformationSemantics;
pub use types::{
    is_known_logical_type, parse_logical_type, Field, LogicalType, Schema, TypeParseError,
};
pub use versioning::Versioning;
