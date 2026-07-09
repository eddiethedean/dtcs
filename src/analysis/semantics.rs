use crate::analysis::{AnalysisFinding, AnalysisReport};
use crate::diagnostics::{codes, DiagnosticCategory, DiagnosticStage, Severity};
use crate::model::{ActionOrdering, RegistryCategory, RegistryDocument, TransformationContract};
use crate::registry;

/// Contract-level transformation semantics checks (SPEC Chapter 7).
#[must_use]
pub fn check_semantics(
    contract: &TransformationContract,
    registry_doc: &RegistryDocument,
) -> AnalysisReport {
    let mut report = AnalysisReport::default();

    check_action_composition(&mut report, contract);
    check_action_ordering(&mut report, contract);
    check_purity(&mut report, contract);
    check_determinism(&mut report, contract, registry_doc);
    check_lineage_consistency_warnings(&mut report, contract);

    report
}

fn check_action_composition(report: &mut AnalysisReport, contract: &TransformationContract) {
    if contract.semantic_actions.is_empty() {
        return;
    }

    if contract
        .semantics
        .as_ref()
        .and_then(|s| s.ordering.as_ref())
        .is_some()
    {
        return;
    }

    let mut seen = std::collections::HashMap::<&str, usize>::new();
    for action in &contract.semantic_actions {
        *seen.entry(action.target.as_str()).or_default() += 1;
    }

    for (target, count) in seen {
        if count > 1 {
            report.diagnostics.push(analysis_error(
                codes::INVALID_SEMANTICS,
                DiagnosticCategory::Semantic,
                format!(
                    "multiple semantic actions target '{target}' without an explicit ordering declaration"
                ),
                Some("semantics.ordering".into()),
                Some("Declare semantics.ordering or avoid overlapping semantic action targets".into()),
            ));
        }
    }
}

fn check_action_ordering(report: &mut AnalysisReport, contract: &TransformationContract) {
    let Some(semantics) = contract.semantics.as_ref() else {
        return;
    };
    let Some(ordering) = semantics.ordering.as_ref() else {
        return;
    };

    match ordering {
        ActionOrdering::Unordered => {}
        ActionOrdering::Explicit { order } => {
            let ids: std::collections::HashSet<_> = contract
                .semantic_actions
                .iter()
                .map(|a| a.id.as_str())
                .collect();

            let mut seen = std::collections::HashSet::new();
            for (index, id) in order.iter().enumerate() {
                if !ids.contains(id.as_str()) {
                    report.diagnostics.push(analysis_error(
                        codes::INVALID_SEMANTICS,
                        DiagnosticCategory::Semantic,
                        format!("semantics.ordering references unknown semantic action '{id}'"),
                        Some(format!("semantics.ordering.order[{index}]")),
                        Some("Reference only declared semantic action identifiers".into()),
                    ));
                } else if !seen.insert(id.as_str()) {
                    report.diagnostics.push(analysis_error(
                        codes::INVALID_SEMANTICS,
                        DiagnosticCategory::Semantic,
                        format!("semantics.ordering contains duplicate action id '{id}'"),
                        Some(format!("semantics.ordering.order[{index}]")),
                        Some("List each semantic action identifier at most once".into()),
                    ));
                }
            }

            for id in ids {
                if !seen.contains(id) {
                    report.diagnostics.push(analysis_error(
                        codes::INVALID_SEMANTICS,
                        DiagnosticCategory::Semantic,
                        format!("semantics.ordering is missing semantic action '{id}'"),
                        Some("semantics.ordering".into()),
                        Some("Include all semantic actions in the explicit order list".into()),
                    ));
                }
            }
        }
    }
}

