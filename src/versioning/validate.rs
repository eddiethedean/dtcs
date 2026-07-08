//! Version identifier validation (SPEC Chapter 25 §10).

use crate::diagnostics::{
    codes, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage, Severity,
};
use crate::model::{is_vendor_namespaced_identifier, TransformationContract};

/// Validate version identifiers on a transformation contract.
#[must_use]
pub fn validate(contract: &TransformationContract) -> DiagnosticReport {
    let mut diagnostics = Vec::new();

    if !is_valid_semver_like(&contract.version) {
        diagnostics.push(
            Diagnostic::new(
                codes::INVALID_VERSION,
                Severity::Error,
                DiagnosticStage::Validation,
                DiagnosticCategory::Compatibility,
                format!(
                    "contract version '{}' is not a valid semver identifier",
                    contract.version
                ),
            )
            .with_object_ref("version")
            .with_remediation("Use semver format such as 1.0.0 or 0.2.0"),
        );
    }

    if let Some(versioning) = &contract.versioning {
        if let Some(policy) = &versioning.compatibility {
            if policy.trim().is_empty() {
                diagnostics.push(
                    Diagnostic::new(
                        codes::INVALID_VERSION,
                        Severity::Error,
                        DiagnosticStage::Validation,
                        DiagnosticCategory::Compatibility,
                        "versioning.compatibility policy must be non-empty when declared",
                    )
                    .with_object_ref("versioning.compatibility"),
                );
            } else if !policy.starts_with("dtcs:") && !is_vendor_namespaced_identifier(policy) {
                diagnostics.push(
                    Diagnostic::new(
                        codes::INVALID_VERSION,
                        Severity::Error,
                        DiagnosticStage::Validation,
                        DiagnosticCategory::Compatibility,
                        format!("versioning.compatibility policy '{policy}' must be namespaced"),
                    )
                    .with_object_ref("versioning.compatibility")
                    .with_remediation("Use a dtcs: or vendor: compatibility policy identifier"),
                );
            }
        }
    }

    DiagnosticReport { diagnostics }
}

fn is_valid_semver_like(version: &str) -> bool {
    !version.trim().is_empty() && semver::Version::parse(version).is_ok()
}
