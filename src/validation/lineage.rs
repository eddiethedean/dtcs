//! Lineage validation phase helpers.

use std::collections::HashSet;

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::TransformationContract;

use super::context::ValidationContext;
use super::field_index::FieldIndex;

pub(crate) fn validate_lineage(ctx: &mut ValidationContext, contract: &TransformationContract) {
    if contract.outputs.is_empty() {
        return;
    }

    let output_ids: HashSet<String> = contract.outputs.iter().map(|o| o.id.clone()).collect();
    let input_ids: HashSet<String> = contract.inputs.iter().map(|i| i.id.clone()).collect();

    let Some(lineage) = &contract.lineage else {
        ctx.error(
            codes::MISSING_LINEAGE,
            DiagnosticCategory::Reference,
            "lineage is required when outputs are declared",
            Some("lineage"),
            Some("Declare lineage mappings from outputs to contributing inputs"),
        );
        return;
    };

    if lineage.mappings.is_empty() {
        ctx.error(
            codes::MISSING_LINEAGE,
            DiagnosticCategory::Reference,
            "lineage mappings are required for every declared output",
            Some("lineage.mappings"),
            Some("Add one mapping per declared output"),
        );
        return;
    }

    let mut mapped_outputs = HashSet::new();
    for mapping in &lineage.mappings {
        if !output_ids.contains(&mapping.output) {
            ctx.error(
                codes::UNRESOLVED_REFERENCE,
                DiagnosticCategory::Reference,
                format!(
                    "lineage output '{}' does not match a declared output",
                    mapping.output
                ),
                Some(&format!("lineage.mappings.{}", mapping.output)),
                Some("Reference declared output identifiers"),
            );
            continue;
        }
        if !mapped_outputs.insert(mapping.output.clone()) {
            ctx.error(
                codes::DUPLICATE_IDENTIFIER,
                DiagnosticCategory::Reference,
                format!("duplicate lineage mapping for output '{}'", mapping.output),
                Some(&format!("lineage.mappings.{}", mapping.output)),
                Some("Provide at most one mapping per output"),
            );
        }
        if mapping.inputs.is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Reference,
                format!(
                    "lineage mapping for output '{}' must reference at least one input",
                    mapping.output
                ),
                Some(&format!("lineage.mappings.{}.inputs", mapping.output)),
                Some("List contributing input identifiers"),
            );
        }
        if mapping.operation.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Reference,
                format!(
                    "lineage mapping for output '{}' must declare a semantic operation",
                    mapping.output
                ),
                Some(&format!("lineage.mappings.{}.operation", mapping.output)),
                Some("Set operation to a registry action or dtcs:derive"),
            );
        }
        for input_id in &mapping.inputs {
            if !input_ids.contains(input_id) {
                ctx.error(
                    codes::UNRESOLVED_REFERENCE,
                    DiagnosticCategory::Reference,
                    format!("lineage input '{input_id}' does not match a declared input"),
                    Some(&format!("lineage.mappings.{}.inputs", mapping.output)),
                    Some("Reference declared input identifiers"),
                );
            }
        }
    }

    for output in &contract.outputs {
        if !mapped_outputs.contains(&output.id) {
            ctx.error(
                codes::MISSING_LINEAGE,
                DiagnosticCategory::Reference,
                format!("output '{}' is missing a lineage mapping", output.id),
                Some(&format!("lineage.mappings.{}", output.id)),
                Some("Map every declared output to one or more inputs"),
            );
        }
    }
}

/// Warn when duplicate field names exist across interfaces without qualification.
pub(crate) fn warn_ambiguous_field_names(ctx: &mut ValidationContext, index: &FieldIndex) {
    use crate::diagnostics::{Diagnostic, DiagnosticStage, Severity};

    for name in index.ambiguous_field_names() {
        let diagnostic = Diagnostic::new(
            codes::AMBIGUOUS_REFERENCE,
            Severity::Warning,
            DiagnosticStage::Validation,
            DiagnosticCategory::Structure,
            format!(
                "field name '{name}' is declared in multiple interfaces; use qualified targets such as interface.{name}"
            ),
        )
        .with_object_ref(name.as_str())
        .with_remediation("Qualify field references with the interface identifier");
        ctx.push(diagnostic);
    }
}
