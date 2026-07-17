//! Reference implementation of the Data Transformation Contract Standard (DTCS).
//!
//! [`SPEC.md`](../SPEC.md) at the repository root is the authoritative normative
//! specification. This crate implements the pipeline through reference runtime execution:
//!
//! ```text
//! DTCS Document → Parser → Canonical Object Model → Validator → Diagnostics
//!                                              ├→ Analyzer → Analysis reports
//!                                              ├→ Registry (identifier resolution)
//!                                              ├→ Planner → Transformation Plan
//!                                              ├→ Optimizer → Optimized Plan
//!                                              ├→ Capability match → Compile → ExecutionPlan
//!                                              └→ Runtime → Output datasets
//! ```
//!
//! # Example
//!
//! ```
//! use dtcs::{parse, validate, DocumentFormat};
//!
//! let yaml = br#"
//! dtcsVersion: "2.0.0"
//! id: "example"
//! name: "Example"
//! version: "0.2.0"
//! inputs:
//!   - id: "in"
//!     schema:
//!       fields:
//!         - name: "value"
//!           type: "string"
//!           nullable: false
//! outputs:
//!   - id: "out"
//!     schema:
//!       fields:
//!         - name: "value"
//!           type: "string"
//!           nullable: false
//! lineage:
//!   mappings:
//!     - output: "out"
//!       inputs: ["in"]
//! "#;
//!
//! let result = parse(yaml, DocumentFormat::Yaml);
//! let contract = result.contract.expect("parse succeeded");
//! let report = validate(&contract);
//! assert!(report.is_valid());
//! ```

/// DTCS specification version this crate targets.
pub const SPEC_VERSION: &str = "2.0.0";

pub mod analysis;
pub mod capability;
pub mod compatibility;
pub mod compile;
pub mod conformance;
pub mod diagnostics;
pub mod lineage;
pub mod metadata;
pub mod model;
pub mod parser;
pub mod plan;
pub mod registry;
pub mod runtime;
pub mod validation;
pub mod versioning;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "python")]
mod python;

pub use analysis::{
    check_contract, check_expression, from_structured_node, to_structured_node, AnalysisFinding,
    AnalysisReport,
};
pub use capability::{
    discover as discover_capabilities, match_plan, match_plan_with_registry, reference_profile,
    reference_portable_manifest, validate as validate_capabilities, CapabilityMatchReport,
    EngineCapabilityDeclaration, EntryCapability, PortableCapabilityManifest,
};
pub use compatibility::{
    analyze as analyze_compatibility, analyze_evolution, ChangeCategory, ComparisonScope,
    CompatibilityLevel, CompatibilityReport, EvolutionReport,
};
pub use compile::{
    compile, compile_after_match, compile_with_capability, validate as validate_execution_plan,
    CompileResult, ExecutionPlan,
};
pub use diagnostics::{
    codes, inspect_contract, Diagnostic, DiagnosticCategory, DiagnosticReport, DiagnosticStage,
    Severity, ValidationReport,
};
pub use lineage::{analyze as analyze_lineage, LineageAnalysisReport, LineageGovernance};
pub use model::{
    parse_logical_type, type_compatible, types_assignable, CompatibilityDeclaration,
    ContractGuarantees, ExtensionCompatibility, FieldConstraints, InformationFlow, LogicalType,
    NullBehavior, RegistryCategory, RegistryDocument, RegistryEntry, RegistryEntryStatus,
    RegistryPublicationStatus, RegistryRef, Rule, RuleOutcome, RulePhase, RuleScope,
    TransformationContract, TypeCompatibility, TypeParseError,
};
pub use parser::{parse, parse_file, parse_json, parse_yaml, DocumentFormat, ParseResult};
pub use plan::{
    equivalent, equivalent as plan_equivalent, export_portable_plan, lower as lower_plan, optimize,
    optimize as optimize_plan, validate as validate_plan, OptimizeOptions, OptimizeResult,
    PlanResult, PortablePlan, RegistryVersions, TransformationPlan, COMPLEX_TYPES_PROFILE,
    KERNEL_PROFILE, RELATIONAL_PROFILE, TRANSFORM_PLAN_IDENTITY, WINDOW_PROFILE,
};
pub use registry::{
    default_registry, is_known_action, is_known_function, is_known_operator, is_known_profile,
    is_known_rule, load as load_registry, load_merged, resolve as resolve_registry,
    resolve_default,
};
pub use runtime::{
    execute, Dataset, ExecuteResult, Row, RuntimeInputs, RuntimeOutputs, RuntimeValue,
};
pub use validation::{validate, validate_with_registry, ValidationPhase};

pub use conformance::{
    declare as conformance_declare, declare_profile as conformance_declare_profile,
    default_fixtures_dir as conformance_fixtures_dir, embedded_profiles as conformance_profiles,
    manifest as conformance_manifest, run_all as conformance_run_all,
    run_for_profiles as conformance_run, ConformanceProfile, ConformanceReport,
    ImplementationCapabilityDeclaration, ImplementationClass,
};

/// Parse and validate a DTCS document in one step.
#[must_use]
pub fn parse_and_validate(content: &[u8], format: DocumentFormat) -> ValidationReport {
    parse(content, format).validate()
}

