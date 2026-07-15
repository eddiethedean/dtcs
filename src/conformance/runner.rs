//! Offline conformance test orchestration (Ch 23 §8).

use std::collections::BTreeMap;
use std::path::Path;

use crate::capability::{match_plan, reference_profile};
use crate::compatibility;
use crate::compile;
use crate::parser::{parse, DocumentFormat};
use crate::plan;
use crate::runtime::{execute, RuntimeInputs, RuntimeValue};
use crate::{analysis, validate};

use super::declare;
use super::fixtures::{default_fixtures_dir, read_fixture};
use super::model::{
    ConformanceAssertion, ConformanceManifest, ConformanceReport, ConformanceTestCase,
    ConformanceTestResult,
};
use super::profiles;
use super::security;

const EMBEDDED_MANIFEST: &str = include_str!("manifest.json");

/// Returns the embedded conformance manifest.
#[must_use]
pub fn manifest() -> ConformanceManifest {
    serde_json::from_str(EMBEDDED_MANIFEST).expect("valid embedded conformance manifest")
}

/// Runs all conformance tests for all profiles.
#[must_use]
pub fn run_all() -> ConformanceReport {
    run_for_profiles(None, default_fixtures_dir().as_path())
}

/// Runs conformance tests for selected profile identifiers (`None` means all).
#[must_use]
pub fn run_for_profiles(profile_ids: Option<&[String]>, fixtures_dir: &Path) -> ConformanceReport {
    let declaration = declare::declare();
    let manifest = manifest();
    let selected = profile_ids.map(|ids| ids.to_vec());
    let mut results = Vec::new();

    for profile in &declaration.profiles {
        if let Some(ids) = &selected {
            if !ids.iter().any(|id| id == &profile.id) {
                continue;
            }
        }
        for test in tests_for_profile(&manifest, &profile.id) {
            results.push(run_test_case(test, &profile.id, fixtures_dir));
        }
    }

    let security = security::run_security_probes(fixtures_dir);
    let passed = results.iter().all(|r| r.passed) && security.iter().all(|r| r.passed);
    let profiles_run: Vec<String> = if let Some(ids) = selected {
        ids
    } else {
        declaration.profiles.iter().map(|p| p.id.clone()).collect()
    };

    ConformanceReport {
        implementation_id: declaration.implementation_id,
        implementation_version: declaration.implementation_version,
        profiles: profiles_run,
        results,
        security,
        passed,
    }
}

fn tests_for_profile<'a>(
    manifest: &'a ConformanceManifest,
    profile_id: &str,
) -> Vec<&'a ConformanceTestCase> {
    manifest
        .tests
        .iter()
        .filter(|test| test.profiles.iter().any(|p| p == profile_id))
        .collect()
}

