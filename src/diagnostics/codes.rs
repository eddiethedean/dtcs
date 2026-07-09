//! Standardized `dtcs:` diagnostic identifiers.

/// Parse failure.
pub const PARSE_ERROR: &str = "dtcs:parse-error";
/// Unsupported specification version.
pub const UNSUPPORTED_VERSION: &str = "dtcs:unsupported-version";
/// Missing required field.
pub const MISSING_REQUIRED_FIELD: &str = "dtcs:missing-required-field";
/// Duplicate object identifier.
pub const DUPLICATE_IDENTIFIER: &str = "dtcs:duplicate-identifier";
/// Invalid object identifier format.
pub const INVALID_IDENTIFIER: &str = "dtcs:invalid-identifier";
/// Unknown top-level document field.
pub const UNKNOWN_FIELD: &str = "dtcs:unknown-field";
/// Missing lineage declaration.
pub const MISSING_LINEAGE: &str = "dtcs:missing-lineage";
/// Ambiguous field reference.
pub const AMBIGUOUS_REFERENCE: &str = "dtcs:ambiguous-reference";
/// Invalid logical type.
pub const INVALID_TYPE: &str = "dtcs:invalid-type";
/// Unresolved object reference.
pub const UNRESOLVED_REFERENCE: &str = "dtcs:unresolved-reference";
/// Invalid semantic action.
pub const INVALID_SEMANTIC_ACTION: &str = "dtcs:invalid-semantic-action";
/// Invalid rule declaration.
pub const INVALID_RULE: &str = "dtcs:invalid-rule";
/// Invalid extension key.
pub const INVALID_EXTENSION: &str = "dtcs:invalid-extension";
/// Invalid metadata declaration.
pub const INVALID_METADATA: &str = "dtcs:invalid-metadata";
/// Invalid input or output interface declaration.
pub const INVALID_INTERFACE: &str = "dtcs:invalid-interface";
/// Incompatible logical types.
pub const TYPE_INCOMPATIBLE: &str = "dtcs:type-incompatible";
/// Invalid type conversion declaration.
pub const INVALID_CONVERSION: &str = "dtcs:invalid-conversion";
/// Invalid function declaration.
pub const INVALID_FUNCTION: &str = "dtcs:invalid-function";
/// Incompatible contract revision or pair.
pub const INCOMPATIBLE_CONTRACT: &str = "dtcs:incompatible-contract";
/// Compatibility holds only under stated conditions.
pub const CONDITIONAL_COMPATIBILITY: &str = "dtcs:conditional-compatibility";
/// Breaking evolution change detected.
pub const EVOLUTION_BREAKING_CHANGE: &str = "dtcs:evolution-breaking-change";
/// Deprecated object referenced or declared.
pub const DEPRECATED_OBJECT: &str = "dtcs:deprecated-object";
/// Invalid version identifier or format.
pub const INVALID_VERSION: &str = "dtcs:invalid-version";
/// Conflicting version declarations.
pub const VERSION_CONFLICT: &str = "dtcs:version-conflict";
/// Unknown or unresolved registry entry.
pub const UNKNOWN_REGISTRY_ENTRY: &str = "dtcs:unknown-registry-entry";
/// Invalid registry document.
pub const INVALID_REGISTRY: &str = "dtcs:invalid-registry";
/// Unsupported mandatory extension.
pub const UNSUPPORTED_EXTENSION: &str = "dtcs:unsupported-extension";
/// Invalid expression syntax or grammar.
pub const INVALID_EXPRESSION: &str = "dtcs:invalid-expression";
/// Invalid transformation semantics.
pub const INVALID_SEMANTICS: &str = "dtcs:invalid-semantics";
/// Declared determinism conflicts with non-deterministic semantics.
pub const NON_DETERMINISTIC_SEMANTICS: &str = "dtcs:non-deterministic-semantics";
/// Implicit or inconsistent null handling in expressions.
pub const NULL_SEMANTICS_VIOLATION: &str = "dtcs:null-semantics-violation";
/// General transformation plan validation failure.
pub const INVALID_PLAN: &str = "dtcs:invalid-plan";
/// Missing required semantic content in a plan.
pub const INCOMPLETE_PLAN: &str = "dtcs:incomplete-plan";
/// Cyclic dependency in the plan graph.
pub const CYCLIC_DEPENDENCY: &str = "dtcs:cyclic-dependency";
/// Type preservation failure during planning.
pub const PLAN_TYPE_MISMATCH: &str = "dtcs:plan-type-mismatch";
/// Unresolved reference in a plan dependency or node.
pub const UNRESOLVED_PLAN_REFERENCE: &str = "dtcs:unresolved-plan-reference";
/// Optimization produced an invalid plan.
pub const INVALID_OPTIMIZATION: &str = "dtcs:invalid-optimization";
/// Optimization pass skipped a rewrite conservatively.
pub const OPTIMIZATION_SKIPPED: &str = "dtcs:optimization-skipped";
/// Unsupported engine capability for a transformation plan.
pub const UNSUPPORTED_CAPABILITY: &str = "dtcs:unsupported-capability";
/// Invalid engine capability declaration.
pub const INVALID_CAPABILITY: &str = "dtcs:invalid-capability";
/// Compilation failure.
pub const COMPILATION_FAILED: &str = "dtcs:compilation-failed";
/// Invalid execution plan.
pub const INVALID_EXECUTION_PLAN: &str = "dtcs:invalid-execution-plan";
/// Invalid runtime input.
pub const INVALID_RUNTIME_INPUT: &str = "dtcs:invalid-runtime-input";
/// Precondition violation at runtime.
pub const PRECONDITION_VIOLATION: &str = "dtcs:precondition-violation";
/// Postcondition violation at runtime.
pub const POSTCONDITION_VIOLATION: &str = "dtcs:postcondition-violation";
/// General runtime execution error.
pub const RUNTIME_ERROR: &str = "dtcs:runtime-error";

