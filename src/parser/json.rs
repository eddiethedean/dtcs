//! JSON parser.

use crate::model::TransformationContract;
use crate::parser::{failure, success, ParseResult};

/// Parse a DTCS document from JSON.
#[must_use]
pub fn parse_json(content: &[u8]) -> ParseResult {
    match serde_json::from_slice::<TransformationContract>(content) {
        Ok(contract) => success(contract),
        Err(error) => failure(error.to_string()),
    }
}