fn run_test_case(
    test: &ConformanceTestCase,
    profile_id: &str,
    fixtures_dir: &Path,
) -> ConformanceTestResult {
    if let ConformanceAssertion::SecurityProbe { probe_id } = &test.assertion {
        let mut result = security::run_security_probe(probe_id, fixtures_dir);
        result.id = test.id.clone();
        result.profile = profile_id.into();
        return result;
    }

    let format = parse_format(&test.format);
    let content = match read_fixture(fixtures_dir, &test.fixture) {
        Ok(bytes) => bytes,
        Err(err) => {
            return fail_result(test.id.clone(), profile_id, err);
        }
    };

    match &test.assertion {
        ConformanceAssertion::ParseValid => {
            let result = parse(&content, format);
            if result.contract.is_some() && result.report.is_valid() {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected parse success: {:?}", result.report.diagnostics),
                )
            }
        }
        ConformanceAssertion::ParseInvalid => {
            let result = parse(&content, format);
            if result.contract.is_none() || !result.report.is_valid() {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(test.id.clone(), profile_id, "expected parse failure".into())
            }
        }
        ConformanceAssertion::ValidateValid => {
            let contract = match load_contract(&content, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let report = validate(&contract);
            if report.is_valid() {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected validation success: {:?}", report.diagnostics),
                )
            }
        }
        ConformanceAssertion::ValidateInvalid { codes } => {
            let contract = match load_contract(&content, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let report = validate(&contract);
            if report.is_valid() {
                return fail_result(
                    test.id.clone(),
                    profile_id,
                    "expected validation failure".into(),
                );
            }
            if codes.is_empty() {
                pass_result(test.id.clone(), profile_id)
            } else {
                let mut actual: Vec<String> =
                    report.diagnostics.iter().map(|d| d.id.clone()).collect();
                actual.sort();
                let mut expected = codes.clone();
                expected.sort();
                if actual == expected {
                    pass_result(test.id.clone(), profile_id)
                } else {
                    fail_result(
                        test.id.clone(),
                        profile_id,
                        format!("diagnostic code mismatch: expected {expected:?}, got {actual:?}"),
                    )
                }
            }
        }
        ConformanceAssertion::AnalyzeValid => {
            let contract = match load_valid_contract(&content, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let report = analysis::check_contract(&contract, None);
            if report.diagnostics.iter().any(|d| d.severity.is_error()) {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected analysis success: {:?}", report.diagnostics),
                )
            } else {
                pass_result(test.id.clone(), profile_id)
            }
        }
        ConformanceAssertion::CompatLevel {
            comparison_fixture,
            level,
        } => {
            let left = match load_valid_contract(&content, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let right_bytes = match read_fixture(fixtures_dir, comparison_fixture) {
                Ok(bytes) => bytes,
                Err(err) => {
                    return fail_result(
                        test.id.clone(),
                        profile_id,
                        format!("read comparison fixture: {err}"),
                    );
                }
            };
            let right = match load_valid_contract(&right_bytes, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let report =
                compatibility::analyze(&left, &right, compatibility::ComparisonScope::all());
            let actual = serde_json::to_value(report.level)
                .ok()
                .and_then(|v| v.as_str().map(str::to_string))
                .unwrap_or_else(|| format!("{:?}", report.level));
            if actual.eq_ignore_ascii_case(level) {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected compat level {level}, got {actual}"),
                )
            }
        }
        ConformanceAssertion::EvolveValid { comparison_fixture } => {
            let older = match load_valid_contract(&content, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let newer_bytes = match read_fixture(fixtures_dir, comparison_fixture) {
                Ok(bytes) => bytes,
                Err(err) => {
                    return fail_result(
                        test.id.clone(),
                        profile_id,
                        format!("read comparison fixture: {err}"),
                    );
                }
            };
            let newer = match load_valid_contract(&newer_bytes, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let report = compatibility::analyze_evolution(&older, &newer);
            if report.diagnostics.iter().any(|d| d.severity.is_error()) {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected evolution success: {:?}", report.diagnostics),
                )
            } else {
                pass_result(test.id.clone(), profile_id)
            }
        }
        ConformanceAssertion::PlanValid => {
            let contract = match load_valid_contract(&content, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let result = plan::lower(&contract, None, None);
            if result.is_valid() && result.plan.is_some() {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected plan lowering success: {:?}", result.diagnostics),
                )
            }
        }
        ConformanceAssertion::OptimizeEquivalent => {
            let contract = match load_valid_contract(&content, format) {
                Ok(contract) => contract,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let lowered = plan::lower(&contract, None, None);
            let Some(original) = lowered.plan else {
                return fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("plan lowering failed: {:?}", lowered.diagnostics),
                );
            };
            let optimized = plan::optimize(&original);
            let Some(optimized_plan) = optimized.plan else {
                return fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("optimization failed: {:?}", optimized.diagnostics),
                );
            };
            if plan::equivalent(&original, &optimized_plan) {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    "optimizer changed semantics".into(),
                )
            }
        }
        ConformanceAssertion::MatchSupported => {
            let plan = match load_plan(&content, format) {
                Ok(plan) => plan,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let report = match_plan(&plan, &reference_profile());
            if report.supported {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!(
                        "expected capability match success: {:?}",
                        report.diagnostics
                    ),
                )
            }
        }
        ConformanceAssertion::CompileValid => {
            let plan = match load_plan(&content, format) {
                Ok(plan) => plan,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let result = compile::compile(&plan);
            if result.is_valid() && result.plan.is_some() {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected compile success: {:?}", result.diagnostics),
                )
            }
        }
        ConformanceAssertion::RuntimeOutput {
            input,
            expected_output,
        } => {
            let plan = match load_plan(&content, format) {
                Ok(plan) => plan,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let compile_result = compile::compile(&plan);
            let Some(execution_plan) = compile_result.plan else {
                return fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("compile failed: {:?}", compile_result.diagnostics),
                );
            };
            let inputs: RuntimeInputs = match read_fixture(fixtures_dir, input)
                .and_then(|bytes| serde_json::from_slice(&bytes).map_err(|e| e.to_string()))
            {
                Ok(inputs) => inputs,
                Err(message) => {
                    return fail_result(
                        test.id.clone(),
                        profile_id,
                        format!("read runtime input: {message}"),
                    );
                }
            };
            let execute_result = execute(&execution_plan, &inputs);
            if !execute_result.is_valid() {
                return fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("runtime failed: {:?}", execute_result.diagnostics),
                );
            }
            let outputs = match execute_result.outputs {
                Some(outputs) => outputs,
                None => {
                    return fail_result(
                        test.id.clone(),
                        profile_id,
                        "runtime produced no outputs".into(),
                    );
                }
            };
            let expected: BTreeMap<String, Vec<BTreeMap<String, RuntimeValue>>> =
                match read_fixture(fixtures_dir, expected_output)
                    .and_then(|bytes| serde_json::from_slice(&bytes).map_err(|e| e.to_string()))
                {
                    Ok(expected) => expected,
                    Err(message) => {
                        return fail_result(
                            test.id.clone(),
                            profile_id,
                            format!("read expected output: {message}"),
                        );
                    }
                };
            if outputs == expected {
                pass_result(test.id.clone(), profile_id)
            } else {
                fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("output mismatch: got {outputs:?}, expected {expected:?}"),
                )
            }
        }
        ConformanceAssertion::RuntimeInvalid { input, codes } => {
            let plan = match load_plan(&content, format) {
                Ok(plan) => plan,
                Err(message) => {
                    return fail_result(test.id.clone(), profile_id, message);
                }
            };
            let compile_result = compile::compile(&plan);
            let Some(execution_plan) = compile_result.plan else {
                return fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("compile failed: {:?}", compile_result.diagnostics),
                );
            };
            let inputs: RuntimeInputs = match read_fixture(fixtures_dir, input)
                .and_then(|bytes| serde_json::from_slice(&bytes).map_err(|e| e.to_string()))
            {
                Ok(inputs) => inputs,
                Err(message) => {
                    return fail_result(
                        test.id.clone(),
                        profile_id,
                        format!("read runtime input: {message}"),
                    );
                }
            };
            let execute_result = execute(&execution_plan, &inputs);
            if execute_result.is_valid() {
                return fail_result(
                    test.id.clone(),
                    profile_id,
                    format!("expected runtime failure: {:?}", execute_result.outputs),
                );
            }
            if codes.is_empty() {
                pass_result(test.id.clone(), profile_id)
            } else {
                let mut actual: Vec<String> = execute_result
                    .diagnostics
                    .iter()
                    .map(|d| d.id.clone())
                    .collect();
                actual.sort();
                let mut expected = codes.clone();
                expected.sort();
                if actual == expected {
                    pass_result(test.id.clone(), profile_id)
                } else {
                    fail_result(
                        test.id.clone(),
                        profile_id,
                        format!(
                            "runtime diagnostic mismatch: expected {expected:?}, got {actual:?}"
                        ),
                    )
                }
            }
        }
        ConformanceAssertion::SecurityProbe { .. } => unreachable!("handled above"),
    }
}

