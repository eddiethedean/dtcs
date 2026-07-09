//! Metadata validation rules (SPEC Chapter 5 §12).

use crate::diagnostics::{codes, DiagnosticCategory, DiagnosticStage};
use crate::model::{is_namespaced_identifier, is_vendor_namespaced_identifier};
use crate::model::{
    ClassificationLevel, DocumentationMetadata, GovernanceMetadata, IdentityMetadata, Metadata,
    ProvenanceMetadata, TransformationContract,
};
use crate::validation::context::ValidationContext;

pub(crate) fn validate_metadata(ctx: &mut ValidationContext, contract: &TransformationContract) {
    if let Some(metadata) = &contract.metadata {
        validate_metadata_block(
            ctx,
            metadata,
            Some(&contract.id),
            Some(&contract.name),
            Some(&contract.version),
            "metadata",
        );
    }

    for input in &contract.inputs {
        if let Some(metadata) = &input.metadata {
            validate_metadata_block(
                ctx,
                metadata,
                Some(&input.id),
                None,
                None,
                &format!("inputs.{}.metadata", input.id),
            );
        }
    }

    for output in &contract.outputs {
        if let Some(metadata) = &output.metadata {
            validate_metadata_block(
                ctx,
                metadata,
                Some(&output.id),
                None,
                None,
                &format!("outputs.{}.metadata", output.id),
            );
        }
    }

    for action in &contract.semantic_actions {
        validate_object_metadata(ctx, &action.metadata, &action.id, "semanticActions");
    }

    for expression in &contract.expressions {
        validate_object_metadata(ctx, &expression.metadata, &expression.id, "expressions");
    }

    for function in &contract.functions {
        validate_object_metadata(ctx, &function.metadata, &function.id, "functions");
    }

    for rule in &contract.rules {
        validate_object_metadata(ctx, &rule.metadata, &rule.id, "rules");
    }
}

fn validate_object_metadata(
    ctx: &mut ValidationContext,
    metadata: &Option<Metadata>,
    object_id: &str,
    collection: &str,
) {
    if let Some(metadata) = metadata {
        validate_metadata_block(
            ctx,
            metadata,
            Some(object_id),
            None,
            None,
            &format!("{collection}.{object_id}.metadata"),
        );
    }
}

fn validate_metadata_block(
    ctx: &mut ValidationContext,
    metadata: &Metadata,
    object_id: Option<&str>,
    object_name: Option<&str>,
    object_version: Option<&str>,
    object_ref: &str,
) {
    if let Some(identity) = &metadata.identity {
        validate_identity(
            ctx,
            identity,
            object_id,
            object_name,
            object_version,
            object_ref,
        );
    }

    if let Some(governance) = &metadata.governance {
        validate_governance(ctx, governance, object_ref);
    }

    if let Some(documentation) = &metadata.documentation {
        validate_documentation(ctx, documentation, object_ref);
    }

    if let Some(provenance) = &metadata.provenance {
        validate_provenance(ctx, provenance, object_ref);
    }

    if metadata.classification == Some(ClassificationLevel::Restricted)
        && !has_restricted_governance(metadata.governance.as_ref())
    {
        ctx.error_with_stage(
            codes::INVALID_METADATA,
            DiagnosticCategory::Structure,
            "restricted classification requires governance.owner or governance.steward",
            Some(object_ref),
            Some("Add governance.owner or governance.steward for restricted objects"),
            DiagnosticStage::CanonicalObjectModel,
        );
    }

    for key in metadata.extensions.keys() {
        if key == "extensions" {
            if metadata.extensions.get(key).is_some_and(|v| v.is_object()) {
                ctx.error_with_stage(
                    codes::INVALID_EXTENSION,
                    DiagnosticCategory::Structure,
                    "vendor metadata keys must be flattened, not nested under 'extensions'",
                    Some(&format!("{object_ref}.extensions")),
                    Some("Use vendor:fieldName directly in the metadata block"),
                    DiagnosticStage::CanonicalObjectModel,
                );
            }
            continue;
        }
        if !is_vendor_namespaced_identifier(key) {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Structure,
                format!("custom metadata key '{key}' must be namespaced"),
                Some(&format!("{object_ref}.{key}")),
                Some("Use vendor:fieldName for custom metadata keys"),
                DiagnosticStage::CanonicalObjectModel,
            );
        }
    }
}