/// All standardized diagnostic identifiers registered by this implementation.
pub const ALL_CODES: &[&str] = &[
    PARSE_ERROR,
    UNSUPPORTED_VERSION,
    MISSING_REQUIRED_FIELD,
    DUPLICATE_IDENTIFIER,
    INVALID_IDENTIFIER,
    UNKNOWN_FIELD,
    MISSING_LINEAGE,
    AMBIGUOUS_REFERENCE,
    INVALID_TYPE,
    UNRESOLVED_REFERENCE,
    INVALID_SEMANTIC_ACTION,
    INVALID_RULE,
    INVALID_EXTENSION,
    INVALID_METADATA,
    INVALID_INTERFACE,
    TYPE_INCOMPATIBLE,
    INVALID_CONVERSION,
    INVALID_FUNCTION,
    INCOMPATIBLE_CONTRACT,
    CONDITIONAL_COMPATIBILITY,
    EVOLUTION_BREAKING_CHANGE,
    DEPRECATED_OBJECT,
    INVALID_VERSION,
    VERSION_CONFLICT,
    UNKNOWN_REGISTRY_ENTRY,
    INVALID_REGISTRY,
    UNSUPPORTED_EXTENSION,
    INVALID_EXPRESSION,
    INVALID_SEMANTICS,
    NON_DETERMINISTIC_SEMANTICS,
    NULL_SEMANTICS_VIOLATION,
    INVALID_PLAN,
    INCOMPLETE_PLAN,
    CYCLIC_DEPENDENCY,
    PLAN_TYPE_MISMATCH,
    UNRESOLVED_PLAN_REFERENCE,
    INVALID_OPTIMIZATION,
    OPTIMIZATION_SKIPPED,
    UNSUPPORTED_CAPABILITY,
    INVALID_CAPABILITY,
    COMPILATION_FAILED,
    INVALID_EXECUTION_PLAN,
    INVALID_RUNTIME_INPUT,
    PRECONDITION_VIOLATION,
    POSTCONDITION_VIOLATION,
    RUNTIME_ERROR,
];
