//! Phase 0.4 — Registries & Extensibility integration tests.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use dtcs::diagnostics::codes;
use dtcs::{
    default_registry, load_registry, parse, parse_and_validate, resolve_registry, validate,
    validate_with_registry, DocumentFormat, ExtensionCompatibility, RegistryCategory,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn registry_fixture(name: &str) -> PathBuf {
    fixture(&format!("registry/{name}"))
}

fn read(path: &Path) -> Vec<u8> {
    std::fs::read(path).expect("read fixture")
}

#[test]
fn resolves_builtin_action_and_rule() {
    let registry = default_registry();
    let action = resolve_registry(registry, "dtcs:lowercase").expect("action");
    assert_eq!(action.category, RegistryCategory::SemanticAction);
    assert_eq!(action.status.as_str(), "standard");

    let rule = resolve_registry(registry, "dtcs:not_null").expect("rule");
    assert_eq!(rule.category, RegistryCategory::Rule);

    let namespace = resolve_registry(registry, "dtcs").expect("namespace");
    assert_eq!(namespace.category, RegistryCategory::ExtensionNamespace);
}

#[test]
fn resolves_all_diagnostic_codes() {
    let registry = default_registry();
    for code in codes::ALL_CODES {
        let entry = resolve_registry(registry, code)
            .unwrap_or_else(|| panic!("missing diagnostic registry entry for {code}"));
        assert_eq!(entry.category, RegistryCategory::Diagnostic);
    }
}

#[test]
fn every_dtcs_identifier_in_valid_contracts_resolves() {
    // Exit criterion: every dtcs: identifier used by valid contracts resolves.
    // Invalid fixtures intentionally reference unknown ids and are excluded.
    let mut ids = HashSet::new();
    let roots = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"),
    ];
    for root in &roots {
        collect_dtcs_ids_from_valid_contracts(root, &mut ids);
    }

    // Diagnostic codes are part of the supported identifier surface.
    for code in codes::ALL_CODES {
        ids.insert((*code).to_string());
    }

    let registry = default_registry();
    let mut unresolved = Vec::new();
    for id in &ids {
        if resolve_registry(registry, id).is_none() {
            unresolved.push(id.clone());
        }
    }
    assert!(
        unresolved.is_empty(),
        "unresolved dtcs: identifiers in valid contracts: {unresolved:?}"
    );
}

fn collect_dtcs_ids_from_valid_contracts(dir: &Path, ids: &mut HashSet<String>) {
    let entries = std::fs::read_dir(dir).expect("read fixtures dir");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Skip external registry catalogs (not transformation contracts).
            if path.file_name().and_then(|n| n.to_str()) == Some("registry") {
                continue;
            }
            collect_dtcs_ids_from_valid_contracts(&path, ids);
            continue;
        }
        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };
        if !matches!(ext, "yaml" | "yml" | "json") {
            continue;
        }
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        let format = if ext.eq_ignore_ascii_case("json") {
            DocumentFormat::Json
        } else {
            DocumentFormat::Yaml
        };
        let report = parse_and_validate(&bytes, format);
        if !report.is_valid() {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for token in tokenize_dtcs_ids(&text) {
            ids.insert(token);
        }
    }
}

fn tokenize_dtcs_ids(text: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if text[i..].starts_with("dtcs:") {
            let start = i;
            i += 5;
            while i < bytes.len() {
                let ch = bytes[i] as char;
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                    i += 1;
                } else {
                    break;
                }
            }
            let id = &text[start..i];
            // Bare namespace "dtcs:" with no suffix is not an identifier.
            if id.len() > 5 {
                ids.push(id.to_string());
            }
        } else {
            i += 1;
        }
    }
    ids
}

#[test]
fn valid_contract_with_standard_identifiers_still_validates() {
    let report = parse_and_validate(&read(&fixture("valid_customer.yaml")), DocumentFormat::Yaml);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn unknown_standard_action_still_rejected() {
    let report = parse_and_validate(
        &read(&fixture("invalid_semantic_action.yaml")),
        DocumentFormat::Yaml,
    );
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_SEMANTIC_ACTION));
}

#[test]
fn unknown_standard_rule_still_rejected() {
    let report = parse_and_validate(&read(&fixture("invalid_rule.yaml")), DocumentFormat::Yaml);
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_RULE));
}

#[test]
fn loads_external_vendor_catalog() {
    let catalog = load_registry(registry_fixture("vendor_catalog.yaml")).expect("load catalog");
    let entry = resolve_registry(&catalog, "acme:transform").expect("vendor action");
    assert_eq!(entry.category, RegistryCategory::SemanticAction);
    assert_eq!(entry.name, "Acme Transform");
}

#[test]
fn merge_preserves_builtin_dtcs_entries() {
    let mut catalog = load_registry(registry_fixture("vendor_catalog.yaml")).expect("load");
    // Attempt to override a builtin entry via merge from a document that redefines it.
    catalog.insert(dtcs::RegistryEntry {
        id: "dtcs:lowercase".into(),
        name: "Hijacked".into(),
        category: RegistryCategory::SemanticAction,
        version: "9.9.9".into(),
        status: dtcs::RegistryEntryStatus::Draft,
        compatibility: None,
        definition: Some("should not win".into()),
        references: Vec::new(),
        supported: true,
    });

    let mut merged = default_registry().clone();
    merged.merge(&catalog);
    let entry = resolve_registry(&merged, "dtcs:lowercase").expect("builtin");
    assert_eq!(entry.name, "Lowercase");
    assert_eq!(entry.version, "1.0.0");

    let vendor = resolve_registry(&merged, "acme:transform").expect("vendor");
    assert_eq!(vendor.name, "Acme Transform");
}

