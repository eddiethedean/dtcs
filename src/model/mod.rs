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
pub use interface::{Input, InterfaceCondition, Output, StreamingDeclaration, StreamingMode};
pub use lineage::{Lineage, LineageMapping};
pub use metadata::{
    ClassificationLevel, DocumentationMetadata, GovernanceMetadata, IdentityMetadata, Metadata,
    ProvenanceMetadata,
};
pub use registry::Registry;
pub use rule::{is_known_rule, Rule, RulePhase};
pub use semantics::TransformationSemantics;
pub use types::{
    infer_logical_type, is_extension_type_identifier, is_known_logical_type, parse_logical_type,
    type_compatible, Field, LogicalType, Schema, TypeCompatibility, TypeConversion, TypeParseError,
};
pub use versioning::Versioning;
