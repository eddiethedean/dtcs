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
                if !arg.is_null() && !arg.is_missing() && !arg.is_invalid() {
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
        "dtcs:is_invalid" => {
            let value = args
                .first()
                .ok_or("dtcs:is_invalid requires one argument")?;
            Ok(RuntimeValue::Boolean(value.is_invalid()))
        }
        "dtcs:if_null" => {
            if args.len() != 2 {
                return Err("dtcs:if_null requires two arguments".into());
            }
            if args[0].is_null() || args[0].is_missing() {
                Ok(args[1].clone())
            } else {
                Ok(args[0].clone())
            }
        }
        "dtcs:null_if" => {
            if args.len() != 2 {
                return Err("dtcs:null_if requires two arguments".into());
            }
            if args[0] == args[1] {
                Ok(RuntimeValue::Null)
            } else {
                Ok(args[0].clone())
            }
        }
        "dtcs:case_when" => {
            if args.len() < 2 {
                return Err("dtcs:case_when requires at least two arguments".into());
            }
            let mut i = 0;
            while i + 1 < args.len() {
                match &args[i] {
                    RuntimeValue::Boolean(true) => return Ok(args[i + 1].clone()),
                    RuntimeValue::Boolean(false)
                    | RuntimeValue::Null
                    | RuntimeValue::Missing(_) => i += 2,
                    other if i + 1 == args.len() - 1 && i % 2 == 0 => {
                        // trailing else
                        return Ok(other.clone());
                    }
                    _ => i += 2,
                }
            }
            if args.len() % 2 == 1 {
                Ok(args[args.len() - 1].clone())
            } else {
                Ok(RuntimeValue::Null)
            }
        }
        "dtcs:try_cast" => {
            if args.len() != 2 {
                return Err("dtcs:try_cast requires value and type name".into());
            }
            let target = args[1]
                .as_str()
                .ok_or("dtcs:try_cast type must be string")?;
            match (target, &args[0]) {
                ("string", v) => call_function("dtcs:to_string", std::slice::from_ref(v)),
                ("integer", v) => match call_function("dtcs:to_integer", std::slice::from_ref(v)) {
                    Ok(v) => Ok(v),
                    Err(_) => Ok(RuntimeValue::Null),
                },
                ("decimal", v) => match call_function("dtcs:to_decimal", std::slice::from_ref(v)) {
                    Ok(v) => Ok(v),
                    Err(_) => Ok(RuntimeValue::Null),
                },
                _ => Ok(RuntimeValue::Null),
            }
        }
        "dtcs:concat_ws" => {
            if args.len() < 2 {
                return Err("dtcs:concat_ws requires separator and at least one value".into());
            }
            let sep = match &args[0] {
                RuntimeValue::String(s) => s.clone(),
                _ => return Err("dtcs:concat_ws separator must be string".into()),
            };
            let mut parts = Vec::new();
            for arg in &args[1..] {
                if let RuntimeValue::String(s) = arg {
                    parts.push(s.clone());
                }
            }
            Ok(RuntimeValue::String(parts.join(&sep)))
        }
        "dtcs:starts_with" => {
            let haystack = args.first().ok_or("dtcs:starts_with requires two arguments")?;
            let needle = args.get(1).ok_or("dtcs:starts_with requires two arguments")?;
            match (haystack, needle) {
                (RuntimeValue::Null, _) | (_, RuntimeValue::Null) => Ok(RuntimeValue::Null),
                (RuntimeValue::String(h), RuntimeValue::String(n)) => {
                    Ok(RuntimeValue::Boolean(h.starts_with(n.as_str())))
                }
                _ => Err("dtcs:starts_with requires string arguments".into()),
            }
        }
        "dtcs:ends_with" => {
            let haystack = args.first().ok_or("dtcs:ends_with requires two arguments")?;
            let needle = args.get(1).ok_or("dtcs:ends_with requires two arguments")?;
            match (haystack, needle) {
                (RuntimeValue::Null, _) | (_, RuntimeValue::Null) => Ok(RuntimeValue::Null),
                (RuntimeValue::String(h), RuntimeValue::String(n)) => {
                    Ok(RuntimeValue::Boolean(h.ends_with(n.as_str())))
                }
                _ => Err("dtcs:ends_with requires string arguments".into()),
            }
        }
        "dtcs:round" => {
            let value = args.first().ok_or("dtcs:round requires one argument")?;
            let digits = args.get(1).and_then(RuntimeValue::as_integer).unwrap_or(0);
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                other => {
                    let v = other
                        .as_decimal()
                        .ok_or_else(|| format!("dtcs:round unsupported type {other:?}"))?;
                    let factor = 10f64.powi(digits as i32);
                    Ok(RuntimeValue::Decimal((v * factor).round() / factor))
                }
            }
        }
        "dtcs:floor" => {
            let value = args.first().ok_or("dtcs:floor requires one argument")?;
            match value.as_decimal() {
                Some(v) => Ok(RuntimeValue::Decimal(v.floor())),
                None if value.is_null() => Ok(RuntimeValue::Null),
                None => Err("dtcs:floor requires numeric".into()),
            }
        }
        "dtcs:ceil" => {
            let value = args.first().ok_or("dtcs:ceil requires one argument")?;
            match value.as_decimal() {
                Some(v) => Ok(RuntimeValue::Decimal(v.ceil())),
                None if value.is_null() => Ok(RuntimeValue::Null),
                None => Err("dtcs:ceil requires numeric".into()),
            }
        }
        "dtcs:power" => {
            if args.len() != 2 {
                return Err("dtcs:power requires two arguments".into());
            }
            match (args[0].as_decimal(), args[1].as_decimal()) {
                (Some(a), Some(b)) => Ok(RuntimeValue::Decimal(a.powf(b))),
                _ if args[0].is_null() || args[1].is_null() => Ok(RuntimeValue::Null),
                _ => Err("dtcs:power requires numeric arguments".into()),
            }
        }
        "dtcs:sqrt" => {
            let value = args.first().ok_or("dtcs:sqrt requires one argument")?;
            match value.as_decimal() {
                Some(v) if v < 0.0 => Err("dtcs:sqrt of negative".into()),
                Some(v) => Ok(RuntimeValue::Decimal(v.sqrt())),
                None if value.is_null() => Ok(RuntimeValue::Null),
                None => Err("dtcs:sqrt requires numeric".into()),
            }
        }
        "dtcs:least" => call_function("dtcs:min", args),
        "dtcs:greatest" => call_function("dtcs:max", args),
        "dtcs:count_all" | "dtcs:count" | "dtcs:count_distinct" | "dtcs:sum" | "dtcs:average" => {
            Err(format!(
                "{callee} is an aggregate function and must be used with dtcs:aggregate/group"
            ))
        }
        "dtcs:current_date" => {
            // Deterministic fixture clock for the reference runtime (run-stable within a process).
            Ok(RuntimeValue::Date(reference_clock_date()))
        }
        "dtcs:current_timestamp" => {
            Ok(RuntimeValue::DateTime(reference_clock_timestamp()))
        }
        "dtcs:date_add" => {
            if args.len() < 2 {
                return Err("dtcs:date_add requires date and integer days".into());
            }
            let base = args[0]
                .as_str()
                .ok_or("dtcs:date_add first argument must be date/datetime string")?;
            let days = args[1]
                .as_integer()
                .ok_or("dtcs:date_add second argument must be integer days")?;
            let unit = args.get(2).and_then(RuntimeValue::as_str).unwrap_or("day");
            if unit != "day" && unit != "days" {
                return Err(format!("dtcs:date_add unsupported unit '{unit}' (reference supports day)"));
            }
            let shifted = shift_iso_date(base, days)?;
            Ok(if matches!(args[0], RuntimeValue::DateTime(_)) {
                RuntimeValue::DateTime(format!("{shifted}T00:00:00Z"))
            } else {
                RuntimeValue::Date(shifted)
            })
        }
        "dtcs:date_diff" => {
            if args.len() < 2 {
                return Err("dtcs:date_diff requires two date/datetime arguments".into());
            }
            let left = args[0]
                .as_str()
                .ok_or("dtcs:date_diff requires date strings")?;
            let right = args[1]
                .as_str()
                .ok_or("dtcs:date_diff requires date strings")?;
            let unit = args.get(2).and_then(RuntimeValue::as_str).unwrap_or("day");
            if unit != "day" && unit != "days" {
                return Err(format!("dtcs:date_diff unsupported unit '{unit}' (reference supports day)"));
            }
            Ok(RuntimeValue::Integer(diff_iso_dates(left, right)?))
        }
        "dtcs:row_number" | "dtcs:rank" | "dtcs:dense_rank" | "dtcs:lag" | "dtcs:lead" => Err(
            format!("{callee} must be used with dtcs:window (not as a scalar call)"),
        ),
        other => Err(format!("unsupported function '{other}'")),
    }
}

