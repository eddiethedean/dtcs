//! Logical schema and field types.

use serde::{Deserialize, Serialize};

/// A logical schema associated with an input or output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    /// Declared fields.
    #[serde(default)]
    pub fields: Vec<Field>,
}

/// A field within a schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    /// Field name.
    pub name: String,
    /// Logical type name.
    #[serde(rename = "type")]
    pub type_name: String,
    /// Whether the field permits null values.
    pub nullable: bool,
}

/// Supported primitive logical types from SPEC Chapter 4.
pub const PRIMITIVE_TYPES: &[&str] = &[
    "boolean", "integer", "decimal", "string", "binary", "date", "time", "datetime", "duration",
];

/// Supported composite logical types from SPEC Chapter 4.
pub const COMPOSITE_TYPES: &[&str] = &["list", "map", "object", "tuple"];

/// Parsed logical type representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalType {
    /// A primitive logical type.
    Primitive(String),
    /// A parameterized composite logical type.
    Composite {
        /// Composite kind (`list`, `map`, etc.).
        kind: String,
        /// Type parameters.
        params: Vec<String>,
    },
}

/// Result of parsing a logical type expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeParseError {
    /// Unknown type name.
    Unknown(String),
    /// Composite type declared without parameters.
    BareComposite(String),
    /// Malformed parameterized type syntax.
    Malformed(String),
    /// Unknown nested type parameter.
    UnknownParameter(String),
}

/// Parse and validate a logical type expression.
pub fn parse_logical_type(type_name: &str) -> Result<LogicalType, TypeParseError> {
    let type_name = type_name.trim();
    if type_name.is_empty() {
        return Err(TypeParseError::Malformed("type is empty".into()));
    }

    if PRIMITIVE_TYPES.contains(&type_name) {
        return Ok(LogicalType::Primitive(type_name.to_string()));
    }

    if COMPOSITE_TYPES.contains(&type_name) {
        return Err(TypeParseError::BareComposite(type_name.to_string()));
    }

    if let Some(open) = type_name.find('<') {
        let kind = type_name[..open].trim();
        if !COMPOSITE_TYPES.contains(&kind) {
            return Err(TypeParseError::Unknown(kind.to_string()));
        }
        let Some(close) = type_name.rfind('>') else {
            return Err(TypeParseError::Malformed(type_name.to_string()));
        };
        if close <= open {
            return Err(TypeParseError::Malformed(type_name.to_string()));
        }
        let inner = type_name[open + 1..close].trim();
        if inner.is_empty() {
            return Err(TypeParseError::Malformed(type_name.to_string()));
        }
        let params: Vec<String> = inner
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(str::to_string)
            .collect();
        if params.is_empty() {
            return Err(TypeParseError::Malformed(type_name.to_string()));
        }
        for param in &params {
            if parse_logical_type(param).is_err() {
                return Err(TypeParseError::UnknownParameter(param.clone()));
            }
        }
        return Ok(LogicalType::Composite {
            kind: kind.to_string(),
            params,
        });
    }

    Err(TypeParseError::Unknown(type_name.to_string()))
}

/// Returns `true` when `type_name` is a recognized DTCS logical type.
#[must_use]
pub fn is_known_logical_type(type_name: &str) -> bool {
    parse_logical_type(type_name).is_ok()
}
