//! YAML parser.

use crate::model::TransformationContract;
use crate::parser::depth;
use crate::parser::{failure, success, ParseResult};

/// Parse a DTCS document from YAML.
#[must_use]
pub fn parse_yaml(content: &[u8]) -> ParseResult {
    let value = match serde_yaml::from_slice::<serde_yaml::Value>(content) {
        Ok(value) => value,
        Err(error) => return failure(error.to_string()),
    };
    if let Err(message) = depth::check_yaml_depth(&value) {
        return failure(message);
    }
    match serde_yaml::from_value::<TransformationContract>(value) {
        Ok(contract) => success(contract),
        Err(error) => failure(error.to_string()),
    }
}
