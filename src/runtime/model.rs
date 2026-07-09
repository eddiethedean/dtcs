//! Runtime value and dataset types.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A runtime value (SPEC logical types).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuntimeValue {
    /// Null value.
    Null,
    /// Boolean.
    Boolean(bool),
    /// String.
    String(String),
    /// Integer.
    Integer(i64),
    /// Decimal.
    Decimal(f64),
    /// Binary encoded as base64 in JSON interchange.
    Binary(String),
    /// Homogeneous list.
    List(Vec<RuntimeValue>),
    /// String-keyed map.
    Map(BTreeMap<String, RuntimeValue>),
}

impl RuntimeValue {
    /// Returns true for null values.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Coerce to string for display and string functions.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Coerce to integer.
    #[must_use]
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(v) => Some(*v),
            Self::Decimal(v) => Some(*v as i64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Coerce to decimal.
    #[must_use]
    pub fn as_decimal(&self) -> Option<f64> {
        match self {
            Self::Decimal(v) => Some(*v),
            Self::Integer(v) => Some(*v as f64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Coerce to boolean.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(v) => Some(*v),
            _ => None,
        }
    }
}

/// One row of field values within an interface workspace.
pub type Row = BTreeMap<String, RuntimeValue>;

/// A logical dataset (row-oriented).
pub type Dataset = Vec<Row>;

/// Runtime inputs keyed by interface identifier.
pub type RuntimeInputs = BTreeMap<String, Dataset>;

/// Runtime outputs keyed by interface identifier.
pub type RuntimeOutputs = BTreeMap<String, Dataset>;

/// Parsed qualified field reference (`interface.field`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedField {
    /// Interface identifier.
    pub interface_id: String,
    /// Field name.
    pub field_name: String,
}

/// Parse `interface.field` qualified references.
#[must_use]
pub fn parse_qualified_field(target: &str) -> Option<QualifiedField> {
    let (interface_id, field_name) = target.split_once('.')?;
    if interface_id.is_empty() || field_name.is_empty() {
        return None;
    }
    Some(QualifiedField {
        interface_id: interface_id.into(),
        field_name: field_name.into(),
    })
}
