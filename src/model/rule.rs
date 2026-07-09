//! Rule model.

use std::fmt;

use indexmap::IndexMap;
use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::metadata::Metadata;

/// Rule evaluation phase per SPEC Chapter 19.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RulePhase {
    /// Evaluated before execution.
    Precondition,
    /// Evaluated during execution.
    Execution,
    /// Evaluated after execution.
    Postcondition,
}

impl RulePhase {
    /// Returns the serialized phase name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Precondition => "precondition",
            Self::Execution => "execution",
            Self::Postcondition => "postcondition",
        }
    }
}

/// A declarative invariant rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    /// Stable rule instance identifier.
    pub id: String,
    /// Rule registry identifier (for example `dtcs:not_null`).
    pub rule: String,
    /// Evaluation target.
    pub target: String,
    /// Evaluation phase.
    pub phase: RulePhase,
    /// Rule parameters (for example `min` for `dtcs:min_length`).
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_unique_parameters"
    )]
    pub parameters: IndexMap<String, Value>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

fn deserialize_unique_parameters<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, Value>, D::Error>
where
    D: Deserializer<'de>,
{
    struct UniqueMapVisitor;

    impl<'de> Visitor<'de> for UniqueMapVisitor {
        type Value = IndexMap<String, Value>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with unique parameter keys")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = IndexMap::new();
            while let Some((key, value)) = access.next_entry::<String, Value>()? {
                if map.contains_key(&key) {
                    return Err(M::Error::custom(format!(
                        "duplicate key '{key}' in rule.parameters"
                    )));
                }
                map.insert(key, value);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(UniqueMapVisitor)
}
