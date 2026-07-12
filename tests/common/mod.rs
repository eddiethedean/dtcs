//! Shared helpers for integration tests.

#![allow(dead_code)]

use std::path::PathBuf;

use dtcs::Diagnostic;

/// Returns diagnostic codes as a sorted multiset (duplicates preserved).
pub fn diagnostic_code_multiset(diagnostics: &[Diagnostic]) -> Vec<String> {
    let mut codes: Vec<String> = diagnostics.iter().map(|d| d.id.clone()).collect();
    codes.sort();
    codes
}

/// Asserts the diagnostic report contains exactly the expected code multiset.
#[allow(dead_code)]
pub fn assert_exact_diagnostic_codes(diagnostics: &[Diagnostic], expected: &[&str]) {
    let actual = diagnostic_code_multiset(diagnostics);
    let mut expected_sorted: Vec<String> = expected.iter().map(|s| (*s).to_string()).collect();
    expected_sorted.sort();
    assert_eq!(
        actual, expected_sorted,
        "diagnostic code multiset mismatch\n  expected: {expected_sorted:?}\n  actual:   {actual:?}"
    );
}

/// Loads exact expected validation codes for a fixture from `fixture_expectations.json`.
pub fn load_fixture_expected_codes(file: &str) -> Option<Vec<String>> {
    #[derive(serde::Deserialize)]
    struct Entry {
        file: String,
        codes: Option<Vec<String>>,
    }
    #[derive(serde::Deserialize)]
    struct Manifest {
        fixtures: Vec<Entry>,
    }
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixture_expectations.json");
    let manifest: Manifest = serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()?;
    manifest
        .fixtures
        .into_iter()
        .find(|entry| entry.file == file)
        .and_then(|entry| entry.codes)
}

/// Assert validation diagnostics match the fixture expectations manifest exactly.
pub fn assert_fixture_validation_codes(file: &str, diagnostics: &[Diagnostic]) {
    let expected = load_fixture_expected_codes(file)
        .unwrap_or_else(|| panic!("fixture {file} missing codes in fixture_expectations.json"));
    let expected_refs: Vec<&str> = expected.iter().map(String::as_str).collect();
    assert_exact_diagnostic_codes(diagnostics, &expected_refs);
}
