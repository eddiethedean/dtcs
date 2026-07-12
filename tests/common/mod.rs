//! Shared helpers for integration tests.

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
