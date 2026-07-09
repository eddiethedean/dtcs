//! Dataset-level lineage analysis (SPEC Chapter 10 §11).

use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::diagnostics::{codes, Diagnostic, DiagnosticCategory, DiagnosticStage, Severity};
use crate::model::TransformationContract;

/// Directed lineage edge from output to inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageEdge {
    /// Output interface identifier.
    pub output: String,
    /// Contributing input identifiers.
    pub inputs: Vec<String>,
}

/// Governance context surfaced alongside lineage.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageGovernance {
    /// Contract owner from metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Contract steward from metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steward: Option<String>,
}

/// Lineage analysis report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageAnalysisReport {
    /// Output-to-input dependency edges.
    pub graph: Vec<LineageEdge>,
    /// Inputs that contribute to a given output (when requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency: Option<LineageDependencyResult>,
    /// Outputs affected by a given input (when requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<LineageImpactResult>,
    /// Governance summary.
    pub governance: LineageGovernance,
    /// Analysis warnings and errors.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}

/// Dependency analysis for one output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageDependencyResult {
    /// Output queried.
    pub output: String,
    /// Required input identifiers.
    pub inputs: Vec<String>,
}

/// Impact analysis for one input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageImpactResult {
    /// Input queried.
    pub input: String,
    /// Affected output identifiers.
    pub outputs: Vec<String>,
}

/// Analyze lineage for a validated contract.
#[must_use]
pub fn analyze(contract: &TransformationContract) -> LineageAnalysisReport {
    analyze_with_options(contract, None, None)
}

/// Analyze lineage with optional impact/dependency queries.
#[must_use]
pub fn analyze_with_options(
    contract: &TransformationContract,
    impact_input: Option<&str>,
    dependency_output: Option<&str>,
) -> LineageAnalysisReport {
    let graph = build_graph(contract);
    let reverse = reverse_graph(&graph);
    let mut diagnostics = Vec::new();

    let input_ids: HashSet<_> = contract
        .inputs
        .iter()
        .map(|input| input.id.as_str())
        .collect();
    let output_ids: HashSet<_> = contract
        .outputs
        .iter()
        .map(|output| output.id.as_str())
        .collect();

    if let Some(input) = impact_input {
        let in_graph = graph
            .iter()
            .any(|edge| edge.inputs.iter().any(|id| id == input));
        if input_ids.contains(input) && !in_graph {
            diagnostics.push(
                Diagnostic::new(
                    codes::UNRESOLVED_REFERENCE,
                    Severity::Warning,
                    DiagnosticStage::Analysis,
                    DiagnosticCategory::Reference,
                    format!(
                        "declared input '{input}' has no lineage graph edge; impact analysis may be incomplete"
                    ),
                )
                .with_object_ref(format!("inputs.{input}")),
            );
        } else if !input_ids.contains(input) && !in_graph {
            diagnostics.push(
                Diagnostic::new(
                    codes::UNRESOLVED_REFERENCE,
                    Severity::Warning,
                    DiagnosticStage::Analysis,
                    DiagnosticCategory::Reference,
                    format!("impact query input '{input}' was not found in the contract"),
                )
                .with_object_ref(format!("inputs.{input}")),
            );
        }
    }

    if let Some(output) = dependency_output {
        let in_graph = graph.iter().any(|edge| edge.output == output);
        if output_ids.contains(output) && !in_graph {
            diagnostics.push(
                Diagnostic::new(
                    codes::UNRESOLVED_REFERENCE,
                    Severity::Warning,
                    DiagnosticStage::Analysis,
                    DiagnosticCategory::Reference,
                    format!(
                        "declared output '{output}' has no lineage graph edge; dependency analysis may be incomplete"
                    ),
                )
                .with_object_ref(format!("outputs.{output}")),
            );
        } else if !output_ids.contains(output) && !in_graph {
            diagnostics.push(
                Diagnostic::new(
                    codes::UNRESOLVED_REFERENCE,
                    Severity::Warning,
                    DiagnosticStage::Analysis,
                    DiagnosticCategory::Reference,
                    format!("dependency query output '{output}' was not found in the contract"),
                )
                .with_object_ref(format!("outputs.{output}")),
            );
        }
    }

    LineageAnalysisReport {
        dependency: dependency_output.map(|output| LineageDependencyResult {
            output: output.into(),
            inputs: graph
                .iter()
                .find(|edge| edge.output == output)
                .map(|edge| edge.inputs.clone())
                .unwrap_or_default(),
        }),
        impact: impact_input.map(|input| LineageImpactResult {
            input: input.into(),
            outputs: transitive_impact(input, &reverse),
        }),
        governance: governance_from(contract),
        graph,
        diagnostics,
    }
}

fn build_graph(contract: &TransformationContract) -> Vec<LineageEdge> {
    contract
        .lineage
        .as_ref()
        .map(|lineage| {
            lineage
                .mappings
                .iter()
                .map(|mapping| LineageEdge {
                    output: mapping.output.clone(),
                    inputs: mapping.inputs.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn reverse_graph(graph: &[LineageEdge]) -> HashMap<String, Vec<String>> {
    let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
    for edge in graph {
        for input in &edge.inputs {
            reverse
                .entry(input.clone())
                .or_default()
                .push(edge.output.clone());
        }
    }
    reverse
}

fn transitive_impact(input: &str, reverse: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(input.to_string());

    while let Some(current) = queue.pop_front() {
        if let Some(outputs) = reverse.get(&current) {
            for output in outputs {
                if seen.insert(output.clone()) {
                    queue.push_back(output.clone());
                }
            }
        }
    }

    seen.remove(input);
    let mut outputs: Vec<_> = seen.into_iter().collect();
    outputs.sort();
    outputs
}

fn governance_from(contract: &TransformationContract) -> LineageGovernance {
    contract
        .metadata
        .as_ref()
        .and_then(|m| m.governance.as_ref())
        .map(|g| LineageGovernance {
            owner: g.owner.clone(),
            steward: g.steward.clone(),
        })
        .unwrap_or_default()
}
