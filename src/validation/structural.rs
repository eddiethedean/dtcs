//! Structural validation phase.

use std::collections::HashSet;

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::TransformationContract;

use super::context::{object_refs, ValidationContext};
use super::field_index::FieldIndex;
use super::lineage::warn_ambiguous_field_names;

pub(crate) fn validate_structural(ctx: &mut ValidationContext, contract: &TransformationContract) {
    if contract.inputs.is_empty() {
        ctx.error(
            codes::MISSING_REQUIRED_FIELD,
            DiagnosticCategory::Structure,
            "at least one input is required",
            Some("inputs"),
            Some("Declare every logical dataset consumed by the transformation"),
        );
    }

    if contract.outputs.is_empty() {
        ctx.error(
            codes::MISSING_REQUIRED_FIELD,
            DiagnosticCategory::Structure,
            "at least one output is required",
            Some("outputs"),
            Some("Declare every logical dataset produced by the transformation"),
        );
    }

    let refs = object_refs(contract);
    ctx.check_unique_ids(
        refs.iter()
            .filter(|(_, object_ref)| object_ref.starts_with("inputs."))
            .map(|(id, object_ref)| (id.clone(), object_ref.clone())),
        "inputs",
    );
    ctx.check_unique_ids(
        refs.iter()
            .filter(|(_, object_ref)| object_ref.starts_with("outputs."))
            .map(|(id, object_ref)| (id.clone(), object_ref.clone())),
        "outputs",
    );
    ctx.check_unique_ids(
        refs.iter()
            .filter(|(_, object_ref)| object_ref.starts_with("semanticActions."))
            .map(|(id, object_ref)| (id.clone(), object_ref.clone())),
        "semanticActions",
    );
    ctx.check_unique_ids(
        refs.iter()
            .filter(|(_, object_ref)| object_ref.starts_with("expressions."))
            .map(|(id, object_ref)| (id.clone(), object_ref.clone())),
        "expressions",
    );
    ctx.check_unique_ids(
        refs.iter()
            .filter(|(_, object_ref)| object_ref.starts_with("functions."))
            .map(|(id, object_ref)| (id.clone(), object_ref.clone())),
        "functions",
    );
    ctx.check_unique_ids(
        refs.iter()
            .filter(|(_, object_ref)| object_ref.starts_with("rules."))
            .map(|(id, object_ref)| (id.clone(), object_ref.clone())),
        "rules",
    );

    for input in &contract.inputs {
        if input.id.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "input id is required",
                Some("inputs"),
                None,
            );
        }
        if let Some(schema) = &input.schema {
            check_duplicate_schema_fields(
                ctx,
                &format!("inputs.{}.schema", input.id),
                &schema.fields,
            );
        }
    }

    for output in &contract.outputs {
        if output.id.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                "output id is required",
                Some("outputs"),
                None,
            );
        }
        if let Some(schema) = &output.schema {
            check_duplicate_schema_fields(
                ctx,
                &format!("outputs.{}.schema", output.id),
                &schema.fields,
            );
        }
    }

    let index = FieldIndex::from_contract(contract);
    warn_ambiguous_field_names(ctx, &index);
}

fn check_duplicate_schema_fields(
    ctx: &mut ValidationContext,
    object_ref: &str,
    fields: &[crate::model::Field],
) {
    let mut seen = HashSet::new();
    for field in fields {
        if field.name.trim().is_empty() {
            continue;
        }
        if !seen.insert(field.name.clone()) {
            ctx.error(
                codes::DUPLICATE_IDENTIFIER,
                DiagnosticCategory::Structure,
                format!("duplicate schema field '{}'", field.name),
                Some(&format!("{object_ref}.fields.{}", field.name)),
                Some("Use unique field names within each schema"),
            );
        }
    }
}
