//! Runtime type conversion helpers.

use crate::model::TypeConversion;
use crate::runtime::model::RuntimeValue;

/// Maximum integer magnitude safely representable as `f64` without precision loss.
pub const MAX_SAFE_INTEGER_FOR_DECIMAL: i64 = 9_007_199_254_740_992;

/// Apply a declared schema conversion to a runtime value.
pub fn apply_conversion(
    value: &RuntimeValue,
    conversion: &TypeConversion,
) -> Result<RuntimeValue, String> {
    if conversion.from == conversion.to {
        return Ok(value.clone());
    }
    match (conversion.from.as_str(), conversion.to.as_str(), value) {
        ("integer", "decimal", RuntimeValue::Integer(v)) => {
            integer_to_decimal(*v).map(RuntimeValue::Decimal)
        }
        ("decimal", "integer", RuntimeValue::Decimal(v)) => {
            if v.fract() != 0.0 {
                return Err(format!(
                    "cannot convert decimal {v} to integer without truncation"
                ));
            }
            if *v < i64::MIN as f64 || *v > i64::MAX as f64 {
                return Err(format!("decimal {v} is out of integer range"));
            }
            Ok(RuntimeValue::Integer(*v as i64))
        }
        (from, to, other) => Err(format!(
            "unsupported conversion from '{from}' to '{to}' for value {other:?}"
        )),
    }
}

/// Promote an integer to decimal with a precision guard.
pub fn integer_to_decimal(value: i64) -> Result<f64, String> {
    if value.abs() > MAX_SAFE_INTEGER_FOR_DECIMAL {
        return Err("integer magnitude exceeds safe decimal precision".into());
    }
    Ok(value as f64)
}

/// Find and apply the first conversion whose target matches `target_type`.
pub fn apply_matching_conversion(
    value: &RuntimeValue,
    conversions: &[TypeConversion],
    target_type: &str,
) -> Result<RuntimeValue, String> {
    for conversion in conversions {
        if conversion.to == target_type {
            return apply_conversion(value, conversion);
        }
    }
    Ok(value.clone())
}