fn check_purity(report: &mut AnalysisReport, contract: &TransformationContract) {
    let Some(semantics) = contract.semantics.as_ref() else {
        return;
    };

    match semantics.pure {
        Some(true) => {
            if !semantics.side_effects.is_empty() {
                report.diagnostics.push(analysis_error(
                    codes::INVALID_SEMANTICS,
                    DiagnosticCategory::Semantic,
                    "transformation declares pure: true but also declares side effects",
                    Some("semantics.sideEffects".into()),
                    Some("Remove sideEffects or set pure: false".into()),
                ));
            }
        }
        Some(false) if semantics.side_effects.is_empty() => {
            report.diagnostics.push(analysis_error(
                codes::INVALID_SEMANTICS,
                DiagnosticCategory::Semantic,
                "transformation declares pure: false but does not declare any side effects",
                Some("semantics.sideEffects".into()),
                Some("Declare sideEffects when pure is false".into()),
            ));
        }
        None => {}
        Some(false) => {}
    }
}

fn check_determinism(
    report: &mut AnalysisReport,
    contract: &TransformationContract,
    registry_doc: &RegistryDocument,
) {
    let deterministic = contract
        .semantics
        .as_ref()
        .and_then(|s| s.deterministic)
        .unwrap_or(false);
    if !deterministic {
        return;
    }

    // Functions declared in the contract may reference registry functions.
    for function in &contract.functions {
        if !function.function.starts_with("dtcs:") {
            continue;
        }
        let Some(entry) = registry::resolve(registry_doc, &function.function) else {
            continue;
        };
        if entry.category != RegistryCategory::Function {
            continue;
        }

        let Some(definition) = entry.definition.as_deref() else {
            continue;
        };
        let definition = definition.trim();
        if !definition.starts_with('{') {
            continue;
        }

        #[derive(serde::Deserialize)]
        struct Def {
            deterministic: Option<bool>,
        }
        let deterministic_fn = serde_json::from_str::<Def>(definition)
            .ok()
            .and_then(|d| d.deterministic)
            .unwrap_or(true);

        if !deterministic_fn {
            report.diagnostics.push(analysis_error(
                codes::NON_DETERMINISTIC_SEMANTICS,
                DiagnosticCategory::Semantic,
                format!(
                    "contract declares deterministic: true but references non-deterministic function '{}'",
                    function.function
                ),
                Some(format!("functions.{}.function", function.id)),
                Some("Remove the non-deterministic function or set semantics.deterministic: false".into()),
            ));
        }
    }
}

fn check_lineage_consistency_warnings(
    report: &mut AnalysisReport,
    contract: &TransformationContract,
) {
    let Some(lineage) = contract.lineage.as_ref() else {
        return;
    };
    let mapped_outputs: std::collections::HashSet<_> =
        lineage.mappings.iter().map(|m| m.output.as_str()).collect();

    for action in &contract.semantic_actions {
        let Some((interface_id, _)) = action.target.split_once('.') else {
            continue;
        };
        if !mapped_outputs.contains(interface_id) {
            report.findings.push(analysis_info_finding(
                format!("semanticActions.{}", action.id),
                "lineageCoverage",
                format!(
                    "semantic action targets '{interface_id}', but lineage mappings do not mention that output"
                ),
            ));
        }
    }
}

pub(crate) fn analysis_error(
    id: &str,
    category: DiagnosticCategory,
    message: impl Into<String>,
    object_ref: Option<String>,
    remediation: Option<String>,
) -> crate::diagnostics::Diagnostic {
    crate::diagnostics::Diagnostic {
        id: id.to_string(),
        severity: Severity::Error,
        stage: DiagnosticStage::Analysis,
        category,
        message: message.into(),
        object_ref,
        remediation,
    }
}

pub(crate) fn analysis_info_finding(
    object_ref: impl Into<String>,
    kind: impl Into<String>,
    message: impl Into<String>,
) -> AnalysisFinding {
    AnalysisFinding {
        object_ref: object_ref.into(),
        kind: kind.into(),
        message: message.into(),
        attributes: Default::default(),
    }
}
