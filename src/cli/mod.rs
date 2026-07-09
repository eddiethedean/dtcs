//! Command-line interface.

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::compatibility::{analyze as analyze_compatibility, analyze_evolution, ComparisonScope};
use crate::diagnostics::{
    codes, inspect_contract, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    Severity,
};
use crate::lineage::analyze_with_options;
use crate::model::TransformationContract;
use crate::parser::parse_file;
use crate::{analysis, validate_with_registry};

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
        /// Optional additional registry file to merge for validation.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Analyze transformation semantics and expressions.
    Analyze {
        /// Path to a DTCS document.
        path: PathBuf,
        /// Optional additional registry file to merge for analysis.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Lower a validated contract to a transformation plan.
    Plan {
        /// Path to a DTCS document.
        path: PathBuf,
        /// Optional additional registry file to merge for planning.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Optimize a transformation plan (contract or serialized plan JSON).
    Optimize {
        /// Path to a DTCS contract or serialized plan JSON.
        path: PathBuf,
        /// Treat the input path as serialized plan JSON from `dtcs plan --json`.
        #[arg(long)]
        plan: bool,
        /// Optional additional registry file to merge.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Skip post-optimization validation.
        #[arg(long)]
        no_validate: bool,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Match a transformation plan against engine capabilities.
    Match {
        /// Path to a DTCS contract or serialized plan JSON.
        path: PathBuf,
        /// Treat the input path as serialized plan JSON from `dtcs plan --json`.
        #[arg(long)]
        plan: bool,
        /// Apply plan optimization before matching.
        #[arg(long)]
        optimize: bool,
        /// Optional additional registry file to merge.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Engine profile identifier (default: `dtcs:reference`).
        #[arg(long, default_value = "dtcs:reference")]
        profile: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Compile a transformation plan to an execution plan.
    Compile {
        /// Path to a DTCS contract or serialized plan JSON.
        path: PathBuf,
        /// Treat the input path as serialized plan JSON from `dtcs plan --json`.
        #[arg(long)]
        plan: bool,
        /// Apply plan optimization before compilation.
        #[arg(long)]
        optimize: bool,
        /// Optional additional registry file to merge.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Engine profile identifier (default: `dtcs:reference`).
        #[arg(long, default_value = "dtcs:reference")]
        profile: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Execute a contract end-to-end using the reference runtime.
    Run {
        /// Path to a DTCS contract.
        path: PathBuf,
        /// JSON file with runtime inputs keyed by interface id.
        #[arg(long)]
        input: PathBuf,
        /// Apply plan optimization before execution.
        #[arg(long)]
        optimize: bool,
        /// Optional additional registry file to merge.
        #[arg(long)]
        registry: Option<PathBuf>,
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
        /// Optional additional registry file to merge for validation.
        #[arg(long)]
        registry: Option<PathBuf>,
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
        /// Optional additional registry file to merge for validation.
        #[arg(long)]
        registry: Option<PathBuf>,
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
        /// Optional additional registry file to merge for validation.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Analyze lineage for a contract.
    Lineage {
        /// Path to a DTCS document.
        path: PathBuf,
        /// Optional additional registry file to merge for validation.
        #[arg(long)]
        registry: Option<PathBuf>,
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
    /// Conformance profiles and offline certification suite (Ch 23).
    Conformance {
        #[command(subcommand)]
        /// Conformance subcommand.
        command: ConformanceCommand,
    },
    /// Inspect the identifier registry catalog.
    Registry {
        #[command(subcommand)]
        /// Registry subcommand.
        command: RegistryCommand,
    },
}

/// Conformance certification commands (Ch 23).
#[derive(Debug, Subcommand)]
pub enum ConformanceCommand {
    /// Emit the implementation capability declaration.
    Declare {
        /// Filter to a single profile identifier.
        #[arg(long)]
        profile: Option<String>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Run the offline conformance test suite.
    Run {
        /// Profile to test (`all` runs every profile).
        #[arg(long, default_value = "integrated-platform")]
        profile: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
}

/// Registry catalog commands.
#[derive(Debug, Subcommand)]
pub enum RegistryCommand {
    /// List registry entries.
    List {
        /// Optional additional registry file to merge.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Resolve a registry identifier.
    Resolve {
        /// Identifier to resolve (for example `dtcs:lowercase`).
        id: String,
        /// Optional additional registry file to merge.
        #[arg(long)]
        registry: Option<PathBuf>,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
}

/// Run the CLI application.
pub fn run(cli: Cli) -> miette::Result<i32> {
    match cli.command {
        Command::Validate {
            path,
            registry,
            json,
        } => {
            let result = parse_file(&path)?;
            let report = validation_report(result, registry.as_ref())?;
            render_report(&report, json, ReportMode::Validate)
                .map_err(|e| miette::miette!("{e}"))?;
            Ok(if report.is_valid() { 0 } else { 1 })
        }
        Command::Analyze {
            path,
            registry,
            json,
        } => {
            let result = parse_file(&path)?;
            let contract = result
                .contract
                .clone()
                .ok_or_else(|| miette::miette!("no contract in {}", path.display()))?;

            let merged = match registry.as_ref() {
                Some(registry_path) => Some(
                    crate::registry::load_merged(registry_path)
                        .map_err(|report| registry_report_error(&report))?,
                ),
                None => None,
            };
            let registry_doc = merged
                .as_ref()
                .unwrap_or_else(|| crate::registry::default_registry());

            let validation = validate_with_registry(&contract, registry_doc);
            let analysis_report = analysis::check_contract(&contract, Some(registry_doc));

            if json {
                let payload = serde_json::json!({
                    "validation": {
                        "valid": validation.is_valid(),
                        "diagnostics": validation.diagnostics,
                    },
                    "analysis": {
                        "valid": analysis_report.is_valid(),
                        "diagnostics": analysis_report.diagnostics,
                        "findings": analysis_report.findings,
                    }
                });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&payload).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                if !validation.diagnostics.is_empty() {
                    render_report(&validation, false, ReportMode::Diagnostics)
                        .map_err(|e| miette::miette!("{e}"))?;
                }
                if analysis_report.diagnostics.is_empty() {
                    println!("no analysis diagnostics");
                } else {
                    for diagnostic in &analysis_report.diagnostics {
                        println!(
                            "[{}] {} ({}) - {}",
                            format!("{:?}", diagnostic.severity).to_lowercase(),
                            diagnostic.id,
                            format!("{:?}", diagnostic.category).to_lowercase(),
                            diagnostic.message,
                        );
                        if let Some(object_ref) = &diagnostic.object_ref {
                            println!("  at: {object_ref}");
                        }
                        if let Some(remediation) = &diagnostic.remediation {
                            println!("  hint: {remediation}");
                        }
                    }
                }
            }

            Ok(if validation.is_valid() && analysis_report.is_valid() {
                0
            } else {
                1
            })
        }
        Command::Plan {
            path,
            registry,
            json,
        } => {
            let contract = load_valid_contract_with_registry(&path, registry.as_ref(), json)?;
            let merged = match registry.as_ref() {
                Some(registry_path) => Some(
                    crate::registry::load_merged(registry_path)
                        .map_err(|report| registry_report_error(&report))?,
                ),
                None => None,
            };
            let registry_doc = merged
                .as_ref()
                .unwrap_or_else(|| crate::registry::default_registry());
            let analysis_report = analysis::check_contract(&contract, Some(registry_doc));
            let plan_result =
                crate::plan::lower(&contract, Some(registry_doc), Some(&analysis_report));

            if !plan_result.is_valid() {
                let report = DiagnosticReport {
                    diagnostics: plan_result.diagnostics,
                };
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            }

            let plan = plan_result.plan.expect("valid plan result");
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&plan).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                let order =
                    crate::plan::topological_order(&contract, &plan.nodes, &plan.dependencies);
                println!("plan: {}", plan.identity.id);
                println!("nodes: {}", plan.nodes.len());
                println!("dependencies: {}", plan.dependencies.len());
                if !order.is_empty() {
                    println!("order: {}", order.join(" -> "));
                }
            }
            Ok(0)
        }
        Command::Optimize {
            path,
            plan: from_plan,
            registry,
            no_validate,
            json,
        } => {
            let merged = match registry.as_ref() {
                Some(registry_path) => Some(
                    crate::registry::load_merged(registry_path)
                        .map_err(|report| registry_report_error(&report))?,
                ),
                None => None,
            };
            let registry_doc = merged
                .as_ref()
                .unwrap_or_else(|| crate::registry::default_registry());

            let input_plan = if from_plan {
                let content = read_bounded_utf8(&path)?;
                serde_json::from_str(&content)
                    .map_err(|e| miette::miette!("invalid plan JSON in {}: {e}", path.display()))?
            } else {
                let contract = load_valid_contract_with_registry(&path, registry.as_ref(), json)?;
                let analysis_report = analysis::check_contract(&contract, Some(registry_doc));
                let plan_result =
                    crate::plan::lower(&contract, Some(registry_doc), Some(&analysis_report));
                if !plan_result.is_valid() {
                    let report = DiagnosticReport {
                        diagnostics: plan_result.diagnostics,
                    };
                    render_report(&report, json, ReportMode::Diagnostics)
                        .map_err(|e| miette::miette!("{e}"))?;
                    return Ok(1);
                }
                plan_result.plan.expect("valid plan result")
            };

            let options = crate::plan::OptimizeOptions {
                validate: !no_validate,
                ..crate::plan::OptimizeOptions::default()
            };
            let mut optimize_result =
                crate::plan::optimize_with_registry(&input_plan, registry_doc, &options);

            if no_validate && from_plan {
                optimize_result.diagnostics.push(
                    Diagnostic::new(
                        codes::OPTIMIZATION_SKIPPED,
                        Severity::Warning,
                        DiagnosticStage::Optimization,
                        DiagnosticCategory::Semantic,
                        "optimized plan was not validated; results may be unsound",
                    )
                    .with_object_ref("plan"),
                );
            }

            if !optimize_result.is_valid() {
                let report = DiagnosticReport {
                    diagnostics: optimize_result.diagnostics,
                };
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            }

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&optimize_result)
                        .map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                let optimized = optimize_result
                    .plan
                    .as_ref()
                    .expect("valid optimize result");
                let contract = crate::plan::plan_as_contract(optimized);
                let order = crate::plan::topological_order(
                    &contract,
                    &optimized.nodes,
                    &optimized.dependencies,
                );
                println!("plan: {}", optimized.identity.id);
                println!(
                    "nodes: {} -> {} ({} transforms)",
                    input_plan.nodes.len(),
                    optimized.nodes.len(),
                    optimize_result.transforms.len()
                );
                if !order.is_empty() {
                    println!("order: {}", order.join(" -> "));
                }
            }
            Ok(0)
        }
        Command::Match {
            path,
            plan: from_plan,
            optimize,
            registry,
            profile,
            json,
        } => {
            let merged = match registry.as_ref() {
                Some(registry_path) => Some(
                    crate::registry::load_merged(registry_path)
                        .map_err(|report| registry_report_error(&report))?,
                ),
                None => None,
            };
            let registry_doc = merged
                .as_ref()
                .unwrap_or_else(|| crate::registry::default_registry());
            let transformation_plan =
                load_transformation_plan(&path, from_plan, optimize, registry.as_ref(), json)?;
            let capability = load_capability_profile(&profile)?;
            let match_report = crate::capability::match_plan_with_registry(
                &transformation_plan,
                &capability,
                registry_doc,
            );
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&match_report)
                        .map_err(|e| miette::miette!("{e}"))?
                );
            } else if match_report.is_valid() {
                println!("supported: {}", transformation_plan.identity.id);
                println!("engine: {}", capability.engine_id);
            } else {
                let report = DiagnosticReport {
                    diagnostics: match_report.diagnostics.clone(),
                };
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
            }
            Ok(if match_report.is_valid() { 0 } else { 1 })
        }
        Command::Compile {
            path,
            plan: from_plan,
            optimize,
            registry,
            profile,
            json,
        } => {
            let transformation_plan =
                load_transformation_plan(&path, from_plan, optimize, registry.as_ref(), json)?;
            let capability = load_capability_profile(&profile)?;
            let compile_result =
                crate::compile::compile_with_capability(&transformation_plan, &capability);
            if !compile_result.is_valid() {
                let report = DiagnosticReport {
                    diagnostics: compile_result.diagnostics,
                };
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            }
            let execution_plan = compile_result.plan.expect("valid compile result");
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&execution_plan)
                        .map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                println!("execution plan: {}", execution_plan.identity.id);
                println!("target: {}", execution_plan.target.engine_id);
                println!("steps: {}", execution_plan.steps.len());
            }
            Ok(0)
        }
        Command::Run {
            path,
            input,
            optimize,
            registry,
            json,
        } => {
            let transformation_plan =
                load_transformation_plan(&path, false, optimize, registry.as_ref(), json)?;
            let compile_result = crate::compile::compile(&transformation_plan);
            if !compile_result.is_valid() {
                let report = DiagnosticReport {
                    diagnostics: compile_result.diagnostics,
                };
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            }
            let execution_plan = compile_result.plan.expect("valid compile result");
            let inputs = load_runtime_inputs(&input)?;
            let execute_result = crate::runtime::execute(&execution_plan, &inputs);
            if !execute_result.is_valid() {
                let report = DiagnosticReport {
                    diagnostics: execute_result.diagnostics,
                };
                render_report(&report, json, ReportMode::Diagnostics)
                    .map_err(|e| miette::miette!("{e}"))?;
                return Ok(1);
            }
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&execute_result.outputs)
                        .map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                let outputs = execute_result.outputs.expect("valid execute result");
                for (interface_id, dataset) in outputs {
                    println!("{interface_id}: {} row(s)", dataset.len());
                }
            }
            Ok(0)
        }
        Command::Inspect { path, json } => {
            let contract = load_valid_contract_with_registry(&path, None, json)?;
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
        Command::Diagnostics {
            path,
            registry,
            json,
        } => {
            let result = parse_file(&path)?;
            let report = validation_report(result, registry.as_ref())?;
            render_report(&report, json, ReportMode::Diagnostics)
                .map_err(|e| miette::miette!("{e}"))?;
            Ok(if report.is_valid() { 0 } else { 1 })
        }
        Command::Compat {
            source,
            target,
            scope,
            registry,
            json,
        } => {
            let source_contract =
                load_valid_contract_with_registry(&source, registry.as_ref(), json)?;
            let target_contract =
                load_valid_contract_with_registry(&target, registry.as_ref(), json)?;
            let scope = match ComparisonScope::from_tokens(&scope) {
                Ok(scope) => scope,
                Err(invalid) => {
                    eprintln!("invalid scope token(s): {}", invalid.join(", "));
                    return Ok(2);
                }
            };
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
        Command::Evolve {
            older,
            newer,
            registry,
            json,
        } => {
            let older_contract =
                load_valid_contract_with_registry(&older, registry.as_ref(), json)?;
            let newer_contract =
                load_valid_contract_with_registry(&newer, registry.as_ref(), json)?;
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
            registry,
            json,
        } => {
            let contract = load_valid_contract_with_registry(&path, registry.as_ref(), json)?;
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
        Command::Conformance { command } => run_conformance(command),
        Command::Registry { command } => run_registry(command),
    }
}

fn run_conformance(command: ConformanceCommand) -> miette::Result<i32> {
    match command {
        ConformanceCommand::Declare { profile, json } => {
            let declaration = match profile.as_deref() {
                Some(id) => crate::conformance::declare_profile(id)
                    .ok_or_else(|| miette::miette!("unknown conformance profile: {id}"))?,
                None => crate::conformance::declare(),
            };
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&declaration)
                        .map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                println!("implementation: {}", declaration.implementation_id);
                println!("version: {}", declaration.implementation_version);
                println!("spec: {}", declaration.dtcs_version);
                println!("primary profile: {}", declaration.primary_profile);
                for profile in &declaration.profiles {
                    println!("  {} ({:?})", profile.id, profile.implementation_class);
                }
            }
            Ok(0)
        }
        ConformanceCommand::Run { profile, json } => {
            let fixtures = crate::conformance::default_fixtures_dir();
            let report = if profile == "all" {
                crate::conformance::run_all()
            } else {
                crate::conformance::run_for_profiles(Some(std::slice::from_ref(&profile)), fixtures.as_path())
            };
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                println!(
                    "conformance {} ({})",
                    if report.passed { "passed" } else { "failed" },
                    report.implementation_version
                );
                for result in report
                    .results
                    .iter()
                    .chain(report.security.iter())
                    .filter(|r| !r.passed)
                {
                    println!(
                        "  FAIL {} [{}]: {}",
                        result.id,
                        result.profile,
                        result.message.as_deref().unwrap_or("failed")
                    );
                }
                let passed = report
                    .results
                    .iter()
                    .chain(report.security.iter())
                    .filter(|r| r.passed)
                    .count();
                let total = report.results.len() + report.security.len();
                println!("  {passed}/{total} checks passed");
            }
            Ok(if report.passed { 0 } else { 1 })
        }
    }
}

