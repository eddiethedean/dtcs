//! Canonical keys for rule deduplication and equivalence fingerprints.

use indexmap::IndexMap;
use serde_json::Value;

use crate::model::Rule;

/// Stable deduplication key for a rule node.
#[must_use]
pub(crate) fn rule_dedup_key(rule: &Rule) -> String {
    format!(
        "{}|{}|{}|{}",
        rule.rule,
        rule.target,
        rule.phase.as_str(),
        canonical_parameters(&rule.parameters)
    )
}

/// Phase and parameters suffix for observable rule effects.
#[must_use]
pub(crate) fn rule_effect_identity(rule: &Rule) -> String {
    format!(
        "{}:{}",
        rule.phase.as_str(),
        canonical_parameters(&rule.parameters)
    )
}

fn canonical_parameters(params: &IndexMap<String, Value>) -> String {
    if params.is_empty() {
        return String::new();
    }
    serde_json::to_string(params).unwrap_or_else(|_| "{}".to_string())
}
