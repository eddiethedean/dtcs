//! Security checklist probes (SPEC Chapter 24).

use std::path::{Path, PathBuf};

use include_dir::{include_dir, Dir, DirEntry};

use crate::diagnostics::codes;
use crate::model::ExtensionCompatibility;
use crate::parser::{parse, DocumentFormat};
use crate::registry::{default_registry, load_bytes, resolve as resolve_registry};
use crate::{parse_and_validate, validate_with_registry};

use super::fixtures::read_fixture;
use super::model::ConformanceTestResult;

static EMBEDDED_RUNTIME_SRC: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/runtime");
static EMBEDDED_VALIDATION_SRC: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/validation");
static EMBEDDED_PARSER_SRC: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/parser");
static EMBEDDED_CONFORMANCE_SRC: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/conformance");

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
    let content = match read_fixture(fixtures_dir, "invalid_rule_duplicate_params.json") {
        Ok(bytes) => bytes,
        Err(err) => {
            return fail("contract-integrity", err);
        }
    };
    let report = parse_and_validate(&content, DocumentFormat::Json);
    if report.is_valid() {
        return fail(
            "contract-integrity",
            "duplicate JSON parameter keys must be rejected".to_string(),
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
    let evil = match load_registry_fixture(fixtures_dir, "registry/evil_dtcs_injection.yaml") {
        Ok(doc) => doc,
        Err(err) => {
            return fail("registry-trust", format!("load evil registry: {err}"));
        }
    };
    let mut merged = default_registry().clone();
    match merged.merge(&evil) {
        Ok(()) => fail(
            "registry-trust",
            "novel dtcs: registry entries must be rejected on merge".to_string(),
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
    let catalog = match load_registry_fixture(fixtures_dir, "registry/vendor_catalog.yaml") {
        Ok(doc) => doc,
        Err(err) => {
            return fail("trusted-extensions", format!("load vendor catalog: {err}"));
        }
    };
    let blocked = match resolve_registry(&catalog, "blocked") {
        Some(entry) => entry,
        None => {
            return fail(
                "trusted-extensions",
                "blocked extension entry missing".to_string(),
            );
        }
    };
    if blocked.compatibility == Some(ExtensionCompatibility::Mandatory) && !blocked.supported {
        pass("trusted-extensions")
    } else {
        fail(
            "trusted-extensions",
            "mandatory unsupported extensions must remain blocked".to_string(),
        )
    }
}

fn load_registry_fixture(
    fixtures_dir: &Path,
    relative: &str,
) -> Result<crate::model::RegistryDocument, String> {
    let bytes = read_fixture(fixtures_dir, relative)?;
    load_bytes(&bytes, DocumentFormat::Yaml).map_err(|report| {
        report
            .diagnostics
            .first()
            .map(|d| d.message.clone())
            .unwrap_or_else(|| "registry load failed".into())
    })
}

fn probe_diagnostics_stability(fixtures_dir: &Path) -> ConformanceTestResult {
    let content = match read_fixture(fixtures_dir, "missing_lineage.yaml") {
        Ok(bytes) => bytes,
        Err(err) => {
            return fail("diagnostics-stability", err);
        }
    };
    let parsed = parse(&content, DocumentFormat::Yaml);
    let Some(contract) = parsed.contract else {
        return fail(
            "diagnostics-stability",
            "expected parse success".to_string(),
        );
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
    // Split literals so this probe file does not self-match the scanned patterns.
    const FORBIDDEN: &[&str] = &[
        concat!("req", "west"),
        concat!("ur", "eq"),
        concat!("hyper", "::"),
        concat!("tokio", "::net"),
        concat!("std", "::net::"),
    ];
    let live_roots = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/runtime"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/validation"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/parser"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/conformance"),
    ];

    let mut files_scanned = 0usize;
    let mut live_dirs = 0usize;
    for root in &live_roots {
        if !root.is_dir() {
            continue;
        }
        live_dirs += 1;
        for entry in walkdir_rs(root) {
            if entry.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            // This probe embeds the forbidden tokens; skip self-scan.
            if entry.file_name().and_then(|n| n.to_str()) == Some("security.rs") {
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
            files_scanned += 1;
            if let Some(pattern) = FORBIDDEN.iter().find(|p| content.contains(*p)) {
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

    if live_dirs == 0 || files_scanned == 0 {
        // Packaged builds: scan the same trees embedded at compile time (fail-closed).
        let embedded = [
            &EMBEDDED_RUNTIME_SRC,
            &EMBEDDED_VALIDATION_SRC,
            &EMBEDDED_PARSER_SRC,
            &EMBEDDED_CONFORMANCE_SRC,
        ];
        let mut embedded_files = Vec::new();
        for dir in embedded {
            collect_embedded_rs(dir, &mut embedded_files);
        }
        if embedded_files.is_empty() {
            return fail(
                "no-network-surface",
                "source roots unavailable and embedded scan found zero Rust files".to_string(),
            );
        }
        for (path, content) in embedded_files {
            if path.ends_with("security.rs") {
                continue;
            }
            files_scanned += 1;
            if let Some(pattern) = FORBIDDEN.iter().find(|p| content.contains(*p)) {
                return fail(
                    "no-network-surface",
                    format!("forbidden network pattern '{pattern}' found in embedded {path}"),
                );
            }
        }
    }

    ConformanceTestResult {
        id: "no-network-surface".to_string(),
        profile: "security".into(),
        passed: true,
        message: Some(format!(
            "core parser/validation/runtime/conformance sources contain no network client imports ({files_scanned} files scanned)"
        )),
    }
}

fn collect_embedded_rs(dir: &Dir<'_>, out: &mut Vec<(String, String)>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(nested) => collect_embedded_rs(nested, out),
            DirEntry::File(file) => {
                if file.path().extension().and_then(|e| e.to_str()) == Some("rs") {
                    if let Ok(text) = std::str::from_utf8(file.contents()) {
                        out.push((file.path().display().to_string(), text.to_string()));
                    }
                }
            }
        }
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

fn pass(id: &str) -> ConformanceTestResult {
    ConformanceTestResult {
        id: id.into(),
        profile: "security".into(),
        passed: true,
        message: None,
    }
}

fn fail(id: &str, message: String) -> ConformanceTestResult {
    ConformanceTestResult {
        id: id.into(),
        profile: "security".into(),
        passed: false,
        message: Some(message),
    }
}

/// Runs a single named security probe (used by conformance assertions).
#[must_use]
pub fn run_security_probe(probe_id: &str, fixtures_dir: &Path) -> ConformanceTestResult {
    let results = run_security_probes(fixtures_dir);
    results
        .into_iter()
        .find(|r| r.id == probe_id)
        .unwrap_or_else(|| fail(probe_id, format!("unknown security probe '{probe_id}'")))
}