fn run_registry(command: RegistryCommand) -> miette::Result<i32> {
    match command {
        RegistryCommand::List { registry, json } => {
            let entries = crate::registry::list(registry.as_deref())
                .map_err(|report| registry_report_error(&report))?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&entries).map_err(|e| miette::miette!("{e}"))?
                );
            } else {
                for entry in &entries {
                    println!(
                        "{}  [{}]  {}  ({})",
                        entry.id,
                        entry.category.as_str(),
                        entry.name,
                        entry.status.as_str()
                    );
                }
            }
            Ok(0)
        }
        RegistryCommand::Resolve { id, registry, json } => {
            let entry = crate::registry::resolve_with_path(&id, registry.as_deref())
                .map_err(|report| registry_report_error(&report))?;
            match entry {
                Some(entry) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&entry)
                                .map_err(|e| miette::miette!("{e}"))?
                        );
                    } else {
                        println!("id: {}", entry.id);
                        println!("name: {}", entry.name);
                        println!("category: {}", entry.category.as_str());
                        println!("version: {}", entry.version);
                        println!("status: {}", entry.status.as_str());
                        if let Some(definition) = &entry.definition {
                            println!("definition: {definition}");
                        }
                        if let Some(compatibility) = entry.compatibility {
                            println!("compatibility: {}", compatibility.as_str());
                        }
                        println!("supported: {}", entry.supported);
                    }
                    Ok(0)
                }
                None => {
                    if json {
                        println!("null");
                    } else {
                        eprintln!("unresolved registry entry: {id}");
                    }
                    Ok(1)
                }
            }
        }
    }
}

