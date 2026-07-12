//! Manifest-driven fixture expectations (mirrors Python parametrization).

mod common;

use std::fs;
use std::path::PathBuf;

use dtcs::{parse, parse_and_validate, DocumentFormat, ParseResult};
use serde::Deserialize;

use common::assert_exact_diagnostic_codes;

#[derive(Debug, Deserialize)]
struct ManifestEntry {
    file: String,
    parse_valid: bool,
    contract: bool,
    validate_valid: bool,
    codes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    fixtures: Vec<ManifestEntry>,
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn parse_fixture(name: &str) -> ParseResult {
    let content = fs::read(fixture(name)).expect("read fixture");
    let format = if name.ends_with(".json") {
        DocumentFormat::Json
    } else {
        DocumentFormat::Yaml
    };
    parse(&content, format)
}

fn load_manifest() -> Manifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixture_expectations.json");
    let content = fs::read_to_string(path).expect("read manifest");
    serde_json::from_str(&content).expect("parse manifest")
}

#[test]
fn manifest_fixture_expectations() {
    for entry in load_manifest().fixtures {
        let result = parse_fixture(&entry.file);
        assert_eq!(
            result.report.is_valid(),
            entry.parse_valid,
            "{} parse_valid",
            entry.file
        );
        assert_eq!(
            result.contract.is_some(),
            entry.contract,
            "{} contract",
            entry.file
        );

        let validate_valid = if let Some(contract) = result.contract.as_ref() {
            contract.validate().is_valid()
        } else {
            result.report.is_valid()
        };
        assert_eq!(
            validate_valid, entry.validate_valid,
            "{} validate_valid",
            entry.file
        );

        if let Some(expected_codes) = entry.codes {
            let report = if let Some(contract) = result.contract.as_ref() {
                contract.validate()
            } else {
                result.report
            };
            let expected: Vec<&str> = expected_codes.iter().map(String::as_str).collect();
            assert_exact_diagnostic_codes(&report.diagnostics, &expected);
        }
    }
}

#[test]
fn manifest_parse_and_validate_matches_validate() {
    for entry in load_manifest().fixtures {
        let content = fs::read(fixture(&entry.file)).expect("read fixture");
        let format = if entry.file.ends_with(".json") {
            DocumentFormat::Json
        } else {
            DocumentFormat::Yaml
        };
        let report = parse_and_validate(&content, format);
        assert_eq!(
            report.is_valid(),
            entry.validate_valid,
            "{} parse_and_validate",
            entry.file
        );
    }
}
