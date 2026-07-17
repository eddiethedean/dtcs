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
        "dtcs:between" | "between" => {
            if args.len() != 3 {
                return Err("dtcs:between requires value, lower, and upper".into());
            }
            eval_between(&args[0], &args[1], &args[2])
        }
        "dtcs:field" | "field" => {
            if args.len() != 2 {
                return Err("dtcs:field requires object/map and field name".into());
            }
            eval_field_access(&args[0], &args[1])
        }
        "dtcs:index" | "index" => {
            if args.len() != 2 {
                return Err("dtcs:index requires collection and index".into());
            }
            eval_index_access(&args[0], &args[1], false)
        }
        "dtcs:element_at" | "element_at" | "elementAt" => {
            if args.len() != 2 {
                return Err("dtcs:element_at requires collection and index/key".into());
            }
            eval_index_access(&args[0], &args[1], true)
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
                return Err("dtcs:date_add requires date and integer amount".into());
            }
            let base = args[0]
                .as_str()
                .ok_or("dtcs:date_add first argument must be date/datetime string")?;
            let amount = args[1]
                .as_integer()
                .ok_or("dtcs:date_add second argument must be integer")?;
            let unit = args.get(2).and_then(RuntimeValue::as_str).unwrap_or("day");
            let shifted = shift_iso_date_unit(base, amount, unit)?;
            Ok(if matches!(args[0], RuntimeValue::DateTime(_)) || base.contains('T') {
                if shifted.contains('T') {
                    RuntimeValue::DateTime(shifted)
                } else {
                    RuntimeValue::DateTime(format!("{shifted}T00:00:00Z"))
                }
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
            Ok(RuntimeValue::Integer(diff_iso_dates_unit(left, right, unit)?))
        }
        "dtcs:date_trunc" => {
            if args.len() != 2 {
                return Err("dtcs:date_trunc requires value and unit".into());
            }
            let value = args[0]
                .as_str()
                .ok_or("dtcs:date_trunc requires date/datetime")?;
            let unit = args[1]
                .as_str()
                .ok_or("dtcs:date_trunc unit must be string")?;
            Ok(RuntimeValue::DateTime(trunc_iso_datetime(value, unit)?))
        }
        "dtcs:extract" | "dtcs:date_part" => {
            if args.len() != 2 {
                return Err("dtcs:extract requires unit and date/datetime".into());
            }
            // Accept extract(unit, value) or extract(value, unit).
            let (unit, value) = if args[0].as_str().is_some_and(is_date_unit) {
                (
                    args[0].as_str().unwrap(),
                    args[1].as_str().ok_or("dtcs:extract value must be date string")?,
                )
            } else {
                (
                    args[1].as_str().ok_or("dtcs:extract unit must be string")?,
                    args[0].as_str().ok_or("dtcs:extract value must be date string")?,
                )
            };
            Ok(RuntimeValue::Integer(extract_date_part(value, unit)?))
        }
        "dtcs:at_timezone" => {
            if args.len() != 2 {
                return Err("dtcs:at_timezone requires datetime and fixed offset".into());
            }
            let value = args[0]
                .as_str()
                .ok_or("dtcs:at_timezone requires datetime string")?;
            let offset = args[1]
                .as_str()
                .ok_or("dtcs:at_timezone offset must be string like +00:00")?;
            Ok(RuntimeValue::DateTime(apply_fixed_offset(value, offset)?))
        }
        "dtcs:row_number" | "dtcs:rank" | "dtcs:dense_rank" | "dtcs:lag" | "dtcs:lead"
        | "dtcs:first_value" | "dtcs:last_value" => Err(format!(
            "{callee} must be used with dtcs:window (not as a scalar call)"
        )),
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

fn shift_iso_date_unit(value: &str, amount: i64, unit: &str) -> Result<String, String> {
    match unit {
        "day" | "days" => shift_iso_date(value, amount),
        "month" | "months" => {
            let (y, m, d) = parse_ymd(value)?;
            let total = y as i64 * 12 + (m as i64 - 1) + amount;
            let ny = total.div_euclid(12) as i32;
            let nm = (total.rem_euclid(12) as u32) + 1;
            let max_day = days_in_month(ny, nm);
            let nd = d.min(max_day);
            Ok(format!("{ny:04}-{nm:02}-{nd:02}"))
        }
        "year" | "years" => {
            let (y, m, d) = parse_ymd(value)?;
            let ny = y + amount as i32;
            let max_day = days_in_month(ny, m);
            let nd = d.min(max_day);
            Ok(format!("{ny:04}-{m:02}-{nd:02}"))
        }
        "hour" | "hours" | "minute" | "minutes" => {
            let (date, hour, minute, second, _) = parse_datetime_parts(value)?;
            let (y, m, d) = parse_ymd(&date)?;
            let mut total_minutes =
                days_from_civil(y, m, d) * 24 * 60 + hour as i64 * 60 + minute as i64;
            match unit {
                "hour" | "hours" => total_minutes += amount * 60,
                _ => total_minutes += amount,
            }
            let day = total_minutes.div_euclid(24 * 60);
            let rem = total_minutes.rem_euclid(24 * 60);
            let nh = (rem / 60) as u32;
            let nm = (rem % 60) as u32;
            let (ny, nmo, nd) = civil_from_days(day);
            Ok(format!(
                "{ny:04}-{nmo:02}-{nd:02}T{nh:02}:{nm:02}:{second:02}Z"
            ))
        }
        other => Err(format!(
            "dtcs:date_add unsupported unit '{other}' (supports day/month/year/hour/minute)"
        )),
    }
}

fn diff_iso_dates_unit(left: &str, right: &str, unit: &str) -> Result<i64, String> {
    match unit {
        "day" | "days" => diff_iso_dates(left, right),
        "month" | "months" => {
            let (ly, lm, _) = parse_ymd(left)?;
            let (ry, rm, _) = parse_ymd(right)?;
            Ok((ly as i64 * 12 + lm as i64) - (ry as i64 * 12 + rm as i64))
        }
        "year" | "years" => {
            let (ly, _, _) = parse_ymd(left)?;
            let (ry, _, _) = parse_ymd(right)?;
            Ok((ly - ry) as i64)
        }
        "hour" | "hours" => Ok(diff_iso_dates(left, right)? * 24),
        "minute" | "minutes" => Ok(diff_iso_dates(left, right)? * 24 * 60),
        other => Err(format!(
            "dtcs:date_diff unsupported unit '{other}' (supports day/month/year/hour/minute)"
        )),
    }
}

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(y) => 29,
        2 => 28,
        _ => 30,
    }
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

