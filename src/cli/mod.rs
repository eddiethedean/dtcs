//! Command-line interface.

use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::diagnostics::{inspect_contract, DiagnosticReport};
use crate::parser::parse_file;

/// DTCS command-line tool.
#[derive(Debug, Parser)]
#[command(
    name = "dtcs",
    version,
    about = "Validate DTCS transformation contracts"
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
            let result = parse_file(&path)?;
            let mut report = result.report;
            if let Some(ref contract) = result.contract {
                report.merge(crate::validate(contract));
            }
            if !report.is_valid() {
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            }
            let Some(contract) = result.contract else {
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            };
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
