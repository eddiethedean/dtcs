//! Runtime value and dataset types.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Result of looking up a field on a row (SPEC Chapter 8 §9).
#[derive(Debug, Clone, PartialEq)]
pub enum FieldLookup {
    /// Field key is absent from the row.
    Missing,
    /// Field is present with a value (possibly null/invalid).
    Present(RuntimeValue),
}

/// A runtime value (SPEC logical types).
///
/// JSON deserialization recognizes `{"$dtcs":"missing"}` / `{"$dtcs":"invalid"}`
/// before falling back to ordinary maps (see `value_serde`).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RuntimeValue {
    /// Null value (present key, null payload).
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
    /// ISO-8601 calendar date (`YYYY-MM-DD`).
    Date(String),
    /// ISO-8601 wall time.
    Time(String),
    /// ISO-8601 date-time.
    DateTime(String),
    /// ISO-8601 duration.
    Duration(String),
    /// Homogeneous list.
    List(Vec<RuntimeValue>),
    /// Explicit missing value token used in expression evaluation (SPEC Chapter 8 §9).
    Missing(MissingValue),
    /// Explicit invalid value (SPEC Chapter 8 §9).
    Invalid(InvalidValue),
    /// String-keyed map (after `$dtcs` tokens).
    Map(BTreeMap<String, RuntimeValue>),
}

impl<'de> Deserialize<'de> for RuntimeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        super::value_serde::deserialize_runtime_value(deserializer)
    }
}

/// Tagged missing payload so JSON can round-trip distinctly from null.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissingValue {
    /// Marker.
    #[serde(rename = "$dtcs")]
    pub marker: MissingMarker,
}

/// Marker discriminant for missing values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MissingMarker {
    /// Missing value marker.
    #[default]
    Missing,
}

/// Tagged invalid payload so JSON can round-trip distinctly from null.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvalidValue {
    /// Marker.
    #[serde(rename = "$dtcs")]
    pub marker: InvalidMarker,
    /// Optional reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Marker discriminant for invalid values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum InvalidMarker {
    /// Invalid value marker.
    #[default]
    Invalid,
}

impl RuntimeValue {
    /// Construct a missing value token.
    #[must_use]
    pub fn missing() -> Self {
        Self::Missing(MissingValue::default())
    }

    /// Construct an invalid value.
    #[must_use]
    pub fn invalid(reason: impl Into<String>) -> Self {
        Self::Invalid(InvalidValue {
            marker: InvalidMarker::Invalid,
            reason: Some(reason.into()),
        })
    }

    /// Returns true for null values (not missing, not invalid).
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns true for missing value tokens.
    #[must_use]
    pub fn is_missing(&self) -> bool {
        matches!(self, Self::Missing(_))
    }

    /// Returns true for invalid values.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }

    /// Coerce to string for display and string functions.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s)
            | Self::Date(s)
            | Self::Time(s)
            | Self::DateTime(s)
            | Self::Duration(s)
            | Self::Binary(s) => Some(s.as_str()),
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

/// Look up a field distinguishing missing from null/invalid.
#[must_use]
pub fn lookup_field(row: &Row, field_name: &str) -> FieldLookup {
    match row.get(field_name) {
        None => FieldLookup::Missing,
        Some(value) => FieldLookup::Present(value.clone()),
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

/// Parse `interface.field` qualified references using longest-prefix interface matching.
#[must_use]
pub fn parse_qualified_field(target: &str) -> Option<QualifiedField> {
    parse_qualified_field_with_interfaces(target, &[])
}

/// Parse a qualified field reference against known interface identifiers.
#[must_use]
pub fn parse_qualified_field_with_interfaces(
    target: &str,
    interface_ids: &[String],
) -> Option<QualifiedField> {
    let target = target.trim();
    if target.is_empty() || !target.contains('.') {
        return None;
    }

    let mut best: Option<(&str, usize)> = None;
    for id in interface_ids {
        let prefix = format!("{id}.");
        if target.starts_with(&prefix) {
            let len = id.len();
            if best.map_or(true, |(_, bl)| len > bl) {
                best = Some((id.as_str(), len));
            }
        }
    }

    if let Some((id, len)) = best {
        let field_name = &target[len + 1..];
        if field_name.is_empty() {
            return None;
        }
        return Some(QualifiedField {
            interface_id: id.to_string(),
            field_name: field_name.to_string(),
        });
    }

    let (interface_id, field_name) = target.split_once('.')?;
    if interface_id.is_empty() || field_name.is_empty() {
        return None;
    }
    Some(QualifiedField {
        interface_id: interface_id.into(),
        field_name: field_name.into(),
    })
}
