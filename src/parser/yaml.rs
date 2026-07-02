//! YAML parser.

use crate::model::TransformationContract;
use crate::parser::{failure, success, ParseResult};

/// Parse a DTCS document from YAML.
#[must_use]
pub fn parse_yaml(content: &[u8]) -> ParseResult {
    match serde_yaml::from_slice::<TransformationContract>(content) {
        Ok(contract) => success(contract),
        Err(error) => failure(error.to_string()),
    }
}
