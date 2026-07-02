//! Command-line interface.

use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::compatibility::{analyze as analyze_compatibility, analyze_evolution, ComparisonScope};
use crate::diagnostics::{inspect_contract, DiagnosticReport};
use crate::lineage::analyze_with_options;
use crate::model::TransformationContract;
use crate::parser::parse_file;

/// DTCS command-line tool.
#[derive(Debug, Parser)]
#[command(
    name = "dtcs",
    version,
    about = "Validate and analyze DTCS transformation contracts"
)]
pub struct Cli {
    #[command(subcommand)]
    /// Subcommand to execute.
    pub command: Command,
}

/// Supported CLI commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Parse and validate a contract.
    Validate {
        /// Path to a DTCS document.
        path: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Print a contract summary.
    Inspect {
        /// Path to a DTCS document.
        path: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Print validation diagnostics.
    Diagnostics {
        /// Path to a DTCS document.
        path: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Compare compatibility between two contracts.
    Compat {
        /// Source (older) contract path.
        source: PathBuf,
        /// Target (newer) contract path.
        target: PathBuf,
        /// Comparison scope (comma-separated: interfaces,types,semantics,lineage,metadata,extensions,all).
        #[arg(long, value_delimiter = ',')]
        scope: Vec<String>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Analyze evolution between two revisions.
    Evolve {
        /// Older revision path.
        older: PathBuf,
        /// Newer revision path.
        newer: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Analyze lineage for a contract.
    Lineage {
        /// Path to a DTCS document.
        path: PathBuf,
        /// List outputs affected by this input id.
        #[arg(long)]
        impact: Option<String>,
        /// List inputs required by this output id.
        #[arg(long)]
        dependency: Option<String>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Print tool and specification versions.
    Version {
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
}

/// Run the CLI application.
pub fn run(cli: Cli) -> miette::Result<i32> {
    match cli.command {
        Command::Validate { path, json } => {
            let result = parse_file(&path)?;
            let report = result.validate();
            render_report(&report, json, ReportMode::Validate)
                .map_err(|e| miette::miette!("{e}"))?;
            Ok(if report.is_valid() { 0 } else { 1 })
        }
        Command::Inspect { path, json } => {
            let contract = load_valid_contract(&path)?;
            if json {
                let summary = InspectSummary::from_contract(&contract);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&summary).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                print!("{}", inspect_contract(&contract));
            }
            Ok(0)
        }
        Command::Diagnostics { path, json } => {
            let result = parse_file(&path)?;
            let report = result.validate();
            render_report(&report, json, ReportMode::Diagnostics)
                .map_err(|e| miette::miette!("{e}"))?;
            Ok(if report.is_valid() { 0 } else { 1 })
        }
        Command::Compat {
            source,
            target,
            scope,
            json,
        } => {
            let source_contract = load_valid_contract(&source)?;
            let target_contract = load_valid_contract(&target)?;
            let scope = ComparisonScope::from_tokens(&scope);
            let report = analyze_compatibility(&source_contract, &target_contract, scope);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                println!("compatibility: {:?}", report.level);
                for aspect in &report.aspects {
                    println!("  {}: {}", aspect.aspect, aspect.message);
                }
                for diagnostic in &report.diagnostics {
                    println!(
                        "[{:?}] {} - {}",
                        diagnostic.severity, diagnostic.id, diagnostic.message
                    );
                }
            }
            Ok(if report.is_compatible() { 0 } else { 1 })
        }
        Command::Evolve { older, newer, json } => {
            let older_contract = load_valid_contract(&older)?;
            let newer_contract = load_valid_contract(&newer)?;
            let report = analyze_evolution(&older_contract, &newer_contract);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                println!(
                    "evolution: {:?} (same identity: {})",
                    report.compatibility, report.same_identity
                );
                for change in &report.changes {
                    println!("  [{:?}] {}", change.category, change.message);
                }
                for hint in &report.migration_hints {
                    println!("  hint: {hint}");
                }
            }
            Ok(
                if report.same_identity
                    && report.compatibility != crate::CompatibilityLevel::Incompatible
                {
                    0
                } else {
                    1
                },
            )
        }
        Command::Lineage {
            path,
            impact,
            dependency,
            json,
        } => {
            let contract = load_valid_contract(&path)?;
            let report = analyze_with_options(&contract, impact.as_deref(), dependency.as_deref());
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                for edge in &report.graph {
                    println!("{} <- {:?}", edge.output, edge.inputs);
                }
                if let Some(impact) = &report.impact {
                    println!("impact {} -> {:?}", impact.input, impact.outputs);
                }
                if let Some(dep) = &report.dependency {
                    println!("dependency {} <- {:?}", dep.output, dep.inputs);
                }
            }
            Ok(0)
        }
        Command::Version { json } => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "crateVersion": env!("CARGO_PKG_VERSION"),
                        "specVersion": crate::SPEC_VERSION,
                    })
                );
            } else {
                println!("dtcs {}", env!("CARGO_PKG_VERSION"));
                println!("spec {}", crate::SPEC_VERSION);
            }
            Ok(0)
        }
    }
}

