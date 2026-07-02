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
