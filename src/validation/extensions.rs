//! Extension validation phase (SPEC Chapters 9 §10, 21).

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::{
    ExtensionCompatibility, RegistryCategory, RegistryDocument, TransformationContract,
};
use crate::registry;

use super::context::{
    is_namespaced_identifier, is_vendor_namespaced_identifier, ValidationContext,
};

pub(crate) fn validate_extensions(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
    registry: &RegistryDocument,
) {
    validate_extension_keys(ctx, contract, registry);
    for action in &contract.semantic_actions {
        validate_nested_extensions(
            ctx,
            registry,
            &action.extensions,
            &format!("semanticActions.{}", action.id),
        );
    }
    for function in &contract.functions {
        validate_nested_extensions(
            ctx,
            registry,
            &function.extensions,
            &format!("functions.{}", function.id),
        );
    }
    for expression in &contract.expressions {
        validate_nested_extensions(
            ctx,
            registry,
            &expression.extensions,
            &format!("expressions.{}", expression.id),
        );
    }
    for rule in &contract.rules {
        validate_nested_extensions(
            ctx,
            registry,
            &rule.extensions,
            &format!("rules.{}", rule.id),
        );
    }
}

fn validate_nested_extensions(
    ctx: &mut ValidationContext,
    registry: &RegistryDocument,
    extensions: &indexmap::IndexMap<String, serde_json::Value>,
    object_ref_prefix: &str,
) {
    for key in extensions.keys() {
        if !is_namespaced_identifier(key) {
            ctx.error(
                codes::UNKNOWN_FIELD,
                DiagnosticCategory::Structure,
                format!("unknown field '{key}' on {object_ref_prefix}"),
                Some(&format!("{object_ref_prefix}.{key}")),
                Some("Use vendor:fieldName for nested extensions"),
            );
            continue;
        }
        if !is_vendor_namespaced_identifier(key) {
            ctx.error(
                codes::INVALID_EXTENSION,
                DiagnosticCategory::Structure,
                format!("extension key '{key}' must use a vendor namespace"),
                Some(&format!("{object_ref_prefix}.{key}")),
                Some("Use vendor:fieldName for nested extensions; dtcs: is reserved"),
            );
            continue;
        }
        if let Some(entry) = registry::resolve(registry, key) {
            check_mandatory_support(ctx, key, entry);
        }
    }
}

/// Validates extension and unknown top-level fields captured by serde flatten.
pub fn validate_extension_keys(
    ctx: &mut ValidationContext,
    contract: &TransformationContract,
    registry: &RegistryDocument,
) {
    for key in contract.extensions.keys() {
        if key == "extensions" {
            ctx.error(
                codes::INVALID_EXTENSION,
                DiagnosticCategory::Structure,
                "vendor keys must be flattened at the contract level, not nested under 'extensions'",
                Some(key),
                Some("Use vendor:fieldName at the top level instead of an extensions wrapper"),
            );
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
            continue;
        }
        if !is_vendor_namespaced_identifier(key) {
            ctx.error(
                codes::INVALID_EXTENSION,
                DiagnosticCategory::Structure,
                format!("extension key '{key}' must use a vendor namespace"),
                Some(key),
                Some("Use vendor:fieldName for contract-level extensions; dtcs: is reserved"),
            );
            continue;
        }

        // Full-id match first, then namespace prefix.
        if let Some(entry) = registry::resolve(registry, key) {
            check_mandatory_support(ctx, key, entry);
            continue;
        }
        if let Some(namespace) = key.split_once(':').map(|(prefix, _)| prefix) {
            if let Some(entry) = registry::resolve(registry, namespace) {
                if entry.category == RegistryCategory::ExtensionNamespace {
                    check_mandatory_support(ctx, key, entry);
                }
            }
        }
    }
}

fn check_mandatory_support(
    ctx: &mut ValidationContext,
    key: &str,
    entry: &crate::model::RegistryEntry,
) {
    let mandatory = entry.compatibility == Some(ExtensionCompatibility::Mandatory);
    if mandatory && !entry.supported {
        ctx.error(
            codes::UNSUPPORTED_EXTENSION,
            DiagnosticCategory::Extension,
            format!("unsupported mandatory extension '{key}'"),
            Some(key),
            Some("Use a supported extension or remove the mandatory requirement"),
        );
    }
}