fn load_valid_contract(path: &PathBuf) -> miette::Result<TransformationContract> {
    let result = parse_file(path)?;
    if !result.report.is_valid() {
        return Err(miette::miette!("parse failed for {}", path.display()));
    }
    result
        .contract
        .ok_or_else(|| miette::miette!("no contract in {}", path.display()))
        .and_then(|contract| {
            let report = crate::validate(&contract);
            if !report.is_valid() {
                return Err(miette::miette!("validation failed for {}", path.display()));
            }
            Ok(contract)
        })
}

#[derive(Debug)]
enum ReportMode {
    Validate,
    Diagnostics,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct InspectSummary {
    id: String,
    name: String,
    version: String,
    dtcs_version: String,
    inputs: usize,
    outputs: usize,
    semantic_actions: usize,
    rules: usize,
    expressions: usize,
    functions: usize,
}

impl InspectSummary {
    fn from_contract(contract: &crate::TransformationContract) -> Self {
        Self {
            id: contract.id.clone(),
            name: contract.name.clone(),
            version: contract.version.clone(),
            dtcs_version: contract.dtcs_version.clone(),
            inputs: contract.inputs.len(),
            outputs: contract.outputs.len(),
            semantic_actions: contract.semantic_actions.len(),
            rules: contract.rules.len(),
            expressions: contract.expressions.len(),
            functions: contract.functions.len(),
        }
    }
}

fn render_report(report: &DiagnosticReport, json: bool, mode: ReportMode) -> std::io::Result<()> {
    let mut stdout = io::stdout().lock();
    if json {
        let payload = match mode {
            ReportMode::Validate => serde_json::json!({
                "valid": report.is_valid(),
                "diagnostics": report.diagnostics,
            }),
            ReportMode::Diagnostics => serde_json::json!({
                "diagnostics": report.diagnostics,
            }),
        };
        writeln!(
            stdout,
            "{}",
            serde_json::to_string_pretty(&payload)
                .map_err(|e| std::io::Error::other(e.to_string()))?
        )?;
        return Ok(());
    }

    if report.diagnostics.is_empty() {
        match mode {
            ReportMode::Validate => writeln!(stdout, "valid")?,
            ReportMode::Diagnostics => writeln!(stdout, "no diagnostics")?,
        }
        return Ok(());
    }

    for diagnostic in &report.diagnostics {
        writeln!(
            stdout,
            "[{}] {} ({}) - {}",
            format!("{:?}", diagnostic.severity).to_lowercase(),
            diagnostic.id,
            format!("{:?}", diagnostic.category).to_lowercase(),
            diagnostic.message,
        )?;
        if let Some(object_ref) = &diagnostic.object_ref {
            writeln!(stdout, "  at: {object_ref}")?;
        }
        if let Some(remediation) = &diagnostic.remediation {
            writeln!(stdout, "  hint: {remediation}")?;
        }
    }

    if matches!(mode, ReportMode::Validate) && report.is_valid() {
        writeln!(stdout, "valid")?;
    }

    Ok(())
}
