//! Metadata validation rules (SPEC Chapter 5 §12).

use crate::diagnostics::{codes, DiagnosticCategory, DiagnosticStage};
use crate::model::{
    ClassificationLevel, GovernanceMetadata, IdentityMetadata, Metadata, ProvenanceMetadata,
    TransformationContract,
};
use crate::validation::context::{is_namespaced_identifier, ValidationContext};

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

    if let Some(provenance) = &metadata.provenance {
        validate_provenance(ctx, provenance, object_ref);
    }

    if metadata.classification == Some(ClassificationLevel::Restricted)
        && metadata.governance.is_none()
    {
        ctx.error_with_stage(
            codes::INVALID_METADATA,
            DiagnosticCategory::Structure,
            "restricted classification should include governance metadata",
            Some(object_ref),
            Some("Add governance.owner or governance.steward for restricted objects"),
            DiagnosticStage::CanonicalObjectModel,
        );
    }

    for key in metadata.extensions.keys() {
        if !is_namespaced_identifier(key) {
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
            if identifier != expected {
                ctx.error_with_stage(
                    codes::INVALID_METADATA,
                    DiagnosticCategory::Structure,
                    format!(
                        "metadata identity identifier '{identifier}' conflicts with object id '{expected}'"
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
            if name != expected {
                ctx.error_with_stage(
                    codes::INVALID_METADATA,
                    DiagnosticCategory::Structure,
                    format!(
                        "metadata identity name '{name}' conflicts with object name '{expected}'"
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
            if version != expected {
                ctx.error_with_stage(
                    codes::INVALID_METADATA,
                    DiagnosticCategory::Structure,
                    format!(
                        "metadata identity version '{version}' conflicts with object version '{expected}'"
                    ),
                    Some(&format!("{object_ref}.identity.version")),
                    Some("Remove identity.version or align it with the object version"),
                    DiagnosticStage::CanonicalObjectModel,
                );
            }
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

fn is_valid_policy_ref(value: &str) -> bool {
    value.starts_with("https://") || value.starts_with("http://") || is_namespaced_identifier(value)
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
        return value[..4].chars().all(|c| c.is_ascii_digit())
            && value[5..7].chars().all(|c| c.is_ascii_digit())
            && value[8..10].chars().all(|c| c.is_ascii_digit());
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
    let time_part = time_part.strip_suffix('Z').unwrap_or(time_part);
    let time_part = if let Some((base, _offset)) = time_part.split_once('+') {
        base
    } else if time_part.matches('-').count() >= 1 && time_part.len() > 8 {
        time_part.split_at(8).0
    } else {
        time_part
    };
    let segments: Vec<&str> = time_part.split(':').collect();
    if segments.len() < 2 || segments.len() > 3 {
        return false;
    }
    segments.iter().all(|segment| {
        let digits = segment.split('.').next().unwrap_or(segment);
        !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
    })
}
