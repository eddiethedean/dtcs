//! Portable regex grammar gate for `dtcs-regex/1` (RE2-like subset).

/// Reject patterns that use lookaround, backreferences, or possessive quantifiers.
pub fn validate_dtcs_regex(pattern: &str) -> Result<(), String> {
    if pattern.len() > 16 * 1024 {
        return Err("regex pattern exceeds 16 KiB budget".into());
    }
    // Reject common non-RE2 / non-portable constructs.
    let forbidden = [
        ("(?=", "lookahead"),
        ("(?!", "negative lookahead"),
        ("(?<=", "lookbehind"),
        ("(?<!", "negative lookbehind"),
        ("*+", "possessive quantifier"),
        ("++", "possessive quantifier"),
        ("?+", "possessive quantifier"),
    ];
    for (token, name) in forbidden {
        if pattern.contains(token) {
            return Err(format!("pattern uses {name} which is outside dtcs-regex/1"));
        }
    }
    if has_numeric_backreference(pattern) {
        return Err("pattern uses backreference which is outside dtcs-regex/1".into());
    }
    // Named backrefs like \k<name>
    if pattern.contains("\\k<") {
        return Err("pattern uses named backreference outside dtcs-regex/1".into());
    }
    Ok(())
}

/// A backslash+digit with an odd number of consecutive preceding backslashes is a backref.
/// Even counts (e.g. `\\1`) are a literal backslash followed by digit.
fn has_numeric_backreference(pattern: &str) -> bool {
    let bytes = pattern.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            let start = i;
            while i < bytes.len() && bytes[i] == b'\\' {
                i += 1;
            }
            let backslashes = i - start;
            if i < bytes.len() && bytes[i].is_ascii_digit() && backslashes % 2 == 1 {
                return true;
            }
            continue;
        }
        i += 1;
    }
    false
}

/// Compile a portable regex after grammar validation.
pub fn compile_dtcs_regex(pattern: &str) -> Result<regex::Regex, String> {
    validate_dtcs_regex(pattern)?;
    regex::Regex::new(pattern).map_err(|error| format!("invalid portable regex: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_simple_pattern() {
        assert!(validate_dtcs_regex(r"^a\d+$").is_ok());
        assert!(compile_dtcs_regex(r"foo|bar").is_ok());
    }

    #[test]
    fn rejects_lookahead() {
        assert!(validate_dtcs_regex(r"a(?=b)").is_err());
    }

    #[test]
    fn rejects_backreference_but_allows_literal_backslash_digit() {
        assert!(validate_dtcs_regex(r"(a)\1").is_err());
        assert!(validate_dtcs_regex(r"\\1").is_ok());
        assert!(validate_dtcs_regex(r"\\\1").is_err());
    }
}
