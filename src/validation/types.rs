//! Type validation phase.

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{
    parse_logical_type, type_compatible, TransformationContract, TypeCompatibility, TypeParseError,
};

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

    let parsed = match parse_logical_type(&field.type_name) {
        Ok(parsed) => parsed,
        Err(error) => {
            emit_type_error(ctx, field_ref, &field.type_name, error);
            return;
        }
    };

    for (index, conversion) in field.conversions.iter().enumerate() {
        validate_conversion(ctx, field_ref, index, conversion, &parsed);
    }
}

fn validate_conversion(
    ctx: &mut ValidationContext,
    field_ref: &str,
    index: usize,
    conversion: &crate::model::TypeConversion,
    field_type: &crate::model::LogicalType,
) {
    let object_ref = format!("{field_ref}.conversions[{index}]");
    let from_type = match parse_logical_type(&conversion.from) {
        Ok(parsed) => parsed,
        Err(error) => {
            emit_type_error(ctx, &format!("{object_ref}.from"), &conversion.from, error);
            return;
        }
    };
    let to_type = match parse_logical_type(&conversion.to) {
        Ok(parsed) => parsed,
        Err(error) => {
            emit_type_error(ctx, &format!("{object_ref}.to"), &conversion.to, error);
            return;
        }
    };

    let from_field = type_compatible(field_type, &from_type);
    if from_field != TypeCompatibility::Identical {
        ctx.error(
            codes::TYPE_INCOMPATIBLE,
            DiagnosticCategory::Type,
            format!(
                "conversion source type '{}' must be identical to the declared field type",
                conversion.from
            ),
            Some(&format!("{object_ref}.from")),
            Some("Set conversion.from to the field's declared logical type"),
        );
    }
    let to_field = type_compatible(field_type, &to_type);
    if to_field == TypeCompatibility::Incompatible {
        ctx.error(
            codes::TYPE_INCOMPATIBLE,
            DiagnosticCategory::Type,
            format!(
                "conversion target type '{}' is incompatible with field type",
                conversion.to
            ),
            Some(&format!("{object_ref}.to")),
            Some("Align conversion.to with the declared field type"),
        );
    }

    let conversion_compat = type_compatible(&from_type, &to_type);
    if conversion_compat == TypeCompatibility::Incompatible {
        ctx.error(
            codes::TYPE_INCOMPATIBLE,
            DiagnosticCategory::Type,
            format!(
                "conversion from '{}' to '{}' is not type-compatible",
                conversion.from, conversion.to
            ),
            Some(&object_ref),
            None,
        );
    } else if conversion_compat == TypeCompatibility::Compatible && !conversion.lossy {
        ctx.error(
            codes::INVALID_CONVERSION,
            DiagnosticCategory::Type,
            format!(
                "conversion from '{}' to '{}' requires lossy: true",
                conversion.from, conversion.to
            ),
            Some(&object_ref),
            Some("Set lossy: true for non-identical compatible conversions"),
        );
    }
}

fn emit_type_error(
    ctx: &mut ValidationContext,
    field_ref: &str,
    type_name: &str,
    error: TypeParseError,
) {
    match error {
        TypeParseError::BareComposite(kind) => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!("composite type '{kind}' must declare type parameters"),
                Some(field_ref),
                Some("Use parameterized composite types such as list<string>"),
            );
        }
        TypeParseError::InvalidArity {
            kind,
            expected,
            actual,
        } => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!(
                    "composite type '{kind}' requires {expected} type parameters, found {actual}"
                ),
                Some(field_ref),
                Some("Use list<T>, map<K,V>, object<T...>, or tuple<T...> with correct arity"),
            );
        }
        TypeParseError::Unknown(unknown) => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!("unknown logical type '{unknown}'"),
                Some(field_ref),
                Some("Use a primitive, parameterized composite, or namespaced extension type"),
            );
        }
        TypeParseError::UnknownParameter(param) => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!("unknown nested type parameter '{param}' in '{type_name}'"),
                Some(field_ref),
                Some("Use a valid primitive, composite, or extension type for each parameter"),
            );
        }
        TypeParseError::Malformed(detail) => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!("malformed logical type '{type_name}': {detail}"),
                Some(field_ref),
                Some("Use syntax such as list<string> or map<string,integer>"),
            );
        }
        TypeParseError::TooDeep => {
            ctx.error(
                codes::INVALID_TYPE,
                DiagnosticCategory::Type,
                format!("logical type '{type_name}' exceeds maximum nesting depth of 32"),
                Some(field_ref),
                Some("Reduce composite type nesting depth"),
            );
        }
    }
}
