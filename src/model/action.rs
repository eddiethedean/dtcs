//! Semantic action model (SPEC Chapter 17).

use std::fmt;

use indexmap::IndexMap;
use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::metadata::Metadata;

/// A standardized semantic action declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticAction {
    /// Stable action instance identifier.
    pub id: String,
    /// Semantic action registry identifier (for example `dtcs:lowercase`).
    pub action: String,
    /// Target field, object, or interface reference.
    pub target: String,
    /// Action parameters (for example join keys, projected fields).
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_unique_parameters"
    )]
    pub parameters: IndexMap<String, Value>,
    /// Whether the action is deterministic (SPEC Chapter 17 §3). Defaults to true.
    #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
    pub deterministic: bool,
    /// Lineage behavior description or registry hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_behavior: Option<String>,
    /// Object metadata (SPEC Chapter 5 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Vendor extension fields preserved verbatim (SPEC Chapter 21 §8).
    #[serde(default, flatten)]
    pub extensions: IndexMap<String, Value>,
}

fn default_true() -> bool {
    true
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
                        "duplicate key '{key}' in semanticAction.parameters"
                    )));
                }
                map.insert(key, value);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(UniqueMapVisitor)
}
