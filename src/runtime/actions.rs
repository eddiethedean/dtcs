//! Semantic action execution.

use crate::runtime::model::{Row, RuntimeValue};

/// Apply a `dtcs:` semantic action to a field value.
pub fn apply_action(action_id: &str, value: &RuntimeValue) -> Result<RuntimeValue, String> {
    match action_id {
        "dtcs:lowercase" => match value {
            RuntimeValue::Null => Err("dtcs:lowercase does not accept null".into()),
            RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_lowercase())),
            other => Err(format!("dtcs:lowercase requires string, got {other:?}")),
        },
        "dtcs:uppercase" => match value {
            RuntimeValue::Null => Err("dtcs:uppercase does not accept null".into()),
            RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_uppercase())),
            other => Err(format!("dtcs:uppercase requires string, got {other:?}")),
        },
        "dtcs:capitalize" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::String(s) => {
                let mut chars = s.chars();
                let out = match chars.next() {
                    Some(first) => {
                        first.to_uppercase().collect::<String>()
                            + chars.as_str().to_lowercase().as_str()
                    }
                    None => String::new(),
                };
                Ok(RuntimeValue::String(out))
            }
            other => Err(format!("dtcs:capitalize requires string, got {other:?}")),
        },
        "dtcs:trim" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::String(s) => Ok(RuntimeValue::String(s.trim().to_string())),
            other => Err(format!("dtcs:trim requires string, got {other:?}")),
        },
        "dtcs:normalize_whitespace" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::String(s) => {
                let normalized = s.split_whitespace().collect::<Vec<_>>().join(" ");
                Ok(RuntimeValue::String(normalized))
            }
            other => Err(format!(
                "dtcs:normalize_whitespace requires string, got {other:?}"
            )),
        },
        "dtcs:hash_sha256" => match value {
            RuntimeValue::Null => Ok(RuntimeValue::Null),
            RuntimeValue::String(s) => {
                use sha2::{Digest, Sha256};
                let digest = Sha256::digest(s.as_bytes());
                Ok(RuntimeValue::String(hex::encode(digest)))
            }
            other => Err(format!("dtcs:hash_sha256 requires string, got {other:?}")),
        },
        other => Err(format!("unsupported semantic action '{other}'")),
    }
}

/// Apply an action to a qualified field across all rows in a workspace.
pub fn apply_action_to_rows(
    action_id: &str,
    rows: &mut [Row],
    field_name: &str,
) -> Result<(), String> {
    for row in rows.iter_mut() {
        let value = row.get(field_name).cloned().unwrap_or(RuntimeValue::Null);
        let updated = apply_action(action_id, &value)?;
        row.insert(field_name.to_string(), updated);
    }
    Ok(())
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
    }
}