fn registry_report_error(report: &DiagnosticReport) -> miette::Error {
    let messages: Vec<_> = report
        .diagnostics
        .iter()
        .map(|d| d.message.as_str())
        .collect();
    miette::miette!("{}", messages.join("; "))
}

fn validation_report(
    result: crate::parser::ParseResult,
    registry: Option<&PathBuf>,
) -> miette::Result<DiagnosticReport> {
    if let (Some(contract), Some(registry_path)) = (&result.contract, registry) {
        let merged = crate::registry::load_merged(registry_path)
            .map_err(|report| registry_report_error(&report))?;
        let mut report = result.report;
        report.merge(crate::validate_with_registry(contract, &merged));
        Ok(report)
    } else {
        Ok(result.validate())
    }
}

fn load_valid_contract_with_registry(
    path: &PathBuf,
    registry: Option<&PathBuf>,
    json: bool,
) -> miette::Result<TransformationContract> {
    let result = parse_file(path)?;
    let contract = result
        .contract
        .clone()
        .ok_or_else(|| miette::miette!("no contract in {}", path.display()))?;
    let report = validation_report(result, registry)?;
    if !report.is_valid() {
        render_report(&report, json, ReportMode::Validate).map_err(|e| miette::miette!("{e}"))?;
        return Err(miette::miette!("validation failed for {}", path.display()));
    }
    Ok(contract)
}

