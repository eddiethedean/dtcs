//! Python bindings exposed through maturin as `dtcs._native`.

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};
use serde::Serialize;

use crate::compatibility::{analyze as analyze_compatibility, analyze_evolution, ComparisonScope};
use crate::diagnostics::inspect_contract;
use crate::lineage::analyze_with_options;
use crate::model::TransformationContract;
use crate::parser::{parse, parse_file, DocumentFormat, ParseResult};
use crate::{analysis, AnalysisReport, ValidationReport};

fn value_to_py(py: Python<'_>, value: &impl Serialize) -> PyResult<Py<PyAny>> {
    let json = serde_json::to_string(value)
        .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}")))?;
    let json_mod = py.import("json")?;
    json_mod
        .call_method1("loads", (json,))
        .map(|obj| obj.unbind())
}

fn parse_format(format: &str) -> PyResult<DocumentFormat> {
    match format.to_lowercase().as_str() {
        "yaml" | "yml" => Ok(DocumentFormat::Yaml),
        "json" => Ok(DocumentFormat::Json),
        other => Err(PyValueError::new_err(format!(
            "unsupported format '{other}'; use 'yaml' or 'json'"
        ))),
    }
}

fn content_to_bytes(content: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    if content.is_none() {
        return Err(PyTypeError::new_err("content must be str or bytes"));
    }
    if let Ok(text) = content.extract::<String>() {
        return Ok(text.into_bytes());
    }
    if let Ok(data) = content.extract::<Vec<u8>>() {
        return Ok(data);
    }
    if let Ok(byte_array) = content.downcast::<PyByteArray>() {
        return Ok(byte_array.to_vec());
    }
    Err(PyTypeError::new_err(
        "content must be str, bytes, or bytearray",
    ))
}

fn contract_from_py(
    py: Python<'_>,
    contract: &Bound<'_, PyAny>,
) -> PyResult<TransformationContract> {
    if contract.is_none() {
        return Err(PyTypeError::new_err("contract must be a dict, not None"));
    }
    let json_mod = py.import("json")?;
    let json_str: String = json_mod
        .call_method(
            "dumps",
            (contract,),
            Some(&{
                let kwargs = PyDict::new(py);
                kwargs.set_item("allow_nan", false)?;
                kwargs
            }),
        )
        .map_err(|err| {
            // `json.dumps(..., allow_nan=False)` raises `ValueError` for NaN/Infinity, but it can
            // also raise other exceptions (e.g. TypeError for non-serializable objects). Only
            // map the former to our friendlier message; otherwise preserve the original error.
            let message = err.to_string();
            if message.contains("Out of range float values are not JSON compliant")
                || message.contains("NaN")
                || message.contains("Infinity")
            {
                PyValueError::new_err("contract contains non-finite float values (NaN or Infinity)")
            } else {
                err
            }
        })?
        .extract()?;
    serde_json::from_str(&json_str).map_err(|e| contract_deserialize_error(&e.to_string()))
}

fn contract_deserialize_error(message: &str) -> PyErr {
    if message.contains("unknown field") && message.contains('_') {
        return PyValueError::new_err(format!(
            "invalid contract: {message}. DTCS contracts use camelCase keys (for example dtcsVersion, semanticActions)"
        ));
    }
    PyValueError::new_err(format!("invalid contract: {message}"))
}

fn parse_result_to_py(py: Python<'_>, result: ParseResult) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    match result.contract {
        Some(contract) => dict.set_item("contract", value_to_py(py, &contract)?)?,
        None => dict.set_item("contract", py.None())?,
    }
    dict.set_item("report", value_to_py(py, &result.report)?)?;
    Ok(dict.into())
}

/// DTCS specification version this crate targets.
#[pyfunction]
fn spec_version() -> &'static str {
    crate::SPEC_VERSION
}

/// Parse a DTCS document from text or bytes.
#[pyfunction]
#[pyo3(signature = (content, format="yaml"))]
fn parse_document(py: Python<'_>, content: &Bound<'_, PyAny>, format: &str) -> PyResult<Py<PyAny>> {
    let bytes = content_to_bytes(content)?;
    let doc_format = parse_format(format)?;
    parse_result_to_py(py, parse(&bytes, doc_format))
}

/// Parse a DTCS document from a file path.
#[pyfunction]
fn parse_path(py: Python<'_>, path: &str) -> PyResult<Py<PyAny>> {
    let result = parse_file(path).map_err(|e| PyValueError::new_err(e.to_string()))?;
    parse_result_to_py(py, result)
}

/// Validate a parsed transformation contract.
#[pyfunction]
#[pyo3(signature = (contract, registry_path=None))]
fn validate_contract(
    py: Python<'_>,
    contract: &Bound<'_, PyAny>,
    registry_path: Option<String>,
) -> PyResult<Py<PyAny>> {
    let contract = contract_from_py(py, contract)?;
    let report = if let Some(path) = registry_path.as_deref() {
        let merged = crate::registry::load_merged(path).map_err(registry_error)?;
        crate::validate_with_registry(&contract, &merged)
    } else {
        crate::validate(&contract)
    };
    value_to_py(py, &report)
}

/// Analyze a parsed transformation contract (expressions + semantics).
#[pyfunction]
#[pyo3(signature = (contract, registry_path=None))]
fn analyze_contract(
    py: Python<'_>,
    contract: &Bound<'_, PyAny>,
    registry_path: Option<String>,
) -> PyResult<Py<PyAny>> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct AnalyzeResult {
        validation: ValidationReport,
        analysis: AnalysisReport,
    }

    let contract = contract_from_py(py, contract)?;
    let registry_doc = if let Some(path) = registry_path.as_deref() {
        crate::registry::load_merged(path).map_err(registry_error)?
    } else {
        crate::registry::default_registry().clone()
    };

    let validation = crate::validate_with_registry(&contract, &registry_doc);
    let analysis = analysis::check_contract(&contract, Some(&registry_doc));
    value_to_py(
        py,
        &AnalyzeResult {
            validation,
            analysis,
        },
    )
}