fn is_date_unit(unit: &str) -> bool {
    matches!(
        unit,
        "year"
            | "years"
            | "month"
            | "months"
            | "day"
            | "days"
            | "hour"
            | "hours"
            | "minute"
            | "minutes"
            | "second"
            | "seconds"
    )
}

fn parse_datetime_parts(value: &str) -> Result<(String, u32, u32, u32, i32), String> {
    let (date, rest) = match value.split_once('T') {
        Some((d, r)) => (d.to_string(), r),
        None => return Ok((value.to_string(), 0, 0, 0, 0)),
    };
    let rest = rest.trim_end_matches('Z');
    let (time, offset_minutes) = if let Some((t, off)) = rest.split_once('+') {
        (t, parse_offset_minutes(&format!("+{off}"))?)
    } else if let Some((t, off)) = rest.split_once('-') {
        // Ambiguous with date; only treat as offset when time already has ':'
        if t.contains(':') {
            (t, parse_offset_minutes(&format!("-{off}"))?)
        } else {
            (rest, 0)
        }
    } else {
        (rest, 0)
    };
    let parts: Vec<_> = time.split(':').collect();
    if parts.len() < 2 {
        return Err(format!("invalid datetime '{value}'"));
    }
    let hour: u32 = parts[0]
        .parse()
        .map_err(|_| format!("invalid hour in '{value}'"))?;
    let minute: u32 = parts[1]
        .parse()
        .map_err(|_| format!("invalid minute in '{value}'"))?;
    let second: u32 = parts
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    Ok((date, hour, minute, second, offset_minutes))
}

fn parse_offset_minutes(offset: &str) -> Result<i32, String> {
    let trimmed = offset.trim();
    if trimmed == "Z" || trimmed == "+00:00" || trimmed == "-00:00" || trimmed.is_empty() {
        return Ok(0);
    }
    let sign = if trimmed.starts_with('-') { -1 } else { 1 };
    let body = trimmed.trim_start_matches(['+', '-']);
    let parts: Vec<_> = body.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "unsupported timezone '{offset}' (reference supports fixed offsets like +00:00)"
        ));
    }
    let hours: i32 = parts[0]
        .parse()
        .map_err(|_| format!("invalid timezone offset '{offset}'"))?;
    let minutes: i32 = parts[1]
        .parse()
        .map_err(|_| format!("invalid timezone offset '{offset}'"))?;
    Ok(sign * (hours * 60 + minutes))
}

fn trunc_iso_datetime(value: &str, unit: &str) -> Result<String, String> {
    let (date, hour, minute, second, _) = parse_datetime_parts(value)?;
    let (y, m, d) = parse_ymd(&date)?;
    Ok(match unit {
        "year" | "years" => format!("{y:04}-01-01T00:00:00Z"),
        "month" | "months" => format!("{y:04}-{m:02}-01T00:00:00Z"),
        "day" | "days" => format!("{y:04}-{m:02}-{d:02}T00:00:00Z"),
        "hour" | "hours" => format!("{y:04}-{m:02}-{d:02}T{hour:02}:00:00Z"),
        "minute" | "minutes" => format!("{y:04}-{m:02}-{d:02}T{hour:02}:{minute:02}:00Z"),
        "second" | "seconds" => {
            format!("{y:04}-{m:02}-{d:02}T{hour:02}:{minute:02}:{second:02}Z")
        }
        other => Err(format!("dtcs:date_trunc unsupported unit '{other}'"))?,
    })
}

