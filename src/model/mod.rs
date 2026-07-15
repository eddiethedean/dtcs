//! Canonical Object Model types derived from `SPEC.md`.

mod action;
mod compatibility_decl;
mod contract;
mod expression;
mod extension;
mod function;
mod guarantees;
mod identifiers;
mod interface;
mod lineage;
mod metadata;
mod null_behavior;
mod registry;
mod rule;
mod semantics;
mod types;
mod versioning;

pub use action::SemanticAction;
pub use compatibility_decl::CompatibilityDeclaration;
pub use contract::{TransformationContract, SUPPORTED_DTCS_VERSIONS};
pub use expression::Expression;
pub use extension::ExtensionBlock;
pub use function::{Function, FunctionParameter};
pub use guarantees::ContractGuarantees;
pub use identifiers::{is_namespaced_identifier, is_vendor_namespaced_identifier};
pub use interface::{Input, InterfaceCondition, Output, StreamingDeclaration, StreamingMode};
pub use lineage::{InformationFlow, Lineage, LineageMapping};
pub use metadata::{
    ClassificationLevel, DocumentationMetadata, GovernanceMetadata, IdentityMetadata,
    LifecycleMetadata, Metadata, OwnershipMetadata, ProvenanceMetadata,
};
pub use null_behavior::NullBehavior;
pub use registry::{
    ExtensionCompatibility, RegistryCategory, RegistryDocument, RegistryEntry, RegistryEntryStatus,
    RegistryPublicationStatus, RegistryRef,
};
pub use rule::{Rule, RuleOutcome, RulePhase, RuleScope};
pub use semantics::{ActionOrdering, SideEffectDeclaration, TransformationSemantics};
pub use types::{
    infer_logical_type, is_extension_type_identifier, is_known_logical_type, parse_logical_type,
    type_compatible, types_assignable, Field, FieldConstraints, LogicalType, Schema,
    TypeCompatibility, TypeConversion, TypeParseError, COMPOSITE_TYPES, PRIMITIVE_TYPES,
};
pub use versioning::Versioning;
