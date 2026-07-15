//! Conformance fixture loading with embedded fallback for packaged builds.

use std::path::{Path, PathBuf};

use include_dir::{include_dir, Dir};

static EMBEDDED_FIXTURES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/fixtures");

/// Resolves the default fixtures directory.
///
/// Order: `DTCS_FIXTURES` env → `CARGO_MANIFEST_DIR/tests/fixtures` when present.
/// Packaged installs may not have an on-disk fixtures tree; conformance runners
/// fall back to the copy of `tests/fixtures` embedded in the binary.
#[must_use]
pub fn default_fixtures_dir() -> PathBuf {
    if let Ok(path) = std::env::var("DTCS_FIXTURES") {
        return PathBuf::from(path);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Reads a fixture relative to `fixtures_dir`, falling back to the embedded tree.
pub fn read_fixture(fixtures_dir: &Path, relative: &str) -> Result<Vec<u8>, String> {
    let normalized = relative.replace('\\', "/");
    let path = fixtures_dir.join(&normalized);
    match std::fs::read(&path) {
        Ok(bytes) => Ok(bytes),
        Err(fs_err) => match embedded_fixture(&normalized) {
            Some(bytes) => Ok(bytes.to_vec()),
            None => Err(format!(
                "read fixture '{relative}': {fs_err} (no embedded fallback)"
            )),
        },
    }
}

fn embedded_fixture(relative: &str) -> Option<&'static [u8]> {
    EMBEDDED_FIXTURES
        .get_file(relative)
        .map(include_dir::File::contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_customer_fixture_is_available() {
        let bytes = embedded_fixture("valid_customer.yaml").expect("embedded");
        assert!(std::str::from_utf8(bytes)
            .unwrap()
            .contains("customer.normalize"));
    }
}