fn load_capability_profile(
    profile: &str,
) -> miette::Result<crate::capability::EngineCapabilityDeclaration> {
    if profile == crate::capability::REFERENCE_ENGINE_ID {
        return Ok(crate::capability::reference_profile());
    }
    Err(miette::miette!(
        "unsupported capability profile '{profile}'"
    ))
}

fn load_transformation_plan(
    path: &PathBuf,
    from_plan: bool,
    optimize: bool,
    registry: Option<&PathBuf>,
    json: bool,
) -> miette::Result<crate::plan::TransformationPlan> {
    let merged = match registry {
        Some(registry_path) => Some(
            crate::registry::load_merged(registry_path)
                .map_err(|report| registry_report_error(&report))?,
        ),
        None => None,
    };
    let registry_doc = merged
        .as_ref()
        .unwrap_or_else(|| crate::registry::default_registry());

    let mut plan = if from_plan {
        let content = read_bounded_utf8(path)?;
        serde_json::from_str(&content)
            .map_err(|e| miette::miette!("invalid plan JSON in {}: {e}", path.display()))?
    } else {
        let contract = load_valid_contract_with_registry(path, registry, json)?;
        let analysis_report = analysis::check_contract(&contract, Some(registry_doc));
        let plan_result = crate::plan::lower(&contract, Some(registry_doc), Some(&analysis_report));
        if !plan_result.is_valid() {
            let report = DiagnosticReport {
                diagnostics: plan_result.diagnostics,
            };
            render_report(&report, json, ReportMode::Diagnostics)
                .map_err(|e| miette::miette!("{e}"))?;
            return Err(miette::miette!(
                "plan lowering failed for {}",
                path.display()
            ));
        }
        plan_result.plan.expect("valid plan result")
    };

    if optimize {
        let optimize_result = crate::plan::optimize_with_registry(
            &plan,
            registry_doc,
            &crate::plan::OptimizeOptions::default(),
        );
        if !optimize_result.is_valid() {
            let report = DiagnosticReport {
                diagnostics: optimize_result.diagnostics,
            };
            render_report(&report, json, ReportMode::Diagnostics)
                .map_err(|e| miette::miette!("{e}"))?;
            return Err(miette::miette!(
                "plan optimization failed for {}",
                path.display()
            ));
        }
        plan = optimize_result.plan.expect("valid optimize result");
    }

    Ok(plan)
}

