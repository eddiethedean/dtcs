//! Extension declarations.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Extension block preserved on the contract.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBlock {
    /// Namespaced extension entries.
    #[serde(flatten)]
    pub entries: IndexMap<String, Value>,
}
