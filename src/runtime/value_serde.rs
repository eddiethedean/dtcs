//! Custom JSON (de)serialization for [`RuntimeValue`] `$dtcs` tokens.

use std::collections::BTreeMap;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer};
use serde_json::Value as JsonValue;

use super::model::{InvalidMarker, InvalidValue, MissingMarker, MissingValue, RuntimeValue};

pub(crate) fn deserialize_runtime_value<'de, D>(deserializer: D) -> Result<RuntimeValue, D::Error>
where
    D: Deserializer<'de>,
{
    let value = JsonValue::deserialize(deserializer)?;
    runtime_value_from_json(value).map_err(D::Error::custom)
}

fn runtime_value_from_json(value: JsonValue) -> Result<RuntimeValue, String> {
    match value {
        JsonValue::Null => Ok(RuntimeValue::Null),
        JsonValue::Bool(b) => Ok(RuntimeValue::Boolean(b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(RuntimeValue::Integer(i))
            } else if let Some(u) = n.as_u64() {
                i64::try_from(u)
                    .map(RuntimeValue::Integer)
                    .map_err(|_| format!("integer out of range: {u}"))
            } else if let Some(f) = n.as_f64() {
                Ok(RuntimeValue::Decimal(f))
            } else {
                Err(format!("unsupported number: {n}"))
            }
        }
        JsonValue::String(s) => Ok(RuntimeValue::String(s)),
        JsonValue::Array(items) => {
            let mut list = Vec::with_capacity(items.len());
            for item in items {
                list.push(runtime_value_from_json(item)?);
            }
            Ok(RuntimeValue::List(list))
        }
        JsonValue::Object(map) => {
            if let Some(marker) = map.get("$dtcs").and_then(JsonValue::as_str) {
                match marker {
                    "missing" => Ok(RuntimeValue::Missing(MissingValue {
                        marker: MissingMarker::Missing,
                    })),
                    "invalid" => Ok(RuntimeValue::Invalid(InvalidValue {
                        marker: InvalidMarker::Invalid,
                        reason: map
                            .get("reason")
                            .and_then(JsonValue::as_str)
                            .map(str::to_string),
                    })),
                    other => Err(format!("unknown $dtcs marker '{other}'")),
                }
            } else {
                let mut out = BTreeMap::new();
                for (key, nested) in map {
                    out.insert(key, runtime_value_from_json(nested)?);
                }
                Ok(RuntimeValue::Map(out))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_missing_and_invalid_tokens() {
        let missing: RuntimeValue =
            serde_json::from_value(serde_json::json!({"$dtcs": "missing"})).unwrap();
        assert!(missing.is_missing());

        let invalid: RuntimeValue =
            serde_json::from_value(serde_json::json!({"$dtcs": "invalid", "reason": "bad"}))
                .unwrap();
        assert!(invalid.is_invalid());
        match invalid {
            RuntimeValue::Invalid(v) => assert_eq!(v.reason.as_deref(), Some("bad")),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn still_deserializes_ordinary_objects_as_maps() {
        let value: RuntimeValue =
            serde_json::from_value(serde_json::json!({"a": 1, "b": "x"})).unwrap();
        match value {
            RuntimeValue::Map(map) => {
                assert_eq!(map.get("a"), Some(&RuntimeValue::Integer(1)));
                assert_eq!(map.get("b"), Some(&RuntimeValue::String("x".into())));
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
