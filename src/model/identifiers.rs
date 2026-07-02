//! Shared identifier validation helpers.

/// Returns `true` when an identifier uses a namespace prefix.
#[must_use]
pub fn is_namespaced_identifier(identifier: &str) -> bool {
    let Some((prefix, suffix)) = identifier.split_once(':') else {
        return false;
    };
    !prefix.is_empty() && !suffix.is_empty()
}

/// Returns `true` for vendor-namespaced identifiers (excludes reserved prefixes).
#[must_use]
pub fn is_vendor_namespaced_identifier(identifier: &str) -> bool {
    let Some((prefix, suffix)) = identifier.split_once(':') else {
        return false;
    };
    !prefix.is_empty()
        && !suffix.is_empty()
        && prefix != "dtcs"
        && prefix != "http"
        && prefix != "https"
}
