//! Root transformation contract document.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    CompatibilityDeclaration, ContractGuarantees, Expression, Function, Input, Lineage, Metadata,
    Output, Rule, SemanticAction, TransformationSemantics, Versioning,
};

/// Supported DTCS specification versions for this implementation.
pub const SUPPORTED_DTCS_VERSIONS: &[&str] = &["1.0.0", "1.0.0-draft"];

/// A DTCS Transformation Contract — the canonical root object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformationContract {
    /// DTCS specification version.
    pub dtcs_version: String,
    /// Stable contract identifier.
    pub id: String,
    /// Human-readable contract name.
    pub name: String,
    /// Contract version.
    pub version: String,
    /// Optional metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Declared inputs.
    #[serde(default)]
    pub inputs: Vec<Input>,
    /// Declared outputs.
    #[serde(default)]
    pub outputs: Vec<Output>,
    /// Semantic actions.
    #[serde(default)]
    pub semantic_actions: Vec<SemanticAction>,
    /// Expressions.
    #[serde(default)]
    pub expressions: Vec<Expression>,
    /// Functions.
    #[serde(default)]
    pub functions: Vec<Function>,
    /// Rules.
    #[serde(default)]
    pub rules: Vec<Rule>,
    /// Lineage metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Lineage>,
    /// Contractual guarantees (SPEC Chapter 2 §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantees: Option<ContractGuarantees>,
    /// Compatibility declaration (SPEC Chapter 3 §9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<CompatibilityDeclaration>,
    /// Versioning metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versioning: Option<Versioning>,
    /// Semantic flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantics: Option<TransformationSemantics>,
    /// Vendor extension fields preserved verbatim.
    #[serde(default, flatten)]
    pub extensions: IndexMap<String, Value>,
}

impl TransformationContract {
    /// Validates the contract and returns a diagnostic report.
    #[must_use]
    pub fn validate(&self) -> crate::ValidationReport {
        crate::validate(self)
    }

    /// Returns all declared schema field names across inputs and outputs.
    #[must_use]
    pub fn declared_field_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for input in &self.inputs {
            if let Some(schema) = &input.schema {
                for field in &schema.fields {
                    names.push(field.name.clone());
                }
            }
        }
        for output in &self.outputs {
            if let Some(schema) = &output.schema {
                for field in &schema.fields {
                    names.push(field.name.clone());
                }
            }
        }
        names
    }

    /// Returns all declared input and output identifiers.
    #[must_use]
    pub fn interface_ids(&self) -> Vec<String> {
        self.inputs
            .iter()
            .map(|i| i.id.clone())
            .chain(self.outputs.iter().map(|o| o.id.clone()))
            .collect()
    }
}
