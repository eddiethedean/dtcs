//! Python bindings exposed through maturin as `dtcs._native`.

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::Serialize;

use crate::diagnostics::inspect_contract;
use crate::model::TransformationContract;
use crate::parser::{parse, parse_file, DocumentFormat, ParseResult};

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
    if let Ok(text) = content.extract::<String>() {
        return Ok(text.into_bytes());
    }
    if let Ok(data) = content.extract::<Vec<u8>>() {
        return Ok(data);
    }
    Err(PyTypeError::new_err("content must be str or bytes"))
}

fn contract_from_py(
    py: Python<'_>,
    contract: &Bound<'_, PyAny>,
) -> PyResult<TransformationContract> {
    let json_mod = py.import("json")?;
    let json_str: String = json_mod.call_method1("dumps", (contract,))?.extract()?;
    serde_json::from_str(&json_str)
        .map_err(|e| PyValueError::new_err(format!("invalid contract: {e}")))
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
fn validate_contract(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let contract = contract_from_py(py, contract)?;
    value_to_py(py, &crate::validate(&contract))
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

/// Return a short human-readable contract summary.
#[pyfunction]
fn inspect(py: Python<'_>, contract: &Bound<'_, PyAny>) -> PyResult<String> {
    let contract = contract_from_py(py, contract)?;
    Ok(inspect_contract(&contract))
}

/// Native extension module for the Python `dtcs` package.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spec_version, m)?)?;
    m.add_function(wrap_pyfunction!(parse_document, m)?)?;
    m.add_function(wrap_pyfunction!(parse_path, m)?)?;
    m.add_function(wrap_pyfunction!(validate_contract, m)?)?;
    m.add_function(wrap_pyfunction!(validate_document, m)?)?;
    m.add_function(wrap_pyfunction!(inspect, m)?)?;
    Ok(())
}
