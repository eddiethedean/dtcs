//! Document validation phase.

use crate::diagnostics::{codes, DiagnosticCategory};
use crate::model::TransformationContract;

use super::context::ValidationContext;

fn is_supported_dtcs_version(version: &str) -> bool {
    if crate::model::SUPPORTED_DTCS_VERSIONS.contains(&version) {
        return true;
    }
    semver::Version::parse(version)
        .map(|parsed| {
            parsed.major == 1
                && parsed.minor == 0
                && parsed.patch == 0
                && parsed.pre.is_empty()
                && parsed.build.is_empty()
        })
        .unwrap_or(false)
}

pub(crate) fn validate_document(ctx: &mut ValidationContext, contract: &TransformationContract) {
    if contract.dtcs_version.trim().is_empty() {
        ctx.error(
            codes::MISSING_REQUIRED_FIELD,
            DiagnosticCategory::Structure,
            "dtcsVersion is required",
            Some("dtcsVersion"),
            Some("Set dtcsVersion to a supported DTCS specification version"),
        );
    } else if !is_supported_dtcs_version(contract.dtcs_version.trim()) {
        let supported = crate::model::SUPPORTED_DTCS_VERSIONS.join(", ");
        ctx.error(
            codes::UNSUPPORTED_VERSION,
            DiagnosticCategory::Compatibility,
            format!(
                "unsupported dtcsVersion '{}'; supported versions: {supported}",
                contract.dtcs_version,
            ),
            Some("dtcsVersion"),
            Some("Use a supported DTCS specification version"),
        );
    }

    for (field, object_ref) in [("id", "id"), ("name", "name"), ("version", "version")] {
        let value = match field {
            "id" => contract.id.as_str(),
            "name" => contract.name.as_str(),
            "version" => contract.version.as_str(),
            _ => unreachable!(),
        };
        if value.trim().is_empty() {
            ctx.error(
                codes::MISSING_REQUIRED_FIELD,
                DiagnosticCategory::Structure,
                format!("{field} is required"),
                Some(object_ref),
                Some(&format!("Provide a non-empty {field}")),
            );
        }
    }
}
