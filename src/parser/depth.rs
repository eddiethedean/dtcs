//! Nesting depth limits for parsed document values.

use serde_yaml::Value as YamlValue;

/// Maximum nesting depth accepted when parsing DTCS documents.
pub const MAX_PARSE_DEPTH: usize = 128;

/// Reject JSON documents whose bracket nesting exceeds [`MAX_PARSE_DEPTH`].
pub fn check_json_depth_bytes(content: &[u8]) -> Result<(), String> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escape = false;
    for &byte in content {
        if in_string {
            if escape {
                escape = false;
            } else if byte == b'\\' {
                escape = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'[' | b'{' => {
                depth += 1;
                if depth > MAX_PARSE_DEPTH {
                    return Err(format!(
                        "document nesting exceeds maximum depth of {MAX_PARSE_DEPTH} (found depth {depth})"
                    ));
                }
            }
            b']' | b'}' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Reject YAML values that exceed [`MAX_PARSE_DEPTH`].
pub fn check_yaml_depth(value: &YamlValue) -> Result<(), String> {
    check_yaml_value_depth(value, 0).map_err(|depth| {
        format!("document nesting exceeds maximum depth of {MAX_PARSE_DEPTH} (found depth {depth})")
    })
}

fn check_yaml_value_depth(value: &YamlValue, depth: usize) -> Result<(), usize> {
    if depth > MAX_PARSE_DEPTH {
        return Err(depth);
    }
    match value {
        YamlValue::Sequence(items) => {
            for item in items {
                check_yaml_value_depth(item, depth + 1)?;
            }
        }
        YamlValue::Mapping(fields) => {
            for (key, item) in fields {
                check_yaml_value_depth(key, depth + 1)?;
                check_yaml_value_depth(item, depth + 1)?;
            }
        }
        _ => {}
    }
    Ok(())
}
