//! Portable format grammar gate for `dtcs-format/1`.

/// Validate a chrono-style format string against the portable `dtcs-format/1` subset.
///
/// Allowed tokens: `%Y`, `%m`, `%d`, `%H`, `%M`, `%S`, `%f`, and `%%`.
/// All other `%` sequences are rejected. Non-`%` characters are treated as literals.
pub fn validate_dtcs_format(format: &str) -> Result<(), String> {
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            continue;
        }
        match chars.next() {
            Some('%' | 'Y' | 'm' | 'd' | 'H' | 'M' | 'S' | 'f') => {}
            Some(other) => {
                return Err(format!(
                    "format uses '%{other}' which is outside dtcs-format/1"
                ));
            }
            None => return Err("format ends with bare '%' outside dtcs-format/1".into()),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_portable_tokens() {
        assert!(validate_dtcs_format("%Y-%m-%d %H:%M:%S.%f").is_ok());
        assert!(validate_dtcs_format("literal %% percent").is_ok());
    }

    #[test]
    fn rejects_timezone_and_weekday() {
        assert!(validate_dtcs_format("%Y-%m-%d%z").is_err());
        assert!(validate_dtcs_format("%Z").is_err());
        assert!(validate_dtcs_format("%A").is_err());
    }
}
