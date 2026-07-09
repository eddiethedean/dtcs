//! JSON parser.

use crate::model::TransformationContract;
use crate::parser::depth;
use crate::parser::duplicate_keys;
use crate::parser::{failure, success, ParseResult};

/// Parse a DTCS document from JSON.
#[must_use]
pub fn parse_json(content: &[u8]) -> ParseResult {
    if let Some(key) = duplicate_keys::scan_rule_parameters_duplicate_keys(content) {
        return failure(format!("duplicate key '{key}' in rule.parameters"));
    }
    if let Err(message) = depth::check_json_depth_bytes(content) {
        return failure(message);
    }
    match serde_json::from_slice::<TransformationContract>(content) {
        Ok(contract) => success(contract),
        Err(error) => failure(error.to_string()),
    }
}
