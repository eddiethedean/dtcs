//! Plan-level expression error modes (SPEC Chapter 27 §14).

use crate::runtime::model::RuntimeValue;

/// How expression evaluation failures are surfaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorMode {
    /// Propagate the error (default).
    #[default]
    Fail,
    /// Return an invalid value carrying the error reason.
    Invalid,
    /// Return null.
    Null,
    /// Signal routing to an invalid-output path (caller handles).
    Route,
}

/// Parse a plan/requirement error-mode string.
pub fn parse_error_mode(s: &str) -> Result<ErrorMode, String> {
    match s {
        "fail" => Ok(ErrorMode::Fail),
        "invalid" => Ok(ErrorMode::Invalid),
        "null" => Ok(ErrorMode::Null),
        "route" => Ok(ErrorMode::Route),
        other => Err(format!(
            "unsupported errorMode '{other}'; expected fail|invalid|null|route"
        )),
    }
}

/// Apply an error mode to an evaluation failure.
pub fn apply_error_mode(mode: ErrorMode, err: String) -> Result<RuntimeValue, String> {
    match mode {
        ErrorMode::Fail => Err(err),
        ErrorMode::Invalid => Ok(RuntimeValue::invalid(err)),
        ErrorMode::Null => Ok(RuntimeValue::Null),
        ErrorMode::Route => Err(format!("route: {err}")),
    }
}