#[test]
fn rejects_invalid_registry_document() {
    let err = load_registry(registry_fixture("invalid_duplicate.yaml")).expect_err("duplicate");
    assert!(!err.is_valid());
    assert!(err
        .diagnostics
        .iter()
        .any(|d| d.id == codes::INVALID_REGISTRY));
}

#[test]
fn optional_vendor_extension_is_preserved() {
    let bytes = read(&registry_fixture("vendor_optional_extension.yaml"));
    let result = parse(&bytes, DocumentFormat::Yaml);
    let contract = result.contract.expect("parsed");
    assert!(contract.extensions.contains_key("acme:feature"));

    let catalog = load_registry(registry_fixture("vendor_catalog.yaml")).expect("catalog");
    let mut registry = default_registry().clone();
    registry.merge(&catalog);
    let report = validate_with_registry(&contract, &registry);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn mandatory_unsupported_extension_fails_validation() {
    let bytes = read(&registry_fixture("vendor_mandatory_extension.yaml"));
    let result = parse(&bytes, DocumentFormat::Yaml);
    let contract = result.contract.expect("parsed");

    let catalog = load_registry(registry_fixture("vendor_catalog.yaml")).expect("catalog");
    let mut registry = default_registry().clone();
    registry.merge(&catalog);
    let report = validate_with_registry(&contract, &registry);
    assert!(!report.is_valid());
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::UNSUPPORTED_EXTENSION));
}

#[test]
fn unknown_vendor_extension_without_registry_is_valid() {
    let bytes = read(&registry_fixture("vendor_optional_extension.yaml"));
    let report = parse_and_validate(&bytes, DocumentFormat::Yaml);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn offline_uri_cache_roundtrip() {
    let uri = "https://example.invalid/dtcs/vendor_catalog.yaml";
    let content = read(&registry_fixture("vendor_catalog.yaml"));
    let path =
        dtcs::registry::store_uri_cache(uri, &content, DocumentFormat::Yaml).expect("cache store");
    assert!(path.exists());

    let loaded = dtcs::registry::load_uri_cached(uri).expect("cache load");
    assert!(resolve_registry(&loaded, "acme:transform").is_some());

    dtcs::registry::cache_remove(uri).expect("cache remove");
}

#[test]
fn offline_uri_cache_roundtrip_json_content_with_yaml_uri() {
    // Use a distinct URI from `offline_uri_cache_roundtrip` to avoid cache-key races when the
    // test harness runs tests in parallel.
    let uri = "https://example.invalid/dtcs/vendor_catalog_json_bytes.yaml";
    let yaml_bytes = read(&registry_fixture("vendor_catalog.yaml"));
    let registry =
        dtcs::registry::load_bytes(&yaml_bytes, DocumentFormat::Yaml).expect("parse fixture");
    let json_bytes = serde_json::to_vec(&registry).expect("to json");

    let path =
        dtcs::registry::store_uri_cache(uri, &json_bytes, DocumentFormat::Json).expect("store");
    assert!(path.exists());

    let loaded = dtcs::registry::load_uri_cached(uri).expect("cache load");
    assert!(resolve_registry(&loaded, "acme:transform").is_some());

    dtcs::registry::cache_remove(uri).expect("cache remove");
}

#[test]
fn cli_registry_list_and_resolve() {
    let bin = env!("CARGO_BIN_EXE_dtcs");

    let list = Command::new(bin)
        .args(["registry", "list"])
        .output()
        .expect("run list");
    assert!(list.status.success());
    let stdout = String::from_utf8_lossy(&list.stdout);
    assert!(stdout.contains("dtcs:lowercase"));
    assert!(stdout.contains("dtcs:not_null"));

    let resolve = Command::new(bin)
        .args(["registry", "resolve", "dtcs:lowercase", "--json"])
        .output()
        .expect("run resolve");
    assert!(resolve.status.success());
    let body = String::from_utf8_lossy(&resolve.stdout);
    assert!(body.contains("\"id\": \"dtcs:lowercase\""));

    let missing = Command::new(bin)
        .args(["registry", "resolve", "dtcs:does-not-exist"])
        .output()
        .expect("run resolve missing");
    assert_eq!(missing.status.code(), Some(1));
}

#[test]
fn cli_registry_resolve_with_vendor_file() {
    let bin = env!("CARGO_BIN_EXE_dtcs");
    let catalog = registry_fixture("vendor_catalog.yaml");
    let resolve = Command::new(bin)
        .args([
            "registry",
            "resolve",
            "acme:transform",
            "--registry",
            catalog.to_str().unwrap(),
        ])
        .output()
        .expect("run resolve vendor");
    assert!(resolve.status.success());
    let stdout = String::from_utf8_lossy(&resolve.stdout);
    assert!(stdout.contains("Acme Transform"));
}

#[test]
fn default_validate_uses_builtin_registry() {
    let report = validate(
        &parse(&read(&fixture("valid_customer.yaml")), DocumentFormat::Yaml)
            .contract
            .expect("contract"),
    );
    assert!(report.is_valid());
}

#[test]
fn extension_compatibility_roundtrips() {
    let catalog = load_registry(registry_fixture("vendor_catalog.yaml")).expect("catalog");
    let blocked = resolve_registry(&catalog, "blocked").expect("blocked");
    assert_eq!(
        blocked.compatibility,
        Some(ExtensionCompatibility::Mandatory)
    );
    assert!(!blocked.supported);
}
