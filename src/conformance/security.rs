//! Security checklist probes (SPEC Chapter 24).

use std::path::{Path, PathBuf};

use crate::diagnostics::codes;
use crate::model::ExtensionCompatibility;
use crate::parser::{parse, DocumentFormat};
use crate::registry::{default_registry, load as load_registry, resolve as resolve_registry};
use crate::{parse_and_validate, validate_with_registry};

use super::model::ConformanceTestResult;

/// Runs automated security checklist probes.
#[must_use]
pub fn run_security_probes(fixtures_dir: &Path) -> Vec<ConformanceTestResult> {
    vec![
        probe_contract_integrity(fixtures_dir),
        probe_registry_trust(fixtures_dir),
        probe_trusted_extensions(fixtures_dir),
        probe_diagnostics_stability(fixtures_dir),
        probe_no_network_surface(),
    ]
}

fn probe_contract_integrity(fixtures_dir: &Path) -> ConformanceTestResult {
    let path = fixtures_dir.join("invalid_rule_duplicate_params.json");
    let content = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            return fail("contract-integrity", format!("read fixture: {err}"));
        }
    };
    let report = parse_and_validate(&content, DocumentFormat::Json);
    if report.is_valid() {
        return fail(
            "contract-integrity",
            "duplicate JSON parameter keys must be rejected".into(),
        );
    }
    let has_duplicate = report
        .diagnostics
        .iter()
        .any(|d| d.message.contains("duplicate key"));
    if has_duplicate {
        pass("contract-integrity")
    } else {
        fail(
            "contract-integrity",
            format!(
                "expected duplicate key diagnostic, got {:?}",
                report.diagnostics
            ),
        )
    }
}

fn probe_registry_trust(fixtures_dir: &Path) -> ConformanceTestResult {
    let path = fixtures_dir.join("registry/evil_dtcs_injection.yaml");
    let evil = match load_registry(&path) {
        Ok(doc) => doc,
        Err(err) => {
            return fail("registry-trust", format!("load evil registry: {err:?}"));
        }
    };
    let mut merged = default_registry().clone();
    match merged.merge(&evil) {
        Ok(()) => fail(
            "registry-trust",
            "novel dtcs: registry entries must be rejected on merge".into(),
        ),
        Err(report) => {
            let rejected = report.diagnostics.iter().any(|d| {
                d.id == codes::INVALID_REGISTRY && d.message.contains("novel standard entry")
            });
            if rejected {
                pass("registry-trust")
            } else {
                fail(
                    "registry-trust",
                    format!(
                        "expected novel dtcs: rejection, got {:?}",
                        report.diagnostics
                    ),
                )
            }
        }
    }
}

fn probe_trusted_extensions(fixtures_dir: &Path) -> ConformanceTestResult {
    let path = fixtures_dir.join("registry/vendor_catalog.yaml");
    let catalog = match load_registry(&path) {
        Ok(doc) => doc,
        Err(err) => {
            return fail(
                "trusted-extensions",
                format!("load vendor catalog: {err:?}"),
            );
        }
    };
    let blocked = match resolve_registry(&catalog, "blocked") {
        Some(entry) => entry,
        None => {
            return fail(
                "trusted-extensions",
                "blocked extension entry missing".into(),
            );
        }
    };
    if blocked.compatibility == Some(ExtensionCompatibility::Mandatory) && !blocked.supported {
        pass("trusted-extensions")
    } else {
        fail(
            "trusted-extensions",
            "mandatory unsupported extensions must remain blocked".into(),
        )
    }
}

fn probe_diagnostics_stability(fixtures_dir: &Path) -> ConformanceTestResult {
    let path = fixtures_dir.join("missing_lineage.yaml");
    let content = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            return fail("diagnostics-stability", format!("read fixture: {err}"));
        }
    };
    let parsed = parse(&content, DocumentFormat::Yaml);
    let Some(contract) = parsed.contract else {
        return fail("diagnostics-stability", "expected parse success".into());
    };
    let report = validate_with_registry(&contract, default_registry());
    let has_code = report
        .diagnostics
        .iter()
        .any(|d| d.id == codes::MISSING_LINEAGE);
    let leaks_path = report
        .diagnostics
        .iter()
        .any(|d| d.message.contains('/') || d.message.contains('\\'));
    if has_code && !leaks_path {
        pass("diagnostics-stability")
    } else {
        fail(
            "diagnostics-stability",
            format!(
                "expected stable dtcs code without paths: {:?}",
                report.diagnostics
            ),
        )
    }
}

fn probe_no_network_surface() -> ConformanceTestResult {
    const FORBIDDEN: &[&str] = &["reqwest", "ureq", "hyper::", "tokio::net", "std::net::"];
    let roots = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/runtime"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/validation"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/parser"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/conformance"),
    ];
    for root in roots {
        if !root.is_dir() {
            continue;
        }
        for entry in walkdir_rs(&root) {
            if !entry.ends_with(".rs") {
                continue;
            }
            let content = match std::fs::read_to_string(&entry) {
                Ok(content) => content,
                Err(err) => {
                    return fail(
                        "no-network-surface",
                        format!("read {}: {err}", entry.display()),
                    );
                }
            };
            for pattern in FORBIDDEN {
                if content.contains(pattern) {
                    return fail(
                        "no-network-surface",
                        format!(
                            "forbidden network pattern '{pattern}' found in {}",
                            entry.display()
                        ),
                    );
                }
            }
        }
    }
    ConformanceTestResult {
        id: "no-network-surface".into(),
        profile: "security".into(),
        passed: true,
        message: Some(
            "core parser/validation/runtime/conformance sources contain no network client imports"
                .into(),
        ),
    }
}

fn walkdir_rs(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files
}

fn pass(probe_id: &str) -> ConformanceTestResult {
    ConformanceTestResult {
        id: probe_id.into(),
        profile: "security".into(),
        passed: true,
        message: None,
    }
}

fn fail(probe_id: &str, message: String) -> ConformanceTestResult {
    ConformanceTestResult {
        id: probe_id.into(),
        profile: "security".into(),
        passed: false,
        message: Some(message),
    }
}

/// Runs a manifest security probe by identifier.
#[must_use]
pub fn run_security_probe(probe_id: &str, fixtures_dir: &Path) -> ConformanceTestResult {
    run_security_probes(fixtures_dir)
        .into_iter()
        .find(|result| result.id == probe_id)
        .unwrap_or_else(|| fail(probe_id, format!("unknown security probe: {probe_id}")))
}