fn validate_identity(
    ctx: &mut ValidationContext,
    identity: &IdentityMetadata,
    object_id: Option<&str>,
    object_name: Option<&str>,
    object_version: Option<&str>,
    object_ref: &str,
) {
    if let Some(identifier) = &identity.identifier {
        if identifier.trim().is_empty() {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Structure,
                "metadata identity identifier must not be empty",
                Some(&format!("{object_ref}.identity.identifier")),
                None,
                DiagnosticStage::CanonicalObjectModel,
            );
        } else if let Some(expected) = object_id {
            if identifier.trim() != expected {
                ctx.error_with_stage(
                    codes::INVALID_METADATA,
                    DiagnosticCategory::Structure,
                    format!(
                        "metadata identity identifier '{}' conflicts with object id '{expected}'",
                        identifier.trim()
                    ),
                    Some(&format!("{object_ref}.identity.identifier")),
                    Some("Remove identity.identifier or align it with the object id"),
                    DiagnosticStage::CanonicalObjectModel,
                );
            }
        }
    }

    if let Some(name) = &identity.name {
        if name.trim().is_empty() {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Structure,
                "metadata identity name must not be empty",
                Some(&format!("{object_ref}.identity.name")),
                None,
                DiagnosticStage::CanonicalObjectModel,
            );
        } else if let Some(expected) = object_name {
            if name.trim() != expected {
                ctx.error_with_stage(
                    codes::INVALID_METADATA,
                    DiagnosticCategory::Structure,
                    format!(
                        "metadata identity name '{}' conflicts with object name '{expected}'",
                        name.trim()
                    ),
                    Some(&format!("{object_ref}.identity.name")),
                    Some("Remove identity.name or align it with the object name"),
                    DiagnosticStage::CanonicalObjectModel,
                );
            }
        }
    }

    if let Some(version) = &identity.version {
        if version.trim().is_empty() {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Structure,
                "metadata identity version must not be empty",
                Some(&format!("{object_ref}.identity.version")),
                None,
                DiagnosticStage::CanonicalObjectModel,
            );
        } else if let Some(expected) = object_version {
            if version.trim() != expected {
                ctx.error_with_stage(
                    codes::INVALID_METADATA,
                    DiagnosticCategory::Structure,
                    format!(
                        "metadata identity version '{}' conflicts with object version '{expected}'",
                        version.trim()
                    ),
                    Some(&format!("{object_ref}.identity.version")),
                    Some("Remove identity.version or align it with the object version"),
                    DiagnosticStage::CanonicalObjectModel,
                );
            }
        }
    }
}

fn validate_documentation(
    ctx: &mut ValidationContext,
    documentation: &DocumentationMetadata,
    object_ref: &str,
) {
    for (index, reference) in documentation.references.iter().enumerate() {
        if reference.trim().is_empty() {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Reference,
                "documentation references entry must not be empty",
                Some(&format!("{object_ref}.documentation.references[{index}]")),
                None,
                DiagnosticStage::CanonicalObjectModel,
            );
            continue;
        }
        if !is_valid_policy_ref(reference) {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Reference,
                format!(
                    "documentation reference '{reference}' must be a URI or namespaced identifier"
                ),
                Some(&format!("{object_ref}.documentation.references[{index}]")),
                Some("Use https://... or vendor:reference-id"),
                DiagnosticStage::CanonicalObjectModel,
            );
        }
    }
}

fn validate_governance(
    ctx: &mut ValidationContext,
    governance: &GovernanceMetadata,
    object_ref: &str,
) {
    if let Some(owner) = &governance.owner {
        if owner.trim().is_empty() {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Structure,
                "governance owner must not be empty when declared",
                Some(&format!("{object_ref}.governance.owner")),
                None,
                DiagnosticStage::CanonicalObjectModel,
            );
        }
    }

    if let Some(steward) = &governance.steward {
        if steward.trim().is_empty() {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Structure,
                "governance steward must not be empty when declared",
                Some(&format!("{object_ref}.governance.steward")),
                None,
                DiagnosticStage::CanonicalObjectModel,
            );
        }
    }

    if let Some(review_date) = &governance.review_date {
        if !is_iso8601_timestamp(review_date) {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Structure,
                format!("governance reviewDate '{review_date}' is not a valid ISO-8601 timestamp"),
                Some(&format!("{object_ref}.governance.reviewDate")),
                Some("Use an ISO-8601 date or datetime such as 2026-01-15 or 2026-01-15T10:00:00Z"),
                DiagnosticStage::CanonicalObjectModel,
            );
        }
    }

    for (index, policy_ref) in governance.policy_refs.iter().enumerate() {
        if policy_ref.trim().is_empty() {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Reference,
                "governance policyRefs entry must not be empty",
                Some(&format!("{object_ref}.governance.policyRefs[{index}]")),
                None,
                DiagnosticStage::CanonicalObjectModel,
            );
            continue;
        }
        if !is_valid_policy_ref(policy_ref) {
            ctx.error_with_stage(
                codes::INVALID_METADATA,
                DiagnosticCategory::Reference,
                format!(
                    "governance policyRef '{policy_ref}' must be a URI or namespaced identifier"
                ),
                Some(&format!("{object_ref}.governance.policyRefs[{index}]")),
                Some("Use https://... or vendor:policy-id"),
                DiagnosticStage::CanonicalObjectModel,
            );
        }
    }
}