fn parse_format(format: &str) -> DocumentFormat {
    match format.to_lowercase().as_str() {
        "json" => DocumentFormat::Json,
        _ => DocumentFormat::Yaml,
    }
}

fn load_contract(
    content: &[u8],
    format: DocumentFormat,
) -> Result<crate::model::TransformationContract, String> {
    let result = parse(content, format);
    result
        .into_contract()
        .map_err(|report| format!("parse failed: {:?}", report.diagnostics))
}

fn load_valid_contract(
    content: &[u8],
    format: DocumentFormat,
) -> Result<crate::model::TransformationContract, String> {
    let contract = load_contract(content, format)?;
    let report = validate(&contract);
    if report.is_valid() {
        Ok(contract)
    } else {
        Err(format!("validation failed: {:?}", report.diagnostics))
    }
}

fn load_plan(content: &[u8], format: DocumentFormat) -> Result<plan::TransformationPlan, String> {
    let contract = load_valid_contract(content, format)?;
    let analysis_report = analysis::check_contract(&contract, None);
    if !analysis_report.is_valid() {
        return Err(format!(
            "analysis failed: {:?}",
            analysis_report.diagnostics
        ));
    }
    let lowered = plan::lower(&contract, None, None);
    if lowered.is_valid() {
        lowered
            .plan
            .ok_or_else(|| "plan missing after successful lowering".into())
    } else {
        Err(format!("plan lowering failed: {:?}", lowered.diagnostics))
    }
}

fn pass_result(id: String, profile: &str) -> ConformanceTestResult {
    ConformanceTestResult {
        id,
        profile: profile.into(),
        passed: true,
        message: None,
    }
}

fn fail_result(id: String, profile: &str, message: String) -> ConformanceTestResult {
    ConformanceTestResult {
        id,
        profile: profile.into(),
        passed: false,
        message: Some(message),
    }
}

/// Returns embedded profile definitions.
#[must_use]
pub fn embedded_profiles() -> Vec<super::model::ConformanceProfile> {
    profiles::all_profiles()
}
