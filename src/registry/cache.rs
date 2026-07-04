//! Offline cache for URI-loaded registry documents.

use std::fs;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use crate::diagnostics::{
    codes, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage, Severity,
};
use crate::model::RegistryDocument;
use crate::parser::DocumentFormat;

use super::load;

/// Default cache directory name under the user cache root.
pub const CACHE_DIR_NAME: &str = "dtcs/registries";

/// Resolve the offline registry cache directory.
///
/// Uses `DTCS_REGISTRY_CACHE` when set; otherwise `~/.cache/dtcs/registries`
/// (or the platform equivalent via `HOME` / `USERPROFILE`).
#[must_use]
pub fn cache_dir() -> PathBuf {
    if let Ok(path) = std::env::var("DTCS_REGISTRY_CACHE") {
        return PathBuf::from(path);
    }
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".cache").join(CACHE_DIR_NAME)
}

/// Load a registry from a URI using the offline cache only (no network).
///
/// Returns an error when the URI has not been cached previously.
pub fn load_uri_cached(uri: &str) -> Result<RegistryDocument, DiagnosticReport> {
    let path = cache_path_for_uri(uri);
    if !path.exists() {
        let mut report = DiagnosticReport::new();
        report.push(
            Diagnostic::new(
                codes::INVALID_REGISTRY,
                Severity::Error,
                DiagnosticStage::Parse,
                DiagnosticCategory::Syntax,
                format!("registry URI '{uri}' is not present in the offline cache"),
            )
            .with_remediation(
                "Cache the registry with registry::cache_store, or load from a local file path",
            ),
        );
        return Err(report);
    }
    load::load(&path)
}

/// Store registry bytes in the offline cache for `uri`.
pub fn cache_store(
    uri: &str,
    content: &[u8],
    format: DocumentFormat,
) -> Result<PathBuf, DiagnosticReport> {
    // Validate before caching.
    let _document = load::load_bytes(content, format)?;
    let path = cache_path_for_uri(uri);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    fs::write(&path, content).map_err(io_error)?;
    Ok(path)
}

/// Returns the cache file path for a URI.
#[must_use]
pub fn cache_path_for_uri(uri: &str) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(uri.as_bytes());
    let digest = hex_encode(&hasher.finalize());
    let ext = if uri.ends_with(".json") {
        "json"
    } else {
        "yaml"
    };
    cache_dir().join(format!("{digest}.{ext}"))
}

/// Returns `true` when `uri` is present in the offline cache.
#[must_use]
pub fn is_cached(uri: &str) -> bool {
    cache_path_for_uri(uri).exists()
}

fn io_error(error: std::io::Error) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.push(
        Diagnostic::new(
            codes::INVALID_REGISTRY,
            Severity::Error,
            DiagnosticStage::Parse,
            DiagnosticCategory::Syntax,
            format!("registry cache I/O failed: {error}"),
        )
        .with_remediation("Check cache directory permissions"),
    );
    report
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Remove a cached registry entry for `uri`, if present.
pub fn cache_remove(uri: &str) -> std::io::Result<()> {
    let path = cache_path_for_uri(uri);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
