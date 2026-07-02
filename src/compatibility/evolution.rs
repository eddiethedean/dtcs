//! Evolution analysis (SPEC Chapter 12).

use crate::diagnostics::{codes, Diagnostic, DiagnosticCategory, DiagnosticStage, Severity};
use crate::model::TransformationContract;

use super::classify::analyze;
use super::report::{ContractChange, EvolutionReport};
use super::types::{ChangeCategory, ComparisonScope, CompatibilityLevel};

/// Compare two revisions of a transformation contract.
#[must_use]
pub fn analyze_evolution(
    older: &TransformationContract,
    newer: &TransformationContract,
) -> EvolutionReport {
    let same_identity = older.id == newer.id;
    let mut diagnostics = Vec::new();

    if !same_identity {
        diagnostics.push(
            Diagnostic::new(
                codes::EVOLUTION_BREAKING_CHANGE,
                Severity::Error,
                DiagnosticStage::Analysis,
                DiagnosticCategory::Compatibility,
                format!(
                    "contract identity differs ('{}' vs '{}'); revisions must share the same id",
                    older.id, newer.id
                ),
            )
            .with_remediation("Compare revisions of the same logical contract id"),
        );
    }

    if older.version == newer.version && older != newer {
        diagnostics.push(
            Diagnostic::new(
                codes::VERSION_CONFLICT,
                Severity::Warning,
                DiagnosticStage::Analysis,
                DiagnosticCategory::Compatibility,
                format!(
                    "revisions share version '{}' but differ semantically",
                    older.version
                ),
            )
            .with_object_ref("version")
            .with_remediation("Use distinct version identifiers for distinct revisions"),
        );
    }

    let compatibility = analyze(older, newer, ComparisonScope::all());
    let compare_outcome = super::compare::compare_contracts(older, newer, ComparisonScope::all());
    let mut changes = compare_outcome.changes;

    changes.extend(detect_deprecations(newer));
    let migration_hints = migration_hints_for(&compatibility.level, &changes);

    diagnostics.extend(compatibility.diagnostics);

    EvolutionReport {
        source_id: older.id.clone(),
        target_id: newer.id.clone(),
        same_identity,
        compatibility: compatibility.level,
        changes,
        migration_hints,
        diagnostics,
    }
}

fn detect_deprecations(contract: &TransformationContract) -> Vec<ContractChange> {
    let mut changes = Vec::new();
    if let Some(metadata) = &contract.metadata {
        if metadata.deprecated == Some(true) {
            changes.push(ContractChange {
                category: ChangeCategory::Metadata,
                message: "contract metadata marks this revision as deprecated".into(),
                object_ref: Some("metadata.deprecated".into()),
            });
        }
        if metadata.replacement.is_some() {
            changes.push(ContractChange {
                category: ChangeCategory::Metadata,
                message: format!(
                    "replacement contract: {}",
                    metadata.replacement.as_deref().unwrap_or("")
                ),
                object_ref: Some("metadata.replacement".into()),
            });
        }
    }
    changes
}

fn migration_hints_for(level: &CompatibilityLevel, changes: &[ContractChange]) -> Vec<String> {
    let mut hints = Vec::new();
    match level {
        CompatibilityLevel::Identical => {}
        CompatibilityLevel::BackwardCompatible => {
            hints.push(
                "Newer revision is backward compatible; downstream consumers may adopt without input changes.".into(),
            );
        }
        CompatibilityLevel::ForwardCompatible => {
            hints.push(
                "Older revision remains usable with consumers expecting the newer interface superset.".into(),
            );
        }
        CompatibilityLevel::ConditionallyCompatible => {
            hints.push(
                "Review conditional differences (often extension keys) before deploying the newer revision.".into(),
            );
        }
        CompatibilityLevel::Incompatible => {
            hints.push(
                "Breaking changes detected; update consumers or provide a migration path before deployment.".into(),
            );
            for change in changes {
                if matches!(
                    change.category,
                    ChangeCategory::Interface | ChangeCategory::Type
                ) {
                    hints.push(format!(
                        "Address {} change: {}",
                        format!("{:?}", change.category).to_lowercase(),
                        change.message
                    ));
                }
            }
        }
    }
    hints
}
