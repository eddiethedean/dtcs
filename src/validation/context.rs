//! Validation context and shared helpers.

use std::collections::HashSet;

use crate::diagnostics::{
    codes, com_error, validation_error, Diagnostic, DiagnosticCategory, DiagnosticReport,
    DiagnosticStage,
};
use crate::model::TransformationContract;

/// Mutable validation state.
pub struct ValidationContext {
    report: DiagnosticReport,
}

impl ValidationContext {
    /// Creates a new validation context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            report: DiagnosticReport::new(),
        }
    }

    /// Returns the collected diagnostics.
    #[must_use]
    pub fn into_report(self) -> DiagnosticReport {
        self.report
    }

    /// Emits a diagnostic.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.report.push(diagnostic);
    }

    /// Emits a validation-stage error.
    pub fn error(
        &mut self,
        id: &str,
        category: DiagnosticCategory,
        message: impl Into<String>,
        object_ref: Option<&str>,
        remediation: Option<&str>,
    ) {
        self.error_with_stage(
            id,
            category,
            message,
            object_ref,
            remediation,
            DiagnosticStage::Validation,
        );
    }

    /// Emits an error at a specific processing stage.
    pub fn error_with_stage(
        &mut self,
        id: &str,
        category: DiagnosticCategory,
        message: impl Into<String>,
        object_ref: Option<&str>,
        remediation: Option<&str>,
        stage: DiagnosticStage,
    ) {
        let mut diagnostic = match stage {
            DiagnosticStage::CanonicalObjectModel => com_error(id, category, message),
            _ => validation_error(id, category, message),
        };
        diagnostic.stage = stage;
        if let Some(object_ref) = object_ref {
            diagnostic = diagnostic.with_object_ref(object_ref);
        }
        if let Some(remediation) = remediation {
            diagnostic = diagnostic.with_remediation(remediation);
        }
        self.push(diagnostic);
    }

    /// Validates unique identifiers within a collection.
    pub fn check_unique_ids(
        &mut self,
        ids: impl IntoIterator<Item = (String, String)>,
        collection: &str,
    ) {
        let mut seen = HashSet::new();
        for (id, object_ref) in ids {
            if id.trim().is_empty() {
                self.error(
                    codes::MISSING_REQUIRED_FIELD,
                    DiagnosticCategory::Structure,
                    format!("{collection} entry is missing a non-empty id"),
                    Some(&object_ref),
                    Some("Provide a stable identifier for every object"),
                );
                continue;
            }
            if !seen.insert(id.clone()) {
                self.error(
                    codes::DUPLICATE_IDENTIFIER,
                    DiagnosticCategory::Structure,
                    format!("duplicate identifier '{id}' in {collection}"),
                    Some(&object_ref),
                    Some("Use unique identifiers within each collection"),
                );
            }
        }
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Collects object references for duplicate checking.
pub fn object_refs(contract: &TransformationContract) -> Vec<(String, String)> {
    let mut refs = Vec::new();
    for input in &contract.inputs {
        refs.push((input.id.clone(), format!("inputs.{}", input.id)));
    }
    for output in &contract.outputs {
        refs.push((output.id.clone(), format!("outputs.{}", output.id)));
    }
    for action in &contract.semantic_actions {
        refs.push((action.id.clone(), format!("semanticActions.{}", action.id)));
    }
    for expression in &contract.expressions {
        refs.push((
            expression.id.clone(),
            format!("expressions.{}", expression.id),
        ));
    }
    for function in &contract.functions {
        refs.push((function.id.clone(), format!("functions.{}", function.id)));
    }
    for rule in &contract.rules {
        refs.push((rule.id.clone(), format!("rules.{}", rule.id)));
    }
    refs
}

/// Returns `true` when an identifier uses a namespace prefix.
#[must_use]
pub fn is_namespaced_identifier(identifier: &str) -> bool {
    crate::model::is_namespaced_identifier(identifier)
}

/// Returns `true` for vendor-namespaced identifiers (excludes reserved `dtcs:` prefix).
#[must_use]
pub fn is_vendor_namespaced_identifier(identifier: &str) -> bool {
    crate::model::is_vendor_namespaced_identifier(identifier)
}

/// Validates extension and unknown top-level fields captured by serde flatten.
pub fn validate_extension_keys(ctx: &mut ValidationContext, contract: &TransformationContract) {
    for key in contract.extensions.keys() {
        if key == "extensions" {
            if contract.extensions.get(key).is_some_and(|v| v.is_object()) {
                ctx.error(
                    codes::INVALID_EXTENSION,
                    DiagnosticCategory::Structure,
                    "vendor keys must be flattened at the contract level, not nested under 'extensions'",
                    Some(key),
                    Some("Use vendor:fieldName at the top level instead of an extensions wrapper"),
                );
            }
            continue;
        }
        if matches!(key.as_str(), "input" | "output")
            && contract.extensions.get(key).is_some_and(|v| v.is_array())
        {
            let suggestion = if key == "input" { "inputs" } else { "outputs" };
            ctx.error(
                codes::UNKNOWN_FIELD,
                DiagnosticCategory::Structure,
                format!("unknown top-level field '{key}'"),
                Some(key),
                Some(&format!("Did you mean '{suggestion}'?")),
            );
            continue;
        }
        if !is_namespaced_identifier(key) {
            ctx.error(
                codes::UNKNOWN_FIELD,
                DiagnosticCategory::Structure,
                format!("unknown top-level field '{key}'"),
                Some(key),
                Some("Check for typos in standard field names such as inputs or outputs"),
            );
        } else if !is_vendor_namespaced_identifier(key) {
            ctx.error(
                codes::INVALID_EXTENSION,
                DiagnosticCategory::Structure,
                format!("extension key '{key}' must use a vendor namespace"),
                Some(key),
                Some("Use vendor:fieldName for contract-level extensions; dtcs: is reserved"),
            );
        }
    }
}
