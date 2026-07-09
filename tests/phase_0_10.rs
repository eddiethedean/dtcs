//! Phase 0.10 integration tests — conformance profiles and offline suite.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dtcs::conformance::{declare, manifest, run_all, run_for_profiles, ImplementationClass};

#[test]
fn embedded_manifest_matches_tests_copy() {
    let tests_manifest =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/conformance/manifest.json");
    let on_disk: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&tests_manifest).expect("read manifest"))
            .expect("parse manifest");
    let embedded = serde_json::to_value(manifest()).expect("serialize embedded manifest");
    assert_eq!(on_disk, embedded);
}

#[test]
fn declares_eight_profiles() {
    let declaration = declare();
    assert_eq!(declaration.profiles.len(), 8);
    assert_eq!(declaration.primary_profile, "integrated-platform");
    assert_eq!(declaration.dtcs_version, dtcs::SPEC_VERSION);
}

#[test]
fn all_implementation_classes_have_profiles() {
    let declaration = declare();
    for class in ImplementationClass::ALL {
        assert!(
            declaration
                .profiles
                .iter()
                .any(|profile| profile.implementation_class == class),
            "missing profile for {:?}",
            class
        );
    }
}

#[test]
fn integrated_platform_conformance_passes() {
    let report = run_for_profiles(
        Some(&["integrated-platform".into()]),
        &dtcs::conformance::default_fixtures_dir(),
    );
    assert!(
        report.is_valid(),
        "integrated-platform conformance failed: {:?}",
        report
            .results
            .iter()
            .chain(report.security.iter())
            .filter(|r| !r.passed)
            .collect::<Vec<_>>()
    );
}

#[test]
fn full_conformance_suite_passes() {
    let report = run_all();
    assert!(report.is_valid(), "conformance suite failed: {report:#?}");
    assert!(report.passed);
    assert!(!report.results.is_empty());
    assert!(!report.security.is_empty());
}

#[test]
fn cli_conformance_run_all_exits_zero() {
    let bin = env!("CARGO_BIN_EXE_dtcs");
    let output = Command::new(bin)
        .args(["conformance", "run", "--profile", "all"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run conformance");
    assert!(
        output.status.success(),
        "stderr: {}\nstdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn cli_conformance_declare_json() {
    let bin = env!("CARGO_BIN_EXE_dtcs");
    let output = Command::new(bin)
        .args(["conformance", "declare", "--json"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run declare");
    assert!(output.status.success());
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("parse declaration json");
    assert_eq!(
        value.get("primaryProfile").and_then(|v| v.as_str()),
        Some("integrated-platform")
    );
}

#[test]
fn customer_normalize_integrated_platform_e2e() {
    let content = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/customer_normalize.dtcs.yaml"),
    )
    .expect("read customer_normalize");
    let inputs: dtcs::RuntimeInputs = serde_json::from_str(
        &fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/runtime/customer_normalize_input.json"),
        )
        .expect("read input"),
    )
    .expect("parse inputs");
    let result = dtcs::parse_validate_and_run(&content, dtcs::DocumentFormat::Yaml, &inputs);
    assert!(result.is_valid(), "{:?}", result.diagnostics);
}
