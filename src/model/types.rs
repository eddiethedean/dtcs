//! Logical schema and field types (SPEC Chapter 4).

use serde::{Deserialize, Serialize};

/// A logical schema associated with an input or output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    /// Declared fields.
    #[serde(default)]
    pub fields: Vec<Field>,
}

/// A declared type conversion (SPEC Chapter 4 §9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeConversion {
    /// Source logical type.
    pub from: String,
    /// Target logical type.
    pub to: String,
    /// Whether the conversion is lossy.
    #[serde(default)]
    pub lossy: bool,
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
    /// Declared type conversions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conversions: Vec<TypeConversion>,
}

/// Supported primitive logical types from SPEC Chapter 4.
pub const PRIMITIVE_TYPES: &[&str] = &[
    "boolean", "integer", "decimal", "string", "binary", "date", "time", "datetime", "duration",
];

/// Supported composite logical types from SPEC Chapter 4.
pub const COMPOSITE_TYPES: &[&str] = &["list", "map", "object", "tuple"];

/// Type compatibility classification (SPEC Chapter 4 §8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeCompatibility {
    /// Identical logical types.
    Identical,
    /// Compatible but not identical types.
    Compatible,
    /// Incompatible types.
    Incompatible,
}

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
    /// An extension logical type with a namespaced identifier.
    Extension(String),
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
    /// Invalid composite type arity.
    InvalidArity {
        /// Composite kind.
        kind: String,
        /// Expected parameter count description.
        expected: String,
        /// Actual parameter count.
        actual: usize,
    },
}

/// Returns `true` when `identifier` is a namespaced extension type.
#[must_use]
pub fn is_extension_type_identifier(identifier: &str) -> bool {
    let identifier = identifier.trim();
    if !identifier.contains(':') {
        return false;
    }
    if identifier.starts_with("dtcs:") {
        return false;
    }
    let prefix = identifier.split(':').next().unwrap_or("");
    !prefix.is_empty() && !COMPOSITE_TYPES.contains(&prefix) && !PRIMITIVE_TYPES.contains(&prefix)
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
            if is_extension_type_identifier(type_name) {
                return Ok(LogicalType::Extension(type_name.to_string()));
            }
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
        validate_composite_arity(kind, params.len())?;
        return Ok(LogicalType::Composite {
            kind: kind.to_string(),
            params,
        });
    }

    if is_extension_type_identifier(type_name) {
        return Ok(LogicalType::Extension(type_name.to_string()));
    }

    Err(TypeParseError::Unknown(type_name.to_string()))
}

fn validate_composite_arity(kind: &str, actual: usize) -> Result<(), TypeParseError> {
    let (expected, valid) = match kind {
        "list" => ("exactly 1", actual == 1),
        "map" => ("exactly 2", actual == 2),
        "object" | "tuple" => ("at least 1", actual >= 1),
        _ => return Ok(()),
    };
    if !valid {
        return Err(TypeParseError::InvalidArity {
            kind: kind.to_string(),
            expected: expected.to_string(),
            actual,
        });
    }
    Ok(())
}

/// Evaluate logical type compatibility (SPEC Chapter 4 §8).
#[must_use]
pub fn type_compatible(a: &LogicalType, b: &LogicalType) -> TypeCompatibility {
    if a == b {
        return TypeCompatibility::Identical;
    }
    match (a, b) {
        (LogicalType::Primitive(a_name), LogicalType::Primitive(b_name))
            if (a_name == "integer" && b_name == "decimal")
                || (a_name == "decimal" && b_name == "integer") =>
        {
            TypeCompatibility::Compatible
        }
        (
            LogicalType::Composite {
                kind: a_kind,
                params: a_params,
            },
            LogicalType::Composite {
                kind: b_kind,
                params: b_params,
            },
        ) if a_kind == b_kind && a_params == b_params => TypeCompatibility::Identical,
        (LogicalType::Extension(a_name), LogicalType::Extension(b_name)) if a_name == b_name => {
            TypeCompatibility::Identical
        }
        _ => TypeCompatibility::Incompatible,
    }
}

/// Infer the logical type of a field from its declaration (SPEC Chapter 4 §10).
#[must_use]
pub fn infer_logical_type(field: &Field) -> Option<LogicalType> {
    parse_logical_type(&field.type_name).ok()
}

/// Returns `true` when `type_name` is a recognized DTCS logical type.
#[must_use]
pub fn is_known_logical_type(type_name: &str) -> bool {
    parse_logical_type(type_name).is_ok()
}
