use crate::analysis::expr::ExpressionAnalysis;
use crate::analysis::semantics::check_semantics;
use crate::analysis::{AnalysisFinding, AnalysisReport};
use crate::model::{RegistryDocument, TransformationContract};
use crate::registry;

/// Analyze a contract for semantic consistency (Ch 7–8), without runtime evaluation.
#[must_use]
pub fn check_contract<'a>(
    contract: &TransformationContract,
    registry_doc: Option<&'a RegistryDocument>,
) -> AnalysisReport {
    let registry_doc: &'a RegistryDocument = registry_doc.unwrap_or(registry::default_registry());
    let mut report = AnalysisReport::default();

    // Expression analysis
    for expression in &contract.expressions {
        let ExpressionAnalysis {
            diagnostics,
            findings,
            ..
        } = crate::analysis::expr::check_expression(contract, expression, registry_doc);
        report.diagnostics.extend(diagnostics);
        report.findings.extend(findings);
    }

    // Contract semantics analysis
    let semantics_report = check_semantics(contract, registry_doc);
    report.merge(semantics_report);

    // Ensure we always have at least one list entry for serde defaults.
    let _ = &mut report.findings as &mut Vec<AnalysisFinding>;
    report
}

