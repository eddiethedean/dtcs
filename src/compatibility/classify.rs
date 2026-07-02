//! Compatibility classification (SPEC Chapter 11 §4).

use super::compare::{compare_contracts, diagnostics_from_outcome, ComparisonOutcome};
use super::report::CompatibilityReport;
use super::types::{AspectResult, ComparisonScope, CompatibilityLevel};
use crate::model::TransformationContract;

/// Analyze compatibility between two validated contracts.
#[must_use]
pub fn analyze(
    source: &TransformationContract,
    target: &TransformationContract,
    scope: ComparisonScope,
) -> CompatibilityReport {
    let outcome = compare_contracts(source, target, scope);
    let level = classify_pair(source, target, scope);
    let aspects = aspect_summaries(&outcome, scope);
    let mut diagnostics = diagnostics_from_outcome(&outcome);
    if matches!(level, CompatibilityLevel::Incompatible) {
        diagnostics.push(
            crate::diagnostics::Diagnostic::new(
                crate::diagnostics::codes::INCOMPATIBLE_CONTRACT,
                crate::diagnostics::Severity::Error,
                crate::diagnostics::DiagnosticStage::Analysis,
                crate::diagnostics::DiagnosticCategory::Compatibility,
                "contracts are incompatible within the requested scope",
            )
            .with_remediation("Review breaking changes and align interfaces, types, or semantics"),
        );
    }

    CompatibilityReport {
        level,
        aspects,
        diagnostics,
    }
}

/// Classify with reverse comparison to detect forward compatibility.
pub(crate) fn classify_pair(
    source: &TransformationContract,
    target: &TransformationContract,
    scope: ComparisonScope,
) -> CompatibilityLevel {
    let forward = compare_contracts(source, target, scope);
    if forward.is_clean() {
        return CompatibilityLevel::Identical;
    }
    let reverse = compare_contracts(target, source, scope);
    let backward_safe = !forward.has_breaking();
    let forward_safe = !reverse.has_breaking();

    if !backward_safe && !forward_safe {
        if forward.has_conditional()
            && !forward.diffs.iter().any(|d| {
                matches!(d.kind, super::types::DiffKind::Breaking)
                    && d.category != super::types::ChangeCategory::Extension
            })
        {
            return CompatibilityLevel::ConditionallyCompatible;
        }
        return CompatibilityLevel::Incompatible;
    }

    if backward_safe && forward_safe {
        if forward.has_conditional() || reverse.has_conditional() {
            return CompatibilityLevel::ConditionallyCompatible;
        }
        if forward.has_additive() && !reverse.has_additive() {
            return CompatibilityLevel::BackwardCompatible;
        }
        if reverse.has_additive() && !forward.has_additive() {
            return CompatibilityLevel::ForwardCompatible;
        }
        if forward.has_additive() && reverse.has_additive() {
            return if interface_footprint(target) >= interface_footprint(source) {
                CompatibilityLevel::BackwardCompatible
            } else {
                CompatibilityLevel::ForwardCompatible
            };
        }
        return CompatibilityLevel::Incompatible;
    }

    if backward_safe {
        if forward.has_conditional() {
            return CompatibilityLevel::ConditionallyCompatible;
        }
        return CompatibilityLevel::BackwardCompatible;
    }

    if forward_safe {
        if reverse.has_conditional() {
            return CompatibilityLevel::ConditionallyCompatible;
        }
        return CompatibilityLevel::ForwardCompatible;
    }

    CompatibilityLevel::Incompatible
}

fn interface_footprint(contract: &TransformationContract) -> usize {
    let mut count = contract.inputs.len() + contract.outputs.len();
    for input in &contract.inputs {
        if let Some(schema) = &input.schema {
            count += schema.fields.len();
        }
    }
    for output in &contract.outputs {
        if let Some(schema) = &output.schema {
            count += schema.fields.len();
        }
    }
    count
}

fn aspect_summaries(outcome: &ComparisonOutcome, scope: ComparisonScope) -> Vec<AspectResult> {
    let mut aspects = Vec::new();
    for (enabled, name) in [
        (scope.interfaces, "interfaces"),
        (scope.types, "types"),
        (scope.semantics, "semantics"),
        (scope.lineage, "lineage"),
        (scope.metadata, "metadata"),
        (scope.extensions, "extensions"),
    ] {
        if !enabled {
            continue;
        }
        let category = match name {
            "interfaces" => super::types::ChangeCategory::Interface,
            "types" => super::types::ChangeCategory::Type,
            "semantics" => super::types::ChangeCategory::Semantic,
            "lineage" => super::types::ChangeCategory::Lineage,
            "metadata" => super::types::ChangeCategory::Metadata,
            _ => super::types::ChangeCategory::Extension,
        };
        let diffs: Vec<_> = outcome
            .diffs
            .iter()
            .filter(|d| {
                d.category == category
                    || (name == "types" && d.category == super::types::ChangeCategory::Type)
            })
            .filter(|d| d.kind != super::types::DiffKind::Neutral)
            .collect();
        let compatible = diffs.is_empty();
        aspects.push(AspectResult {
            aspect: name.into(),
            compatible,
            message: if compatible {
                format!("{name} match within scope")
            } else {
                format!("{} difference(s) in {name}", diffs.len())
            },
        });
    }
    aspects
}