fn reference_clock_date() -> String {
    // Fixed reference clock so conformance / tests are deterministic.
    "2026-01-01".into()
}

fn reference_clock_timestamp() -> String {
    "2026-01-01T00:00:00Z".into()
}

fn parse_ymd(value: &str) -> Result<(i32, u32, u32), String> {
    let date = value.split('T').next().unwrap_or(value);
    let parts: Vec<_> = date.split('-').collect();
    if parts.len() != 3 {
        return Err(format!("invalid date '{value}'"));
    }
    let y: i32 = parts[0]
        .parse()
        .map_err(|_| format!("invalid year in '{value}'"))?;
    let m: u32 = parts[1]
        .parse()
        .map_err(|_| format!("invalid month in '{value}'"))?;
    let d: u32 = parts[2]
        .parse()
        .map_err(|_| format!("invalid day in '{value}'"))?;
    Ok((y, m, d))
}

fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    // Howard Hinnant civil_from_days algorithm (proleptic Gregorian).
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let m = m as i64;
    let d = d as i64;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe as i64 * 365 + yoe as i64 / 4 - yoe as i64 / 100 + doy;
    era as i64 * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = (yoe + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn shift_iso_date(value: &str, days: i64) -> Result<String, String> {
    let (y, m, d) = parse_ymd(value)?;
    let (ny, nm, nd) = civil_from_days(days_from_civil(y, m, d) + days);
    Ok(format!("{ny:04}-{nm:02}-{nd:02}"))
}

fn diff_iso_dates(left: &str, right: &str) -> Result<i64, String> {
    let (ly, lm, ld) = parse_ymd(left)?;
    let (ry, rm, rd) = parse_ymd(right)?;
    Ok(days_from_civil(ly, lm, ld) - days_from_civil(ry, rm, rd))
}
