//! Offline cache for URI-loaded registry documents.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use sha2::{Digest, Sha256};

use crate::diagnostics::{
    codes, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage, Severity,
};
use crate::model::RegistryDocument;
use crate::parser::DocumentFormat;

use super::load::{self, MAX_REGISTRY_BYTES};

/// Default cache directory name under the user cache root.
pub const CACHE_DIR_NAME: &str = "dtcs/registries";

/// Maximum number of cached registry files retained.
pub const MAX_CACHE_ENTRIES: usize = 100;

/// Maximum total size of all cached registry files.
pub const MAX_CACHE_BYTES: u64 = 64 * 1024 * 1024;

/// Resolve the offline registry cache directory.
///
/// Uses `DTCS_REGISTRY_CACHE` when set; otherwise `~/.cache/dtcs/registries`
/// (or the platform equivalent via `HOME` / `USERPROFILE`).
#[must_use]
pub fn cache_dir() -> PathBuf {
    if let Ok(path) = std::env::var("DTCS_REGISTRY_CACHE") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        return PathBuf::from(home).join(".cache").join(CACHE_DIR_NAME);
    }
    std::env::temp_dir().join("dtcs").join("registries")
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
    touch_cache_file(&path);
    // Cached registries may be stored under a path whose extension was derived
    // from the URI. To avoid format/extension mismatches, parse by sniffing both
    // supported encodings.
    let metadata = fs::metadata(&path).map_err(io_error)?;
    if metadata.len() as usize > MAX_REGISTRY_BYTES {
        return Err(oversize_error(metadata.len() as usize));
    }
    let bytes = fs::read(&path).map_err(io_error)?;
    match load::load_bytes(&bytes, DocumentFormat::Yaml) {
        Ok(document) => Ok(document),
        Err(yaml_error) => match load::load_bytes(&bytes, DocumentFormat::Json) {
            Ok(document) => Ok(document),
            Err(_json_error) => Err(yaml_error),
        },
    }
}

/// Store registry bytes in the offline cache for `uri`.
pub fn cache_store(
    uri: &str,
    content: &[u8],
    format: DocumentFormat,
) -> Result<PathBuf, DiagnosticReport> {
    if content.len() > MAX_REGISTRY_BYTES {
        return Err(oversize_error(content.len()));
    }
    // Validate before caching.
    let _document = load::load_bytes(content, format)?;
    let path = cache_path_for_uri(uri);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    fs::write(&path, content).map_err(io_error)?;
    touch_cache_file(&path);
    prune_cache_if_needed()?;
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

fn touch_cache_file(path: &Path) {
    let now = SystemTime::now();
    let _ = fs::File::open(path).and_then(|file| file.set_modified(now));
}

fn prune_cache_if_needed() -> Result<(), DiagnosticReport> {
    let dir = cache_dir();
    if !dir.exists() {
        return Ok(());
    }

    let mut entries = Vec::new();
    let read_dir = fs::read_dir(&dir).map_err(io_error)?;
    for entry in read_dir {
        let entry = entry.map_err(io_error)?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let metadata = entry.metadata().map_err(io_error)?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        entries.push((path, metadata.len(), modified));
    }

    entries.sort_by_key(|(_, _, modified)| *modified);

    let mut total_bytes: u64 = entries.iter().map(|(_, size, _)| *size).sum();
    let mut count = entries.len();

    for (path, size, _) in entries {
        if count <= MAX_CACHE_ENTRIES && total_bytes <= MAX_CACHE_BYTES {
            break;
        }
        if fs::remove_file(&path).is_ok() {
            count = count.saturating_sub(1);
            total_bytes = total_bytes.saturating_sub(size);
        }
    }

    Ok(())
}
