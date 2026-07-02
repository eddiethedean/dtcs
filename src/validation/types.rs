//! Type validation phase.

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{parse_logical_type, TransformationContract, TypeParseError};

use super::context::ValidationContext;

pub(crate) fn validate_types(ctx: &mut ValidationContext, contract: &TransformationContract) {
    for input in &contract.inputs {
        let Some(schema) = &input.schema else {
            continue;
        };
        for field in &schema.fields {
            validate_field_type(
                ctx,
                &format!("inputs.{}.schema.fields.{}", input.id, field.name),
                &format!("inputs.{}.schema", input.id),
                field,
            );
        }
    }

    for output in &contract.outputs {
        let Some(schema) = &output.schema else {
            continue;
        };
        for field in &schema.fields {
            validate_field_type(
                ctx,
                &format!("outputs.{}.schema.fields.{}", output.id, field.name),
                &format!("outputs.{}.schema", output.id),
                field,
            );
        }
    }
}

fn validate_field_type(
    ctx: &mut ValidationContext,
    field_ref: &str,
    schema_ref: &str,
    field: &crate::model::Field,
) {
    if field.name.trim().is_empty() {
        ctx.error(
            codes::MISSING_REQUIRED_FIELD,
            DiagnosticCategory::Type,
            "schema field name is required",
            Some(schema_ref),
            None,
        );
        return;
    }

    match parse_logical_type(&field.type_name) {
        Ok(_) => {}
        Err(TypeParseError::BareComposite(kind)) => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!("composite type '{kind}' must declare type parameters"),
                Some(field_ref),
                Some("Use parameterized composite types such as list<string>"),
            );
        }
        Err(TypeParseError::Unknown(_))
        | Err(TypeParseError::UnknownParameter(_))
        | Err(TypeParseError::Malformed(_)) => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!("unknown logical type '{}'", field.type_name),
                Some(field_ref),
                Some("Use a primitive or parameterized composite type from SPEC Chapter 4"),
            );
        }
    }
}