fn load_runtime_inputs(path: &Path) -> miette::Result<crate::runtime::RuntimeInputs> {
    let content = read_bounded_utf8(path)?;
    serde_json::from_str(&content)
        .map_err(|e| miette::miette!("invalid runtime input JSON in {}: {e}", path.display()))
}

/// Maximum runtime input / serialized plan JSON size accepted by the CLI (64 MiB).
const MAX_RUNTIME_INPUT_BYTES: usize = 64 * 1024 * 1024;

fn read_bounded_utf8(path: &Path) -> miette::Result<String> {
    let metadata = std::fs::metadata(path)
        .map_err(|e| miette::miette!("failed to read {}: {e}", path.display()))?;
    if metadata.len() as usize > MAX_RUNTIME_INPUT_BYTES {
        return Err(miette::miette!(
            "file exceeds maximum size of {} bytes: {}",
            MAX_RUNTIME_INPUT_BYTES,
            path.display()
        ));
    }
    let file = std::fs::File::open(path)
        .map_err(|e| miette::miette!("failed to read {}: {e}", path.display()))?;
    let mut content = Vec::new();
    file.take((MAX_RUNTIME_INPUT_BYTES as u64).saturating_add(1))
        .read_to_end(&mut content)
        .map_err(|e| miette::miette!("failed to read {}: {e}", path.display()))?;
    if content.len() > MAX_RUNTIME_INPUT_BYTES {
        return Err(miette::miette!(
            "file exceeds maximum size of {} bytes: {}",
            MAX_RUNTIME_INPUT_BYTES,
            path.display()
        ));
    }
    String::from_utf8(content)
        .map_err(|e| miette::miette!("invalid UTF-8 in {}: {e}", path.display()))
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
