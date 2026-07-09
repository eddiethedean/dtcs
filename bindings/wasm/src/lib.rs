//! WebAssembly bindings for DTCS parse, validate, and conformance declare.

use dtcs::conformance;
use dtcs::parser::{parse, DocumentFormat};
use dtcs::validate;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

fn to_js<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    to_value(value).map_err(|err| JsValue::from_str(&format!("serialization failed: {err}")))
}

/// Parse a DTCS document from UTF-8 bytes.
#[wasm_bindgen(js_name = parseDocument)]
pub fn parse_document(content: &[u8], format: &str) -> Result<JsValue, JsValue> {
    let format = match format.to_lowercase().as_str() {
        "json" => DocumentFormat::Json,
        _ => DocumentFormat::Yaml,
    };
    let result = parse(content, format);
    to_js(&serde_json::json!({
        "contract": result.contract,
        "report": result.report,
    }))
}

/// Validate a parsed transformation contract object.
#[wasm_bindgen(js_name = validateContract)]
pub fn validate_contract(contract: JsValue) -> Result<JsValue, JsValue> {
    let contract: dtcs::TransformationContract = from_value(contract)
        .map_err(|err| JsValue::from_str(&format!("invalid contract: {err}")))?;
    to_js(&validate(&contract))
}

/// Emit the implementation capability declaration (Ch 23 §9).
#[wasm_bindgen(js_name = conformanceDeclare)]
pub fn conformance_declare(profile: Option<String>) -> Result<JsValue, JsValue> {
    let declaration = match profile.as_deref() {
        Some(id) => conformance::declare_profile(id)
            .ok_or_else(|| JsValue::from_str(&format!("unknown profile: {id}")))?,
        None => conformance::declare(),
    };
    to_js(&declaration)
}

/// DTCS specification version string.
#[wasm_bindgen(js_name = specVersion)]
pub fn spec_version() -> String {
    dtcs::SPEC_VERSION.to_string()
}
