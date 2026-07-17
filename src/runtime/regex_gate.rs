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
        ("\\1", "backreference"),
        ("\\2", "backreference"),
        ("\\3", "backreference"),
        ("\\4", "backreference"),
        ("\\5", "backreference"),
        ("\\6", "backreference"),
        ("\\7", "backreference"),
        ("\\8", "backreference"),
        ("\\9", "backreference"),
        ("*+", "possessive quantifier"),
        ("++", "possessive quantifier"),
        ("?+", "possessive quantifier"),
    ];
    for (token, name) in forbidden {
        if pattern.contains(token) {
            return Err(format!("pattern uses {name} which is outside dtcs-regex/1"));
        }
    }
    // Named backrefs like \k<name>
    if pattern.contains("\\k<") {
        return Err("pattern uses named backreference outside dtcs-regex/1".into());
    }
    Ok(())
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
}