/// Parse, validate, and lower a DTCS document to a transformation plan.
#[must_use]
pub fn parse_validate_and_plan(content: &[u8], format: DocumentFormat) -> plan::PlanResult {
    parse_validate_and_plan_with_registry(content, format, None)
}

/// Parse, validate, and lower a DTCS document using an optional vendor registry.
#[must_use]
pub fn parse_validate_and_plan_with_registry(
    content: &[u8],
    format: DocumentFormat,
    registry_path: Option<&std::path::Path>,
) -> plan::PlanResult {
    let parse_result = parse(content, format);
    let Some(contract) = parse_result.contract else {
        return plan::PlanResult {
            diagnostics: parse_result.report.diagnostics,
            ..plan::PlanResult::default()
        };
    };

    let registry_doc = match registry_path {
        Some(path) => match load_merged(path) {
            Ok(merged) => merged,
            Err(report) => {
                return plan::PlanResult {
                    diagnostics: report.diagnostics,
                    ..plan::PlanResult::default()
                };
            }
        },
        None => default_registry().clone(),
    };

    let validation = validate_with_registry(&contract, &registry_doc);
    if !validation.is_valid() {
        return plan::PlanResult {
            diagnostics: validation.diagnostics,
            ..plan::PlanResult::default()
        };
    }

    let analysis = analysis::check_contract(&contract, Some(&registry_doc));
    plan::lower(&contract, Some(&registry_doc), Some(&analysis))
}

/// Parse, validate, lower, and optimize a DTCS document.
#[must_use]
pub fn parse_validate_and_optimize(content: &[u8], format: DocumentFormat) -> plan::OptimizeResult {
    parse_validate_and_optimize_with_registry(content, format, None)
}

/// Parse, validate, lower, and optimize a DTCS document using an optional vendor registry.
#[must_use]
pub fn parse_validate_and_optimize_with_registry(
    content: &[u8],
    format: DocumentFormat,
    registry_path: Option<&std::path::Path>,
) -> plan::OptimizeResult {
    let plan_result = parse_validate_and_plan_with_registry(content, format, registry_path);
    let Some(plan) = plan_result.plan else {
        return plan::OptimizeResult {
            diagnostics: plan_result.diagnostics,
            ..plan::OptimizeResult::default()
        };
    };

    let registry_doc = match registry_path {
        Some(path) => match load_merged(path) {
            Ok(merged) => merged,
            Err(report) => {
                return plan::OptimizeResult {
                    diagnostics: report.diagnostics,
                    ..plan::OptimizeResult::default()
                };
            }
        },
        None => default_registry().clone(),
    };

    let mut result =
        plan::optimize_with_registry(&plan, &registry_doc, &plan::OptimizeOptions::default());
    result.diagnostics = plan_result
        .diagnostics
        .into_iter()
        .chain(result.diagnostics)
        .collect();
    result
}

/// Parse, validate, lower, compile, and optimize a DTCS document.
#[must_use]
pub fn parse_validate_and_compile(
    content: &[u8],
    format: DocumentFormat,
) -> compile::CompileResult {
    parse_validate_and_compile_with_registry(content, format, None)
}

/// Parse, validate, lower, and compile using an optional vendor registry.
#[must_use]
pub fn parse_validate_and_compile_with_registry(
    content: &[u8],
    format: DocumentFormat,
    registry_path: Option<&std::path::Path>,
) -> compile::CompileResult {
    let plan_result = parse_validate_and_plan_with_registry(content, format, registry_path);
    let Some(plan) = plan_result.plan else {
        return compile::CompileResult {
            diagnostics: plan_result.diagnostics,
            ..compile::CompileResult::default()
        };
    };

    let mut result = compile::compile(&plan);
    result.diagnostics = plan_result
        .diagnostics
        .into_iter()
        .chain(result.diagnostics)
        .collect();
    result
}

/// Parse, validate, lower, compile, and execute a DTCS document.
#[must_use]
pub fn parse_validate_and_run(
    content: &[u8],
    format: DocumentFormat,
    inputs: &runtime::RuntimeInputs,
) -> runtime::ExecuteResult {
    parse_validate_and_run_with_registry(content, format, None, inputs)
}

/// Parse through compile and execute with an optional vendor registry.
#[must_use]
pub fn parse_validate_and_run_with_registry(
    content: &[u8],
    format: DocumentFormat,
    registry_path: Option<&std::path::Path>,
    inputs: &runtime::RuntimeInputs,
) -> runtime::ExecuteResult {
    let compile_result = parse_validate_and_compile_with_registry(content, format, registry_path);
    let Some(execution_plan) = compile_result.plan else {
        return runtime::ExecuteResult {
            diagnostics: compile_result.diagnostics,
            ..runtime::ExecuteResult::default()
        };
    };

    let mut result = runtime::execute(&execution_plan, inputs);
    result.diagnostics = compile_result
        .diagnostics
        .into_iter()
        .chain(result.diagnostics)
        .collect();
    result
}

impl TransformationContract {
    /// Parse a contract from YAML text.
    pub fn from_yaml(content: &str) -> ParseResult {
        parse(content.as_bytes(), DocumentFormat::Yaml)
    }

    /// Parse a contract from JSON text.
    pub fn from_json(content: &str) -> ParseResult {
        parse(content.as_bytes(), DocumentFormat::Json)
    }

    /// Parse a contract from a file path.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> miette::Result<ParseResult> {
        parse_file(path)
    }
}
