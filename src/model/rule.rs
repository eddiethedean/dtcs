//! Rule model (SPEC Chapter 19).

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

/// Rule evaluation scope (SPEC Chapter 19 §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RuleScope {
    /// Applies to the transformation contract.
    Contract,
    /// Applies to an input interface or field.
    #[default]
    Input,
    /// Applies to an output interface or field.
    Output,
    /// Applies to an expression.
    Expression,
    /// Applies to a semantic action.
    SemanticAction,
    /// Applies to a transformation plan.
    Plan,
    /// Applies to an execution plan.
    ExecutionPlan,
}

impl RuleScope {
    /// Serialized scope name.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Contract => "contract",
            Self::Input => "input",
            Self::Output => "output",
            Self::Expression => "expression",
            Self::SemanticAction => "semanticAction",
            Self::Plan => "plan",
            Self::ExecutionPlan => "executionPlan",
        }
    }
}

/// Rule evaluation outcome (SPEC Chapter 19 §7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RuleOutcome {
    /// Rule holds.
    Satisfied,
    /// Rule does not hold.
    Violated,
    /// Outcome cannot be determined (explicitly permitted).
    Indeterminate,
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
    /// Evaluation scope (SPEC Chapter 19 §6). Defaults from target when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<RuleScope>,
    /// Whether indeterminate outcomes are permitted.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_indeterminate: bool,
    /// Whether the rule is deterministic. Defaults to true.
    #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
    pub deterministic: bool,
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
