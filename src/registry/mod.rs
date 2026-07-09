//! Identifier registry resolution (SPEC Chapters 21–22).
//!
//! Registries are authoritative catalogs of standardized identifiers
//! (`dtcs:lowercase`, diagnostic codes, extension namespaces, …).

mod builtin;
mod cache;
mod load;

use std::path::Path;
use std::sync::OnceLock;

use crate::diagnostics::DiagnosticReport;
use crate::model::{RegistryCategory, RegistryDocument, RegistryEntry};
use crate::parser::DocumentFormat;

pub use cache::{
    cache_dir, cache_path_for_uri, cache_remove, cache_store, is_cached, load_uri_cached,
};
pub use load::{load, load_bytes, validate_document, MAX_REGISTRY_BYTES};

/// Returns the embedded standard registry for this implementation.
#[must_use]
pub fn default_registry() -> &'static RegistryDocument {
    static REGISTRY: OnceLock<RegistryDocument> = OnceLock::new();
    REGISTRY.get_or_init(builtin::builtin_registry)
}

/// Resolve an identifier in `registry`.
#[must_use]
pub fn resolve<'a>(registry: &'a RegistryDocument, id: &str) -> Option<&'a RegistryEntry> {
    registry.get(id)
}

/// Resolve an identifier in the default (builtin) registry.
#[must_use]
pub fn resolve_default(id: &str) -> Option<&'static RegistryEntry> {
    resolve(default_registry(), id)
}

/// Load a registry from a path and merge it with the builtin catalog.
///
/// Builtin `dtcs:` entries remain authoritative.
pub fn load_merged(path: impl AsRef<Path>) -> Result<RegistryDocument, DiagnosticReport> {
    let mut registry = default_registry().clone();
    let loaded = load(path)?;
    registry.merge(&loaded)?;
    Ok(registry)
}

/// Load a registry from a URI using the offline cache, merged with builtin.
pub fn load_uri_merged(uri: &str) -> Result<RegistryDocument, DiagnosticReport> {
    let mut registry = default_registry().clone();
    let loaded = load_uri_cached(uri)?;
    registry.merge(&loaded)?;
    Ok(registry)
}

/// Returns `true` when `action` is a recognized semantic action identifier.
#[must_use]
pub fn is_known_action(action: &str) -> bool {
    resolve_default(action).is_some_and(|entry| entry.category == RegistryCategory::SemanticAction)
}

/// Returns `true` when `rule` is a recognized rule identifier.
#[must_use]
pub fn is_known_rule(rule: &str) -> bool {
    resolve_default(rule).is_some_and(|entry| entry.category == RegistryCategory::Rule)
}

/// Returns `true` when `function` is a recognized function identifier.
#[must_use]
pub fn is_known_function(function: &str) -> bool {
    resolve_default(function).is_some_and(|entry| entry.category == RegistryCategory::Function)
}

/// List entries from the default registry, optionally merged with a file.
pub fn list(registry_path: Option<&Path>) -> Result<Vec<RegistryEntry>, DiagnosticReport> {
    let registry = match registry_path {
        Some(path) => load_merged(path)?,
        None => default_registry().clone(),
    };
    Ok(registry.list().into_iter().cloned().collect())
}

/// Resolve an identifier, optionally using an additional registry file.
pub fn resolve_with_path(
    id: &str,
    registry_path: Option<&Path>,
) -> Result<Option<RegistryEntry>, DiagnosticReport> {
    let registry = match registry_path {
        Some(path) => load_merged(path)?,
        None => default_registry().clone(),
    };
    Ok(resolve(&registry, id).cloned())
}

/// Store registry content in the offline cache for later URI resolution.
pub fn store_uri_cache(
    uri: &str,
    content: &[u8],
    format: DocumentFormat,
) -> Result<std::path::PathBuf, DiagnosticReport> {
    cache_store(uri, content, format)
}