fn extract_date_part(value: &str, unit: &str) -> Result<i64, String> {
    let (date, hour, minute, second, _) = parse_datetime_parts(value)?;
    let (y, m, d) = parse_ymd(&date)?;
    Ok(match unit {
        "year" | "years" => y as i64,
        "month" | "months" => m as i64,
        "day" | "days" => d as i64,
        "hour" | "hours" => hour as i64,
        "minute" | "minutes" => minute as i64,
        "second" | "seconds" => second as i64,
        other => return Err(format!("dtcs:extract unsupported unit '{other}'")),
    })
}

fn apply_fixed_offset(value: &str, offset: &str) -> Result<String, String> {
    let offset_minutes = parse_offset_minutes(offset)?;
    let (date, hour, minute, second, current_offset) = parse_datetime_parts(value)?;
    let (y, m, d) = parse_ymd(&date)?;
    let mut total_minutes =
        days_from_civil(y, m, d) * 24 * 60 + hour as i64 * 60 + minute as i64 - current_offset as i64;
    total_minutes += offset_minutes as i64;
    let day = total_minutes.div_euclid(24 * 60);
    let rem = total_minutes.rem_euclid(24 * 60);
    let nh = (rem / 60) as u32;
    let nm = (rem % 60) as u32;
    let (ny, nmo, nd) = civil_from_days(day);
    let sign = if offset_minutes >= 0 { '+' } else { '-' };
    let abs = offset_minutes.abs();
    Ok(format!(
        "{ny:04}-{nmo:02}-{nd:02}T{nh:02}:{nm:02}:{second:02}{sign}{:02}:{:02}",
        abs / 60,
        abs % 60
    ))
}

fn eval_between(
    value: &RuntimeValue,
    lo: &RuntimeValue,
    hi: &RuntimeValue,
) -> Result<RuntimeValue, String> {
    if value.is_null()
        || value.is_missing()
        || lo.is_null()
        || lo.is_missing()
        || hi.is_null()
        || hi.is_missing()
    {
        return Ok(RuntimeValue::Null);
    }
    if value.is_invalid() || lo.is_invalid() || hi.is_invalid() {
        return Ok(RuntimeValue::Null);
    }
    let ge_lo = compare_ordered_values(value, lo)?;
    let le_hi = compare_ordered_values(hi, value)?;
    Ok(RuntimeValue::Boolean(
        ge_lo != std::cmp::Ordering::Less && le_hi != std::cmp::Ordering::Less,
    ))
}

fn compare_ordered_values(
    left: &RuntimeValue,
    right: &RuntimeValue,
) -> Result<std::cmp::Ordering, String> {
    match (left, right) {
        (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(a.cmp(b)),
        (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) => a
            .partial_cmp(b)
            .ok_or_else(|| "decimal comparison failed".to_string()),
        (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) => (*a as f64)
            .partial_cmp(b)
            .ok_or_else(|| "decimal comparison failed".to_string()),
        (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) => a
            .partial_cmp(&(*b as f64))
            .ok_or_else(|| "decimal comparison failed".to_string()),
        (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(a.cmp(b)),
        (RuntimeValue::Date(a), RuntimeValue::Date(b))
        | (RuntimeValue::DateTime(a), RuntimeValue::DateTime(b))
        | (RuntimeValue::Date(a), RuntimeValue::DateTime(b))
        | (RuntimeValue::DateTime(a), RuntimeValue::Date(b)) => Ok(a.cmp(b)),
        _ => Err("between comparison type mismatch".into()),
    }
}

fn eval_field_access(
    container: &RuntimeValue,
    field: &RuntimeValue,
) -> Result<RuntimeValue, String> {
    let name = field
        .as_str()
        .ok_or("dtcs:field name must be a string")?;
    match container {
        RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
        RuntimeValue::Map(map) => Ok(map
            .get(name)
            .cloned()
            .unwrap_or_else(RuntimeValue::missing)),
        other => Err(format!("dtcs:field requires map/object, got {other:?}")),
    }
}

fn eval_index_access(
    container: &RuntimeValue,
    index: &RuntimeValue,
    null_on_oob: bool,
) -> Result<RuntimeValue, String> {
    match container {
        RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
        RuntimeValue::List(items) => {
            let idx = index
                .as_integer()
                .ok_or("list index must be an integer")?;
            if idx < 0 {
                return if null_on_oob {
                    Ok(RuntimeValue::Null)
                } else {
                    Err("list index must be non-negative".into())
                };
            }
            match items.get(idx as usize) {
                Some(v) => Ok(v.clone()),
                None if null_on_oob => Ok(RuntimeValue::Null),
                None => Err(format!("list index {idx} out of bounds")),
            }
        }
        RuntimeValue::Map(map) => {
            let key = index.as_str().ok_or("map index/key must be a string")?;
            match map.get(key) {
                Some(v) => Ok(v.clone()),
                None if null_on_oob => Ok(RuntimeValue::Null),
                None => Ok(RuntimeValue::missing()),
            }
        }
        other => Err(format!(
            "index/element_at requires list or map, got {other:?}"
        )),
    }
}
