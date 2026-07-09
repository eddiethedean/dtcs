//! Load and validate registry documents from files.

use std::path::Path;

use crate::diagnostics::{
    codes, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage, Severity,
};
use crate::model::{is_namespaced_identifier, RegistryDocument, RegistryEntry};
use crate::parser::DocumentFormat;

/// Maximum registry document size (matches contract parser limit).
pub const MAX_REGISTRY_BYTES: usize = 16 * 1024 * 1024;

/// Load a registry document from a filesystem path.
///
/// Format is inferred from the extension (`.json` → JSON, otherwise YAML).
pub fn load(path: impl AsRef<Path>) -> Result<RegistryDocument, DiagnosticReport> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path).map_err(|error| {
        let mut report = DiagnosticReport::new();
        report.push(
            Diagnostic::new(
                codes::INVALID_REGISTRY,
                Severity::Error,
                DiagnosticStage::Parse,
                DiagnosticCategory::Syntax,
                format!("failed to read registry '{}': {error}", path.display()),
            )
            .with_remediation("Provide a readable registry file path"),
        );
        report
    })?;
    if metadata.len() as usize > MAX_REGISTRY_BYTES {
        return Err(oversize_error(metadata.len() as usize));
    }
    let bytes = std::fs::read(path).map_err(|error| {
        let mut report = DiagnosticReport::new();
        report.push(
            Diagnostic::new(
                codes::INVALID_REGISTRY,
                Severity::Error,
                DiagnosticStage::Parse,
                DiagnosticCategory::Syntax,
                format!("failed to read registry '{}': {error}", path.display()),
            )
            .with_remediation("Provide a readable registry file path"),
        );
        report
    })?;
    let format = if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        DocumentFormat::Json
    } else {
        DocumentFormat::Yaml
    };
    load_bytes(&bytes, format)
}

/// Load a registry document from bytes.
pub fn load_bytes(
    content: &[u8],
    format: DocumentFormat,
) -> Result<RegistryDocument, DiagnosticReport> {
    if content.len() > MAX_REGISTRY_BYTES {
        return Err(oversize_error(content.len()));
    }
    let mut document: RegistryDocument = match format {
        DocumentFormat::Yaml => serde_yaml::from_slice(content)
            .map_err(|error| parse_error(format!("invalid registry YAML: {error}")))?,
        DocumentFormat::Json => serde_json::from_slice(content)
            .map_err(|error| parse_error(format!("invalid registry JSON: {error}")))?,
    };

    // Fill entry ids from map keys when omitted.
    for (key, entry) in &mut document.entries {
        if entry.id.trim().is_empty() {
            entry.id = key.clone();
        }
    }

    let report = validate_document(&document);
    if report.is_valid() {
        Ok(document)
    } else {
        Err(report)
    }
}

/// Validate a registry document (Ch 22 §9).
#[must_use]
pub fn validate_document(document: &RegistryDocument) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if document.id.trim().is_empty() {
        report.push(registry_error(
            "registry id is required",
            Some("id"),
            Some("Provide a stable registry identifier"),
        ));
    }
    if document.version.trim().is_empty() {
        report.push(registry_error(
            "registry version is required",
            Some("version"),
            Some("Provide a registry version"),
        ));
    }
    if document.governing_specification.trim().is_empty() {
        report.push(registry_error(
            "governingSpecification is required",
            Some("governingSpecification"),
            Some("Provide the governing DTCS specification version"),
        ));
    }

    let mut seen = std::collections::HashSet::new();
    for (key, entry) in &document.entries {
        if key != &entry.id {
            report.push(registry_error(
                format!("entry key '{key}' does not match entry id '{}'", entry.id),
                Some(&format!("entries.{key}")),
                Some("Use the entry id as the map key"),
            ));
        }
        if !seen.insert(entry.id.clone()) {
            report.push(registry_error(
                format!("duplicate registry entry id '{}'", entry.id),
                Some(&format!("entries.{}", entry.id)),
                Some("Registry entry identifiers must be unique"),
            ));
        }
        validate_entry(&mut report, entry);
    }

    report
}

fn validate_entry(report: &mut DiagnosticReport, entry: &RegistryEntry) {
    if entry.id.trim().is_empty() {
        report.push(registry_error(
            "registry entry id is required",
            Some("entries"),
            Some("Provide a stable entry identifier"),
        ));
        return;
    }

    let is_namespace = matches!(
        entry.category,
        crate::model::RegistryCategory::ExtensionNamespace
    );
    if is_namespace {
        if entry.id.contains(':') {
            report.push(registry_error(
                format!(
                    "extension namespace '{}' must not include a colon",
                    entry.id
                ),
                Some(&format!("entries.{}", entry.id)),
                Some("Use a bare namespace prefix such as 'dtcs' or 'vendor'"),
            ));
        }
    } else if !is_namespaced_identifier(&entry.id) {
        report.push(registry_error(
            format!(
                "registry entry id '{}' must be a namespaced identifier",
                entry.id
            ),
            Some(&format!("entries.{}", entry.id)),
            Some("Use a namespaced identifier such as dtcs:lowercase"),
        ));
    }

    if entry.name.trim().is_empty() {
        report.push(registry_error(
            format!("registry entry '{}' is missing a name", entry.id),
            Some(&format!("entries.{}.name", entry.id)),
            Some("Provide a human-readable name"),
        ));
    }
    if entry.version.trim().is_empty() {
        report.push(registry_error(
            format!("registry entry '{}' is missing a version", entry.id),
            Some(&format!("entries.{}.version", entry.id)),
            Some("Provide an entry version"),
        ));
    }
}

fn oversize_error(size: usize) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.push(
        Diagnostic::new(
            codes::INVALID_REGISTRY,
            Severity::Error,
            DiagnosticStage::Parse,
            DiagnosticCategory::Syntax,
            format!(
                "registry document exceeds maximum size of {MAX_REGISTRY_BYTES} bytes (got {size})"
            ),
        )
        .with_remediation("Provide a smaller registry document"),
    );
    report
}

fn parse_error(message: impl Into<String>) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.push(
        Diagnostic::new(
            codes::INVALID_REGISTRY,
            Severity::Error,
            DiagnosticStage::Parse,
            DiagnosticCategory::Syntax,
            message,
        )
        .with_remediation("Provide a valid registry document"),
    );
    report
}

fn registry_error(
    message: impl Into<String>,
    object_ref: Option<&str>,
    remediation: Option<&str>,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::INVALID_REGISTRY,
        Severity::Error,
        DiagnosticStage::Validation,
        DiagnosticCategory::Structure,
        message,
    );
    if let Some(object_ref) = object_ref {
        diagnostic = diagnostic.with_object_ref(object_ref);
    }
    if let Some(remediation) = remediation {
        diagnostic = diagnostic.with_remediation(remediation);
    }
    diagnostic
}
