//! Duplicate-key detection for JSON rule parameters.

use std::collections::HashSet;

/// Returns the first duplicate key in any `rules[].parameters` object.
pub(crate) fn scan_rule_parameters_duplicate_keys(content: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(content).ok()?;
    let mut pos = 0;
    while let Some(rel) = text[pos..].find("\"parameters\"") {
        let abs = pos + rel;
        let mut index = abs + "\"parameters\"".len();
        index = skip_ws(text, index)?;
        if text.as_bytes().get(index) != Some(&b':') {
            pos = abs + 1;
            continue;
        }
        index += 1;
        index = skip_ws(text, index)?;
        if text.as_bytes().get(index) != Some(&b'{') {
            pos = abs + 1;
            continue;
        }
        if let Some(key) = duplicate_keys_in_object(text, index) {
            return Some(key);
        }
        pos = abs + 1;
    }
    None
}

fn duplicate_keys_in_object(text: &str, open: usize) -> Option<String> {
    let bytes = text.as_bytes();
    let mut index = open + 1;
    let mut keys = HashSet::new();
    loop {
        index = skip_ws(text, index)?;
        if bytes.get(index) == Some(&b'}') {
            return None;
        }
        let (key, next) = parse_json_string(text, index)?;
        index = next;
        if !keys.insert(key.clone()) {
            return Some(key);
        }
        index = skip_ws(text, index)?;
        if bytes.get(index) != Some(&b':') {
            return None;
        }
        index += 1;
        index = skip_ws(text, index)?;
        index = skip_json_value(text, index)?;
        index = skip_ws(text, index)?;
        match bytes.get(index) {
            Some(b',') => index += 1,
            Some(b'}') => return None,
            _ => return None,
        }
    }
}

fn parse_json_string(text: &str, start: usize) -> Option<(String, usize)> {
    let bytes = text.as_bytes();
    if bytes.get(start) != Some(&b'"') {
        return None;
    }
    let mut index = start + 1;
    let mut value = String::new();
    while let Some(&byte) = bytes.get(index) {
        match byte {
            b'"' => return Some((value, index + 1)),
            b'\\' => {
                index += 1;
                let escaped = *bytes.get(index)?;
                value.push(match escaped {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{8}',
                    b'f' => '\u{c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'u' => {
                        let hex = &text[index + 1..index + 5];
                        let code = u16::from_str_radix(hex, 16).ok()?;
                        index += 4;
                        char::from_u32(u32::from(code))?
                    }
                    _ => return None,
                });
                index += 1;
            }
            _ if byte.is_ascii() => {
                value.push(byte as char);
                index += 1;
            }
            _ => return None,
        }
    }
    None
}

fn skip_json_value(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    match bytes.get(start)? {
        b'"' => {
            let (_, next) = parse_json_string(text, start)?;
            Some(next)
        }
        b'{' => skip_balanced(text, start, b'{', b'}'),
        b'[' => skip_balanced(text, start, b'[', b']'),
        b't' if text[start..].starts_with("true") => Some(start + 4),
        b'f' if text[start..].starts_with("false") => Some(start + 5),
        b'n' if text[start..].starts_with("null") => Some(start + 4),
        b'-' | b'0'..=b'9' => skip_number(text, start),
        _ => None,
    }
}

fn skip_balanced(text: &str, start: usize, open: u8, close: u8) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth = 0;
    let mut index = start;
    let mut in_string = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_string {
            if byte == b'\\' {
                index += 2;
                continue;
            }
            if byte == b'"' {
                in_string = false;
            }
            index += 1;
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b if b == open => depth += 1,
            b if b == close => {
                depth -= 1;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn skip_number(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut index = start;
    if bytes.get(index) == Some(&b'-') {
        index += 1;
    }
    while matches!(bytes.get(index), Some(b'0'..=b'9')) {
        index += 1;
    }
    if bytes.get(index) == Some(&b'.') {
        index += 1;
        while matches!(bytes.get(index), Some(b'0'..=b'9')) {
            index += 1;
        }
    }
    if matches!(bytes.get(index), Some(b'e' | b'E')) {
        index += 1;
        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }
        while matches!(bytes.get(index), Some(b'0'..=b'9')) {
            index += 1;
        }
    }
    Some(index)
}

fn skip_ws(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut index = start;
    while matches!(bytes.get(index), Some(b' ' | b'\n' | b'\r' | b'\t')) {
        index += 1;
    }
    Some(index)
}

#[cfg(test)]
mod tests {
    use super::scan_rule_parameters_duplicate_keys;

    #[test]
    fn detects_duplicate_rule_parameter_keys() {
        let json = br#"{"rules":[{"parameters":{"min":3,"min":10}}]}"#;
        assert_eq!(
            scan_rule_parameters_duplicate_keys(json).as_deref(),
            Some("min")
        );
    }

    #[test]
    fn accepts_unique_rule_parameter_keys() {
        let json = br#"{"rules":[{"parameters":{"min":3,"max":10}}]}"#;
        assert!(scan_rule_parameters_duplicate_keys(json).is_none());
    }
}