/// Parse and validate a DTCS document in one step.
#[pyfunction]
#[pyo3(signature = (content, format="yaml"))]
fn validate_document(
    py: Python<'_>,
    content: &Bound<'_, PyAny>,
    format: &str,
) -> PyResult<Py<PyAny>> {
    let bytes = content_to_bytes(content)?;
    let doc_format = parse_format(format)?;
    value_to_py(py, &crate::parse_and_validate(&bytes, doc_format))
}

/// Validate metadata for a parsed transformation contract.
#[pyfunction]
fn metadata_validate(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let contract = contract_from_py(py, contract)?;
    value_to_py(py, &crate::metadata::validate(&contract))
}

/// Return a short human-readable contract summary.
#[pyfunction]
fn inspect(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<String> {
    let contract = contract_from_py(py, contract)?;
    Ok(inspect_contract(&contract))
}

/// Analyze compatibility between two contracts.
#[pyfunction]
#[pyo3(signature = (source, target, scope=None))]
fn compat_analyze(
    py: Python<'_>,
    source: &Bound<'_, PyAny>,
    target: &Bound<'_, PyAny>,
    scope: Option<Vec<String>>,
) -> PyResult<Py<PyAny>> {
    let source = contract_from_py(py, source)?;
    let target = contract_from_py(py, target)?;
    let scope = ComparisonScope::from_tokens(&scope.unwrap_or_default()).map_err(|invalid| {
        PyValueError::new_err(format!("invalid scope token(s): {}", invalid.join(", ")))
    })?;
    value_to_py(py, &analyze_compatibility(&source, &target, scope))
}

/// Analyze evolution between two contract revisions.
#[pyfunction]
fn evolve_analyze(
    py: Python<'_>,
    older: &Bound<'_, PyAny>,
    newer: &Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    let older = contract_from_py(py, older)?;
    let newer = contract_from_py(py, newer)?;
    value_to_py(py, &analyze_evolution(&older, &newer))
}

/// Analyze lineage for a contract.
#[pyfunction]
#[pyo3(signature = (contract, impact=None, dependency=None))]
fn lineage_analyze(
    py: Python<'_>,
    contract: &Bound<'_, PyAny>,
    impact: Option<String>,
    dependency: Option<String>,
) -> PyResult<Py<PyAny>> {
    let contract = contract_from_py(py, contract)?;
    value_to_py(
        py,
        &analyze_with_options(&contract, impact.as_deref(), dependency.as_deref()),
    )
}

/// Validate version identifiers on a contract.
#[pyfunction]
fn version_validate(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let contract = contract_from_py(py, contract)?;
    value_to_py(py, &crate::versioning::validate(&contract))
}

/// List registry entries, optionally merged with a registry file.
#[pyfunction]
#[pyo3(signature = (registry_path=None))]
fn registry_list(py: Python<'_>, registry_path: Option<String>) -> PyResult<Py<PyAny>> {
    let path = registry_path.as_deref().map(std::path::Path::new);
    let entries = crate::registry::list(path).map_err(registry_error)?;
    value_to_py(py, &entries)
}

/// Resolve a registry identifier, optionally using an additional registry file.
#[pyfunction]
#[pyo3(signature = (id, registry_path=None))]
fn registry_resolve(
    py: Python<'_>,
    id: &str,
    registry_path: Option<String>,
) -> PyResult<Py<PyAny>> {
    let path = registry_path.as_deref().map(std::path::Path::new);
    let entry = crate::registry::resolve_with_path(id, path).map_err(registry_error)?;
    match entry {
        Some(entry) => value_to_py(py, &entry),
        None => Ok(py.None()),
    }
}

/// Load a registry document from a file path.
#[pyfunction]
fn registry_load(py: Python<'_>, path: &str) -> PyResult<Py<PyAny>> {
    let document = crate::registry::load(path).map_err(registry_error)?;
    value_to_py(py, &document)
}

fn registry_error(report: crate::diagnostics::DiagnosticReport) -> PyErr {
    let messages: Vec<_> = report
        .diagnostics
        .iter()
        .map(|d| d.message.as_str())
        .collect();
    PyValueError::new_err(messages.join("; "))
}

/// Native extension module for the Python `dtcs` package.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spec_version, m)?)?;
    m.add_function(wrap_pyfunction!(parse_document, m)?)?;
    m.add_function(wrap_pyfunction!(parse_path, m)?)?;
    m.add_function(wrap_pyfunction!(validate_contract, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_contract, m)?)?;
    m.add_function(wrap_pyfunction!(metadata_validate, m)?)?;
    m.add_function(wrap_pyfunction!(validate_document, m)?)?;
    m.add_function(wrap_pyfunction!(inspect, m)?)?;
    m.add_function(wrap_pyfunction!(compat_analyze, m)?)?;
    m.add_function(wrap_pyfunction!(evolve_analyze, m)?)?;
    m.add_function(wrap_pyfunction!(lineage_analyze, m)?)?;
    m.add_function(wrap_pyfunction!(version_validate, m)?)?;
    m.add_function(wrap_pyfunction!(registry_list, m)?)?;
    m.add_function(wrap_pyfunction!(registry_resolve, m)?)?;
    m.add_function(wrap_pyfunction!(registry_load, m)?)?;
    Ok(())
}
