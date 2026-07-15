//! Stdlib function execution.

use crate::runtime::model::RuntimeValue;

/// Evaluate a `dtcs:` function call.
pub fn call_function(callee: &str, args: &[RuntimeValue]) -> Result<RuntimeValue, String> {
    match callee {
        "dtcs:lower" => {
            let value = args.first().ok_or("dtcs:lower requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_lowercase())),
                other => Err(format!("dtcs:lower requires string, got {other:?}")),
            }
        }
        "dtcs:upper" => {
            let value = args.first().ok_or("dtcs:upper requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_uppercase())),
                other => Err(format!("dtcs:upper requires string, got {other:?}")),
            }
        }
        "dtcs:concat" => {
            if args.len() < 2 {
                return Err("dtcs:concat requires at least two arguments".into());
            }
            let mut out = String::new();
            for arg in args {
                let RuntimeValue::String(s) = arg else {
                    return Err("dtcs:concat requires string arguments".into());
                };
                out.push_str(s);
            }
            Ok(RuntimeValue::String(out))
        }
        "dtcs:substr" => {
            let s = args
                .first()
                .ok_or("dtcs:substr requires at least two arguments")?;
            let start = args
                .get(1)
                .and_then(RuntimeValue::as_integer)
                .ok_or("dtcs:substr start must be integer")?;
            if start < 0 {
                return Err("dtcs:substr start must be non-negative".into());
            }
            match s {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::String(text) => {
                    let chars: Vec<char> = text.chars().collect();
                    let start = start as usize;
                    if start >= chars.len() {
                        return Ok(RuntimeValue::String(String::new()));
                    }
                    let slice = if let Some(len_arg) = args.get(2) {
                        let len = len_arg
                            .as_integer()
                            .ok_or("dtcs:substr length must be integer")?;
                        if len < 0 {
                            return Err("dtcs:substr length must be non-negative".into());
                        }
                        let end = (start + len as usize).min(chars.len());
                        &chars[start..end]
                    } else {
                        &chars[start..]
                    };
                    Ok(RuntimeValue::String(slice.iter().collect()))
                }
                other => Err(format!("dtcs:substr requires string, got {other:?}")),
            }
        }
        "dtcs:replace" => {
            if args.len() < 3 {
                return Err("dtcs:replace requires three arguments".into());
            }
            let RuntimeValue::String(text) = &args[0] else {
                return Err("dtcs:replace requires string haystack".into());
            };
            let RuntimeValue::String(from) = &args[1] else {
                return Err("dtcs:replace requires string needle".into());
            };
            let RuntimeValue::String(to) = &args[2] else {
                return Err("dtcs:replace requires string replacement".into());
            };
            Ok(RuntimeValue::String(text.replace(from, to)))
        }
        "dtcs:coalesce" => {
            for arg in args {
                if !arg.is_null() {
                    return Ok(arg.clone());
                }
            }
            Ok(RuntimeValue::Null)
        }
        "dtcs:length" => {
            let value = args.first().ok_or("dtcs:length requires one argument")?;
            let len = match value {
                RuntimeValue::Null => {
                    return Err("dtcs:length does not accept null".into());
                }
                RuntimeValue::String(s) => s.chars().count() as i64,
                RuntimeValue::Binary(b) => b.len() as i64,
                other => return Err(format!("dtcs:length unsupported type {other:?}")),
            };
            Ok(RuntimeValue::Integer(len))
        }
        "dtcs:to_string" => {
            let value = args.first().ok_or("dtcs:to_string requires one argument")?;
            Ok(RuntimeValue::String(match value {
                RuntimeValue::Null | RuntimeValue::Missing(_) => return Ok(RuntimeValue::Null),
                RuntimeValue::Invalid(_) => return Ok(RuntimeValue::Null),
                RuntimeValue::String(s)
                | RuntimeValue::Binary(s)
                | RuntimeValue::Date(s)
                | RuntimeValue::Time(s)
                | RuntimeValue::DateTime(s)
                | RuntimeValue::Duration(s) => s.clone(),
                RuntimeValue::Boolean(b) => b.to_string(),
                RuntimeValue::Integer(i) => i.to_string(),
                RuntimeValue::Decimal(d) => d.to_string(),
                RuntimeValue::List(_) | RuntimeValue::Map(_) => {
                    return Err("dtcs:to_string unsupported for collections".into())
                }
            }))
        }
        "dtcs:to_integer" => {
            let value = args
                .first()
                .ok_or("dtcs:to_integer requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::Integer(i) => Ok(RuntimeValue::Integer(*i)),
                RuntimeValue::Decimal(d) => {
                    if d.fract() != 0.0 {
                        return Err("dtcs:to_integer requires integer-valued decimal".into());
                    }
                    Ok(RuntimeValue::Integer(*d as i64))
                }
                RuntimeValue::String(s) => s
                    .parse::<i64>()
                    .map(RuntimeValue::Integer)
                    .map_err(|_| "dtcs:to_integer parse failed".to_string()),
                other => Err(format!("dtcs:to_integer unsupported type {other:?}")),
            }
        }
        "dtcs:to_decimal" => {
            let value = args
                .first()
                .ok_or("dtcs:to_decimal requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::Decimal(d) => Ok(RuntimeValue::Decimal(*d)),
                RuntimeValue::Integer(i) => Ok(RuntimeValue::Decimal(*i as f64)),
                RuntimeValue::String(s) => s
                    .parse::<f64>()
                    .map(RuntimeValue::Decimal)
                    .map_err(|_| "dtcs:to_decimal parse failed".to_string()),
                other => Err(format!("dtcs:to_decimal unsupported type {other:?}")),
            }
        }
        "dtcs:abs" => {
            let value = args.first().ok_or("dtcs:abs requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::Integer(i) => Ok(RuntimeValue::Integer(i.abs())),
                RuntimeValue::Decimal(d) => Ok(RuntimeValue::Decimal(d.abs())),
                other => Err(format!("dtcs:abs requires numeric, got {other:?}")),
            }
        }
        "dtcs:min" => {
            if args.len() < 2 {
                return Err("dtcs:min requires at least two arguments".into());
            }
            let mut best: Option<f64> = None;
            for arg in args {
                if arg.is_null() {
                    return Ok(RuntimeValue::Null);
                }
                let Some(v) = arg.as_decimal() else {
                    return Err(format!("dtcs:min requires numeric, got {arg:?}"));
                };
                best = Some(best.map_or(v, |b| b.min(v)));
            }
            Ok(RuntimeValue::Decimal(best.unwrap()))
        }
        "dtcs:max" => {
            if args.len() < 2 {
                return Err("dtcs:max requires at least two arguments".into());
            }
            let mut best: Option<f64> = None;
            for arg in args {
                if arg.is_null() {
                    return Ok(RuntimeValue::Null);
                }
                let Some(v) = arg.as_decimal() else {
                    return Err(format!("dtcs:max requires numeric, got {arg:?}"));
                };
                best = Some(best.map_or(v, |b| b.max(v)));
            }
            Ok(RuntimeValue::Decimal(best.unwrap()))
        }
        "dtcs:contains" => {
            let haystack = args.first().ok_or("dtcs:contains requires two arguments")?;
            let needle = args.get(1).ok_or("dtcs:contains requires two arguments")?;
            match (haystack, needle) {
                (RuntimeValue::Null, _) | (_, RuntimeValue::Null) => Ok(RuntimeValue::Null),
                (RuntimeValue::String(h), RuntimeValue::String(n)) => {
                    Ok(RuntimeValue::Boolean(h.contains(n.as_str())))
                }
                _ => Err("dtcs:contains requires string arguments".into()),
            }
        }
        "dtcs:is_null" => {
            let value = args.first().ok_or("dtcs:is_null requires one argument")?;
            Ok(RuntimeValue::Boolean(value.is_null()))
        }
        "dtcs:is_missing" => {
            let value = args
                .first()
                .ok_or("dtcs:is_missing requires one argument")?;
            Ok(RuntimeValue::Boolean(value.is_missing()))
        }
        other => Err(format!("unsupported function '{other}'")),
    }
}