fn validate_provenance(
    ctx: &mut ValidationContext,
    provenance: &ProvenanceMetadata,
    object_ref: &str,
) {
    for (field, value) in [
        ("createdAt", provenance.created_at.as_deref()),
        ("modifiedAt", provenance.modified_at.as_deref()),
    ] {
        if let Some(timestamp) = value {
            if !is_iso8601_timestamp(timestamp) {
                ctx.error_with_stage(
                    codes::INVALID_METADATA,
                    DiagnosticCategory::Structure,
                    format!("provenance {field} '{timestamp}' is not a valid ISO-8601 timestamp"),
                    Some(&format!("{object_ref}.provenance.{field}")),
                    Some("Use an ISO-8601 date or datetime such as 2026-01-15T10:00:00Z"),
                    DiagnosticStage::CanonicalObjectModel,
                );
            }
        }
    }
}

fn has_restricted_governance(governance: Option<&GovernanceMetadata>) -> bool {
    governance.is_some_and(|governance| {
        governance
            .owner
            .as_ref()
            .is_some_and(|owner| !owner.trim().is_empty())
            || governance
                .steward
                .as_ref()
                .is_some_and(|steward| !steward.trim().is_empty())
    })
}

fn is_valid_policy_ref(value: &str) -> bool {
    if value.starts_with("https://") {
        return value.len() > 8 && value.as_bytes().get(8) != Some(&b'/');
    }
    if value.starts_with("http://") {
        return value.len() > 7 && value.as_bytes().get(7) != Some(&b'/');
    }
    is_namespaced_identifier(value)
}

/// Lightweight ISO-8601 date/datetime validation without external dependencies.
fn is_iso8601_timestamp(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() {
        return false;
    }

    // Date: YYYY-MM-DD
    if value.len() == 10
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
    {
        if !value[..4].chars().all(|c| c.is_ascii_digit())
            || !value[5..7].chars().all(|c| c.is_ascii_digit())
            || !value[8..10].chars().all(|c| c.is_ascii_digit())
        {
            return false;
        }
        let year: u32 = value[..4].parse().unwrap_or(0);
        let month: u32 = value[5..7].parse().unwrap_or(0);
        let day: u32 = value[8..10].parse().unwrap_or(0);
        return valid_calendar_date(year, month, day);
    }

    // Datetime: YYYY-MM-DDTHH:MM:SS[.frac][Z|±HH:MM]
    if value.len() < 19 || !value.contains('T') {
        return false;
    }
    let date_part = &value[..10];
    if !is_iso8601_timestamp(date_part) {
        return false;
    }
    let time_part = &value[11..];
    let (time_part, offset) = split_datetime_offset(time_part);
    if let Some(offset) = offset {
        if !is_valid_utc_offset(offset) {
            return false;
        }
    }
    let segments: Vec<&str> = time_part.split(':').collect();
    if segments.len() != 3 {
        return false;
    }
    for (index, segment) in segments.iter().enumerate() {
        let digits = segment.split('.').next().unwrap_or(segment);
        if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let Ok(value) = digits.parse::<u32>() else {
            return false;
        };
        let valid = match index {
            0 => value <= 23,
            1 | 2 => value <= 59,
            _ => false,
        };
        if !valid {
            return false;
        }
    }
    true
}

fn valid_calendar_date(year: u32, month: u32, day: u32) -> bool {
    if !(1..=12).contains(&month) {
        return false;
    }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => 29,
        2 => 28,
        _ => return false,
    };
    (1..=max_day).contains(&day)
}

fn split_datetime_offset(time_part: &str) -> (&str, Option<&str>) {
    if let Some(stripped) = time_part.strip_suffix('Z') {
        return (stripped, Some("Z"));
    }
    if let Some(index) = time_part.rfind('+') {
        return (&time_part[..index], Some(&time_part[index + 1..]));
    }
    if time_part.len() > 6 {
        if let Some(index) = time_part.rfind('-') {
            if index > time_part.find(':').unwrap_or(0) {
                return (&time_part[..index], Some(&time_part[index + 1..]));
            }
        }
    }
    (time_part, None)
}

fn is_valid_utc_offset(offset: &str) -> bool {
    if offset == "Z" {
        return true;
    }
    let bytes = offset.as_bytes();
    if bytes.len() != 5 || bytes[2] != b':' {
        return false;
    }
    if !offset[..2].chars().all(|c| c.is_ascii_digit())
        || !offset[3..].chars().all(|c| c.is_ascii_digit())
    {
        return false;
    }
    let hour: u32 = offset[..2].parse().unwrap_or(99);
    let minute: u32 = offset[3..].parse().unwrap_or(99);
    hour <= 14 && minute <= 59
}
